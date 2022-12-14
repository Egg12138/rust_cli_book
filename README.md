我将CLI book的仓库直接push到我的这个仓库里，到时的改动都在这里更新

原本的CLI book的仓库的README见下页……

---

# Command-Line Rust: A Project-Based Primer for Writing Rust CLIs

This is the code repository for the O'Reilly book [_Command-Line Rust_](https://learning.oreilly.com/library/view/command-line-rust/9781098109424/) (ISBN 9781098109417) by Ken Youens-Clark.

For several consecutive years, Rust has been voted "most loved programming language" in Stack Overflow's annual developer survey. This open source systems programming language is now used for everything from game engines and operating systems to browser components and virtual reality simulation engines. But Rust is also an incredibly complex language with a notoriously difficult learning curve.

Rather than focus on the language as a whole, this guide teaches Rust using a single small, complete, focused program in each chapter. Author Ken Youens-Clark shows you how to start, write, and test each of these programs to create a finished product. You'll learn how to handle errors in Rust, read and write files, and use regular expressions, Rust types, structs, and more.

Discover how to:

* Use Rust's standard libraries and data types to create command-line programs
* Write and test Rust programs and functions
* Read and write files, including stdin, stdout, and stderr
* Document and validate command-line arguments
* Write programs that fail gracefully
* Parse raw and delimited text
* Use and control randomness


# NOTES for packages

## 01: hello:

`assert_cmd.rs`, easy for cli testing.有一些比较方便的：
`assert_cmd::Command::cargo_bin(nameof_bin)`, easy for specific the binary to be tested

`man true` and `man false` will help. 

## 02: 
`clap`需要启动`features = ["derive]`派生宏才可以有`#[derive(Parser)]`





