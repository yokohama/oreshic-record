# OreshicRecord

## Overview

OreshicRecord is a CLI-based knowledge management tool inspired by the concept of the Akashic Records.  
It is designed for engineers who work primarily in the Linux terminal environment.  
PowerShell support is planned as a future extension.

The name comes from "Ore" (俺, meaning "I" in Japanese) + Record.

Let's  Keep happy hacking and Keep growing forever!!

---

## Highlights

### Knowledge Accumulation

Record commands and notes seamlessly from the CLI.

- **command**  
  Store daily investigation and operational commands as reproducible logs.

- **track**  
  Associate logs with a current working task (track).

- **writeup**  
  Manage free-form writeups with full-text search and listing.

Example:

```sh
ors record -t "Run ping 3 times" ping -c 3 8.8.8.8
```

---

### Knowledge Utilization

Extract and execute stored commands using search options.

Example:

```sh
ors search query ping          // search by keyword
ors search query ping 1        // show details of entry 1
ors search query ping 1 --run  // execute entry 1
```

---

## Install

### Requirements

- Rust (https://rustup.rs)

---

### Install from source

```bash
git clone https://github.com/yokohama/OreshicRecord.git
cd OreshicRecord
cargo install --path .
```

After installation, the binary will be located at:

```
$HOME/.cargo/bin
```

---

### Add Cargo bin to PATH

If the `ors` command is not found, add it to your PATH.

#### For bash

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### For zsh

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

Verify installation:

```bash
which ors
```

---

## Setting

### Environment Variables

```bash
export ORS_RECORDS_DIR=/path/to/your/records
export EDITOR=nvim
export MD_VIEWER=glow
```

---

### Prepare Directory Structure

```bash
mkdir -p $ORS_RECORDS_DIR/commands
mkdir -p $ORS_RECORDS_DIR/tracks
mkdir -p $ORS_RECORDS_DIR/writeups
```

Final structure:

```
$ORS_RECORDS_DIR/
  commands/
  tracks/
  writeups/
```

---

## First Run

```bash
ors --help
```

Example output:

```
Usage: ors <COMMAND>

Commands:
  record
  set
  unset
  search
  help

Options:
  -h, --help
  -V, --version
```
