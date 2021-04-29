# String

## string-str
when str and string share in ...
AsRef<str>

```
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}

let s = "hello";
is_hello(s);

let s = "hello".to_string();
is_hello(s);
```

```
#![allow(unused)]
fn main() {
let s = String::from("hello");

let b = s.into_boxed_str();
println!("{}",b);
}
```

```
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}
```