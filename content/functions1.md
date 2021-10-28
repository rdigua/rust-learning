# How to Read Rust Functions, Part 1

Jan 26th, 2021 ·  [Guide](https://www.possiblerust.com/guide)  ·  [#functions](https://www.possiblerust.com/tags#functions)  · By  **Andrew Lilley Brinker**

Rust functions are surprisingly diverse, sitting at the intersection of multiple language features which may take time to understand. In this post, we’ll walk through those features and explain how they appear in function signatures, so you can be well-equipped to understand functions you see in the wild, or identify the best way to write the functions you need in your own code.

## Table of Contents

-   [Preface](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#preface)
-   [Signatures](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#signatures)
-   [Destructuring](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#destructuring)
    -   [Refutability](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#refutability)
    -   [Renaming](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#renaming)
-   [Bindings](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#bindings)
    -   [Move](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#owning-binding)
    -   [`mut`](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#mutable-owning-binding)
    -   [`ref`](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#reference-binding)
    -   [`ref mut`](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#mutable-reference-binding)
    -   [Binding vs. Type](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#binding-vs-type)
-   [Associated Functions](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#associated-functions)
    -   [Constructors](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#constructors)
    -   [Deref Collision & Smart Pointers](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#deref-collision--smart-pointers)
-   [Methods](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#methods)
    -   [Owning Receiver](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#owning-receiver)
    -   [Reference Receiver](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#reference-receiver)
    -   [Mutable Reference Receiver](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#mutable-reference-receiver)
    -   [Owning Pointer Receiver](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#owning-pointer-receiver)
    -   [Reference-Counted Pointer Receivers](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#reference-counted-pointer-receivers)
    -   [Pinned Receiver](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#pinned-receiver)
    -   [Nested Receivers](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#nested-receivers)
-   [Conclusion](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#conclusion)

## Preface

#### INFO 1  Before We Begin

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-info-1)

### Describing, not Recommending

This is a survey of what function signatures  _can_  look like in Rust, not a commentary on what they  _should_  look like. Any one of the patterns shown here may be seen in the wild, and learning to read other people’s code in any language is a valuable skill.

### Part of a Series

This is the first of a pair of posts describing how to read Rust function signatures. Part 2, tackling generic functions, is currently in the works.

## Signatures

First things first, we need a function signature. Let’s start with something basic.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-1)

```
fn example_1(x: i32, y: i32) -> i32;
//           ↑  ↑    ↑  ↑       ↑
//           |  |    |  |       |
//           | type  | type    type
//         pattern  pattern

```

CODE 1

A simple function signature example.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-1)

This is a function that takes in two 32-bit integers (the  `i32`  type), and returns a 32-bit integer as well. The arrow (`->`) indicates the return type.

Note that the types are all explicit. Rust  _does_  support type inference, but not for function signatures. All parameter and return types must be specified.

#### ASIDE 1:WHY NO TYPE INFERENCE FOR FUNCTIONS?[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#aside-1)

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-aside-1)

Rust could support type inference for functions, but chooses not to. To understand why, it’s useful to look at Haskell, which  _does_  offer type inference for functions. After years of experience, the Haskell community has strongly settled on annotating types anyway, because without them code is harder to read and type errors may be excessively generic or vague making them hard to debug. Rust, wanting to avoid this, requires all function parameter and return types to be explicit. The return type may only be omitted if the function does not return anything (in which case, its return type is an empty tuple,  `()`).

We can also see that the pattern comes before the type, separated by a colon. This  `<pattern>: <type>`  syntax matches Rust’s general syntax for  type ascription, which is the Rust term for specifying types explicitly.

## Destructuring

Notice too that I keep saying “pattern” instead of “variable name.” That’s because Rust parameters are  patterns, meaning you can destructure and bind against the internal structure of the types.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-2)

```
struct IpV4Address(u8, u8, u8, u8);

fn print_ipv4addr(IpV4Address(o1, o2, o3, o4): &IpV4Address) {
    println!("{}.{}.{}.{}", o1, o2, o3, o4);
}

fn main() {
    let addr = IpV4Address(127, 0, 0, 1);
    print_ipv4addr(&addr);
}

```

CODE 2

An example of a function with destructuring in its parameter list.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-2)

In this example, the  `IpV4Address`  type is destructured in the function signature, binding four variables (`o1`,  `o2`,  `o3`, and  `o4`) to the respective items in the tuple struct. These variables are then used in the function body to print the address.

### REFUTABILITY

One restriction of pattern matching in function signatures is that you can only use  irrefutable  patterns, meaning  _patterns that always match_. By contrast,  refutable  patterns may sometimes fail to match, perhaps because they specify only a single variant of an  `enum`  with multiple variants. Refutable patterns can be used in a  `match`  expression or equivalent construct, where the collection of patterns is checked for exhaustiveness (meaning all values are guaranteed to match at least one pattern), but patterns in function signatures are all alone, so they must be irrefutable.

#### ASIDE 2:PARTIAL FUNCTIONS[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#aside-2)

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-aside-2)

If Rust allowed the use of refutable patterns in function signatures, it would be allowing the creation of  _partial functions_, which are defined only for some subset of values of their input types. Other languages, like Haskell, do support partial functions, though their creation and use is considered a hazard and antipattern.

Note as well that partial functions are distinct from  _partial application_. Partial application is when a language includes some mechanism to provide fewer than the full list of inputs to a function, and returns a new function accepting the remaining inputs (the ones already provided have been “filled in”). Some languages, like Haskell, provide this as part of the language. Others, like C++ or Clojure, provide it with a library function (`std::bind`  in C++ or  `partial`  in Clojure).

Here’s a table to help explain:

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-table-1)

TABLE 1

A survey of pattern refutability.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#table-1)

Pattern

Refutability

`a`

Irrefutable

`_`

Irrefutable

`MyStruct { f1: x, f2: y }`

Irrefutable

`Some(b)`

Refutable

`Err(MyError::Foo)`

Refutable

The first three patterns could be found in a function signature. The last two could not.

### RENAMING

Another feature of declaring and assigning to new variables in a function signature pattern is that you can create a variable with a name different from the name of the field in the relevant type.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-3)

```
struct Something {
    field_1: i32,
    field_2: f64,
}

// The `field_1: x` and `field_2: y` parts are assigning
// the values of `field_1` to `x` and `field_2` to `y`.
fn func(Something { field_1: x, field_2: y }: Something) {
    println!("x: {}, y: {}", x, y);
}

fn main() {
    let x = Something {
        field_1: 5,
        field_2: 1.0,
    };
    
    func(x);
}

```

CODE 3

An example of renaming.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-3)

This can be useful when you want to locally use a different name than the name of the field, and do so in a single line rather than binding to a local variable with a new name in the body of the function.

## Bindings

As we use patterns to bind, we can also set the kind of binding, which may be the default (with no specifier)  `mut`,  `ref`, or  `ref mut`. Collectively, the binding options are as follows:

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-table-2)

TABLE 2

Binding syntax and the resulting types

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#table-2)

Binding Specifier

Mutability

Ownership

No specifier

Immutable

Owned

`mut`

Mutable

Owned

`ref`

Immutable

Borrowed

`ref mut`

Mutable

Borrowed

The  binding  pattern specifies how a binding should occur; should it be by-value (meaning in Rust that it either takes ownership of the bound value, or makes a copy of it, depending on whether the type of that value implements the  `Copy`  trait), by-reference, or by-mutable-reference?

### OWNING BINDING

An owning binding may display one of two behaviors, depending on the type involved in the binding. If the type implements the  `Copy`  trait, then it will be copied to the new owner at the binding site. If the type  _does not_  implement  `Copy`, then it will be moved from the prior owner to the new owner at the binding site.

#### ASIDE 3:POINTER CHASING[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#aside-3)

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-aside-3)

I’m borrowing the term “trivially copyable” from C++, where its definition is a little intricate, but the notion is the same in both C++ and Rust: a trivially copyable type can be safely copied with a  `memcpy`  call. This means no “pointer chasing.”

In this context, “pointer chasing” means needing to go through a pointer to free some memory at a location outside of the contiguous sequence of values you have on hand. If you have to chase pointers to copy a type (make a new copy of some value on the heap and update the pointer to go to the new location), then the type is not trivially copyable, and can’t implement the  `Copy`  trait.

To understand this, let’s talk about  `Copy`.  `Copy`  is a trait indicating a type is “trivially copyable,” meaning it can be copied with only a call to  `memcpy`, so all the data contained in the structure is contiguous; there are no pointers to chase.  `Copy`  tells us that copying a piece of data is fast.

At the same time, it improves ergonomics for certain types which may otherwise be tedious to use under Rust’s ownership semantics. Imagine if number types (which all implement  `Copy`) were moved any time they were assigned. Something as simple as  `x = y`  would invalidate  `y`, and thus make mathematical code much more frustrating to write.

So,  `Copy`  pulls double-duty. It tells us something is cheap to copy, and it permits that copying to be done implicitly.

In contexts where a type doesn’t implement  `Copy`  but  _does_  implement the  `Clone`  trait, you can instead call  `.clone()`  on it explicitly to create a duplicate which will be moved into the new owner, without invalidating the prior owner.

Now, what does this look like in the context of a binding?

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-3)

```
#[derive(Clone, Debug)]
struct Thing;

fn main() {
    let t = Thing;

    // This moves `t` into the `takes_ownership` function.
    // `t` no longer owns any data, and thus is invalid
    // to use again.
    takes_ownership(t);

    // If uncommented, this line would not compile.
    //
    // println!("{:#?}", t);
}

fn takes_ownership(x: Thing) {
    // `x` is the owner of whatever `Thing` is passed
    // into the `takes_ownership` function, and when
    // `x` goes out of scope at the end of the function,
    // any necessary cleanup of the data will be
    // performed (only relevant if the type in question
    // implements `Drop`).
}

```

CODE 3

An example of a broken use of owning binding in a function signature.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-3)

#### ASIDE 4:MORE ON BUILDERS[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#aside-4)

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-aside-4)

The  [official Rust style guide](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html)  has more information on the two variants of the builder pattern (“consuming,” and “non-consuming”) and explains when you’d want to use one or the other.

Sometimes this kind of binding is exactly what you want. For example, you may have a need for a “consuming builder,” one of two forms the Builder Pattern can take in Rust. In a consuming builder, the builder type passes ownership of some data to the type that it’s building, because that type will need ownership of the data to operate.

If you take ownership of a piece of data with an owning binding and want to return ownership to the calling context, you can return it from the function. For example, the popular  `once_cell`  crate features a type which can only be written to once. If you try to write to it again, it returns ownership of the value you attempted to set.

#### ASIDE 5:`once_cell`  AND THE STANDARD LIBRARY[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#aside-5)

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-aside-5)

The  `once_cell`  crate is very popular, and is being  [considered for inclusion in the Rust standard library](https://github.com/rust-lang/rfcs/pull/2788). In addition to its core  `OnceCell`  types (available in both thread-safe and non-thread-safe versions) it provides an alternative to the also-popular  `lazy_static`  crate.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-4)

```
impl<T> OnceCell<T> {
    /// Returns `value` as the `Result::Err` variant
    /// if the `OnceCell` has already been set.
    fn set(&self, value: T) -> Result<(), T> {
        // ...
    }
}

```

CODE 4

An example of returning ownership from the  `once_cell`  crate

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-4)

### MUTABLE OWNING BINDING

Sometimes, in addition to taking ownership of a piece of data, you’d like for that data to be mutable from the start as well. In that case, you can use a mutable owning binding.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-5)

```
#[derive(Copy, Clone)]
struct Number(i32);

fn main() {
    let x = Number(5);
    let y = Number(6);

    process_and_print_1(x);
    process_and_print_2(x);
}

// Declares `x` as mutable.
fn process_and_print_1(mut x: Number) {
    x = Number(10);
    println!("{}", x.0);
}

// Declares `x` as immutable.
fn process_and_print_2(x: Number) {
    // Then shadows it with a new (mutable)
    // binding of the same name.
    let mut x = x;

    x = Number(11);
    println!("{}", x.0);
}

```

CODE 5

Examples of mutable owning bindings.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-5)

#### ASIDE 6:SHADOWING[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#aside-6)

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-aside-6)

Shadowing, where one declaration of a variable hides the presence of another at a higher scope, is often considered an anti-pattern or is outright rejected in other languages, but in Rust it is often part of good, ergonomic code.

A common pattern in a Rust program is to process some piece of data repeatedly, perhaps going from a string containing some raw data to a processed form (a URI in a  `String`  into some  `Uri`  type). In that case, shadowing the old value makes sure it isn’t unintentionally used again, and it therefore considered good and ergonomic in Rust.

The use of a mutable owning binding can always be replaced with an immutable owning binding followed by a mutable rebinding to a variable of the same name in the body of the function (shadowing the parameter from that line onward). The choice is one of taste.

### REFERENCE BINDING

Reference bindings inside function signatures in Rust can seem a little unusual, but they are permitted. The idea is that the binding performed is a reference to the type of the value. If the value was passed in by value, then it’s either moved or copied as discussed in the owning binding section, and in the body of the function the value is of a reference to the post-move data (if the type is  `Copy`, the difference doesn’t amount to much). This is different from an owning binding of a reference type both for the caller of the function, and inside the function itself (`x: &Number`  is  _not_  the same as  `ref x: Number`).

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-6)

```
struct Number(i32);

fn main() {
    let x = Number(5);

    // This call moves `x` into the function body, making the
    // call to `print_addr_2` invalid. Switching the order of
    // the calls would resolve the compilation issue.
    print_addr_1(x);
    print_addr_2(&x);
}

// This moves the `Number` into the function body, then binds
// `x` as a reference to that data.
fn print_addr_1(ref x: Number) {
    println!("ref: {:p}", x);
}

fn print_addr_2(x: &Number) {
    println!("&:   {:p}", x);
}

```

CODE 6

Example of a reference binding with a non-reference type.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-6)

Reference bindings are more useful in the presence of a reference type, along with destructuring. In that case, they permit convenient access to bind-by-reference the internal fields of a type which has been passed by reference.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-7)

```
#[derive(Debug)]
struct Data;

#[derive(Debug)]
struct Holder(Data);

fn print_data(&Holder(ref data): &Holder) {
    println!("{:?}", data)
}

// This is equivalent, accessing the field inside
// the function body, instead of doing it in the
// signature.
fn print_data_2(data: &Holder) {
    let data = &data.0;
    println!("{:?}", data)
}

/*
// This example would fail to compile, because it
// tries to take ownership of the inner field of
// the reference type, and you can't take ownership
// through a borrow.
fn print_data_3(&Holder(data): &Holder) {
    println!("{:?}", data)
}
*/

fn main() {
    let holder = &Holder(Data);
    print_data(holder);
}

```

CODE 7

A reference binding with a reference type. Based on a code sample from  [“juggle-tux” on the Rust User Forum](https://users.rust-lang.org/t/ref-and-for-function-arguments/12790/5)

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-7)

### MUTABLE REFERENCE BINDING

Mutable reference bindings are similar to the above examples for immutable reference bindings, except they’re mutable.

Same as the other reference bindings, they may be considered surprising when used in the presence of a type passed by-value. When working instead with a type passed by reference, there is one additional thing to consider: you can’t get a mutable reference out of a value passed by immutable reference.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-8)

```
#[derive(Debug)]
struct Data;

#[derive(Debug)]
struct Holder(Data);

// You can't borrow data in an immutable reference as
// mutable, so this doesn't compile.
fn print_data(&Holder(ref mut data): &Holder) {
    println!("{:?}", data)
}

fn main() {
    let holder = &Holder(Data);
    print_data(holder);
}

```

CODE 8

Trying to take a field by mutable reference when it’s passed by immutable reference.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-8)

