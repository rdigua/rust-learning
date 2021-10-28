# Install

## Install Rust

### linux 

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### windows

download rustup_init:

https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe 

more see rustup doc

https://rust-lang.github.io/rustup/


**more**

More [rust doc book-installation](https://doc.rust-lang.org/book/ch01-01-installation.html)



## update rust

```
rustup update
```

***delete rustup***

```
rustup self uninstall
```

***'when error'***

```
git clone https://github.com/rust-lang/rustup 
cargo build --release
mv ./target/release/rustup-init.exe rustup
...
```


## rustup

```
rustup show
```

```
rustup check
```

```
rustup doc
```

```
rustup target list
```

## cargo 

```
cargo new project_name
```


```
cargo new --lib project_name
```

more

```
rustup doc --cargo 
```

Then to start using Rust.

`rustc`, `cargo` and `rustup` 


***important***

More [rust doc book-installation](https://doc.rust-lang.org/book/ch01-01-installation.html)

## Using Cargo

`main.rs lib.rs other.rs another.rs`

-lib.rs

```
pub mod other;
pub mod another; 

```

-main.rs

```
// Cargo.toml name="xxx"
use xxx::{...*...};

```

other.rs

```
//if use another.rs
use super::another...


```

## about test

/tests/*.rs

```
use xxx::...

```

_**The  `lib.rs`  and  `main.rs`  files are two independent entry points for your package.**_

[more](https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs)

[Read more: Clear explanation of Rust’s module system](http://www.sheshbabu.com/posts/rust-module-system/)


