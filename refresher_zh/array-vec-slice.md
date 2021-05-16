# Arrays, vectors and slices in Rust

**2020-10-11**

## Introduction

In this post I will introduce you to arrays, vectors and slices in Rust. Programmers coming from C or C++ will already be familiar with arrays and vectors, but because of Rust's focus on safety there are some differences from their unsafe language counterparts. Slices, on the other hand, will entirely be a new, albeit a very useful concept.

## Arrays

Arrays are one of the first data types beginner programmers learn. An array is a collection of elements of the same type allocated in a contiguous memory block. For example, if you allocate an array like this:

```
let array: [i32; 4] = [42, 10, 5, 2];

```

Then all the  `i32`  integers are allocated next to each other on the stack:

![array memory layout](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/array-layout.svg)

In Rust, an array's size is part of the type. For example, this code will not compile:

```
//error:expected an array with a fixed size of 4 elements,
//found one with 3 elements
let array: [i32; 4] = [0, 1, 2];

```

Rust's strictness also prevents problems like array to pointer decay in C/C++:

```
//C++ code
#include <iostream>

using namespace std;

//Looks can be deceiving: arr is not a pointer
//to an array of 5 integers. It has decayed to
//a pointer to an integer.
void print_array_size(int (*arr)[5]) {
    //prints 8 (the size of a pointer)
    cout << "Array size in print_array_size function: " << sizeof(arr) << endl;
}

int main()
{
    int arr[5] = {1, 2, 3, 4, 5};
    //prints 20 (size of 5 4-byte integers)
    cout << "Array size in main function: " << sizeof(arr) << endl;
    print_array_size(&arr);
    return 0;
}

```

The  `print_array_size`  function prints 8 instead of the expected 20 (5 integers of 4 bytes) because  `arr`  has decayed from a pointer to an array of 5 integers to just a pointer to an integer. Similar code in Rust does the right thing:

```
use std::mem::size_of_val;

fn print_array_size(arr: [i32; 5]) {
    //prints 20
    println!("Array size in print_array_size function: {}", size_of_val(&arr));
}

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    //print 20
    println!("Array size in main function: {}", size_of_val(&arr));
    print_array_size(arr);
}

```