### BINDING VS. TYPE

Note as well that these bindings are relative to the type on the right hand side of the ascriptive clause. To explain, let’s see some examples, annotated with the resulting types.

#### ASIDE 7:A NOTE ON THIS CODE BLOCK’S FORMATTING[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#aside-7)

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-aside-7)

The parameter list for this function is formatted in an unusual way, to make clear where the inputs are mutable bindings and/or mutable or immutable references. In general, I recommend sticking with the formatting output by  `rustfmt`, which helps to standardize across projects and across the Rust language ecosystem.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-9)

```
fn do_numbery_things(
    mut x: &i32,
        y: &mut i32,
    mut z: &mut i32
) {
    // This compiles, but doesn't mutate the value of `x` in
    // the calling context, because it's a mutable binding to
    // an immutable reference, meaning we can change what
    // it's bound to (binding it to `&6` here), but not the
    // value of what it's bound to.
    x = &6;

    // For that reason, this wouldn't compile, as it attempts
    // to mutate a value that is behind an immutable reference.
    //
    // *x = 6;

    // This works, because `y` is an immutable binding to a
    // mutable reference, so you can freely mutate the
    // referenced value so long as Rust's aliasing XOR
    // mutability rule isn't violated.
    *y = 12;

    // This wouldn't compile, however, because `y` is an
    // immutable binding, so you can't change what it's
    // bound to.
    //
    // y = 0;

    // This works, because `z` is a mutable reference, so same
    // as `y`, you can mutate the value it's referencing.
    *z = 9;

    // However, this _doesn't_ work, but for a distinct reason.
    // `&mut 8` is a temporary value which stops existing when
    // the function ends, and for a mutable reference (unlike
    // an immutable reference), the value needs to exist past the
    // end of the function, in the calling context. That's not
    // possible for a temporary value, so this would fail to
    // compile.
    //
    // z = &mut 8;
}

fn main() {
    let mut x = 1;
    let mut y = 2;
    let mut z = 3;

    do_numbery_things(&x, &mut y, &mut z);

    println!("({}, {}, {})", x, y, z);

    // Prints "(1, 12, 9)" showing that `y` and `z`
    // were mutated by the function.
}

```

