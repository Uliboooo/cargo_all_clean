# cargoAllCleanGo

Recursively finds and cleans Rust projects (containing `Cargo.toml`) starting from a specified directory.

## Usage

Run the executable by providing a command and optionally a root directory path. If no path is provided, it defaults to the current working directory.

```bash
cac <command> [/path/to/projects]
```

### Commands

- `c`: Runs `cargo clean` in all found projects.
- `b`: Runs `cargo build` in all found projects.
- Any other string (e.g., `r`): Runs `cargo build --release` in all found projects.

### Example

```bash
cac c ~/Develop
```

```bash
~/Develop
❯ diskus .
17.24 GB (17,235,255,296 bytes)

~/Develop
❯ cac c
run as "clean"
found these fodlers
~/Develop/correspondene_quoridor
~/Develop/totuzen_no_shi_bot
~/Develop/lifi
~/Develop/histstat
~/Develop/fork_notes
~/Develop/rust_playground
~/Develop/ghost_git_writer
~/Develop/zed
~/Develop/local_issues_gui/Ramifi/src-tauri
~/Develop/totuzen_no_shi_bot_bk_bk
~/Develop/hyprshell
~/Develop/practice
~/Develop/local_issues_lib
~/Develop/cargo_all_clean
ok?(y/n)
y
cleaned
     Removed 3851 files, 1.4GiB total
     Removed 3206 files, 1.4GiB total
     Removed 1550 files, 853.4MiB total
     Removed 408 files, 123.9MiB total
     Removed 4225 files, 2.3GiB total
     Removed 44 files, 9.4MiB total
     Removed 3019 files, 1.1GiB total
     Removed 9097 files, 2.5GiB total
     Removed 6415 files, 2.8GiB total
     Removed 3245 files, 1.5GiB total
     Removed 7245 files, 3.5GiB total
     Removed 113 files, 42.6MiB total
     Removed 511 files, 183.1MiB total
     Removed 485 files, 97.1MiB total

~/Develop
❯ diskus .
956.78 MB (956,776,448 bytes)
```

## Build

```bash
git clone https://github.com/Uliboooo/cargo_all_clean
cargo build --release
```

