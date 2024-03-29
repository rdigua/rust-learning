# collections


## hash

```
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

macro_rules! seq_to_map {
    ($seq:expr) => {{
        let mut map = HashMap::<_, _, RandomState>::default();
        let mut iter = $seq.into_iter();
        while let Some(e) = iter.next() {
            map.entry(e).or_insert(iter.next().unwrap_or_default());
        }
        map
    }};
}

fn seq_to_map<T: IntoIterator>(vec: T) -> HashMap<T::Item, T::Item>
where
    T::Item: Hash + Default + Eq,
{
    let mut map = HashMap::default();
    let mut iter = vec.into_iter();
    while let Some(e) = iter.next() {
        map.entry(e).or_insert(iter.next().unwrap_or_default());
    }
    map
}

fn main() {

	let mut v=Vec::new();
	for i in "a1b2c3d4e5f6g7h8i9j".chars(){
		v.push(i)
	}
	let h=seq_to_map(v);
	println!("{:?}",h);
	//vec.chunks_exact(2).map(|chunk| (chunk[0], chunk[1])).collect::<HashMap::<_, _>>()

}

```

```
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}

let person1 = Person {
    id: 5,
    name: "Janet".to_string(),
    phone: 555_666_7777,
};
let person2 = Person {
    id: 5,
    name: "Bob".to_string(),
    phone: 555_666_7777,
};

assert!(calculate_hash(&person1) != calculate_hash(&person2));

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
```

```
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Person {
    id: u32,
    name: String,
    phone: u64,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.phone.hash(state);
    }
}

let person1 = Person {
    id: 5,
    name: "Janet".to_string(),
    phone: 555_666_7777,
};
let person2 = Person {
    id: 5,
    name: "Bob".to_string(),
    phone: 555_666_7777,
};

assert_eq!(calculate_hash(&person1), calculate_hash(&person2));

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
```


## slice 

***copy_from_slice***:

```
#[repr(C)]
pub struct Row {
    id: i32,
    user: [char; 32],
    email: [char; 255],
}

impl Row {
    pub fn new(id: i32, user_str: &str, email_str: &str) -> Self {
        let mut user: [char; 32] = ['\0'; 32];
        let mut email: [char; 255] = ['\0'; 255];
        user.copy_from_slice(&user_str.chars().collect::<Box<[char]>>());
        email.copy_from_slice(&email_str.chars().collect::<Box<[char]>>());
        Row { id, user, email }
    }
}
```

*** How about RandomState with new?***

```
#![allow(unused)]
fn main() {
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

let s = RandomState::new();
let mut map = HashMap::with_hasher(s);
if map.is_empty(){
    println!("The map it empty");
} else  {
    println!("{:?}",map);
}
map.insert(1, 2);
println!("{:?}",map)
}
```

*** Vec<Vec<String>>***
```
fn main() {
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // 1 element: prints 4. Vecs with 1 item always start with capacity 4
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // 4 elements: still prints 4.
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8. We have 5 elements, but it doubled 4 to 8 to make space
    let mut num_vec: Vec<Vec<String>> = Vec::new();
    num_vec.push(vec!["1string".to_string(), "2string".to_string()]);
    num_vec.push(vec!["3string".to_string(), "4string".to_string()]);
    num_vec.push(["5string".to_string(), "6string".to_string()].into());
    num_vec.push(["7string".to_string(), "8string".to_string()].into());
    println!("{:?}", num_vec);
}

```