CODE 9

Examples of bindings in function signatures

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-9)

## Associated Functions

Associated functions  are functions which are “associated” with a type, meaning they live under the namespace of that type. Otherwise, they behave like normal functions.

### CONSTRUCTORS

Constructors, which usually return the associated type (called  `Self`, with an uppercase “S”) or some wrapper of it (like  `Result<Self, SomeErrorType>`  or  `Option<Self>`), are usually written as associated functions. It would be perfectly valid, for a typo  `Foo`, to write a constructor as a free function (meaning not associated with the type):

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-10)

```
#[derive(Debug)]
struct Foo {}

fn new_foo() -> Foo {
    Foo {}
}

fn main() {
    let my_foo = new_foo();
    println!("{:?}", my_foo);
}

```

CODE 10

Examples of a constructor written as a free function

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-10)

However, doing this isn’t ideal Rust style. Instead, you’d use an associated function, like so:

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-11)

```
#[derive(Debug)]
struct Foo {}

impl Foo {
    fn new() -> Self {
        Foo {}
    }
}

fn main() {
    let my_foo = Foo::new();
    println!("{:?}", my_foo);
}

```

CODE 11

Examples of a constructor written as an associated function

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-11)

Note that  `Foo::new`  has access to the  `Self`  type (which is most convenient for complex  `Self`  types), and is called as a path starting at the name of the type.

