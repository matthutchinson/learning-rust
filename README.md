# Learning [Rust](https://www.rust-lang.org/en-US/)

Working code examples from [The Rust Programming
Language](https://doc.rust-lang.org/stable/book/), an introductory book about
Rust.

Rather than organise code examples into chapters, the folder names should
give some clue for each topic.

Install Rust with:

    curl https://sh.rustup.rs -sSf | sh
    # update rust with
    rustup update

Compile or test code with:

    cargo build
    cargo test

Create new [Cargo](https://crates.io) crates with:

    cargo new {some_name}
    # or library crates with
    cargo new {some_lib} --lib

