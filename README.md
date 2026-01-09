# Rust Shell

A Unix-like shell written in Rust, built as part of the CodeCrafters Shell challenge and extended beyond the basics to cover real shell behavior.

This project focuses on understanding how shells actually work under the hood, not just spawning processes and calling it a day.

## Features

The shell supports a substantial subset of POSIX-style shell behavior:

* Command execution using `fork` and `exec`
* Built-in commands like `cd`, `pwd`, `exit`, and `history`
* Input and output redirection using `<`, `>`, and `>>`
* Command piping with `|`
* Quoting support for single and double quotes
* Command history with navigation
* Tab-based autocompletion for commands and paths
* Proper error handling and exit codes

In short, it behaves like a shell you would not immediately rage-quit.

## Why this exists

Shells sit at the boundary between humans and the kernel. Writing one forces you to understand:

* Process creation and lifecycle
* File descriptors and IO redirection
* Parsing and tokenization
* Unix conventions that everyone uses but few explain clearly

Rust adds an extra layer of discipline. You get memory safety while still doing very un-safety-adjacent things.

## Architecture overview

* A tokenizer that handles whitespace, quotes, and operators correctly
* A parser that builds execution plans for pipelines and redirections
* A command executor that manages processes and file descriptors
* Built-in command handling without spawning unnecessary processes
* History and completion layers that integrate cleanly with the REPL loop

The design favors clarity over cleverness. Debuggable beats magical.

## Getting started

### Prerequisites

* Rust (stable)
* A Unix-like OS. Linux or macOS recommended

### Build and run

```bash
git clone <repo-url>
cd rust-shell
cargo build
cargo run
```

You will land in the shell prompt. From there, use it like a normal shell and try to break it.

## Examples

```bash
ls | grep src > files.txt
cat "files.txt"
echo hello world >> log.txt
```

If you do something illegal, it complains politely.

## Known limitations

* Not a full POSIX shell
* No job control yet
* No environment variable expansion beyond basics

These are deliberate omissions, not accidents.

## What I learned

* Writing a shell is mostly about edge cases
* Parsing is harder than execution
* Unix APIs are sharp tools that reward respect
* Rust is very good at preventing “oops” moments when dealing with low-level systems code

## Credits

Built as part of the CodeCrafters Shell challenge, with all extensions completed.

## License

MIT
