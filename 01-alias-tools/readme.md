# Alias Tools <small>*(Project 1)*</small>

These tools are a set of 4 individual tools that do certain things:

- `gitup` does the whole set upstream thing when you ceebs writing the whole command. ✅
  - This tool teaches how to run child processes and get their `stdout`.
  - It will also try to pipe the output from a child to the current proc's `stdout`.
  - It also does error handling on some level.
- `math` lets you evaluate mathematical expressions (js version was just a shortcut to a sandboxed `eval`).
  - This tool teaches basic arg handling
  - It also has <u>*potential*</u> to teach about `stdin` and `stdout` at the same time.
  - It'll also be a good exercise in evaluating expressions from strings (because I'll bet Rust doesn't have a built-in `eval`).
- `sizeof` lets you get the size of files/folders (and recursively!) ✅
  - This tool teaches basic fs traversal and stat interaction.
  - It should also be a good exercise into recursive functions in Rust.
  - It'll also hopefully teach me about dealing with the PWD and resolving relative/absolute paths.
- `token` generates random strings conforming to specific rules. ✅
  - This tool will teach basic String manipulation, as well as Randomness in Rust.
  - While the original implementation depends on modules, this will be zero-dep (only built-in modules).
  - This will also start with more typeless arg parsing (varargs, parse to int, etc).

Each project will be in its own subfolder, and pushed to my [custom crate index](../12-custom-crate-index/) (when it's built).