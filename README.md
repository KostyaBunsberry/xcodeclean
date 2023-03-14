
# xcodeclean

A simple CLI to clean Xcode /DerivedData and /Archives folders.

⚠️ WARNING! This CLI does delete ALL directories in folders /DerivedData and /Archives in the default Developer directory or custom one you specified. Use carefully and at your own risk.




## Installation

You can run this binary on any platform but there is no point without Xcode, so here is how to install xcodeclean with homebrew

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
Specify custom Xcode path (in a directory or explicitly)
```bash
  xcodeclean --set-custom-path /Users/awesomename/Developer/Xcode
  xcodeclean --set-custom-path {pwd}
```
Remove custom Xcode path and set to default
```bash
  xcodeclean --set-default-path
```
Options
```bash
  xcodeclean --help
  Usage: xcodeclean [OPTIONS]

  Options:
    -d                                   Should delete Xcode /DerivedData contents
    -a                                   Should delete Xcode /Archives contents
        --set-custom-path <CUSTOM_PATH>  Sets custom path to Xcode folder (saves between sessions), use pwd in your Xcode folder to set this argument
        --set-default-path               Resets Xcode path to default
    -h, --help                           Print help
    -V, --version                        Print version                     Print version
```

