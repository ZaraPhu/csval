# Introduction
Hi there, my name is Zara! This CLI tool ```csval``` is used to validate the checksum of a file using the checksum you've obtained from the file's owner. This is often done to make sure that the file you've received has not been tampered with by any unauthorized persons. It's built using Rust, and currently only has an implementation for checking sha256 sums.
 \
This is meant to be a learning project, and not a maintained tool or program. If you wish to expand on it, please fork this repository to make changes and create a pull request so that I can look it over and approve it.

# Installation
So far, I do not have an installation script nor a pre-built binary available through any package manager. I intend to add this soon.

# Build Instructions
To be able to build this project yourself, all you need is the ```rustc``` compiler and/or the ```cargo``` toolchain. Depending on your OS, you can obtain these through your package manager (Homebrew for MacOS, apt/dnf/pacman for Linux distros) or through the installation available on the [Rust website](https://www.rust-lang.org/). 

Clone this repository to your location of choice on your computer using 
```
git clone https://github.com/ZaraPhu/csval.git
```
and navigate to the root directory of the project files
```
cd csval
```
and compile the program using
```
cargo build --release
```
 \
The compiled program will be stored inside target/release folder inside the project files.

The compiled program can be run by navigating into that folder
```
cd target/release
```
and run the script using
```
./csval [FILE TO CHECK] [CHECKSUM VALUE]
```
Enjoy!
