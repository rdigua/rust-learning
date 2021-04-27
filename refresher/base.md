# Rust小白基础篇


## 基础类型

`Rust`  语言中有四种标量数据类型：

-   整型
    
-   浮点型
    
-   布尔类型
    
-   字符类型
    

**类型列表👇**

> 当前只列出整数类型，其他类型自行查询资料，大家都有经验的开发者了，吃个螃蟹🦀还不容易吗？

| size  | signed | unsigned |
| :---: | :----: | :------: |
| 8bit | i8 | u8  |
| 16bit | i16  | u16 |
| 32bit | i32 | u32 |
| 64bit | i64 | u64 | 
|128bit | i128 | u128 |
| Arch | isize | usize |

Decimal:	98_222
Hex:	0xff
Octal:	0o77
Binary:	0b1111_0000
Byte(u8)only:	b'A'

`arch`  是由  `CPU`  构架决定的大小的整型类型，大小为  `arch`  的整数在  `x86`  机器上为  `32`  位，在`x64`  机器上为  `64`  位。

```
fn main() {  
      let name = "Jarvib Ding";  
      const MY_AGE: u8 = 0x16;  
      let weight: f32 = 67.8;  
      let is_man = true;  
      println!(  
          "Hi, my name is {},age is {} and weight is {} kg",  
          name, MY_AGE, weight  
      );  
}  
```

-   `name`字符串字面量模式是静态的这就意味着字符串字面量从创建时开始会一直保存到程序结束，并且它一个不可变变量，如果想二次赋值必须通过`mut`声明。
    
-   `MY_AGE`是常量，常量名的命名规则可变量的命名规则一样，但常量名一般都是大写字母，定义常量时必须指定数据类型，而定义变量时数据类型可以省略。常量一旦定义就永远不可变更和重新赋值。
    
-   `weight`是浮点型，区分整型和浮点型的唯一指标就是 有没有小数点且不能互相转换，定义浮点型变量的时候要注意每种浮点型的最大值和最小值，如果超出可能会赋值失败，也有可能结果不是预期的结果。
    

## 算数运算符

运算符是用于对数据执行一些操作，被运算符执行的数据称为操作数。

```
fn main() {  
    // 加减乘除  
    let result = 11 + 11;  
    println!("11 + 11 = {}", result);  
    println!("11 - 11 = {}", 11 - 11);  
    println!("11 * 11 = {}", 11 * 11);  
    println!("11 / 11 = {}", 11 / 11);  
    println!("11 % 11 = {}", 11 % 11);  
}  
```

## 关系运算符

关系运算符测试或定义两个实体之间的关系类型，关系运算符用于比较两个或多个值之间的关系，是大于，是等于还是小于，关系运算符的返回结果为  `布尔类型`。

`== = != < > <= >=`

```
let a = 1;  
let b = 2;  
  
let c = a == b; //false  
let d = a != b; //true  
let e = a < b; //true  
let f = a > b; //false  
let g = a <= a; //true  
let h = a >= a; //true  
  
let i = true > false; //true  
let j = 'a' > 'A'; //true  
```

更多运算操作可以查看笔者的`https://getrust.tech/`。

## 条件表达式

在计算机科学中，`条件表达式`又称条件运算式、`条件表示式`，是一种编程语言的功能，它可以用来决定当程序指定的布尔运算值为真或假时，程序接下来将会采取的行动。

计算机之所以能做很多自动化的任务，因为它可以自己做条件判断。

