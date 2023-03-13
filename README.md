
# xcodeclean

A simple CLI to clean Xcode /DerivedData and /Archives folders.

⚠️ WARNING! This CLI does delete ALL directories in folders /DerivedData and /Archives in the default Developer directory or custom one you specified. Use carefully and at your own risk.




## Installation

Install my-project with npm

```bash
  brew tap kostyabunsberry/tap
  brew install xcodeclean
```

or

```bash
  brew install kostyabunsberry/tap/xcodeclean
```
    
## Usage
Delete DerivedData (-d) or Archives (-a)
```bash
  xcodeclean -d -a
```
Specify custom Developer path (in a directory or explicitly)
```bash
  xcodeclean --set-custom-path /Users/awesomename/Developer
  xcodeclean --set-custom-path {pwd}
```
Remove custom Developer path and set to default
```bash
  xcodeclean --set-default-path
```
Options
```bash
  xcodeclean --help
  Options:
  -d                                   Should the program delete Xcode derived data
  -a                                   Should the program delete Xcode archives
      --set-custom-path <CUSTOM_PATH>  Sets custom path to Developer folder (saves between sessions), use pwd in your Developer folder to set this argument
      --set-default-path               Resets path to default
  -h, --help                           Print help
  -V, --version                        Print version
```

