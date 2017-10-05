# notem

Super simple note taking system. Stop adding text files to your Desktop. Start
writing plain text notes with one command in the terminal:

```sh
$ notem goal planning
# Opens ~/notes/2017-10-04-goal-planning in your default editor.
```

List all notes:

```sh
$ notem --list
```

Search for a note by its title with grep:

```sh
$ notem --search goal-planning
```

## Install

With Homebrew (recommended):

```sh
$ brew install notem
```

The default editor is `vi`. To change this update your shell's EDITOR
environment variable:

```sh
export EDITOR="/usr/bin/nano"
```
