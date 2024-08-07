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
Usage: syracuse [OPTIONS] [COMMAND]

Commands:
  add      Add a new entry to syracuse
  list     List out all entries
  remove   Remove a single entry
  start    Start the daily stopwatch for an entry
  update   Manually update the time of an entry
  today    Display the time tracked today
  backup   Backup entries
  unindex  Unindexes a specified entry
  reindex  Reindexes a specified entry
  sum      Sums up the time tracked by entries
  prune    Keeps only the blocs younger than a certain date old
  graph    Creates a graph
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
  -h, --help     Print help
  -V, --version  Print version
```

### Configuring

When running syracuse for the first time, a default configuration file will be created.

```
{
  // should info statements be printed
  "debug": false,

  // what set of characters separate the names of an entry stored as a file
  "entry_file_name_separtor": "-·-",

  // how often should progress be automatically saved in seconds
  "autosave_period": 30,

  // local utc offset to get accurate dates [HH, MM, SS]
  // e.g. western europe : [1,0,0] or [2,0,0] generally depending on daylight saving time
  // you will have to manually change the config to account for changes in your timezone
  "local_offset": [
    0,
    0,
    0
  ],

  // default backup path
  "backup_path": "",

  // when starting a stopwatch for a given entry, should the initial time be displayed?
  "stopwatch_explicit": false,

  // by how many hours should the day be extended after midnight
  // e.g. 2 -> timers started until 2 a.m. on a given day will count towards the previous day
  // useful for night owls
  "night_owl_hour_extension": 0,

  // minimum threshold for entries to be considered as options to present to the user
  "search_threshold": 0.0,

  // Smith-Waterman and Needleman-Wunsch algorithm weight, SW is geared for local matches while NW global matching
  "sw_nw_ratio": 0.5,

  // score table
  "match_score": 2,
  "mismatch_penalty": -1,
  "gap_penalty": -1,

  // for the animation that plays when a stopwatch is running
  "frame_period": 150,

  // the animation played, first string in each array is for the left side, and second one for the right
  "animation": [
    [
      "|  ",
      "  |"
    ],
    [
      "/  ",
      "  /"
    ],
    [
      "-  ",
      "  -"
    ],
    [
      "\\  ",
      "  \\"
    ]
  ],

  // where graph.png will be created
  "graph_output_dir": "",

  // interpolation method, Linear or Makima, keep in mind that Makima can overshoot unlike Linear
  "graph_interpolation_method": "Linear",

  // misc. graphing options
  "graph_nb_interpolated_points": 1500,
  "graph_marker_size": 6,
  "graph_background_rgb": [...],
  "graph_foreground_rgb": [...],
  "graph_coarse_grid_rgb": [...],
  "graph_fine_grid_rgb": [...],
  "graph_sum_line_rgb": [...],
  "graph_marker_rgb": [...]
}
```



### Version 2

Main improvements

* modular entry system                                                  ✓
* increased stability, no mem::transmute                                ✓
* Smith-Waterman + Needleman-Wunsch algorithms for string matching      ✓
* makima interpolation for graphs                                       ✓
* more graphing options                                                 ✓
* proper directory usage                                                ✓
* new subcommands, "today", "backup", "prune"                           ✓
