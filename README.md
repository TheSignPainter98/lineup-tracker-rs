# A simple lineup-tracker

At its heart, this program takes maps which may have zones and abilities which may have usages and stores a user’s progress towards a target.
Commands are available to update progress and targets as well as to add new maps, zones, abilities and usages.

This is a re-implementation of my other [lineup-tracker][lineup-tracker], but this time written in Rust.

<!-- ![Example progress](https://raw.githubusercontent.com/TheSignPainter98/lineup-tracker-rust/master/img/example.png) -->

<!-- ## Contents -->

<!-- <!-1- vim-markdown-toc GFM -1-> -->

<!-- * [How do I install this?](#how-do-i-install-this) -->
<!-- * [How do I use this?](#how-do-i-use-this) -->
<!--     * [Abbreviations](#abbreviations) -->
<!-- * [What’s the motivation?](#whats-the-motivation) -->
<!-- * [Why does this exist?](#why-does-this-exist) -->
<!-- * [Author and License](#author-and-license) -->

<!-- <!-1- vim-markdown-toc -1-> -->

<!-- ## How do I install this? -->

<!-- The following steps are required. -->

<!-- 1. Install dependencies: -->
<!--     - [GNU Make][make] -->
<!--     - [Moonscript][moonscript] -->
<!--     - [Lua][lua] -->
<!-- 2. Install [Lua YAML][lyaml]: `luarocks install lyaml` -->
<!-- 3. Clone this repo and `cd` into its directory -->
<!-- 4. Run `make`, which will build the binary, `lineup-tracker` -->
<!-- 5. Install. If you’re on: -->
<!--     - Linux or macos—run `sudo make install` -->
<!--     - Windows—copy the binary into somewhere in your path and pray because I haven’t tested this on Windows. -->

<!-- ## How do I use this? -->

<!-- Once `lineup-tracker` is executed, it behaves like a shell. -->
<!-- It must be noted that this shell tracks a ‘query-state,’ which defines what data the user is currently modifying. -->
<!-- The current query state will appear in the shell’s prompt, and can also be checked by typing `state`. -->

<!-- Type `exit`, `quit` or EOF (Ctrl+D on Linux) to exit. -->
<!-- The program saves before exiting, but this can be invoked earlier by using the `Save` command. -->

<!-- Type `help` for more info on which commands are available. -->
<!-- More generally, any command or sub-command with `help` as its first argument will output a help message. -->

<!-- ### Abbreviations -->

<!-- To make this program easier to use, it is possible to shorten commands and arguments. -->

<!-- Command (and sub-command) abbreviations are just the first few letters---the shell will try to match the rest of the command. -->
<!-- For example `n m haven` will expand to `new map haven`, which creates a new map called ‘haven.’ -->

<!-- To abbreviate arguments, numeric indices may be used, to find the list of currently available indices, enter `list`. -->
<!-- Shortened abbreviations may be supported at some point in future. -->

<!-- ## What’s the motivation? -->

<!-- Aim-training makes a good solo-player; util-training makes a good team-player. -->
<!-- I enjoy playing the latter. -->

<!-- ## Why does this exist? -->

<!-- The first version of this used a spreadsheet. -->
<!-- I wanted to make a command-line tool to track my progress instead so I made this. -->

<!-- ## Author and License -->

<!-- This project was created by Ed Jones and is licensed under GPL3. -->

<!-- [make]: https://www.gnu.org/software/make/ -->
<!-- [moonscript]: https://moonscript.org -->
<!-- [lua]: https://www.lua.org -->
<!-- [lyaml]: https://luarocks.org/modules/gvvaughan/lyaml -->
[lineup-tracker]: https://github.com/TheSignPainter98/lineup-tracker