![图片](https://mmbiz.qpic.cn/mmbiz_jpg/LIIp1fVaxzDyvTZGx1BYSShClH3MicTjicSW9Nt2plBiboAGnialx2uzoVHyIrhEoDg066ibiby1b9F4O7oq2JezCdRw/640?wx_fmt=jpeg&tp=webp&wxfrom=5&wx_lazy=1&wx_co=1)

条件判断

计算机程序会根据条件不同执行不同的代码。

![图片](https://mmbiz.qpic.cn/mmbiz_jpg/LIIp1fVaxzDyvTZGx1BYSShClH3MicTjicwwgFickKRyjuhN2UBKiavicxcOI4UQMV90X3UdIkgm3a2gL3ibR38ibRcLA/640?wx_fmt=jpeg&tp=webp&wxfrom=5&wx_lazy=1&wx_co=1)

条件判断

## `if`  语句语法

`Rust`  语言中使用`if`语句来模拟现实生活中这种`如果...就`  的情况.


```
fn main() {  
    let age = 22;  
    if age > 18 {  
        println!("你已经成年了！")  
    }  
}  
// OUT  
// 你已经成年了！  
```

## `if else`  语句

1.  在  `if else`  语句中，`if`  语句才是最主要的。如果  `条件`  为真，就没  `else`  语句啥事了。
    
2.  其实  `if`  语句后面的  `else`  语句是可选的。就像我们所说的，如果`条件`为假就什么都不做，那要  `else`语句有什么用呢？`else`  语句的唯一作用，就是  `if`  语句中的  `条件`  为假时做些什么，执行些什么。
    

![图片](https://mmbiz.qpic.cn/mmbiz_jpg/LIIp1fVaxzDyvTZGx1BYSShClH3MicTjicBhiaB2HqiaHA1D5tz1gxLhGKTkia7micrwPIHdc5WzW1ibV6yrHpicSLBhZw/640?wx_fmt=jpeg&tp=webp&wxfrom=5&wx_lazy=1&wx_co=1)

if else 语句

我们写一段代码，使用  `if else`  语句来判断一个数是否偶数或奇数，如果是偶数则输出  `偶数`，如果是奇数则输出  `奇数`。

```
fn main() {  
   let num = 12;  
   if num % 2==0 {  
      println!("偶数"); //√  
   } else {  
      println!("奇数");  
   }  
}  
```

## `if...else if...`  语句

`if...else if...`特点是那个条件先满足就执行那块代码。

我们使用嵌套  `if`  语句来写一段代码，判断某个值是  `大于、小于、等于 0`。

```
fn main() {  
   let num = 2 ;  
   if num > 0 {  
      println!("{} is positive",num); // √  
   } else if num < 0 {  
      println!("{} is negative",num);  
   } else {  
      println!("{} is neither positive nor negative",num) ;  
   }  
}  
```

## `match`  语句

`match`  语句用于检查 某个当前的值 是否匹配`一组/列值`  中的某一个。

如果你会`C`语言，那么  `Rust`  中的  `match`  表达式则类似于  `C`  语言中的  `switch`  语句。

```
// match 语句有返回值，它把 匹配值 后执行的最后一条语句的结果当作返回值。

  
let expressionResult = match variable_expression {  
   constant_expr1 => {  
      // 语句;  
   },  
   constant_expr2 => {  
      // 语句;  
   },  
   _ => {  
      // 默认  
      // 其它语句  
   }  
};  
```

首先要说明的是  `match`  关键字后面的表达式不必括在括号中。也就是  `variable_expression`  不需要用一对 括号(`()`) 括起来。

其次，`match`  语句在执行的时候，会计算  `variable_expression`表达式的值，然后把计算后的结果和每一个  `constant_exprN`  匹配，使用的是 全等于 也就是  `===`  来匹配。如果匹配成功则执行`=> {}`  里面的语句。

如果  `variable_expression`  表达式的值没有和任何一个  `constant_exprN`  匹配，那么它会默认匹配  `_`。

因此，当没有匹配时，默认会执行  `_ => {}`  中的语句。

`match`  语句有返回值，它把 匹配值 后执行的最后一条语句的结果当作返回值。

`_ => {}`  语句是可选的，也就是说  `match`  语句可以没有它。

```
 let month = "二月";  
  
    let english_month = match month {  
        "一月" => "January",  
        "二月" => "February", // ✅  
        "三月" => "March",  
        "四月" => "April",  
        _ => "Unknown",  
    };  
    println!("{}", english_month)
```

## 循环语句

`循环语句`一般是只在程序重复执行某块代码逻辑的词语，`循环`其实就是一种重复，在`满足指定的条件`下，`重复的做某些事情`。就好比如只要时间没到 18:30，那么我们一直在重复的上班。

![图片](https://mmbiz.qpic.cn/mmbiz_jpg/LIIp1fVaxzDyvTZGx1BYSShClH3MicTjicL4GZjUMfkKtqPzKDsiapu9HVMZRmicCl084HEeEkKFoeNicGMC3RsxS3w/640?wx_fmt=jpeg&tp=webp&wxfrom=5&wx_lazy=1&wx_co=1)

循环

`Rust`循环语句有三种：

-   `loop`  语句。一种重复执行且永远不会结束的循环。
    
-   `while`  语句。一种在某些条件为真的情况下就会永远执行下去的循环。
    
-   `for`  语句。一种有确定次数的循环。
    

1.  能确定次数的循环，比如  `for`  循环。
    
2.  满足条件就是永动机的循环，比如  `while`  循环。
    
3.  死循环，比如  `loop`  循环。
    

## `for`  循环语句

`for`  语句用于执行代码块指定的次数，可能和其它语言有所不同，`Rust`  中的  `for`  循环只有  `for..in`  这种格式，常用于迭代一组固定的值，例如`数组、向量`等。

```
fn main() {  
    let mut sum: u16 = 0;  
    for i in 1..100 {  
        sum += i;  
    }  
    println!("sum = {}", sum)  
}  
// sum = 4950  
```

## `while`  循环语句

`while`  循环会在每次重复执行前先判断条件是否满足，满足则执行，不满足则退出。

 `sum = 0;  
    while sum < 100 {  
        sum += 1;  
    }  
    println!("sum = {}", sum); //100`

## `loop`  循环语句

`loop`  语句代表着一种死循环。它没有循环条件，也没有循环次数，它就是一个永动机。

`loop {  
    // action 要重复执行的代码  
}  
`

真正意义上的死循环，这里就不做演示！

## `break`  循环控制语句

`break`  语句的出现，就是为了在  `action`  语句块中可以退出循环语句。

```
let mut n = 0;  
    loop {  
        if n == 100 {  
            // stop action  
            break;  
        }  
        n += 1;  
    }  
    println!("n = {}", n);  // 100
```

## `continue`  循环控制语句

`continue`  语句，简单的说，就是停止执行剩下的语句，直接进入下一个循环。

 ```
 // 100以内偶数  
    n = 0;  
    loop {  
        n += 1;  
        if n == 100 {  
            // stop action  
            break;  
        }  
        if n % 2 == 0 {  
            // stop action  
            println!("偶数 {}", n);  
            continue;  
        }  
    }
```	

## 函数定义

`函数`是一组可以执行的任务代码块，函数是一段可读的，可维护的和可重用的代码语句块。每个  `Rust`  程序都至少有一个函数，即主函数  `main()`，除了使用  `Rust`  核心和标准库提供的函数外，我们还可以自己定义自己的函数。

## 函数声明

`函数说明`就是告诉编译器一个`函数的名称`、`变量`、和`返回值类型`。这三个合在一起组成了`函数的签名`，`函数签名`的作用就是防止出现两个相同的函数。

![图片](https://mmbiz.qpic.cn/mmbiz_jpg/LIIp1fVaxzDyvTZGx1BYSShClH3MicTjicfsPia5SbgiayGMIQehf1ibKMiamx1L4Glezyr5mU0sicdQaIQxiceGQhLlFQ/640?wx_fmt=jpeg&tp=webp&wxfrom=5&wx_lazy=1&wx_co=1)

函数说明

## 函数定义

`函数`可以帮我们把可以复用的代码组装到一个函数块里面，方便在其他地方进行调用。我们可以把代码划分到不同的函数中，这样可以使得代码可读性更强，逻辑更简单。

因此，定义函数时首先想的并不是我要定义一个函数，而是我这个任务要怎么做，要定义哪些函数来完成。

定义函数时必须以  `fn`  关键字开头，`fn`  关键字是  `function`  的缩写，函数内部必须包含函数要执行的具体代码，我们把这些代码称之为  `函数体`。

定义函数的语法如下，定义函数时必须使用  `fn`关键字开头，后面跟着要定义的`函数名`。

`fn funcation_name(parma:data_type){  
    // 函数代码  
}  
`

下面的代码，我们定义了一个函数名为  `say_hi`  的函数，用于输出一些信息

`fn say_hi() {  
    println!("👋 Hello!");  
}  
// 👋 Hello!  
`

为了运行一个函数首先必须调用它。函数不像普通的语句，写完了会自动执行，函数需要调用才会被执行。

👇下面，函数  `main()`  就是  `调用者函数`，也就是  `调用者`。
```
fn main() {  
    //调用函数 out: 👋 Hello!  
    say_hi();   
}  
  
fn say_hi() {  
    println!("👋 Hello!");  
}  
```

## 函数返回值

在我们的函数代码块可能需要处理一些逻辑，然后把处理的结果返回给调用者，我们将这些值称为`函数返回值`。

Rust 语言的返回值定义语法与其它语言有所不同，它是通过在  `()小括号后面`使用 箭头`->`加上数据类型 来定义的。

有  `return`  语句:

```
fn main() {  
    assert_eq!(10,sum());  
}  
  
// 有返回值的函数  
fn sum() -> i8 {  
    return 5 + 5;  
}  
```

没有  `return`  语句则使用最后一条语句的结果作为返回值：

```
fn main() {  
    assert_eq!(assert_sum(),sum());  
}  
  
fn sum() -> i8 {  
    return 5 + 5;  
}  
  
fn assert_sum() -> i8 {  
    5 + 5 // 最后一条语句返回 并且没有‘;’  
}  
```

## 函数参数

`函数参数`  是一种将外部变量和值带给函数内部代码的一种机制，函数参数是函数签名的一部分，函数签名的最大作用，就是防止定义两个相同的签名的函数。

-   我们把函数定义时指定的参数名叫做  `形参`
    
-   把调用函数时传递给函数的变量值叫做  `实参`
    

函数参数有两种传值方法，一种是把`值的值接传递`给函数，另一种是把`值在内存上的保存位置`传递给函数。

## 传值调用

`传值调用`  就是简单的把传递的变量的值传递给函数的  `形参`，从某些方面说了，就是把函数参数也赋值为传递的值。

因为是赋值，所以函数参数和传递的变量其实是各自保存了相同的值，`互不影响`，因此函数内部修改函数参数的值并不会影响外部变量的值。

```
fn main() {  
    let name = "Jarvib";  
    edit_name(name);  
    println!("main() Your name is {}",name);  
    // edit_name() Your name is Jarvib Ding  
    // main() Your name is Jarvib  
}  
  
fn edit_name(mut name:&'static str){  
    name = "Jarvib Ding";  
    println!("edit_name() Your name is {}",name)  
}  
```

## 引用传递

值传递只是会重新创建一个变量，但引用传递则不会，引用传递把当前变量的内存位置传递给函数。

下面的代码中，星号`（*）`  用于访问变量  `param_no`  指向的内存位置上存储的变量的值，这种操作也称为  `解引用`。

```
fn main() {  
    let mut no:i32 = 22;  
    println!("The value of no is:{}",no);  
    mutate_no_to_zero(&mut no);  
    println!("The value of no is:{}",no);  
    // The value of no is:22  
    // The value of no is:0  
}  
  
fn mutate_no_to_zero(param_no:&mut i32){  
   *param_no = 0; //解引用操作  
}  
```

## `array`数组

虽然大部分变量都是基本数据类型，虽然这些基本数据类型可以满足我们一些开发需求，但是他们不是万能的。

-   基本数据类型的变量也有它们的局限性。
    
-   基本数据类型的变量本质上是`标量`，这意味着每个基本数据类型的变量一次只能存储一个值。
    
-   如果我们要存储的值非常多，成百上千，这种重复定义变量的方法是行不通的。
    

## 数组的特性

`数组`  是用来存储一系列数据，但它往往被认为是一系列相同类型的变量，也就是说，数组 是可以存储一个固定大小的相同类型元素的顺序集合。

-   数组的定义其实就是为分配一段`连续的相同数据类型`的内存块。
    
-   数组是静态的，这意味着一旦定义和初始化，则永远不可更改它的长度。
    
-   数组的元素有着相同的数据类型，每一个元素都独占者数据类型大小的内存块，也就是说。数组的内存大小等于数组的长度乘以数组的数据类型。
    
-   数组中的每一个元素都按照顺序依次存储，这个顺序号既代表着元素的存储位置，也是数组元素的唯一标识。我们把这个标识称之为`数组下标`。
    
-   可以更新或修改数组元素的值，但不能删除数组元素。如果要删除功能，你可以将它的值赋值为 0 或其它表示删除的值。
    

## 声明和初始化数组

1.  `Rust`语言为数组的声明和初始化提供了三种语法
    

`let year: [i32; 4] = [1999, 2019, 2020, 2021]; `

2.  省略数组类型的语法
    

`let arr = [10,20,30,40]; `

3.  指定默认初始值的语法，这种语法有时候称为 默认值初始化。
    

**如果不想为每一个元素指定初始值，则可以为所有元素指定一个默认的初始值，例如下面的代码为每一个元素指定初始值为  `-1`了。**

`let arr:[i32;4] = [-1;4];`

**综合使用例子:**

```
fn main() {  
    let year: [i32; 4] = [1999, 2019, 2020, 2021];  
    // let arr = [10,20,30,40];  
    println!("array is {:?}", year);  
    // len() 数组长度  
    println!("array size is :{}", year.len());  
}  

//output:

array is [1999, 2019, 2020, 2021]  
array size is :4  
```

## `for in`  循环遍历数组

在其它语言中，一般使用  `for`  `循环来遍历数组，Rust`  语言也可以，只不过时使用  `for`  语句的变种  `for ... in ..`  语句。

**因为数组的长度在编译时就时已知的，因此我们可以使用  `for ... in`  语句来遍历数组。**

```
for index in 0..year.len() {  
        println!("index: {} , value: {}", index, year[index])  
}  
```

## 迭代数组`iter()`

我们可以使用  `iter()`  函数为数组生成一个迭代器。

```
for value in year.iter() {  
        println!("value is: {}", value)  
}  
```

## 数组作为函数参数

数组可以作为函数的参数，而传递方式有  `传值传递`  和  `引用传递`  两种方式。

-   `传值传递`  就是传递数组的一个副本给函数做参数，函数对副本的任何修改都不会影响到原来的数组。
    
-   `引用传递`  就是传递数组在内存上的位置给函数做参数，因此函数对数组的任何修改都会影响到原来的数组。
    

**引用传递例子:**

```
fn main() {  
    let mut year: [i32; 4] = [1999, 2019, 2020, 2021];  
    // let arr = [10,20,30,40];  
    println!("array is {:?}", year);  
    // len() 数组长度  
    println!("array size is :{}", year.len());  
  
    for index in 0..year.len() {  
        println!("index: {} , value: {}", index, year[index]);  
    }  
  
    for value in year.iter() {  
        println!("value is: {}", value);  
    }  
    // updated of [1999, 2019, 2021, 2021]  
    updated_by_index(2, 2021, &mut year);  
    println!("updated of {:?}", year)  
}  
  
// 通过下标修改某个元素的值  
fn updated_by_index(index: usize, value: i32, arr: &mut [i32; 4]) {  
    arr[index] = value;  
}  
```

## 元组Tuple

`tuple`是`复合类型`可以存储多个不同类型的数据，`复合类型`就像我们的菜篮子，里面可以放各种类型的菜。

-   `tuple`长度是固定的，而且一旦定义了，就不能再次更改。
    
-   `tuple`是下标从`0`开始。
    

## `tuple`元组的定义

在定义的时候可以指定存储的数据类型：

`let tuple_name:(data_type1,data_type2,data_type3) = (value1,value2,value3);  
`

`Rust`  中元组的定义很简单，就是使用一对小括号  `()`  把所有元素放在一起，元素之间使用逗号`,`  分隔，当然也可以忽略类型声明。

`let tuple_name = (v1,v2,v3)  
`

## `tuple`元组的使用

-   下标访问
    
-   解构赋值
    

我们可以使用  `元组名.索引数字`  来访问元组中相应索引位置的元素。索引从  `0`  开始。

如果要输出元组中的所有元素，必须使用  `{:?}`  格式化符：

```
fn main() {  
    let tuples: (&'static str, i8, f64) = ("🦀", 22, 3.1415927);  
    println!("{:?}", tuples);  
    // 声明一个可变的tuple  
    let mut people = ("tom", "robin", "jarvib");  
    // 通过下标访问  
    println!("{},{},{}", people.0, people.1, people.2);  
    // 修改下标为2的值  
    people.2 = "Jarvib Ding";  
    // 通过 解构赋值 (destructing)  
    let (v1, v2, v3) = people;  
    println!("{},{},{}", v1, v2, v3);  
}  

/*
output:

`("🦀", 22, 3.1415927)  
tom,robin,jarvib  
tom,robin,Jarvib Ding  
*/
```

## 结构体使用

结构体(`struct`)支持组合不同的数据类型，但不同于元组，结构体需要给每一部分数据命名以标志其含义，因而结构体比元组更加灵活，不依赖顺序来指定或访问实例中的值。

1.  定义结构体
    

```
struct User {  
    name: String,  
    age: u8,  
    is_man: bool,  
    weight: f32,  
}  
```

2.  创建实例
    

```
let mut user1 = user {  
      // 根据指定的字符串字面量来创建字符串对象，使用 String::from() 方法。  
      name: String::form("Jarvib Ding"),  
      age: 0x16,  
      is_man: true,  
      weight: 67.8,  
};  
```

3.  实现方法
    

```
struct User {  
    name: String,  
    age: u8,  
    is_man: bool,  
    weight: f32,  
}  
  
impl User {  
    // gender 返回性别字符串  
    fn gender(&self) -> &'static str {  
        if self.is_man {  
            return "is it a boy.";  
        }  
        "is a girl"  
    }  
    // say_hi 打招呼函数  
    fn say_hi(&self) {  
        println!(  
            "Hi, my name is {},age is {} and weight is {} kg,{}",  
            self.name,  
            self.age,  
            self.weight,  
            self.gender()  
        );  
    }  
}  
  
fn main() {  
    let userinfo = User {  
        name: String::from("Jarvib Ding"),  
        age: 0x16,  
        is_man: true,  
        weight: 67.8,  
    };  
    userinfo.say_hi()  
}  
```

`impl`是`implementation`的缩写，后面加结构体名，表示为该结构体实现方法，要实现的方法包在`impl`块中，依然使用`fn`关键字。类似`Java`中的`this`关键字。

好了，运行这个`say_hi`函数:

``cargo run  
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s  
     Running `target/debug/demo`  
       
Hi, my name is Jarvib Ding,age is 22 and weight is 67.8 kg,is it a boy.  
``

## 小结

本文就写这么多吧，更多详细内容查看`https://getrust.tech/`！！卷起来！！卷起来！！哈哈哈哈，来吧互相伤害！！欢迎抬杠！！今晚的🦀螃蟹你吃的香吗？