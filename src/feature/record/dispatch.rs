use anyhow::Result;

use crate::{
    context::Context, 
    cli::RecordArgs,
    feature::record::{
        md::Md,
        executor::{
            self,
            RecordExecutor, 
            batch::Batch,
            pty::Pty,
        },
    },
};

pub fn run(args: RecordArgs, ctx: &Context) -> Result<()> {
    let (command, command_args) = args.command.split_first().unwrap();

    if args.pty {
        run_with_executor(
            Pty::new(command, command_args.to_vec()),
            args,
            ctx,
        )
    } else {
        run_with_executor(
            Batch::new(command, command_args.to_vec()),
            args,
            ctx,
        )
    }
}

fn run_with_executor<T: RecordExecutor>(
    executor: T,
    args: RecordArgs,
    ctx: &Context,
) -> Result<()> {

    let result = if args.quiet {
        None
    } else {
        Some(String::from_utf8_lossy(&executor.run()?.stdout).to_string())
    };

    let track_name = executor::get_track_name(ctx);

    let md = Md::new(
        args.title,
        args.message,
        args.command,
        executor.mode().to_string(),
        result,
        args.tag,
        track_name.clone(),
    );

    let _ = md.write(ctx);

    if let Some(t) = &track_name {
        executor::print_track_banner("Recorded", t.to_string());
    }

    Ok(())
}
