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
