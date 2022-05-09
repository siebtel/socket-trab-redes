
# Installation

#### If you are on Unix install rustup:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Other platforms: https://forge.rust-lang.org/infra/other-installation-methods.html#rustup

More information: [Rust Docs installation](https://doc.rust-lang.org/book/ch01-01-installation.html)

#### Check if it is installed
```bash
rustc --version
```
#### Cargo
Cargo is a package manager for Rust. It comes with rustup. Check if it is installed:
```bash
cargo --version
```
##### General Commands to use Cargo
We can build (compile the code) a project using `cargo build`.

We can build and run a project in one step using `cargo run`.

We can build a project without producing a binary to check for errors using `cargo check`.

Instead of saving the result of the build in the same directory as our code, 
Cargo stores it in the target/debug directory.

`cargo build --release` to compile it with optimizations. This command will create an executable in target/release instead of target/debug.

More information: [Rust Docs Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

# Usage

Nothing is better than the documentation of rust and community docs. Check it out:
- [Rust Docs](https://doc.rust-lang.org/book/title-page.html)
- [Community Docs](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html)