# Presentation
Flint Mueller, Kellen Yu

## Hex Editor
After doing alot of hex editing in class, we wanted to make an editor specifically for editing files in hex format. We were inspired by terminal-based editors such as Vim and nano in our project, and decided to make an editor similar to these.
We decided to use rust to create this language just to try out something new.

## Raw Mode Terminal
Typically terminal input is first processed by the OS (Things like CTRL-C, arrow keys, are not typed out and cannot be read by stdin). We need to put our terminal into raw mode so that we can read any user input and process it. 

### Termion
We use a rust package called termion, which simplifies handling raw-mode input and writing to the terminal.

## Displaying data as hex


## How to use
### Prerequisites
Make sure to install the latest version of cargo: [download link](https://www.rust-lang.org/tools/install). You can install cargo locally without needing sudo permissions.

### Running the program
To actually run the program, use the makefile:
```make run ARGS=<filename>```
Once you run this, the hex contents of the file will be displayed on the screen.
Inspired by Vim, our editor has 2 states: read, write. The user starts out in read mode, and can use the hjkl characters to move the cursor around. By pressing q, users can exit the program, which automatically saves whatever edits were made to the file.
By pressing i, users can enter write mode. In this mode, users can type in hex. By pressing the escape key, users can return to read mode.
