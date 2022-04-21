# Learn Rust in 10 Projects

Well, hey. Here's another of those "learn x in y" things!

I've been wanting to learn Rust for quite some time now, but I've been struggling to find ways to get started. Like yeah, I could just go to the Rust Docs and start with their "Hello World" programs, but that's so slow compared to what I could achieve in the same amount of time - not to mention, I tried their quick cookbook-style guide, and gee, wasn't that confusing as well.

Then all of a sudden, I stumbled upon [this article by Fasterthanlime](https://fasterthanli.me/articles/a-half-hour-to-learn-rust), and I started reading. Then read some more. Then some more. Then things started clicking. And now I think I'm in a good spot to actually start trying.

And what better way to start learning a new language than to rewrite some other projects into the new language?! I have [soooo many Javascript projects](https://github.com/TheBrenny?tab=repositories&q=&type=source&language=javascript&sort=name) that I can convert, so my plan is to do that, and hopefully get some useful ports made in the process. Here are the projects I plan on porting (and what they aim to achieve), in order from what I expect to be easiest to hardest

1. https://github.com/TheBrenny/node-alias-tools ❌
   - CLI Apps, Mutables, Anon Funcs, Loops, (basics)
2. https://github.com/TheBrenny/big-kahuna ❌
   - Library, Argument parsing
3. https://github.com/TheBrenny/mini-word-smith ❌
   - Library/CLI Combo, read/write temporary files
4. https://github.com/TheBrenny/hosts-etc ❌
   - Library, Filesystem interaction with privileged files
5. https://github.com/TheBrenny/hosts-etc-cli ❌
   - CLI App, imports a library
6. https://github.com/TheBrenny/transposition-cracker ❌
   - CLI App, (Realistic use-case?)
7. https://github.com/TheBrenny/sql-cli-repl ❌
   - CLI App, Realistic, DB calls, Dynamic import of libs
8. https://github.com/TheBrenny/scetch ❌
   - Library, Complex, RegEx, Async, Mutables
9. https://github.com/TheBrenny/timer.js ❌
   - Application (Tauri), simple web app wrapper
10. https://github.com/TheBrenny/Souard ❌
    - Application (Tauri), make music and potentially record/save it

Bonus projects:

11. https://github.com/TheBrenny/Auvis ❌
    - Application (Tauri), WASAPI(?) and message passing
12. A personal Crate Index *(from scratch)* ❌
    - Web Server, Scaleable, Deployable, Etc


The projects are numbered from 1 to 10 in this repo, and they also match with the file structure (`./[xx]-[project name]/`, ie `./08-scetch/`). Each project [whill](https://youtu.be/7ZmqJQ-nc_s) try to match the original as close as possible, however, I will update them as necessary (to try Rust language features, or because the original implementation was bad).

Oh, and the readmes in each package will explain my main lessons and and on how effective the lessons were.

**Lesgo!**