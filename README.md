## Syracuse

A cross-platform, flexible, and easy to use cli-app written in rust meant to keep track of your day to day productivity.

### Example
```
syracuse graph --days 14
```

<img src="https://github.com/anesthetice/Syracuse/blob/main/assets/linear_interpolation.png" alt="linear interpolation" width="70%"/>

<img src="https://github.com/anesthetice/Syracuse/blob/main/assets/makima_interpolation.png" alt="makima interpolation" width="70%"/>

### Build

```
git clone https://github.com/anesthetice/Syracuse.git
cd Syracuse
cargo build --release
```

### Installation

* follow the build instructions or download a pre-compiled binary from the releases
* place the binary file into the directory of your choice
* (on windows) add the directory to your PATH env variables
* (on linux) add a function to call syracuse in the shell of your choice

example for bash:
```
# .bashrc (at the end of the file)
function syr {
    $HOME/Documents/Syracuse/syracuse-x86_64-unkown-linux-gnu $@
}
```

example for fish:
```
# .config/fish/functions/syr.fish
function syr
    $HOME/Documents/Syracuse/syracuse-x86_64-unkown-linux-gnu $argv
end
```

### Usage
```
Usage: syracuse [COMMAND]

Commands:
  add         Add a new entry to syracuse
  list        List out stored entries
  remove      Remove an entry
  start       Start the daily stopwatch for an entry
  update-add  Manually increase the time tracked by an entry
  update-sub  Manually decrease the time tracked by an entry
  today       Display the time tracked today
  backup      Create a backup of all entries
  unindex     Unindexes a specified entry
  reindex     Reindex one or more entries
  sum         Sum up the time tracked by entries
  prune       Discard all blocs that are less recent than the cutoff date
  graph       Graphs the stored entries
  check-in    Check-in an entry
  check-out   Check-out an entry
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Configuring

When running syracuse for the first time, a default configuration file will be created.
See [config.rs](/src/config.rs) for more info.

### Version 2

* Modular entries
* Improved reliability and stability (no unsafe code)
* Smith-Waterman and Needleman-Wunsch algorithms entry queries
* Revamped graphs with interpolation
* More graphing options
* Better directory usage
* New subcommands, "today", "backup", "prune"

### Version 3

* Code clean-up
* Improved visuals
* New subcommands, "check-in", "check-out", "sum", "index", "unindex"
* More that I am probably forgetting
