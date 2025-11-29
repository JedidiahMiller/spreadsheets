# Spreadsheets

Pardon the lack of Git history. This project was originally built into a larger repo before being pulled out. 

## Running

The easiest way to get this running is by simply cloning and using `cargo run` in a dedicated terminal window. This assumes you have Rust/Cargo set up on your system.

## Important user notes:

- Do not resize the terminal window while using spreadheet
- Numbers are mostly stored in i32 form. This may create some issues if you attempt to use numbers that are too large.
- To exit, press q while not in edit mode. Hopefully we won't pull a vim and trap you.
- There is a minimum terminal size requirement. It will let you know if the window is too small.
