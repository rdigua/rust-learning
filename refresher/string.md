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

```
//Selcect number form string, it will be a usize.
String:String = ss.chars().filter(|x| x.is_digit(10)).collect();
//Selcect number form string, it will be a float.
String:String = ss.chars().filter(|x| x.is_digit(10)||x='.').collect();

```

```
// From implies Into
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

```
let s:String="How to do!".into();
```

```
// From (and thus Into) is reflexive
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> From<T> for T {
    fn from(t: T) -> T {
        t
    }
}
```

```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
	println!("{}",word );

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
	println!("{}",word );

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
	println!("{}",word );
}
```