Another difference between an array in C/C++ and Rust is that accessing elements in Rust does bounds checking. For example, in the following C++ code, we try to access 5th element in an array of size 3. This produces  [undefined behaviour](https://blog.regehr.org/archives/213):

```
#include <iostream>

using namespace std;

int main()
{
    int arr[3] = {1, 2, 3};
    const auto index = 5;
    //arr[index] is undefined behaviour
    cout << "Integer at index " << index << ": " << arr[index] << endl;
    return 0;
}

```

While similar code in Rust is a panic:

```
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let index = 5;
    //arr[index] panics with the following message:
    //index out of bounds: the len is 3 but the index is 5
    println!("Integer at index {}: {}", index, arr[index]);
}

```

You might wonder, how is the Rust version better than the C++ version? Well, because the C++ version exhibits undefined behaviour, it gives a no holds barred license to the compiler to do anything in the name of optimizations. In the worst case, this can leak information to an attacker.

The Rust version, in contrast, will always panic. Moreover, because the process terminates due to the panic, a programmer is more likely to notice and fix this bug. In contrast, C++ sweeps the problem under the rug and the process could carry on as if nothing had happened. I will take a Rust panic any day over a C/C++ undefined behaviour.

## Vectors

The big limitation of arrays is that they are fixed in size. In contrast, vectors can grow at runtime:

```
fn main() {
    //There are three elements in the vector initially
    let mut v: Vec<i32> = vec![1, 2, 3];
    //prints 3
    println!("v has {} elements", v.len());
    //but you can add more at runtime
    v.push(4);
    v.push(5);
    //prints 5
    println!("v has {} elements", v.len());
}

```

How does a vector allow dynamic growth? Internally, a vector keeps all the elements in an array allocated on the heap. When a new element is pushed, the vector checks if there is still some capacity left in the array. If not, the vector allocates a bigger array, copies all the elements to the new array and deallocates the old array. This can be seen in the following code:

```
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4];
    //prints 4
    println!("v's capacity is {}", v.capacity());
    println!("Address of v's first element: {:p}", &v[0]);//{:p} prints the address
    v.push(5);
    //prints 8
    println!("v's capacity is {}", v.capacity());
    println!("Address of v's first element: {:p}", &v[0]);
}

```

Initially the capacity of the  `v`'s backing array is 4:

![Vector with capacity 4](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/vec-capacity-4.svg)

A new element is then pushed onto the vector. This makes the vector copy all the elements to a new backing array of capacity 8:

![Vector with capacity 8](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/vec-capacity-8.svg)

The program also prints the address of the first element of the array before and after pushing a new element onto the vector. Both these printed addresses will be different from each other. This change in addresses is clear evidence of a new array of size 8 being allocated behind the scenes.

NOTE  NOTE

If you do not see a different address after pushing more elements onto a vector, it might be because the allocator had enough space at the end of the original buffer such that the new and the old buffers have the same starting address. Try pushing more elements and you will see a different address. Read about C library function  `realloc`  to understand how this might happen.

## Slices

Slices act like temporary views into an array or a vector. For example if you have an array:

```
let arr: [i32; 4] = [10, 20, 30, 40];

```

You can create a slice containing second and third elements like this:

```
let s = &arr[1..3];

```

The  `[1..3]`  syntax creates a range from index 1 (inclusive) to 3 (exclusive). If you omit the first number in the range (`[..3]`) it defaults to zero and if you omit the last number (`[1..]`) it defaults to the length of the array. If you print the elements in the  `[1..3]`  slice, you get 20 and 30:

```
//prints 20
println!("First element in slice: {:}", s[0]);
//prints 30
println!("Second element in slice: {:}", s[1]);

```

But if you try to access an element outside the range of the slice, it will panic:

```
//panics: index out of bounds
println!("Third element in slice: {:}", s[2]);

```

But how does the slice know that it has only two elements? That's because a slice is not simply a pointer to the array, it also carries around the number of elements of the slice in an additional length field.

NOTE  NOTE

A pointer with some additional data besides just the address of the pointed to object is called a fat pointer. Slices are not the only kind of fat pointer in Rust. Trait objects, for example, carry a vtable pointer in addition to the pointer to an object.

For example, if you create a slice to a vector:

```
let v: Vec<i32> = vec![1, 2, 3, 4];
let s = &v[1..3];

```

Then in addition to a pointer to the second element in  `v`'s buffer,  `s`  also has an 8 byte length field with value 2:

![Slice of a vector](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/slice-of-vec.svg)

The presence of the length field can also be seen in the following code in which the size of a slice(`&[i32]`) is 16 bytes (8 for the buffer pointer and 8 for the length field):

```
use std::mem::size_of;

fn main() {
    //prints 8
    println!("Size of a reference to an i32: {:}", size_of::<&i32>());
    //print 16
    println!("Size of a slice: {:}", size_of::<&[i32]>());
}

```

Slices of arrays are similar, but instead of the buffer pointer pointing to a buffer on the heap, it points to the array on the stack.

Since slices borrow from the underlying data structure, all the usual borrowing rules apply. For example, this code is rejected by the compiler:

```
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4];
    let s = &v[..];
    v.push(5);
    println!("First element in slice: {:}", s[0]);
}

```

Why? Because when the slice is created, it points to the first element of the vector's backing buffer and as a new element is pushed onto the vector, it allocates a new buffer and the old buffer is deallocated. This leaves the slice pointing to an invalid memory address, which if accessed would have lead to undefined behaviour. Rust has saved you from disaster again.

NOTE  NOTE

Since slices can be created from both arrays and vectors, they are a very powerful abstraction. Hence for arguments in functions, the default choice should be to accept a slice instead of an array or a vector. In fact many functions like  `len`,  `is_empty`  etc. work on slices instead of on vectors or arrays.

## Conclusion

Arrays and vectors being one of the first few data structures that new programmers learn, it is no surprise that Rust too has a solid support for them. But as we saw, Rust's safety guarantees do not allow programmers to abuse these fundamental data types. Slices are a novel concept in Rust but since they are such a useful abstraction, you will find them used pervasively in any Rust codebase.
---


简介
本文主要讨论Rust中的数组(Array)、动态数组(Vector)和切片(Slice)。 C/C++程序员应该很熟悉数组和动态数组，由于rust注重安全性，和其它语言相比这些还是略有区别。 另外切片属于rust全新的非常重要的概念。

数组（Arrays）
数组是初级程序员就会学到的数据类型。一个数组是一组相同数据类型元素组成的集合，存储 在连续的内存块中。例如定义如下数组：

let array: [i32; 4] = [42, 10, 5, 2];
Rust
所有的i32整形数据都存储在彼此地址相邻的栈内存中：

             stack
            |-----|
            |  42 |
            |-----|
            |  10 |
            |-----|
            |  5  |
            |-----|
            |  2  |
            |-----|
Plaintext
在 Rust 中，数组大小是类型的一部分。如下代码不能通过编译：

// error: expected an array with a fixed size of 4 elements,
// found one with 3 elements
let array: [i32; 4] = [0, 1, 2];
Rust
Rust 的严谨性还可以防止 C/C++ 中的数组衰退为指针问题：

// c++ code
#include <iostream>

using namespace std;

// 别被外表欺骗：`arr` 不是指向5个整形元素的数组指针。
// 它已经衰退为指向整数的指针
void print_array_size(int (*arr)[5]) {
      // 输出8（指针的大小）
      cout << "在 print_array_size 函数中数组大小是: " << sizeof(arr) << endl;
}

int main()
{
      int arr[5] = {1, 2, 3, 4, 5};
      // 输出20（5个4字节的整数的大小）
      cout << "main函数中数组的大小：" << sizeof(arr) << endl;
      print_array_size(&arr);
      return 0;
}
C
print_array_size 函数输出8而不是期望的20（5个4字节整数），原因就是 arr 从5个整数的数组指针衰退成了整数的指针。同样的代码在 rust 中就没有问题：

use std::mem::size_of_val;

fn print_array_size(arr: [i32; 5]) {
      // 输出 20
      println!("print_array_size 函数中数组大小：{}", size_of_val(&arr));
}

fn main() {
      let arr: [i32; 5] = [1, 2, 3, 4, 5];
      // print 20
      println!("main函数中数组大小：{}", size_of_val(&arr));
      print_array_size(arr);
}
Rust
C/C++和Rust处理数组的另一个不同点是，Rust访问元素会进行边界检查。如下c++代码，我们想从长度为3的数组获取第5个元素。会产生未知行为:

#include <iostream>

using namespace std;

int main() 
{
      int arr[3] = {1, 2, 3};
      const auto index = 5;
      // arr[index] 是未知的行为
      cout << "Integer at index" << index << ": " << arr[index] << endl;
      return 0
}
C
而同样的代码在rust中会报错：

fn main() {
      let arr: [i32; 3] = [1, 2, 3];
      let index = 5;
      
      // arr[index] 提示如下错误信息:
      // index out of bounds: the len is 3 but the index is 5
      println!("Integer at index {}: {}", index, arr[index]);
}
Rust
你或许会疑惑，rust的处理方式怎么就比C++的好了？好吧，由于C++可以输出未知的行为，相当于给了编译器一张王牌通行证，可以以优化的名义做任何事。最严重的情况就是泄露信息给攻击者。

相反Rust总是会报错。并且，错误会直接终止程序运行，程序员也更方便发现并修复这些BUG。而C++无视这种 错误，如果不出错程序还继续使用错误数据正常执行。不管什么时候我都更顷向于选择Rust的报错而不是C/C++的未定义行为。

动态数组（Vectors）
数组最大的限制是它们的大小是固定的，而动态数组可以在运行时增加：

fn main() {
      // 初始化有三个元素的动态数组
      let mut v: Vec<i32> = vec![1, 2, 3];
      // 输出3
      println!("v有 {} 个元素", v.len());
      // 在运行时可以增加更多
      v.push(4);
      v.push(5);
      // 输出5
      println!("v有 {} 个元素", v.len());
}
Rust
动态数组是如何实现动态增长的呢？本质上动态数组将数组的所有元素存储在堆内存中。当增加新元素时， 动态数组会检查数组是否还有剩余空间，如果没有，动态数组会重新生成一个更大的数组，复制所有的元素到新数组并释放之前的数组空间。如下代码演示：

fn main() {
      let mut v: Vec<i32> = vec![1, 2, 3, 4];
      // 输出4
      println!("v 的容量是 {}", v.capacity());
      println!("v 中第一个元素的地址：{:p}", &v[0]); // {:p} 打印内存地址
      v.push(5);
      // 输出 8
      println!("v 的容量是 {}", v.capacity());
      println!("v 中第一个元素的地址：{:p}", &v[0]);
}
Rust
初始化时 v 背后的数组大小是4：

     |----------------|         |-------|
     | buffer pointer |-------->|   1   |
     |----------------|         |-------|
v -->|  capacity(4)   |         |   2   |
     |----------------|         |-------|
     |  length(4)     |         |   3   |
     |----------------|         |-------|
                                |   4   |
                                |-------|
Plaintext
随后插入一个新的元素，动态数组会复制所有元素到一个新的容量为8的数组中：

            栈内存                 堆内存              堆内存
     |----------------|         |-------|          |-------|
     | buffer pointer |--+      |   1   |      +-->|   1   |
     |----------------|  |      |-------|      |   |-------|
v -->|  capacity(4)   |  |      |   2   |      |   |   2   |
     |----------------|  |      |-------|      |   |-------|
     |  length(4)     |  |      |   3   |      |   |   3   |
     |----------------|  |      |-------|      |   |-------|
                         |      |   4   |      |   |   4   |
                         |      |-------|      |   |-------|
                         |        已回收        |   |   5   |
                         |                     |   |-------|
                         +---------------------+   |       |
                                                   |-------|
                                                   |       |
                                                   |-------|
                                                   |       |
                                                   |-------|
                                                     已分配
Plaintext
在给动态数组插入新元素的前后程序都会打印第一个元素的内存地址，两次输出的地址都是不同的。 第一个元素地址的变化很好的证明了会重新生成大小为8的数组。

注意： 如果内存地址没有变化，原因是数组的末尾有足够的空间保存新的数组内容。试着插入足够多的元素就会发现地址变化。可以查看C语言的函数 realloc 文档了解更多。

切片（Slices）
切片好比是数组和动态数组的临时视图。如下数组：

let arr: [i32; 4] = [10, 20, 30, 40];
Rust
可以创建一个包含第二个和第三个元素的切片如下：

let s = &arr[1..3];
Rust
[1..3]语句创建一个从下标1（包含）到3（不包含）的区间。如果省略区间的第一个数字（[..3]）它默认是零，如果省略后面那个数字默认是数组的长度。打印切片[1..3]的元素，就会输出20和30：

// 输出 20
println!("切片的第一个元素：{:}", s[0]);
// 输出 30
println!("切片的第二个元素: {:}", s[1]);
Rust
如果获取超出区间范围的元素就会发生 panic 报错：

// panics: index out of bounds
println!("切片的第三个元素: {:}", s[2]);
Rust
切片是如何知道它只有两个元素呢？原因就在于切片不是简单的数组指针，它还有一个额外的长度字段保存当前切片的元素个数。

注意： 一个指针除了指向对象的地址外如果还有额外的数据就是胖指针（fat pointer)。切片并不是rust中唯一一个胖指针。例如还有特性对象（trait object)，除了对象指针还有虚表指针（vtable pointer)。

