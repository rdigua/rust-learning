# The Rust Standard Library


## std::mem


Error E0507:

```
#![allow(unused)]
fn main() {
struct Buffer<T> { buf: Vec<T> }

impl<T> Buffer<T> {
    fn replace_index(&mut self, i: usize, v: T) -> T {
        // error: cannot move out of dereference of `&mut`-pointer
        let t = self.buf[i];
        self.buf[i] = v;
        t
    }
}
}
```
Chaneged:
```
#![allow(unused)]
#![allow(dead_code)]
fn main() {
use std::mem;

struct Buffer<T> { buf: Vec<T> }
impl<T> Buffer<T> {
    fn replace_index(&mut self, i: usize, v: T) -> T {
        mem::replace(&mut self.buf[i], v)
    }
}

let mut buffer = Buffer { buf: vec![0, 1] };
assert_eq!(buffer.buf[0], 0);

assert_eq!(buffer.replace_index(0, 2), 0);
assert_eq!(buffer.buf[0], 2);
}
```

```
#![allow(unused)]
fn main() {
use std::mem;

struct Buffer<T> { buf: Vec<T> }
impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        mem::take(&mut self.buf)
    }
}

let mut buffer = Buffer { buf: vec![0, 1] };
assert_eq!(buffer.buf.len(), 2);

assert_eq!(buffer.get_and_reset(), vec![0, 1]);
assert_eq!(buffer.buf.len(), 0);
}
```
