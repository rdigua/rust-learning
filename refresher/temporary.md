# temporary

```
use std::path::Path;

assert_eq!("rs", Path::new("foo.rs").extension().unwrap());
assert_eq!("gz", Path::new("foo.tar.gz").extension().unwrap());
```

```
        path.as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .ok_or(AssetServerError::MissingAssetLoader(None))
            .and_then(|extension| self.get_asset_loader(extension))
```

```
#![allow(unused)]
fn main() {
let maybe_name = Some(String::from("Alice"));
// Using `ref`, the value is borrowed, not moved ...
match maybe_name {
    Some(ref n) => println!("Hello, {}", n),
    _ => println!("Hello, world"),
}
// ... so it's available here!
println!("Hello again, {}", maybe_name.unwrap_or("world".into()));
}
```

```
fn max_num<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        &x
    } else {
        &y
    }
}
fn main() {
    let x = 1;                // -------------+-- x start
    let max;                  // -------------+-- max start
                             //              |
        let y = 8;              // -------------+-- y start
        max = max_num(&x, &y);  //              |
                             // -------------+-- y over
    println!("max: {}", max); //              |
}
```

```
fn main() {

    let g;
    let s1 = "ASd";

    {
        let s2 = "asad";
        g = longer(s1,s2);
    }

    println!("{} is longer", g);
}


fn longer<'a>(s1:&'a str, s2:&'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}
```