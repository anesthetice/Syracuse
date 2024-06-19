## Syracuse

A cross-platform, cli-application written in rust meant to track and analyse your daily productivity.

### Example
```
syr graph --days 14
```

Clean old-fashioned linear interpolation
![with linear interpolation](https://private-user-images.githubusercontent.com/118751106/341134433-24810497-b03b-4b7d-8c19-3820473f4194.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MTg4MTE3OTksIm5iZiI6MTcxODgxMTQ5OSwicGF0aCI6Ii8xMTg3NTExMDYvMzQxMTM0NDMzLTI0ODEwNDk3LWIwM2ItNGI3ZC04YzE5LTM4MjA0NzNmNDE5NC5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjQwNjE5JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI0MDYxOVQxNTM4MTlaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT1hY2I2MmVkYThhNzNlMGY1ZDRmMmUwMzEyNzA3OTAwOWFmYjNhZmQ5ODhiZWEwNDc5ZjFlYjhkMDhkMTBmYjM3JlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCZhY3Rvcl9pZD0wJmtleV9pZD0wJnJlcG9faWQ9MCJ9.XSS_4RKYslKgrv56XNeGhrTPM76li6tHb-2ZDTdy8as)


Makima interpolation (modified akima) is also available, keep in mind that makima can overshoot unlike linear, I might implement pchip in a future release
![with makima interpolation](https://private-user-images.githubusercontent.com/118751106/341134384-46e8d934-efe3-4a84-8c29-edaf6fe3a84b.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MTg4MTE3OTksIm5iZiI6MTcxODgxMTQ5OSwicGF0aCI6Ii8xMTg3NTExMDYvMzQxMTM0Mzg0LTQ2ZThkOTM0LWVmZTMtNGE4NC04YzI5LWVkYWY2ZmUzYTg0Yi5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjQwNjE5JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI0MDYxOVQxNTM4MTlaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT0wNjZjYjg5OWFlMDRjMzEzYmQwNjcyM2EwYTczMjRhZTNmZTQwMzlhNTBhM2ZiNDhkMjQ2MmVmYmRkZTIzOWJjJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCZhY3Rvcl9pZD0wJmtleV9pZD0wJnJlcG9faWQ9MCJ9.SfK0w-KM5__p3ERenvM55wVBIY4-l5yuzmpnU9wwdME)

```
syr add MATH-251, numerical_analysis_2
syr s math
syr today --explicit
```

String matching is done using the Needleman-Wunsch and Smith-Waterman algorithms, making starting a stopwatch for the right entry a breeze

### Build

```
git clone https://github.com/anesthetice/Syracuse.git
cd Syracuse
cargo build --release
```

### Installation

* follow the build instructions or download a pre-compiled binary from the releases
* place the binary file into the directory of your choice
* (on windows) add the directory where syracuse is stored to your PATH env variables
* (on linux) add a method to call syracuse in the shell of your choice

example for bash:
```
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

once installed simply run the binary with the --help flag for more info

### Configuring

see config.rs to view all options

### Version 2

Main improvements

* modular entry system                                                  ✓
* increased stability, no mem::transmute                                ✓
* Smith-Waterman + Needleman-Wunsch algorithms for string matching      ✓
* makima interpolation for graphs                                       ✓
* more graphing options                                                 ✓
* proper directory usage                                                ✓
* new subcommands, "today", "backup", "prune"                           ✓