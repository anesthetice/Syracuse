## Syracuse

A cross-platform, cli-application written in rust meant to track and analyse your productivity

### Build

```
git clone https://github.com/anesthetice/Syracuse.git
cd Syracuse
cargo build --release
```

### Version 2

Here are the improvements I am working on

* modular entry system                                                  ✓
* increased stability, no mem::transmute                                ✓
* Smith-Waterman + Needleman-Wunsch algorithms for string matching      ✓
* makima interpolation for graphs                                       ✓
* more graphing options                                                 ✓
* proper directory usage                                                ✓
* new subcommands, "today", "backup", "prune"                           ✓