### DEREF COLLISION & SMART POINTERS

Another context where associated functions are commonly written is for  smart pointers, which are types which wrap another type while still being usable as if they were the original type. The most common smart pointer types in Rust are  `Box`,  `Rc`, and  `Arc`, and they all rely on a special trait called  `Deref`.  `Deref`  enables a feature in Rust called  deref coercion, which is used whenever a method call is made. Rust, at compile time, checks if the method is defined on the type it’s being called with,  _and_  on whatever type may be returned by that type’s  `Deref`  or  `DerefMut`  implementations (depending on the mutability of  `self`  in the method being checked), doing so for however many layers of deref-ing are available. This is what makes smart pointers easy to use in place of the original type!

However, because of deref coercion, defining methods on the smart pointer may make it difficult to call any methods on the contained type which have the same name. To avoid this collision, methods on smart pointer types are often defined as associated functions instead. The  `Rc`  type has multiple examples of this, with functions like  [`Rc::strong_count`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.strong_count)  (which returns the number of strong pointers to the underlying data currently live), being defined as associated functions.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-code-12)

```
/*
// This is the signature in the standard library.

impl<T> Rc<T> {
    // Notice the use of `this` instead of `self`,
    // making `strong_count` an associated function
    // instead of a method.
    pub fn strong_count(this: &Rc<T>) -> usize {}
}
*/

use std::rc::Rc;

fn main() {
    let five = Rc::new(5);
    let _also_five = Rc::clone(&five);
    
    println!("strong count is {}", Rc::strong_count(&five));
}

```

