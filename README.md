## Syracuse

Syracuse is a simple and well-polished cli application used to keep track of your day to day productivity.

It operates through an entry system, allowing you to start timers, check in and out, unindex old entries, generate graphs, etc. Everything is designed to be as effortless as possible, need a quick overview of your day? Just run `syr today`. What about your entire week? Then run `syr week`. Did you actually need the previous week? Run `syr week -w 1`.

### Example

Create a new entry named 'STAT-110' with 'PROBSTAT' as one of its aliases:
``` bash
syr add STAT-110 PROBSTAT
```

Standard usage:
``` bash
# The name used to search for the entry really doesn't have to be perfect
# Syracuse uses two alignment algorithms to find the best matches
syr cin pstat

# ...

syr cout
```

Create a graph displaying the time tracked in the past 14 days:
``` bash
syr graph --days 14
```

<img src="https://github.com/anesthetice/Syracuse/blob/main/assets/linear_interpolation.png" alt="linear interpolation" width="70%"/>

<img src="https://github.com/anesthetice/Syracuse/blob/main/assets/makima_interpolation.png" alt="makima interpolation" width="70%"/>

### Installation

Syracuse can be installed very simply using cargo:
``` bash
cargo install syracuse
```

### Build

```
git clone https://github.com/anesthetice/Syracuse.git
cd Syracuse
cargo build --release
```

### Usage
```
Usage: syr [COMMAND]

Commands:
  add              Add a new entry to syracuse
  list             List out stored entries
  remove           Remove an entry
  start            Start the daily stopwatch for an entry
  update-add       Manually increase the time tracked by an entry
  update-sub       Manually decrease the time tracked by an entry
  today            Display the time tracked today
  backup           Create a backup of all entries
  unindex          Unindex one or more entries
  reindex          Reindex one or more entries
  sum              Sum up the time tracked by entries
  prune            Discard all blocs that are less recent than the cutoff date
  graph            Graph the time tracked by entries in a given timeframe
  check-in         Check-in an entry
  check-out        Check-out an entry
  week             Display the time tracked this week
  gen-completions  Generate completions for your desired shell
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Configuring

When running Syracuse for the first time, a default config file will be created.
See [config.rs](/src/config.rs) for more info.


## Planned improvements
- [ ] single file for unindexed entries
- [ ] `rename` command (currently you have to just rename the entry file, which can be found in the project directory, refer to [directories](https://docs.rs/directories/6.0.0/directories/struct.ProjectDirs.html) for help)
