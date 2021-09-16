# RFA - Rust File Analyzer
Generate JSON or CSV reports of file metadata
* * * *
**Disclaimer:** This was originally a project I created for a freshman computer science course to learn Rust and demonstrate the concepts from the class. I am re-using it as a codebase for a seperate project. In it's current state it is only intended for my personal use.
This project is only tested on Ubuntu 21.04, using Rust's stable branch. I previously used it on macOS 10.15 but this is not tested.
* * * *

## Usage:

The reccommend way to use RFA is to supply a folder or file as an argument, i.e.
```bash
rfa                 # Menu mode
rfa /tmp/file       # Asked for output format
rfa -c samples/     # Specify CSV, scan samples folder
rfa -j subject.exe  # Specify JSON, scan individual file
```

