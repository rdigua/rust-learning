# About Project 

## Cargo

`main.rs lib.rs other.rs another.rs`

- lib.rs

```
pub mod other;
pub mod another; 

```

- main.rs

```
// Cargo.toml name="xxx"
use xxx::{...*...};

```

- other.rs

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