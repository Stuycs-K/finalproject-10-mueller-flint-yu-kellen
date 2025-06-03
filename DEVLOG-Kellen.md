# Dev Log:

This document must be updated daily every time you finish a work session.

## Kellen Yu

### 2025-05-27 - Researched termion (like ncurses)
Researched the termion library, familiarized myself with rust syntax

### 2025-05-28 - Worked on file functions
Wrote function that converts a file's raw bytes into a hex string

### 2025-05-29 - Worked on file functions and conversions
Wrote function that converts a hex string into a byte vector,
Tested both functions from yesterday and today and verified that it works

### 2025-05-30 - Worked on editor class
Started state machine for various modes (READ/WRITE), started
functions to handle characters passed depending on the mode

### 2025-05-31 - Tried to figure out iterators
Rust is a very new language for me, and I didnt know what 
iterators were. They are pretty cool

### 2025-06-02 - Used iterators to write save function
Splits string into hex bytes then turns it into binary 
and saves it to a file. Save function works

### 2025-06-03 - Added status bar to display current mode
Added status bar, fixed some bugs in formatting.
