# J

My jump helper in shell

## Installation

Dependency: sh, ruby

1. Clone this repository
2. Copy `-J-clone` and `-J-getdir` in some good directory, which of the path specified in `$PATH` ENV.
   - e.g. Put in `~/.local/bin` and put `export PATH="$PATH:$HOME/.local/bin` in your shell profile (`.bashrc`, `.zshrc`, etc.).
3. Copy `J` in some good directory.
4. Put `source /path/to/J` in your shell profile.
5. If you want to change the search directory, put below codes **after `source /path/to/J`** in your shell profile too! (Or, modify `J`)
   ```sh
   export J_HOME="$HOME"  # Your home directory
   export J_REPOS="$HOME/repos"  # Your repos directory
   export J_BASES="$HOME/repos:$HOME/workspaces"  # colon-separated directories to be searched
   ```

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
