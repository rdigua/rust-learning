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


-   `+`  - This is intended for numeric types and indicates that the sign should always be printed. Positive signs are never printed by default, and the negative sign is only printed by default for the  `Signed`  trait. This flag indicates that the correct sign (`+`  or  `-`) should always be printed.
-   `-`  - Currently not used
-   `#`  - This flag indicates that the "alternate" form of printing should be used. The alternate forms are:
    -   `#?`  - pretty-print the  [`Debug`](../../std/fmt/trait.Debug.html "Debug")  formatting
    -   `#x`  - precedes the argument with a  `0x`
    -   `#X`  - precedes the argument with a  `0x`
    -   `#b`  - precedes the argument with a  `0b`
    -   `#o`  - precedes the argument with a  `0o`
-   `0`  - This is used to indicate for integer formats that the padding to  `width`  should both be done with a  `0`  character as well as be sign-aware. A format like  `{:08}`  would yield  `00000001`  for the integer  `1`, while the same format would yield  `-0000001`  for the integer  `-1`. Notice that the negative version has one fewer zero than the positive version. Note that padding zeros are always placed after the sign (if any) and before the digits. When used together with the  `#`  flag, a similar rule applies: padding zeros are inserted after the prefix but before the digits. The prefix is included in the total width.

```

```