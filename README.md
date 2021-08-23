# stump
A lightweight, vaporwave themed replacement for the unix tree command. 

## Installation

### 1. Install a Nerd Font in your Terminal
 - The [nerd font site](https://www.nerdfonts.com/) has a great selection of Nerd Patched monospace fonts. You should use them!
 - This was built and tested with JetBrainsMono Nerd Font ([download link](https://github.com/ryanoasis/nerd-fonts/releases/download/v2.1.0/JetBrainsMono.zip)). No guarantees it works with comic sans but let me know if you try it.
 - Use a terminal emulator that has good utf-8 support! If you want to test yours run the following command and check for broken glyphs:
```
curl https://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-demo.txt | less
```
 - I can recommend kitty (unix based), alacritty (you can build from source), hyper.js (cross-platform), iterm2 (osx).

### 2. Build from source:
 - Install the rust toolchain [here](https://www.rust-lang.org/tools/install) 
 - Clone the repo
 ```
 git clone https://github.com/tjweldon/stump
 ```
 - Build from source in the repo:
 ```
 cargo build --package stump --bin stump
 ```
 - Symlink the binary into your $PATH
 - Congrats, you're stumped!
 
## Usage
 - The default behavior of stump is to recursively list the files and directories in a folder you supply.
 - The screenshot below is (a truncated) view of  its output when run with no args in the project root directory:

![stump in action](https://raw.githubusercontent.com/tjweldon/stump/main/doc/img.png)

 - The `-d` option allows you to control how deepo into the directory tree you want to enumerate. A value of `-d 0` will just list your supplied root dir
 - If no dir argument is supplied, the default is your current working directory.
 - The `-h` option does what you'd expect:

```
myapp 0.1
github.com/tjweldon
A nicer looking version of unix tree

USAGE:
    stump [FLAGS] [OPTIONS] [workingdir]

FLAGS:
    -a, --all        Shows hidden files/folders
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --depth <DEPTH>    Sets the how deep into the directory structure the tree will recurse

ARGS:
    <workingdir>    Specifies the root folder to produce a tree from, defaults
```
    
    
Comments/Feedback are very welcome! This is my second project in rust, having come from PHP/python so I have no idea what I'm doing!

Enjoy!
