use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use std::os::fd::BorrowedFd;

use anyhow::Result;

use nix::poll::{poll, PollFd, PollFlags};
use nix::unistd::Pid;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::sys::signal::{kill, Signal};

use portable_pty::{native_pty_system, CommandBuilder, PtySize, PtyPair};

use termios::*;

use super::{RecordExecutor, ExecResult};

pub struct Pty {
    pub command: String,
    pub args: Vec<String>,
}

impl Pty {
    pub fn new(command: &str, args: Vec<String>) -> Self {
        Self {
            command: command.to_string(),
            args,
        }
    }
}

impl RecordExecutor for Pty {
    fn mode(&self) -> &'static str {
        "pty"
    }

    fn run(&self) -> Result<ExecResult> {

        /* ===== raw mode ===== */
        let stdin_raw = std::io::stdin().as_raw_fd();
        let mut term = Termios::from_fd(stdin_raw).unwrap();
        let orig_term = term.clone();
        cfmakeraw(&mut term);
        tcsetattr(stdin_raw, TCSANOW, &term).unwrap();

        /* ===== PTY ===== */
        let pty = native_pty_system();
        let pair = pty.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        }).unwrap();

        let PtyPair { master, slave } = pair;

        let mut builder = CommandBuilder::new(&self.command);
        builder.args(&self.args);

        let child = slave.spawn_command(builder).unwrap();
        let child_pid_u32 = child.process_id().unwrap();
        let child_pid = Pid::from_raw(child_pid_u32 as i32);

        drop(slave);

        let mut reader = master.try_clone_reader().unwrap();
        let mut writer = master.take_writer().unwrap();

        /* ===== poll 用 fd ===== */
        let stdin_bfd = unsafe { BorrowedFd::borrow_raw(stdin_raw) };
        let pty_raw = master.as_raw_fd().unwrap();
        let pty_bfd = unsafe { BorrowedFd::borrow_raw(pty_raw) };

        /* ===== record ===== */
        let recorded = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
        let mut buf = [0u8; 8192];

        /* ===== poll loop（終了検知込み） ===== */
        loop {
            let mut fds = [
                PollFd::new(&stdin_bfd, PollFlags::POLLIN),
                PollFd::new(&pty_bfd,   PollFlags::POLLIN),
            ];

            let _ = poll(&mut fds, -1).unwrap();

            let stdin_re = fds[0].revents().unwrap_or(PollFlags::empty());
            let pty_re   = fds[1].revents().unwrap_or(PollFlags::empty());

            // 1) HUP/ERR/NVAL
            if pty_re.intersects(PollFlags::POLLHUP | PollFlags::POLLERR | PollFlags::POLLNVAL) {
                break;
            }

            // 2) 子の終了
            match waitpid(Some(child_pid), Some(WaitPidFlag::WNOHANG)) {
                Ok(WaitStatus::StillAlive) => {}
                Ok(_) => break,
                Err(_) => break,
            }

            // stdin -> PTY
            if stdin_re.contains(PollFlags::POLLIN) {
                let n = match std::io::stdin().read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };
                if writer.write_all(&buf[..n]).is_err() {
                    break;
                }
                let _ = writer.flush();
            }

            // PTY -> stdout (+ record)
            if pty_re.contains(PollFlags::POLLIN) {
                let n = match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };
                let s = String::from_utf8_lossy(&buf[..n]);
                print!("{}", s);
                let _ = std::io::stdout().flush();
                recorded.lock().unwrap().extend_from_slice(&buf[..n]);
            }
        }

        /* ===== cleanup ===== */
        let _ = kill(child_pid, Signal::SIGTERM);
        let _ = waitpid(Some(child_pid), None);

        drop(writer);
        drop(master);

        let _ = tcsetattr(stdin_raw, TCSANOW, &orig_term);

        let mut out = recorded.lock().unwrap().clone();
        out.retain(|&b| b != b'\r');
        out.retain(|&b| b != 0x07);

        Ok(ExecResult {
            stdout: strip_ansi(out),
        })
    }
}

fn strip_ansi(v: Vec<u8>) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len());
    let mut i = 0;

    while i < v.len() {
        if v[i] == 0x1b {
            i += 1;
            if i >= v.len() { break; }

            match v[i] {
                // CSI
                b'[' => {
                    i += 1;
                    while i < v.len() && !(0x40..=0x7e).contains(&v[i]) {
                        i += 1;
                    }
                    i += 1;
                }

                // OSC
                b']' => {
                    i += 1;
                    while i < v.len() && v[i] != 0x07 {
                        i += 1;
                    }
                    i += 1;
                }

                // Charset designation (ESC ( B / ESC ) B)
                b'(' | b')' => {
                    i += 2; // ★これが本質
                }

                // その他の単発 ESC
                _ => {
                    i += 1;
                }
            }
        } else {
            out.push(v[i]);
            i += 1;
        }
    }

    out
}
