# Print

```
cargo new two
cd ./two/src
```

main.rs
```
fn main() {
    println!("Hello, world!");	
}
```

Then 

```
Cargo expand
```

```
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Hello, world!\n"],
            &match () {
                () => [],
            },
        ));
    };
}
```

see rustdoc doc/rust/html/std/fmt/index.html

```

```