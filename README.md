# J

--> https://github.com/lumiknit/J2

My jump helper in shell

## Installation

Dependency: sh, rust

Run `sh ./install.sh`.

## Usage

To see usage, type `J`

```sh
J - lumiknit's jump helper
Usage: J <COMMAND> <PATH>
Commands:
  cd,c               - cd to the path
  pushd,p            - pushd to the path
  code,vscode,v      - open the path in vscode
  emacs,e            - open the path in emacs
  open,o             - open the path using open
  clone,cl1,1        - clone the path
  help,*             - show this help
```

Examples:

```sh
J cd hello world     # Fuzzy find hello world and go to best match
J c hello world      # Same one
j hello   world      # j is a shorthand for `J cd`
J p  luaparse        # pushd with fuzzy finding
J v  my awesome repo # open vscode with fuzzyfinding. It requires `code` command in your terminal
J e  hohh            # open emacs. It will launch emacs with `open -a emacs`
J 1 https://github.com/lumiknit/J
  # This clones the repo in `$J_REPOS/github.com/lumiknit/J`
```
