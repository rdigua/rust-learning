
# [Rust FFI 编程 - FFI 概述](https://rust.cc/article?id=3b8241d0-c4ca-4f49-8e07-0a5142b00f59)

[Mike Tang](https://rust.cc/blog_with_author?author_id=09e42b7c-c2bc-410a-9079-8ad0370d2603)  发表于  2020-04-09 21:03

Tags：rust,ffi

FFI（Foreign Function Interface）是这样一种机制：用一种编程语言写的程序能调用另一种编程语言写的函数（routines）。

一般来说，FFI 场景是在当前正在使用的语言（host）中，调用由其它语言（guest）提供的库。反过来的场景也是存在的，即，使用当前语言（host）写库，供其它语言（guest）调用。不过，后者这个场景，不是任何语言都能做到，有些语言即使能做，也会非常吃力。

## FFI 的历史和现状

FFI 这个术语最早来自 Common Lisp 的[规范](https://en.wikipedia.org/wiki/Foreign_function_interface#cite_note-1)。目前几乎所有严肃编程的语言都有提供 FFI 的支持，但大多数是单向功能。

不同语言称呼这种语言间调用的功能名字可能不同。Common Lisp、Haskell、Python、Rust 这些叫 FFI，Java 叫 JNI 或 JNA，还有一些其它语言叫 “绑定”。严格来说，FFI 与 绑定，意义并不相同，绑定可以理解为 FFI 中的一种实现。

不同语言实现 FFI 的方式不尽相同。有的语言，比如，要调用 C 库，必须用 C 语言，按那种语言的绑定规范，实现一个 C 项目，用 C 编译器编译并链接，生成库文件，再由这种语言调用（这种语言本身已经实现了加载其定义的规范 C 库的能力）。

有的语言，比如，Rust，要调用 C 库，不再需要使用 C 语言写绑定工程，而是直接使用 Rust 语言写。这样，就有个好处是，你不再需要掌握 C 语言的那么多的繁文缛节和工具链（但是还是必须懂 C 语言）。

## FFI 调用原理

为什么不同的语言之间能互相调用呢？

我们知道，计算机的运算，最底层的数据/代码都是以二进制的形式存在。所有的语言在编译后，都会以二进制的形式去执行（即使编译后的代码为字节码，虚拟机在运行的时候，也会继续翻译成 CPU 认识的二进制指令）。这就为不同语言间的调用提供了可能性。

但是，可能归可能。二进制毕竟太底层了。没有大家一致认可的[调用约定](https://zh.wikipedia.org/wiki/%E8%B0%83%E7%94%A8%E7%BA%A6%E5%AE%9A)，那也是不可能互通的。于是，ABI（应用程序二进制接口） 就出现了。调用约定，类型表示和名称修饰这三者的统称，即是众所周知的应用二进制接口（ABI）。

试想，如果所有的语言在调用时都能认识同样一套 ABI 规范，那么就能完全畅通的调用了。可惜，世界不会像我们人为想象的那样干净。

在计算机技术发展的过程中，出现了各种 ABI 规范，它们有的看起来相似，但在具体编译器的实现上，又有细微不同。所以，这是一件很麻烦的事情。大体来说，有如下规范：

-   cdecl
-   syscall
-   optlink
-   pascal
-   register
-   stdcall
-   fastcall
-   thiscall
-   winapi
-   Intel ABI
-   System V

等。详情可参考：[X86调用约定](https://zh.wikipedia.org/wiki/X86%E8%B0%83%E7%94%A8%E7%BA%A6%E5%AE%9A#cdecl)。

而 Rust 目前支持如下 ABI  [约定](https://doc.rust-lang.org/nomicon/ffi.html)：

-   stdcall
-   aapcs
-   cdecl
-   fastcall
-   vectorcall
-   Rust
-   rust-intrinsic
-   system
-   C
-   win64
-   sysv64

不过，值得庆幸的是，目前我们 IT 工业的基石，绝大部分是由 C 语言写成。于是自然而然，绝大多数库都遵循 cdecl（或 C）规范。所以我们可以专注于 C 规范来讨论问题。

## FFI 的困难之处

FFI 实现起来，比想像的要复杂许多，困难体现在：

-   如果 host 语言（调用主动方）带 GC（垃圾收集器），而 guest 语言（调用被动方）不带，那么可能会在资源管理（创建，释放）上面造成一些问题，需要特别细致地处理；
-   复杂对象或类型，在映射到两边的时候，可能会有一些不协调甚至失真的现象；
-   两边要同时引用一个可变对象的时候，可能会遇到问题；
-   如果两边的语言都是运行在 VM 之上的语言，那么这两个语言之间的直接 FFI 非常困难甚至不可能；
-   类型系统/对象组合模型/继承机制等其它细节，可能在跨语言的时候，成为障碍；
-   其它。

所以，虽然都能做 FFI，但是不同语言实现 FFI 的困难程度是不同的。

## 哪些语言可以方便地对外提供 FFI 库支持

可惜，大部分语言只能单向地“索取”。目前所知，能（较方便地）对其它语言提供 FFI 库支持的语言有：

-   C
-   C++（通过定义 C 接口）
-   Rust（通过使用 C 约定）
-   Ada
-   Fortran

小编能力所限，如有未列举完整之处，欢迎补充。

## 偷懒的程序员

在开发的过程中，要一个一个对大量的 C/C++ 库写绑定来进行 FFI，毕竟是一项费时费力的活儿。聪明的程序员们就开始构想一些“通用”的方案，实现批量快速绑定。

### SWIG

以下定义来自 https://zh.wikipedia.org/wiki/SWIG：

> 简单包装界面产生器(SWIG)是一个开源软件工具，用来将C语言或C++写的计算机程序或函式库，连接脚本语言，例如Lua, Perl, PHP, Python, R, Ruby, Tcl, 和其它语言，例如C#, Java, JavaScript, Go, D, OCaml, Octave, Scilab以及Scheme. 也可以输出成XML格式。

SWIG 官网：http://swig.org/ 。

### Gnome 社区关于构建通用 GI 规范的理想和实践

Gnome/Gtk 那一帮理想主义青年，发明了 GI（GObject Introspection）。用于对基于 glib/gobject 生态的众多软件（C 代码库）自动生成完整的接口描述文件（及 typelib），然后其它语言只要实现了对 Gir 这一个标准的支持，那么就可以无缝调用所有经过 Gir 化处理的 C 库。而不再需要单独为每一个 C 库做绑定了。这样就大大简化了 FFI 接口项目的编写工作。

目前这一杰出创意的重量级工作成果有 cairo, pango, gtk 等库。

更多信息请参考：https://gi.readthedocs.io/en/latest/。

## 基于字节码的平台级思路

另一种思路，建立一个共同的字节码平台，这个平台之上的所有语言，皆可便捷地相互调用。

### JVM 平台语言之间的 FFI

Java 发展到现在，已经形成了一个强大的 JVM 生态。JVM 平台上有大量的新语言产生，比如 Scala, Clojure, JRuby, Jython 等。这些语言前端不同，但是共享同一套 JVM 字节码和调用规范。因此，这些语言和 Java 之间，以及这些衍生语言之间，能比较容易地实现相互调用。

JVM 平台的缺点在于，其生态中的成果，被局限在了 JVM 平台内，无法（或很难）被其它语言平台所享用。

### WASM 平台的 FFI

Web Assembly（WASM）是一个新的字节码平台，其势头发展很猛。其有着比 JVM 平台更大的野心和联盟。因为是新设计的字节码，故其在设计的时候，就对 JVM 平台的一些问题做了规避（这方面可 Google 查阅相关资料）。

目前几乎所有主流语言都已实现将 WASM 作为编译目标，并且有相当一部分语言能够加载 WASM 库文件，调用其中的函数。不同的语言编译出的 WASM 效能和体积大小也是不同的。目前来看，C、C++、Rust 这些非 GC 语言能够编译出最精简，执行效率最高的 WASM 字节码。

WASM 的规范还在快速完善中。

## 结语

本篇描述了 FFI （外部程序接口）的概念和基本原理，并对其历史、内在的困难，以及程序员在 FFI 发展上的各种尝试，都做了简单介绍。

本篇大量内容参考 wikipedia 的 Foreign function interface  [页面](https://en.wikipedia.org/wiki/Foreign_function_interface)。

恕小编能力所限，如有描述不当或不完整之处，欢迎同行指正或补充，感谢！