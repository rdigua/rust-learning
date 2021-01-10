# struct

```
trait Shape {
	
	fn area(&self)->f64;
	
}

trait Round {
	
	fn get_radius(&self)->f64;
	
}


struct Circle {
radius: f64,
}

impl Round for Circle {
fn get_radius(&self) -> f64 { self.radius }
}

// 注意这里是 impl Trait for Trait

impl Shape for dyn Round {
fn area(&self) -> f64 {
std::f64::consts::PI * self.get_radius() * self.get_radius()
}
}

fn main() {
let c = Circle { radius : 2f64};
// 编译错误
 //c.area();
let b = Box::new(Circle {radius : 4f64}) as Box<dyn Round>;
// 编译正确
let c = b.area();

println!("{}",c);

}
```

srqc-p94

```
// . and ::
struct T(i32);
impl T {
// 这是一个静态方法
fn func(this: &Self) {
println!{"value {}", this.0};
}
// 这不是一个静态方法
fn first(&Self) {
println!{"value {}", self.0};
}

}
fn main() {
let x = T(42);
// x.func(); 小数点方式调用是不合法的
T::func(&x);

let x = T(4);

x.first();

}
```

## trait

### impl trait

```
pub trait animal{
    fn print_name(&self);
}

struct cat{
    name:String
}

struct dog{
    name:String
}

impl animal for cat{
    fn print_name(&self){
        println!("{}",self.name);
    }
}

impl animal for dog{
    fn print_name(&self){
        println!("{}",self.name);
    }
}

fn who(who:i32)->impl animal{ //注意只能返回同一个类型
    if who== 1{
        cat{name:"cat one ".to_string()}
    } else{
        cat{name:"cat two ".to_string()}
    }
}

fn main(){
    let a = who(1);
    a.print_name();
}
```