CODE 12

Example of an associated function on a smart pointer:  `Rc::strong_count`  from the standard library.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#code-12)

## Methods

Next, let’s look at methods in Rust.  Methods  are functions which are attached to a type, meaning they take a parameter called  `self`. These are distinct from associated functions syntactically by the presence of the “receiver.”

The  receiver  is  `self`, and represents the specific datum of the type on which the method is being called. The receiver can have a number of possible types, three of which come which special shorthand syntax because they are the most common options.

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-table-3)

TABLE 3

List of receiver types and syntactic sugar for them.

[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#table-3)

Receiver

Type

Shorthand

`self: Self`

Owning

`self`

`self: &Self`

Reference

`&self`

`self: &mut Self`

Mutable reference

`&mut self`

`self: Box<Self>`

Owning pointer

None

`self: Rc<Self>`

Reference counted pointer

None

`self: Arc<Self>`

Thread-safe reference counted pointer

None

`self: Pin<&mut Self>`

Pinned mutable reference

None

…

Nested combinations of any of the above

None

Each of these has their own distinct meaning, and it’s worthwhile to discuss when and why you’d use each of them.

### OWNING RECEIVER

Taking ownership of  `self`  means that, unless you pass ownership out to a new owner, the  `self`  object will be dropped at the end of the function, as its owner has gone out of scope. If the type implement the  `Drop`  trait, its  `Drop::drop`  implementation will be run to perform any deallocation or cleanup work necessary. Taking  `self`  by value is commonly used in situations like the Builder pattern, where you want to consume the builder and return whatever object it’s designed to build.

### REFERENCE RECEIVER

Taking  `self`  by reference means that  `self`  will be borrowed for the duration of the function call. Rust’s rules disallow simultaneous mutable and immutable borrows, so if a function takes  `self`  by reference, the caller will be unable to mutate the object until the function call ends.

### MUTABLE REFERENCE RECEIVER

Taking self by mutable reference means that  `self`  will be mutably borrowed for the duration of the function call. As always, Rust’s “aliasing XOR mutability” rule is in play.

#### ASIDE 8:BORROW SPLITTING[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#aside-8)

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-aside-8)

