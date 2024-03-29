# trait

```
trait Foo {
    fn run(&mut self) {
        println!("Foo::run()");
    }
}

fn run_base(foo: &mut Foo) {
    println!("run_base()");
}

struct Bar;

impl Foo for Bar {
    fn run(&mut self) {
        run_base(self);
        println!("Bar::run()");
    }
}

fn main() {
    let mut bar = Bar;
    bar.run();
}
```

```
fn sum<T:From<f32>>(min:i32, max:i32)->T{   
	let result = 3.14 + min as f32 + max as f32;
	return result.into();
}

fn main(){
	 let a:f64  = sum(15,100);    
	 let b:f32  = sum(15,100);    
	 let c = sum::<f32>(15,100);  
	 let d = sum::<f64>(15,100);  
	 println!("{},{},{},{}",a,b,c,d);
}
```

```rust
use std::fmt;
struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old.", self.name, self.age)
    }
}

fn print_cats(pet: String) {
    println!("{}", pet);
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    print_cats(mr_mantle.to_string()); // Turn him into a String here
    println!("Mr. Mantle's String is {} letters long.", mr_mantle.to_string().chars().count()); // Turn him into chars and count them
}
```


By the way, if you implement `Display` then you get the `ToString` trait for free. That's because you use the `format!` macro for the `.fmt()` function, which lets you make a `String` with `.to_string()`. So we could do something like this where we pass `reggie_mantle` to a function that wants a `String`, or anything else.




## PhantomData 













## base

### where


```rust
use std::fmt::Debug;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {:?}", item);
}

fn main() {
    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 1,
    };

    let number = 55;

    print_item(charlie);
    print_item(number);
}

```

```rust
use std::fmt::Display;
use std::cmp::PartialOrd;

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

fn main() {
    compare_and_display("Listen up!", 9, 8);
}
```


```rust
use std::cmp::PartialOrd;
use std::fmt::Display;

fn compare_and_display<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

fn main() {
    compare_and_display("Listen up!", 9, 8);
}
```

Using `where` is a good idea when you have many generic types.

Also note:

- If you have one type T and another type T, they must be the same.
- If you have one type T and another type U, they can be different. But they can also be the same.
