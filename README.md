# notem

A super simple note taking system ✏️. Stop adding text files to your desktop,
start writing plain text notes with one command in the terminal.

## Usage

```sh
$ notem goal planning
# Opens ~/notes/2017-10-04-goal-planning in your default editor.
```

Then your notes are sorted by date and easily searched using notem.

List all notes:

```sh
$ notem --list
```

Search for a note by its title (like [grep][]):

```sh
$ notem --search goal planning
```

Use `notem --help` for a full list of commands.

## Editor

The default editor is determined first by the EDITOR environment variable, with
several fallbacks to common text editors. To change this update your shell's
EDITOR environment variable:

```sh
export EDITOR="/usr/bin/nano"
```

## Notes directory

By default all notes are saved in a folder called `notes` in the home directory.
To change this update the NOTEM_PATH environment variable:

```sh
# Use absolute path
export NOTEM_PATH="/Users/computer-name/my-custom-path"
```

## Install

Notem is written in Rust, so use [Cargo][] to install:

```sh
$ cargo install notem
```

[grep]: https://www.gnu.org/software/grep/manual/grep.html
[Cargo]: https://crates.io/


# Developing

First clone the project, and ensure you have [rustup](https://rustup.rs/) installed. Then run

```sh
make rust-update
```

To run linting and tests use

```sh
make test
```