如下创建一个动态数组的切片：

let v: Vec<i32> = vec![1, 2, 3, 4];
let s = &v[1..3];
Rust
除了指向数组v的第二个元素的指针，切片s还有一个8字节长的字段值为2:

           栈                                 堆
      |--------------|                   |--------|
      |buffer pointer|------------------>|    1   |
      |--------------|                   |--------|
v --> | capacity(4)  |       +---------->|    2   |
      |--------------|       |           |--------|
      |  length(4)   |       |           |    3   |
      |--------------|       |           |--------|
                             |           |    4   |
      |--------------|       |           |--------|
      |buffer pointer|-------+
s --> |--------------|
      | length(2)    |
      |--------------|
Plaintext
下面的代码也可以证明长度字段的存在，切片(&[i32])长度是16字节(8字节的缓冲区指针和8字节的长度字段)：

use std::mem::size_of;

fn main() {
      // prints 8
      println!("i32整形引用的大小是： {:}", size_of::<&i32>());
      // prints 16
      println!("i32整形切片大小是： {:}", sizeof::<&[i32]>());
}
Rust
数组切片也是一样的，但缓冲区指针不再是指向堆内存而是指向数组的栈内存。

切片借用底层的数据结构，所有的正常借用规则也都适用。如下代码编译器不能编译能过：

fn main() {
      let mut v: Vec<i32> = vec![1, 2, 3, 4];
      let s = &v[..];
      v.push(5);
      println!("切片的第一个元素: {:}", s[0]);
}
Rust
为什么? 当切片创建时，它指向的是动态数组的第一个元素地址，而当插入一个新元素时，会重新分配新的内存，老的内存就被释放。这时就导致切片指向无效的内存地址，如果访问无效数据就造成未定义行为。Rust又一次从灾难拯救了你。

注意：由于数组和动态数组都可以创建切片，它们是非常强大的抽象，因此对于函数参数，默认最好接收切片而不是数组或动态数组。事实上有很多函数如 len, is_empty等，都是处理切片而不是数组或动态数组。

结语
数组和动态数组是新手程序员首要都会学习的数据结构，Rust 原生支持它们也就不足为奇。如我们所见，Rust的安全性防止我们滥用这些基础数据类型。切片是rust的新奇概念，是非常有用的抽象，在rust代码库中被广泛使用。

著作权归作者所有。
商业转载请联系作者获得授权,非商业转载请注明出处。
原文: http://aqrun.oicnp.com/2020/10/25/array-vector-slice-in-rust.html