Rust knows that borrows of different fields of a struct are disjoint, but isn’t smart enough to make similar determinations for other kinds of types. The Rustonomicon (Advanced Rust Programming Language book), describes how to  [refactor data from one representation to another](https://doc.rust-lang.org/nomicon/borrow-splitting.html)  (“borrow splitting”) to ensure the Rust compiler can tell that two borrows are disjoint.

#### INFO 2  Disjoint Borrows and Partial Moves in Method Calls

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-info-2)

Whenever two or more pieces of data from a single struct are borrowed at the same time, Rust performs an analysis to see if the two borrows are  disjoint borrows. For example, it is perfectly fine to immutably borrow one field of a struct, and mutably borrow another field of the same struct at the same time, as those two fields are different.

This analysis can be stymied by hiding a borrow behind a method. If one or more of the borrows happens within a method of the outer type, then the borrows are no longer seen as disjoint, because the method on the outer type would take  `self`  by some sort of reference. From the perspective of the borrow checker, with the introduction of a method call, all of  `self`  is now borrowed at the same time as one of its fields is borrowed, and unlike the original case, these borrows are  _not_  disjoint, and do not pass borrow checking.

The same problem arises with  partial moves, where a field of a type is moved, but not the whole type. If a partial move is relocated to a method that takes ownership of  `self`, then the move is no longer partial in the calling context, which may cause a compilation error. Dr. David Pearce has  [a more in-depth guide to partial moves](https://whileydave.com/2020/11/30/understanding-partial-moves-in-rust/)  which explains them nicely.

The remaining receiver types are less common, but no less important.

### OWNING POINTER RECEIVER

First,  `Box<Self>`  indicates that you’re taking ownership of a pointer to  `self`. Most of the time this isn’t necessary, but one particular use case arises when working with  _unsized types_. Rust requires (and many CPU architectures require) function parameters to have sizes known at compile-time; because of this, special care must be taken with the treatment of types without a known size. Slices are one example, because they are an arbitrarily-sized view into a memory location, and trait objects are another, because the size of the actual data is hidden when the concrete type is erased as part of trait object construction. When implementing a method for an unsized type, taking a parameter as  `self: Self`  is invalid, because  `self: !Sized`  (this is the notation indicating that self does not implement the  `Sized`  trait). However, taking it as  `Box<Self>`  _is_  valid, because the size of the pointer is known at compile-time, and it’s now a pointer being passed instead of the underlying data.

#### INFO 3  Why Unsized Receivers Don’t Work

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-info-3)

