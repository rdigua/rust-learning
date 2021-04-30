# A number tool.


## Explanation

A lazy of men will be try to save themselves typing as did not found his calculator.

Buying and selling of stocks, it has Initial minimum. The number is ten thousand.

```



fn main() {
    use std::env;

    let mut k = env::args();
    if k.len() == 1 {
        println!("Must be input a price!");
        return;
    }
    let mut s = "".to_string();
    match k.nth(1) {
        Some(x) => s = x,
        _ => return,
    }

    let num = strtof32(s.clone());
    if num == 0.0 {
        println!("Error, {}", s);
        return;
    }
    println!("Input is {}, and get {}", s, (10000.0 / num) as u32);

}

pub fn strtof32<T: AsRef<str>>(s: T)  -> f32 {
    let picks: String =s.as_ref().chars().filter(|x| x.is_digit(10)||*x=='.').collect();

    if picks.len() == 0 {
        return 0.0;
    }

    picks.parse::<f32>().unwrap_or(0.0)

}

```