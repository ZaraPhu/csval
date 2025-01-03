# Introduction
Hi there, my name is Zara! This CLI tool ```csval``` is used to validate the checksum of a file using the checksum you've obtained from the file's owner. This is often done to make sure that the file you've received has not been tampered with by any unauthorized persons. It's built using Rust, and currently has implementation for both sha256 sums and md5 sums.\
 \
This is meant to be a learning project, and not a maintained tool or program. It is not currently open for contribution.

# Build Instructions
To build this project, you need the ```make``` and ```cargo``` tools. Begin by cloning this repository to your local device using
```
git clone https://github.com/ZaraPhu/csval.git
```
Then navigate into the project root directory using 
```
cd csval
```
The package can be built and installed by running 
```
make install
```
which will build the program and copy it into your /usr/local/bin directory. If you would rather install it in a different location, the PREFIX variable in the Makefile can be altered. But if you want to call ```csval``` from any location, then append that installation directory to the PATH variable.\
 \
After the installation process is complete, you can use the program by calling 
```
csval [file_path] [FLAGS]
```
# Examples
An example of how this program can be used is checking the sums of the LICENSE file in the project directory
```
csval LICENSE --sha256 3972dc9744f6499f0f9b2dbf76696f2ae7ad8af9b23dde66d6af86c9dfb36986 --md5 1ebbd3e34237af26da5dc08a4e440464
```
and the expected output is
```
Md5 sums: Match.
SHA256 sums: Match.
```