Yandros, on the Rust User Forum, has a  [more thorough explanation of this subject](https://users.rust-lang.org/t/explanation-on-fn-self-box-self-for-trait-objects/34024/5), which I recommend reading for a deeper understanding.

As a summary: Rust generics are  monomorphized, meaning the compiler generates individual copies of generic code for each each unique set of types it’s called with. On most computer architectures, function calls require knowing the exact size of the parameters passed to them, and in the case of unsized types, that size is unknown. So monomorphizing generic code where  `Self`  doesn’t implement  `Sized`  doesn’t work in all cases, and is therefore rejected by the Rust compiler. Wrapping  `Self`  in a  `Box`  or other pointer makes the size known (it’s the size of a pointer type).

#### ASIDE 9:`Rc`,  `Arc`, & INTERIOR MUTABILITY[↺](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#aside-9)

[Skip this content.](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1#skip-aside-9)

`Rc`  and  `Arc`  _do not_  violate Rust’s aliasing XOR mutability rule by permitting multiple pointers to the same data, because they do not provide  _interior mutability_, which is when data can be mutable inside of an immutable context. The data inside of them is immutable, and while other types exist to provide interior mutability (like  `Mutex`,  `Cell`, or  `RefCell`), they do so in ways which maintain Rust’s safety guarantees.

### REFERENCE-COUNTED POINTER RECEIVERS

The two other pointer types are provided for similar reasons.  `Rc`  and  `Arc`  are respectively the not-thread-safe and thread-safe versions of a reference-counted pointer, and they provide the same value as a receiver type that  `Box`  does, with the addition of permitting multiple pointers to exist to the same data.

### PINNED RECEIVER

Then there’s  `Pin<&mut self>`.  `Pin`  is a type which indicates the data pointed to by the pointer inside of it never moves in memory (unless that data implements the  `Unpin`  trait, in which case it may be safely moved even when inside of a  `Pin`).  `Pin<&mut self>`  means that  `self`  is pinned, and may not move. The context you’re most likely to see this in Rust today is around the  `Future`  trait in the standard library, which defines a single method where the receiver type is  `Pin<&mut Self>`. Explaining  _why_  futures need pinning is a more involved topic though, so I recommend reading the  [Rust Async Book](https://rust-lang.github.io/async-book/04_pinning/01_chapter.html)  if you’re interested, as it’s covered in great detail and care there.

### NESTED RECEIVERS

Finally, you can nest any of these receiver types as well, so  `self: Box<Box<Self>>`  or  `self: Rc<Box<Pin<&mut Self>>>`  work as receiver types, although these are even less likely to be necessary than the un-nested versions we’ve just covered.

## Conclusion

This covers the basics of reading Rust functions. After reading this post, you should hopefully have a better understanding of some of the following concepts:

-   That the left-hand side of each parameter defined in a function signature is an irrefutable pattern which can feature destructuring, renaming, and one of four possible bindings.
-   That the right-hand sand of each parameter in a function signature is a type, which may be a reference or owning type, and that the selection of type interacts with the selection of binding to determine whether the actual parameter passed in the calling context is moved into the function, and whether the formal parameter inside the function is a reference or non-reference type.
-   That associated functions may be used to put functions inside of the namespace of a particular type, signaling their association to that type, and may include constructors or (in the case of smart pointers) functions which would normally be written as methods, but are written as associated functions to avoid possible naming conflicts due to deref coercion.
-   That methods may feature a number of different receiver types, which are selected based on the needs of the function and the future callers of it.