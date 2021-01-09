# type

## number

### flot
```
use num_traits::float::Float; // 0.2.6
use std::ops::Mul;

fn scale_float<T>(x: T) -> T
where
    T: Float + Mul<f64, Output = T>,
{
    x * 0.54
}

fn main() {
    let a: f64 = scale_float(1.23);
}
```

### integer
```
fn f<T: FromU64>(v: u64) -> T {
    FromU64::from_u64(T)
}
```

### Variables without values (//easy_rust)

A variable without a value is called an "uninitialized" variable. Uninitialized means "hasn't started yet". They are simple: just write `let` and the variable name:

```rust
fn main() {
    let my_variable; // ⚠️
}
```

But you can't use it yet, and Rust won't compile if anything is uninitialized.

But sometimes they can be useful. A good example is when:

- You have a code block and the value for your variable is inside it, and
- The variable needs to live outside of the code block.

```rust
fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}

fn main() {
    let my_number;

    {
        // Pretend we need to have this code block
        let number = {
            // Pretend there is code here to make a number
            // Lots of code, and finally:
            57
        };

        my_number = loop_then_return(number);
    }

    println!("{}", my_number);
}
```

This prints `100`.

You can see that `my_number` was declared in the `main()` function, so it lives until the end. But it gets its value from inside a loop. However, that value lives as long as `my_number`, because `my_number` has the value. And if you wrote `let my_number = loop_then_return(number)` inside the block, it would just die right away.

It helps to imagine if you simplify the code. `loop_then_return(number)` gives the result 100, so let's delete it and write `100` instead. Also, now we don't need `number` so we will delete it too. Now it looks like this:

```rust
fn main() {
    let my_number;
    {
        my_number = 100;
    }

    println!("{}", my_number);
}
```

So it's almost like saying `let my_number = { 100 };`.

Also note that `my_number` is not `mut`. We didn't give it a value until we gave it 50, so it never changed its value. In the end, the real code for `my_number` is just let `my_number = 100;`.
