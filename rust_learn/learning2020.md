# learning --2020

## January --202001

### Rust裡怎麼使用建造者模式 builder pattern

[Read more](http://segfaultsourcery.s3-website.eu-central-1.amazonaws.com/snippets/rust/builder-pattern/landing.html)

### Rust編譯模型災難

文章作者Brian Anderson是Rust編程語言 及其姊妹項目Servo Web瀏覽器的共同創始人之一。

他現在在PingCAP擔任高級數據庫工程師。

他希望解決TiKV編譯緩慢的問題

在開發模式下進行完全重建可能需要15分鐘，在發布模式下可能需要30分鐘。

對於大型系統項目的開發人員來說，這聽起來可能並不那麼糟糕，

但是它比許多開發人員對現代編程環境所期望的要慢得多。

TiKV是一個相對較大的Rust代碼庫，

有200萬行程式碼。相比之下，Rust本身包含300萬行，而Servo則包含270萬行。

編程語言設計充滿了權衡利弊。這些基本選擇之一是runtime性能與編譯性能，

Rust團隊幾乎總是選擇runtime性能而不是編譯更快速。

如果快速編譯時間不是Rust設計的核心原則，那麼Rust的核心設計原則是什麼？這裡有一些：

實用性-它應該是一種可以在現實世界中使用的語言。

實用主義-它應該要讓人覺得可用，並且將其整合到之前的系統中。

內存安全性-它必須強制執行內存安全性，並且不能接受記憶體存取錯誤。

性能-它必須與C++在一樣快。

並發-它必須提供現代的解決方案來編寫並發代碼。

但這並不是說Rust設計師沒有在快速編譯時間中考慮任何因素。

但因為利弊的權衡，編譯器的性能還是愈來愈慢。

當作者每天使用Rust編譯器工作時，

電腦上至少擁有三份程式碼是很常見的，在其他所有版本都在構建和測試的同時。

我將開始在工作區1編寫程式，開始編譯，然後跳到工作區2，

開始在工作區2工作，編譯後再切換回工作區1。不斷進行在不同的工作區中切換。

雖然在2019年Rust的編譯速度有了提升，但目前Rust還是編譯的不夠快。

下一集會是作者如何優化Rust的編譯速度以達到產品經理的期待

[read more](https://bit.ly/2vD5I4v)

### google/evcxr 项目

`google/evcxr`  : A Jupyter Kernel for the Rust programming language.

The following example shows how you might provide a custom display function for a type Matrix.

```
use std::fmt::Debug;
pub struct Matrix<T> {pub values: Vec<T>, pub row_size: usize}
impl<T: Debug> Matrix<T> {
    pub fn evcxr_display(&self) {
        let mut html = String::new();
        html.push_str("<table>");
        for r in 0..(self.values.len() / self.row_size) {
            html.push_str("<tr>");
            for c in 0..self.row_size {
                html.push_str("<td>");
                html.push_str(&format!("{:?}", self.values[r * self.row_size + c]));
                html.push_str("</td>");
            }
            html.push_str("</tr>");
        }
        html.push_str("</table>");
        println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", html);
    }
}
let m = Matrix {values: vec![1,2,3,4,5,6,7,8,9], row_size: 3};
m

```

Reddit 上参与讨论：https://www.reddit.com/r/rust/comments/evrexn/evcxrevcxr_jupyter_jupyter_kernel_for_rust/

### 【博客】Rust 嵌入式项目——钢琴测量

此项目为了研究钢琴键与弹奏着的关系，给实验用的琴键安装了一组感应器，当机器手指用不用的力度按压琴键时，这些传感器会将琴键的行为进行视觉信息进行编码。最终的目的是把这些编码信息可以让人类识别并在电脑上演奏.  [项目详情](https://jitter.company/blog/2020/01/28/measuring-space-time-behaviours-of-piano-keys-with-rust/)

![image.png](https://i.loli.net/2020/01/30/L48oi9ZQqAYmE5M.png)

### Nushell 0.9.0

许多改进以及新的命令，[详情](https://www.nushell.sh/blog/2020/01/28/nushell-0_9_0.html)

### Bitfields Forever: Why we need a C-compatible Rust Crate

Rust中关于位运算编程的一篇综合性的文章，推荐阅读。

[https://immunant.com/blog/2020/01/bitfields/](https://immunant.com/blog/2020/01/bitfields/)

### Security By Design, A Brief Introduction To Rust

从原理性的层面，讲Rust这个语言出身的环境，要解决的问题，以及从安全性出发进行的设计。推荐阅读。

[https://medium.com/tadaweb/security-by-design-a-brief-introduction-to-rust-378060e45038](https://medium.com/tadaweb/security-by-design-a-brief-introduction-to-rust-378060e45038)

### diffeq - 基础常微分方程解法器

Basic Ordinary Differential Equation solvers。

[https://github.com/mattsse/diffeq](https://github.com/mattsse/diffeq)

### Units of Measure in Rust with Refinement Types

Rust在测量单位上的Refinement Types模式的开发尝试。

To this day, F### is the only mainstream programming language which provides first class support to make sure that you will not accidentally confuse meters and feet, euros and dollars, but that you can still convert between watts·hours and joules.

作者用Rust尝试了一下，发现还挺好使。不但好使，还挺快乐！

[https://yoric.github.io/post/uom.rs/](https://yoric.github.io/post/uom.rs/)

### 范例（CSWAP）

这是一个小电路的示例，其中两组寄存器在第三个寄存器之间交换。该电路非常小，只有三个操作加上一个测量值，因此，与之相比，样板看起来会很大，但是这种设置能够在电路变大时轻松、安全地构造电路。

```
use qip::*;

// Make a new circuit builder.
let mut b = OpBuilder::new();

// Make three registers of sizes 1, 3, 3 (7 qubits total).
let q = b.qubit();  // Same as b.register(1)?;
let ra = b.register(3)?;
let rb = b.register(3)?;

// We will want to feed in some inputs later, hang on to the handles
// so we don't need to actually remember any indices.
let a_handle = ra.handle();
let b_handle = rb.handle();

// Define circuit
// First apply an H to r
let q = b.hadamard(q);
// Then swap ra and rb, conditioned on q.
let (q, _, _) = b.cswap(q, ra, rb)?;
// Finally apply H to q again.
let q = b.hadamard(q);

// Add a measurement to the first qubit, save a reference so we can get the result later.
let (q, m_handle) = b.measure(q);

// Now q is the end result of the above circuit, and we can run the circuit by referencing it.

// Make an initial state: |0,000,001> (default value for registers not mentioned is 0).
let initial_state = [a_handle.make_init_from_index(0b000)?,
                     b_handle.make_init_from_index(0b001)?];
// Run circuit with a given precision.
let (_, measured) = run_local_with_init::<f64>(&q, &initial_state)?;

// Lookup the result of the measurement we performed using the handle, and the probability
// of getting that measurement.
let (result, p) = measured.get_measurement(&m_handle).unwrap();

// Print the measured result
println!("Measured: {:?} (with chance {:?})", result, p);

```

[Github仓库](https://github.com/Renmusxd/RustQIP "Github仓库")

[博客文章](https://docs.rs/qip/0.10.4/qip/ "博客文章")


#### ureq HTTP客户端库的未来

该库提供一个方便的具有最小的依赖关系树和明显的API的请求库。

ureq来自以用户需求为中心（或者也许是“人体工程学”？）库的想法。[SuperAgent](https://visionmedia.github.io/superagent/ "SuperAgent")是简单易用的API的一大灵感。这并不是说reqwest不容易使用，reqwest还是可以的。但是，面对简易API和高性能API之间的折衷，它又向“简易”迈进了多远呢？

Hyper是reqwest的主要支撑，其主要目标是“ 为Rust提供快速、正确的 HTTP 实现”。这有时会将重要信息“泄漏”给用户。

具有明确的“用户至上”理念的库可能仍然是一个好的出发点。将用户输入视为“让它起作用”的作用，而不是强制正确性。

前往[GitHub](https://github.com/algesten/ureq/blob/future/THOUGHTS.md "GitHub")阅读文章原文。

#### 部署容器运行时的Shim：交互式容器

容器只是孤立的Linux进程的幻想。每个进程都有一个`stdin`流从`stdout`  /  `stderr`流中读取输入数据，并将产生的输出打印到该输出中。容器也是如此。

从前面的文章中我们了解到，当我们创建一个容器时，其`stdout`和`stderr`会受到相应的运行时填充程序进程的控制。通常，这些流的内容将转发到容器日志文件。读者还可以注意到，容器的标准输入流只是默默地设置为`/dev/null`。

但是，如果我们想将一些数据发送到容器的`stdin`并在运行时将其`stdout`和/或`stderr`流返回该怎么办？至少在调试会话期间，这个工具就可能非常有用。

![](https://iximiuz.com/implementing-container-runtime-shim-3/interactive-containers-top-level-overview.png)

上面的图只是一个简化。由于Docker（或Kubernetes）分层设计，在流数据的方式上可能会有更多的中间组件，因此图上的容器管理器应被视为容器管理软件的相当高级的抽象。最接近图真实世界的设置将会是[crictl](https://github.com/kubernetes-sigs/cri-tools "crictl")（作为一个命令行客户端）与交互[CRI-O](https://github.com/cri-o/cri-o "CRI-O")  （作为CRI兼容的容器管理器）。

至少在以下情况下，我们可以发现在第三方应用的相同的交互式容器技术：

```
### Docker
docker run -i   ### or --interactive
docker attach   ### interactive by default
docker exec -i  ### or --interactive

### Kubernetes
kubectl run --stdin     ### or -i
kubectl run --attach
kubectl attach --stdin  ### or -i
kubectl exec --stdin    ### or -i

### ctr (containerd CLI)
ctr run  ### interactive by default

### CLI for kubelet CRI
crictl attach --stdin

```

前往[作者个人博客](https://iximiuz.com/en/posts/implementing-container-runtime-shim-3/?utm_medium=reddit&utm_source=r_rust "博客")浏览更多信息。

#### 在Rust中编写操作系统：分配器设计

[此篇文章](https://os.phil-opp.com/allocator-designs/ "此篇文章")解释了如何从头开始实现堆分配器。它提出并讨论了不同的分配器设计，包括凹凸分配，链表分配和固定大小的块分配。对于这三种设计中的每一种，我们将创建一个可用于内核的基本实现。

分配器的职责是管理可用的堆内存。它需要在`alloc`调用时返回未使用的内存，并跟踪释放的内存，`dealloc`以便再次使用它。最重要的是，它绝不能分发已经在其他地方使用的内存，因为这会导致不确定的行为。

除了正确性之外，还有许多次要设计目标。例如，分配器应有效地利用可用内存并使碎片减少。此外，它对于并发应用程序应能很好地工作，并可以扩展到任意数量的处理器。为了获得最佳性能，它甚至可以针对CPU缓存优化内存布局，以提高缓存位置并避免错误共享。

这些要求会使好的分配器非常复杂。例如，jemalloc具有超过30.000行代码。这种复杂性在内核代码中通常是不希望的，因为单个错误会导致严重的安全漏洞。幸运的是，与用户空间代码相比，内核代码的分配模式通常要简单得多，因此相对简单的分配器设计通常就足够了。

前往[博客文档](https://os.phil-opp.com/allocator-designs/ "博客文档")了解更多。

### 用 Rust 来诠释 Epoll, Kqueue 和 IOCP

这其实是一本书，旨在说明 Epoll，Kqueue 和 IOCP 的工作原理，我们可以将其用于高效率、高性能的 I/O。其中一些实现将会使用 rust，原文地址：https://cfsamsonbooks.gitbook.io/epoll-kqueue-iocp-explained/

扩展阅读：[Green Threads Explained in 200 Lines of Rust](https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust/)

reddit 上参与讨论：https://www.reddit.com/r/rust/comments/ephm4t/epoll_kqueue_and_iocp_explained_with_rust/

### Deadpool

`Deadpool`  是一个死的简单异步池，用于任何类型的连接和对象。

#### Example

```
use async_trait::async_trait;

#[derive(Debug)]
enum Error { Fail }

struct Computer {}
struct Manager {}
type Pool = deadpool::managed::Pool<Computer, Error>;

impl Computer {
    async fn get_answer(&self) -> i32 {
        42
    }
}

#[async_trait]
impl deadpool::managed::Manager<Computer, Error> for Manager {
    async fn create(&self) -> Result<Computer, Error> {
        Ok(Computer {})
    }
    async fn recycle(&self, conn: &mut Computer) -> deadpool::managed::RecycleResult<Error> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mgr = Manager {};
    let pool = Pool::new(mgr, 16);
    let mut conn = pool.get().await.unwrap();
    let answer = conn.get_answer().await;
    assert_eq!(answer, 42);
}

```

这个库还提供：Database connection pools，GitHub 地址：https://github.com/bikeshedder/deadpool

### factori

`Factori`  is a testing factory library inspired by Ruby's FactoryBot.

Example

`Factori`  provides two macros: factori!, which defines a factory for a type, and  `create!`  which instantiates it:

```
#[macro_use]
extern crate factori;

pub struct Vehicle {
    number_wheels: u8,
    electric: bool,
}

factori!(Vehicle, {
    default {
        number_wheels = 4,
        electric = false,
    }

    mixin bike {
        number_wheels = 2,
    }
});

fn main() {
    let default = create!(Vehicle);
    assert_eq!(default.number_wheels, 4);
    assert_eq!(default.electric, false);

    // Its type is Vehicle, nothing fancy:
    let vehicle: Vehicle = default;

    let three_wheels = create!(Vehicle, number_wheels: 3);
    assert_eq!(three_wheels.number_wheels, 3);

    let electric_bike = create!(Vehicle, :bike, electric: true);
    assert_eq!(electric_bike.number_wheels, 2);
    assert_eq!(electric_bike.electric, true);
}

```

### 如何你想深入探讨 ELF、x86指令、内存映射、gdb、动态加载程序等知识, 以下内容对你有用.

[Linux 可执行文件是什么？](https://fasterthanli.me/blog/2020/whats-in-a-linux-executable/)

[不使用 exec 运行可执行文件](https://fasterthanli.me/blog/2020/running-an-executable-without-exec/)

......

有兴趣的, 可以关注作者.

### 为什么Rust编译出的可执行文件那么大？

#rust #exe

为什么相同应用用Rust编译出的可执行文件比C编译出的要大？下面这篇文章详细阐述了原因，并提出了多重可行的方案，帮你减小可执行文件的体积，这几种方法是：

-   使用`--release`模式进行编译
-   在发布之前，开启LTO压缩二进制文件体积
-   如果你的应用不是内存密集型，使用系统分配器（需要nightly)
-   你可以开启编译优化等级s/z
-   还有一点建议对小的可执行文件效果不明显，但是你可以尝试UPX和其他可执行文件压缩，如果你的应用很大的话

[Read More](https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html?nsukey=0pCaOk1Jqd5C4VixWGd5tgKPJdT%2BZ0JBecpENw8OxAfDUNwoEn9uLbfSVdGlOi5PIs2B9IhRzfMsCx8zvneeOgjek%2Bkx4%2FR7f9RckVFUSUyrPgIo7Hwke%2BrRae6LN897PVrUF1IRCIHCN1m9cwhpVMxMhGERUJ8oN1f4W4%2BcI3AJ9yQ8zuVvWcftwoMCvVQJV9m%2FglpOiosgLKu0UBEfBg%3D%3D)

### postgres-query

这个crate提供了方便的宏和trait，可帮助编写SQL查询并将其结果收集到静态类型的结构中。

示例代码：

```
// Connect to the database
let client: Client = connect(/* ... */);

// Construct the query
let query = query!(
    "SELECT name, age FROM people WHERE age >= $min_age",
    min_age = 18
);

// Define the structure of the data returned from the query
#[derive(FromSqlRow)]
struct Person {
    age: i32,
    name: String,
}

// Execute the query
let people: Vec<Person> = query.fetch(&client).await?;

// Use the results
for person in people {
    println!("{} is {} years young", person.name, person.age);
}

```

[Github](https://github.com/nolanderc/rust-postgres-query)

### 新的分词器库tokenizers

分词器的核心是用Rust编写的。提供当今最常用的分词器的实现，重点是性能和多功能性。

示例代码

```
use tokenizers::tokenizer::{Result, Tokenizer, EncodeInput};
use tokenizers::models::bpe::BPE;

fn main() -> Result<()> {
	let bpe_builder = BPE::from_files("./path/to/vocab.json", "./path/to/merges.txt")?;
	let bpe = bpe_builder
		.dropout(0.1)
		.unk_token("[UNK]".into())
		.build()?;

	let mut tokenizer = Tokenizer::new(Box::new(bpe));

	let encoding = tokenizer.encode(EncodeInput::Single("Hey there!".into()))?;
	println!("{:?}", encoding.get_tokens());

	Ok(())
}

```

[Github](https://github.com/huggingface/tokenizers/tree/master/tokenizers)

### track_caller 錯誤處理大突破

`Option::{expect,unwrap}`  跟  `Result::{expect, expect_err, unwrap, unwrap_err}`  有  `#[track_caller]`  了

從

```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/da3629b05f8f1b425a738bfe9fe9aedd47c5417a/src/libcore/macros/mod.rs:16:40
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

```

變成

```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

```

[Read more](https://github.com/rust-lang/rust/pull/67887)

### canduma

该仓库包含样板 Rust 代码，用于通过 JWT 启动并快速运行 GraphQL 原型。

它使用  [actix-web](https://actix.rs/)，[Juniper](https://graphql-rust.github.io/juniper/current/)，[Diesel](https://diesel.rs/)  和  [jsonwebtoken](https://docs.rs/jsonwebtoken)。

Benchmarks with insert into PostgreSQL:

```
▶ ./bombardier -c 125 -n 10000000 http://localhost:3000/graphql -k -f body --method=POST -H "Content-Type: application/json" -s
Bombarding http://localhost:3000/graphql with 10000000 request(s) using 125 connection(s)

10000000 / 10000000 [===========================================================================] 100.00% 28777/s 5m47s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     28788.66    2183.47   34605.95
  Latency        4.32ms   543.07us   110.95ms
  HTTP codes:
    1xx - 0, 2xx - 10000000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    20.75MB/s

```

项目地址：https://github.com/clifinger/canduma

reddit 参与讨论：https://www.reddit.com/r/rust/comments/em8bx9/update_of_our_rust_boilerplate_server_with/

### RustZone: Writing Trusted Applications in Rust (Black Hat Asia 2018)

演讲中将探索使用 Rust 语言编写受信任的应用程序。 Rust 允许开发人员编写系统级代码，但提供安全性功能，包括内存安全性，类型安全性和错误处理。这些是开发受信任的应用程序的理想功能。油管地址：https://www.youtube.com/watch?v=5fxPuOrlE2I&feature=youtu.be

### Rust Belt Rust 2019 的视频已发布

Rust Belt Rust（http://www.rust-belt-rust.com/）是在美国 Rust Belt 地区举行的关于 Rust 编程语言的会议。 Rust Belt Rust 2019 于 10 月 18 日星期五和 10 月 19 日星期六在俄亥俄州代顿举行。再次感谢我们所有的与会者，演讲者和赞助商！油管地址：https://t.co/DTFoG1dDyr?amp=1

### 编写 Web server 及其以外的技巧

油管地址：https://youtu.be/ZDy71QtAQgs?list=PLgC1L0fKd7UkVwjVlOySfMnn80Qs5TOLb


### 改非同步沒什麼壓力啊

作者想講的是背壓的問題

Back Pressure

以作者的說法就是生產者消費者問題

生產者速度太快，消費者跟不上的壓力叫背壓

非同步就是把這個等待的時間壓力分散掉的一種方式

當消費者是OS或是其它程式時，用async await寫非同步

就跟寫同步程式一樣簡單

[Read more](https://lucumr.pocoo.org/2020/1/1/async-pressure/)

### 理解  `Tokio`, pt. 1

“我想了解 Tokio 的工作方式。我的兴趣涉及事物的实时性和并行性，但是我对 Tokio 本身并不了解。在引入异步和稳定的期货之前，我或多或少有意地避免学习它，这并不是毫无道理的认为 Tokio 是错的，但是只有有限的时间来学习东西，学习一些新东西是一项艰巨的任务。”

“因此我写下了这些学习 Tokio 的笔记。我没有计划如何学习它的内部原理，但是总的来说，当我有某种项目可以帮助我阅读时，我会学得最好。上下文确实有帮助。我不知道我要长期构建什么，但是一个 HTTP 负载生成器可以很好地工作，它可以扩展自身以找到服务器每秒可以处理的最大请求，同时仍然满足一些延迟约束。这确实意味着我需要将我的学习与另一个库 -  `hyper`  相结合，但是我以前使用过它，并且认为我可以将其保留为黑匣子。”

阅读原文：https://blog.troutwine.us/2019/12/31/understanding-tokio-pt1/

### Spinlocks Considered Harmful

In this post, I will be expressing strong opinions about a topic I have relatively little practical experience with, so feel free to roast and educate me in comments (link at the end of the post) :-)

Specifically, I’ll talk about:

-   spinlocks,
-   spinlocks in Rust with #[no_std],
-   priority inversion,
-   CPU interrupts,
-   and a couple of neat/horrible systemsy Rust hacks.

Read more ：https://matklad.github.io//2020/01/02/spinlocks-considered-harmful.html

reddit参与讨论：https://www.reddit.com/r/rust/comments/eis1tr/blog_post_spinlocks_considered_harmful/

### 用 Rust 重写 m4vgalib

作者用 Rust 重写了自己多年前用 C++ 写的视频生成库  [m4vgalib](https://github.com/cbiffle/m4vgalib). 介绍了 Rust 的一些优点，如包管理、API 设计、内存安全等.  
[详情](http://cliffle.com/blog/m4vga-in-rust/#rust-has-a-package-manager)

## Fabruary --202002


### 使用 WebAssembly 和 Rust 保护 Firefox

这是个很有意思的博文。保护个人的安全和隐私是 Mozilla 使命的核心原则，因此 Mozilla 不断努力使用户在线更加安全。 对于像 Firefox 这样的复杂且高度优化的系统，内存安全是最大的安全挑战之一。

Firefox 主要是用 C 和 C++ 编写的，而尽管 Firefox 中广泛使用沙箱（sandboxing）和 Rust，但它们都有其局限性。流程级沙箱可很好地用于大型的现有组件，但会消耗大量系统资源，因此必须谨慎使用。Rust 是轻量级的，但是重写数百万行的现有 C++ 代码是一个极其劳动密集型的过程。

所以 Mozilla 是如何使用 WebAssembly 和 Rust 保护 Firefox 的呢？  [请看原文](https://hacks.mozilla.org/2020/02/securing-firefox-with-webassembly/)：https://hacks.mozilla.org/2020/02/securing-firefox-with-webassembly/

[Reddit 上参与讨论](https://www.reddit.com/r/rust/comments/f9qk28/securing_firefox_with_webassembly_and_rust/):https://www.reddit.com/r/rust/comments/f9qk28/securing_firefox_with_webassembly_and_rust/

### Rust: 教你如何比sort|uniq快30倍

簡單好用的小技巧

[https://medium.com/adobetech/filtering-duplicates-on-the-command-line-30x-faster-than-sort-uniq-96ca5f7b4277](https://medium.com/adobetech/filtering-duplicates-on-the-command-line-30x-faster-than-sort-uniq-96ca5f7b4277)

### Rust: rustc profiler

profiler教學搭配官方維護的profiler

分別是 crox flamegraph summarize

有時間比較、火焰圖、函數時間圖

![image.png](https://i.loli.net/2020/02/27/OsZhHY2LSpfrVQi.png)  ![image.png](https://i.loli.net/2020/02/27/tvACRj5pBikOwUu.png)  ![image.png](https://i.loli.net/2020/02/27/mWDajrPKhMlZnGS.png)

[https://blog.rust-lang.org/inside-rust/2020/02/25/intro-rustc-self-profile.html](https://blog.rust-lang.org/inside-rust/2020/02/25/intro-rustc-self-profile.html)

### Rust的 Type-Driven 开发简介

这篇博客的目的是研究Rust的Type-Driven开发。Type-Driven开发是一种使用类型系统开发强大且经过验证的软件的方法。

博客原文：[https://medium.com/@11Takanori/introduction-to-type-driven-development-with-rust-6f8a767cc3df](https://medium.com/@11Takanori/introduction-to-type-driven-development-with-rust-6f8a767cc3df)

## Rust/WinRT即将到来

在过去五个月左右的时间里，团队一直在疯狂地研究Rust / WinRT，因此我团队在rust方面的努力仍在继续。我期待着尽快向社区开放。即使那样，这仍将是早期的日子，但仍有很多工作要做，我们基本上同意建立语言投影大约需要三年。自然地，这其中蕴含着十分大的价值。

仍然可以使用Rust / WinRT进行API调用，并且看到它们结合在一起非常令人满意。因此，我将带给您一些先睹为快的信息，以使您了解Rust中调用Windows API的外观。这是古老的`Windows.Foundation.Uri`类：

```
use windows::foundation::*;
 
let uri = Uri::create_uri("https://kennykerr.ca")?;
assert!(uri.domain()? == "kennykerr.ca");
assert!(uri.port()? == 443);
assert!(uri.to_string()? == "https://kennykerr.ca/");

```

这是使用`Windows.ApplicationModel.DataTransfer`命名空间将一些值复制到剪贴板的另一个示例：

```
use windows::application_model::data_transfer::*;
 
let content = DataPackage::new()?;
content.set_text("Rust/WinRT")?;
 
Clipboard::set_content(content)?;
Clipboard::flush()?;

```

这里我们调用了DataPackage的默认构造函数，但是Rust当然没有构造函数。因此，默认构造函数被常规的new方法替换。

最后，这是使用`Windows.UI.Composition API`的示例：

```
use windows::foundation::numerics::*;
use windows::ui::composition::*;
use windows::ui::*;
 
let compositor = Compositor::new()?;
let visual = compositor.create_sprite_visual()?;
let red = Colors::red()?;
assert!(red == Color { a: 255, r: 255, g: 0, b: 0 });
 
let brush = compositor.create_color_brush_with_color(red)?;
visual.set_brush(brush)?;
 
visual.set_offset(Vector3 { x: 1.0, y: 2.0, z: 3.0, })?;
assert!(visual.offset()? == Vector3 { x: 1.0, y: 2.0, z: 3.0 });

```

在这里您可以看到我们正在创建一个合成器。我们使用合成器使用红色笔刷创建一个精灵视觉效果，然后设置视觉效果的偏移量。这看起来很简单，但这证明了Rust / WinRT的开发已经进行了大量工作，以使其看起来像Rust一样自然和原生。Composition API是Windows API中仅有的两种类型层次结构之一，需要特别注意才能正确使用任何语言，更不用说缺乏传统继承的语言了。

Rust / WinRT允许您使用直接从描述API的规范元数据中即时生成的代码调用，现在和将来的任何 Windows API，然后直接进入您的Rust包，在其中您可以像调用另一个一样调用它们的rust模块。

博客原文：[https://kennykerr.ca/2020/02/22/rust-winrt-coming-soon/](https://kennykerr.ca/2020/02/22/rust-winrt-coming-soon/)

## Plotly for Rust

由Plotly JS支持的Rust绘图库。

#### Plotly的功能

绘制折现与散点图

```
extern crate plotly;
use plotly::charts::{Mode, Scatter};
use plotly::Plot;

fn line_and_scatter_plot() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .name("trace1")
        .mode(Mode::Markers);
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .name("trace2")
        .mode(Mode::Lines);
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12]).name("trace3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.show();
}

fn main() -> std::io::Result<()> {
    line_and_scatter_plot();
    Ok(())
}

```

![](https://github.com/igiagkiozis/plotly/raw/master/docs/images/line_and_scatter_plot.png)

```
extern crate plotly;
use plotly::charts::{Line, LineShape, Legend, Font};
use plotly::charts::Layout;
use plotly::charts::{Mode, Scatter};
use plotly::Plot;

fn line_shape_options_for_interpolation() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4, 5], vec![1, 3, 2, 3, 1])
        .mode(Mode::LinesMarkers)
        .name("linear")
        .line(Line::new().shape(LineShape::Linear));
    let trace2 = Scatter::new(vec![1, 2, 3, 4, 5], vec![6, 8, 7, 8, 6])
        .mode(Mode::LinesMarkers)
        .name("spline")
        .line(Line::new().shape(LineShape::Spline));
    let trace3 = Scatter::new(vec![1, 2, 3, 4, 5], vec![11, 13, 12, 13, 11])
        .mode(Mode::LinesMarkers)
        .name("vhv")
        .line(Line::new().shape(LineShape::Vhv));
    let trace4 = Scatter::new(vec![1, 2, 3, 4, 5], vec![16, 18, 17, 18, 16])
        .mode(Mode::LinesMarkers)
        .name("hvh")
        .line(Line::new().shape(LineShape::Hvh));
    let trace5 = Scatter::new(vec![1, 2, 3, 4, 5], vec![21, 23, 22, 23, 21])
        .mode(Mode::LinesMarkers)
        .name("vh")
        .line(Line::new().shape(LineShape::Vh));
    let trace6 = Scatter::new(vec![1, 2, 3, 4, 5], vec![26, 28, 27, 28, 26])
        .mode(Mode::LinesMarkers)
        .name("hv")
        .line(Line::new().shape(LineShape::Hv));

    let mut plot = Plot::new();
    let layout = Layout::new()
        .legend(Legend::new().y(0.5).trace_order("reversed")
            .font(Font::new().size(16)));
    plot.add_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    plot.show();
}

fn main() -> std::io::Result<()> {
    line_shape_options_for_interpolation();
    Ok(())
}

```

![](https://github.com/igiagkiozis/plotly/raw/master/docs/images/line_shape_options_for_interpolation.png)

了解其更多用法与工程源码请访问[GitHub仓库](https://github.com/igiagkiozis/plotly/tree/master)。

### 为什么Rust同时有String和&str?

[Read More](https://fasterthanli.me/blog/2020/working-with-strings-in-rust/)

### kibi 文本编辑器

用不超过 1024行 Rust 代码写的文本编辑器， 支持 UTF-8、增量搜索、语法高亮、行号等，欢迎贡献.  [详情](https://github.com/ilai-deutel/kibi)

### 给 Java 程序员的 Rust 教程

[全文](https://leshow.github.io/post/rust_for_java_devs/)

# 看nnethercote怎麼優化程式的

他利用Callgrind來看程式碼的執行時間

```
265,344,872 ( 2.97%)  :rustc::ty::query::on_disk_cache::__ty_decoder_impl
236,097,015 ( 2.64%)  :<rustc::ty::query::on_disk_cache::CacheEncoder<E>
213,551,888 ( 2.39%)  :rustc::ty::codec::encode_with_shorthand
165,042,682 ( 1.85%)  :<rustc_target::abi::VariantIdx
 40,540,500 ( 0.45%)  :<u32 as serialize::serialize::Encodable>::encode
 24,026,292 ( 0.27%)  :serialize::serialize::Encoder::emit_seq
 20,160,540 ( 0.23%)  :<rustc::dep_graph::serialized::SerializedDepNodeIndex
  9,661,323 ( 0.11%)  :serialize::serialize::Decoder::read_tuple
  4,898,927 ( 0.05%)  :<rustc::ty::query::on_disk_cache::CacheEncoder<E>
  3,384,018 ( 0.04%)  :<rustc_metadata::rmeta::encoder::EncodeContext
  2,296,440 ( 0.03%)  :<rustc::ty::UniverseIndex

```

一步一步的迭代 最後優化了11~13%

[read more](https://bit.ly/2OTrffR)

### 在 VSCode 中调试 Rust 程序

作者的这个博文基于上文提到的  [Boa](https://github.com/jasonwilliams/boa)  项目。我们可以有多种方法调试  [Boa](https://github.com/jasonwilliams/boa)  的操作，以此去了解它是如何工作的，甚至测试一些 javaScript 的代码。

了解具体的配置方法以及具体实现请看  [博文地址](https://jason-williams.co.uk/debugging-rust-in-vscode)：https://jason-williams.co.uk/debugging-rust-in-vscode

### Rust 零成本的抽象

零成本抽象的概念对于某些编程语言非常重要，比如 Rust 和 C++，这些语言的目的是使用户能够用相对较少的努力编写具有出色性能的程序。

作者认为他写的[这篇文章](https://carette.xyz/posts/zero_cost_abstraction/)正确地反映什么是零成本抽象. 实际上，零成本抽象(即“零开销”)是很难理解的, 也很难与其他编译器优化分离开来，并且很容易被误解.  [这篇博客文章中](https://carette.xyz/posts/zero_cost_abstraction/)，讨论了这个特性，并给出了 Rust 如何使用它来交付您的抽象项目的优化代码的示例.

[https://carette.xyz/posts/zero_cost_abstraction/](https://carette.xyz/posts/zero_cost_abstraction/)

### Pointer-utils: 指针工具集

a collection of crates, providing simple custom DSTs, pointer unions, borrowed reference counts, and more!

[https://github.com/CAD97/pointer-utils/blob/master/blog/Announcement.md](https://github.com/CAD97/pointer-utils/blob/master/blog/Announcement.md)

### Async Diesel

这个仓库简洁、有效地将  `Diesel`  集成到  `async-std`  中,如果你用 Rust 构建后端程序的时候想使用数据库连接池，可以考虑这种方式。

使用示例：

```
  
#[macro_use]
extern crate diesel;

use async_diesel::*;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::error::Error;
use uuid::Uuid;

// Schema
table! {
    users (id) {
        id -> Uuid,
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect
    let manager =
        ConnectionManager::<PgConnection>::new("postgres://postgres@localhost/async_diesel__test");
    let pool = Pool::builder().build(manager)?;

    // Add
    println!("add a user");
    diesel::insert_into(users::table)
        .values(users::id.eq(Uuid::new_v4()))
        .execute_async(&pool)
        .await?;

    // Count
    let num_users: i64 = users::table.count().get_result_async(&pool).await?;
    println!("now there are {:?} users", num_users);

    Ok(())
}

```

项目地址：https://github.com/mehcode/async-diesel

### Cross Compiling Rust for the Raspberry Pi

This guide covers how to set up your linux computer to  `compile`,  `upload`, and  `run`  a Rust binary on your Raspberry Pi.

read more:https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/

### 用rust实现操作系统教程

#rust #os

用rust实现操作系统的教程，rust官方还有一个用树莓派3实现操作系统的教程。

[Read More](https://os.phil-opp.com/)  [Repo](https://github.com/phil-opp/blog_os)

### 关于executor的博客文章

推荐一篇博客文章，介绍如何从头开始编写更现代，更干净的[juliex](https://github.com/withoutboats/juliex)（最小化executor，是Rust中最早支持async/await的其中之一）。

博客文章：[Build your own executor](https://stjepang.github.io/2020/01/31/build-your-own-executor.html)

## March --202003


## 剑桥大学技术报告 - 《ASAP：静态优先内存管理》

> ASAP: As Static As Possible memory management
> 
> [https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-908.pdf](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-908.pdf)

剑桥大学计算机实验室技术报告，《ASAP：静态优先内存管理》

## Rust concurrency: the single-writer principle

> Rust concurrency: the single-writer principle
> 
> [https://medium.com/@polyglot_factotum/rust-concurrency-the-single-writer-principle-applied-aada2cdc6fb0?source=friends_link&sk=cafc8dcf8babf4ec95b1b62ccde7e54b](https://medium.com/@polyglot_factotum/rust-concurrency-the-single-writer-principle-applied-aada2cdc6fb0?source=friends_link&sk=cafc8dcf8babf4ec95b1b62ccde7e54b)

Rust concurrency: the single-writer principle An example of applying the single-writer principle to a concurrent Rust system.

Rust分布式并发编程：single-writer原则，这篇文章解释怎么应用这个原则并给出代码例子。

[https://medium.com/@polyglot_factotum/rust-concurrency-the-single-writer-principle-applied-aada2cdc6fb0?source=friends_link&sk=cafc8dcf8babf4ec95b1b62ccde7e54b](https://medium.com/@polyglot_factotum/rust-concurrency-the-single-writer-principle-applied-aada2cdc6fb0?source=friends_link&sk=cafc8dcf8babf4ec95b1b62ccde7e54b)

[https://github.com/gterzian/single-writer](https://github.com/gterzian/single-writer)

### 一些对Rust初学者更实用的资源

-   [https://ferrous-systems.github.io/teaching-material/](https://ferrous-systems.github.io/teaching-material/)
-   [https://tourofrust.com/index.html](https://tourofrust.com/index.html)
-   [https://github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)
-   [https://github.com/rust-unofficial/patterns](https://github.com/rust-unofficial/patterns)


### Rust异步编程，一篇实用的介绍文章

这篇文章是针对异步编程的初学者的，文中包含了一个 Rust 异步编程的简短示例，使用的是 tokio runtime。作者希望通过这篇文章帮助人们更好地了解何时以及如何在 Rust 中进行异步编程。

[博客文章](http://jamesmcm.github.io/blog/2020/05/06/a-practical-introduction-to-async-programming-in-rust/#en)，http://jamesmcm.github.io/blog/2020/05/06/a-practical-introduction-to-async-programming-in-rust/#en

#### 博文：Macros vs Rename

[Macros vs Rename](https://rust-analyzer.github.io/blog/2020/03/30/macros-vs-rename.html)

#### 用Github workflow cross-compiling 多个Linux版本的rust可执行文件。

[用Github workflow cross-compiling 多个Linux版本的rust可执行文件](https://gist.github.com/FedericoPonzi/873aea22b652572f5995f23b86543fdb)

#### 自己动手写Web Assembly解析器（2）

[Let’s Write a Web Assembly Interpreter (Part 2)](https://medium.com/@richardanaya/lets-write-a-web-assembly-interpreter-part-2-6c430f3f4bfd)

#### 自己动手写Web Assembly解析器（1）

[Let’s Write a Web Assembly Interpreter (Part 1)](https://medium.com/@richardanaya/lets-write-a-web-assembly-interpreter-part-1-287298201d75)

### 上海科技大学GeekPie社团 WorkShop#7「关于Rust你需要了解的…」

#rust #workshop

B站视频回放地址：

-   [https://www.bilibili.com/video/BV1ti4y1b7xy/](https://www.bilibili.com/video/BV1ti4y1b7xy/)

三份 PPT 下载地址：

-   [https://c-t.work/s/74b9dcf657be4d](https://c-t.work/s/74b9dcf657be4d)
-   [https://cowtransfer.com/s/c98f417a076d48](https://cowtransfer.com/s/c98f417a076d48)  密码shanghaitech
-   [https://c-t.work/s/37a60fd0da9041](https://c-t.work/s/37a60fd0da9041)  密码shanghaitech

### 两篇关于「K-Rust ： Rust 可执行形式语义」的论文

#rust #paper

-   上科大宋老师：  [https://arxiv.org/abs/1804.10806](https://arxiv.org/abs/1804.10806)
-   Cyber Security Lab , NTU ：  [https://arxiv.org/abs/1804.07608](https://arxiv.org/abs/1804.07608)

### 使用no_std crate开发webAssembly

#rust #wasm

推荐阅读，使用no_std的 crate 开发webAssembly，通过开发web模拟器来实现嵌入式图形库举例说明。

[Read More](https://rahul-thakoor.github.io/using-no-standard-library-crates-with-webassembly/)

### 如何在Android设备上运行rust应用

#rust #android

[Read More](https://krupitskas.github.io/posts/quest-dev-part-2/)

### Rust 项目常用的 GitHub Actions

#rust #ci #cd

[@svartalf](https://twitter.com/svartalf)开发了一个网站，列举了rust项目常用的一些GitHub Action。

[Repo](https://actions-rs.github.io/)

### Rust编写操作系统系列：async/await

在本文中，我们将探讨协作式多任务处理以及Rust的async/await功能。

这篇[博客文章链接](https://os.phil-opp.com/async-await/)：https://os.phil-opp.com/async-await/

### PingCAP：使用 Go 工具快速在线查找 Rust 程序瓶颈

在线分析大型 Rust 应用程序很困难，目前常见的分析器无法胜任该工作。来自 PingCAP 官博的分享，介绍了他们在工程上是如何使用 go 工具分析 Rust 程序性能瓶颈的。详情请看原文：https://pingcap.com/blog/quickly-find-rust-program-bottlenecks-online-using-a-go-tool/

![](https://download.pingcap.com/images/blog/find-rust-program-bottlenecks-online-using-go-tool.png)

### 通过开发一个 JIRA 来学习 Rust

测试驱动的 Rust 学习项目，适合有其他语言编程经验的 Rust 新手. 在这个项目中，你可以通过一系列测试驱动的练习以及阅读材料来学习如何构建一个 JIRA，并在此过程中学习 Rust.

详情：[https://github.com/LukeMathWalker/build-your-own-jira-with-rust](https://github.com/LukeMathWalker/build-your-own-jira-with-rust)

### 对于国内程序员来说, 即能练习英文又能学习Rust, 是不是很Cool.

作者很努力的录制了大量Rust基础学习教程, 拼命代码例子进行讲解, 也可以练习听力, 推荐给大家:  [https://egghead.io/lessons/rust-integer-types-in-rust](https://egghead.io/lessons/rust-integer-types-in-rust)

### AWS的程序员: 马克·布鲁克, 写了一篇关于他最近2年使用Rust的一些心得

文章链接:  [http://brooker.co.za/blog/2020/03/22/rust.html](http://brooker.co.za/blog/2020/03/22/rust.html)

#### Actix中如何通过RustLS应用TLS和SNI(server name identification)

https://stephanheijl.com/rustls_sni.html

### 一篇分析Rust内存安全的论文

Rust是否可以实现内存安全的承诺？

[论文](https://arxiv.org/abs/2003.03296)

### Rust安全指南文档

文档的目的是进行安全的应用程序开发建议，同时利用Rust语言提供的各种可能性。

[指南](https://anssi-fr.github.io/rust-guide/)

### 一份Rust中`Option`和Haskell中`Maybe`的速查表

这是针对那些希望快速在`Option`值上找到相应函数名称的人。例如，对于Rust，在特定情况下使用哪一个？是`or_else`，`unwrap_or`还是`unwrap_or_else`？

[速查表](https://notes.iveselov.info/cheatsheet-rust-option-vs-haskell-maybe)

### 用Rust重写 Dropbox 同步引擎核心功能

Dropbox是最实用且免费的文件同步、备份、共享云存储软件, 同步引擎是桌面电脑上 Dropbox 文件夹背后的魔法，也是 Dropbox 最古老、最重要的代码之一。用Rust来对它进行重写, 足以展现出Rust强大的能力.

这篇文本[https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine](https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine)  详细说明了重写Dropbox同步引擎的过程, 值得大家阅读.

### Rust中的引用

Rust中的引用允许你使用值但不获取其所有权, 理解引用对于Rust使用者来说非常重要. 这里有一篇文章, 对引用解释的比较通俗易懂. 推荐给大家.  [https://blog.thoughtram.io/references-in-rust/](https://blog.thoughtram.io/references-in-rust/)

### 是优化？不是优化？关于对 COW 的深入思考

文章从 Copy-On-Write 的概念入手，探讨了 C++ 中的 COW，和 Rust 中的 COW 的设计。然后做了简单的性能评测，以及解释了 Rust 中的睿智设计。推荐阅读。

https://oribenshir.github.io/afternoon_rusting/blog/copy-on-write

### staticvec - 静态 Vec

静态 Vec 的意思就是非动态分配内存的 Vec。使用了预先分配的一定容量的内存。 它使用 const generics，基于一个 array 实现。

https://github.com/slightlyoutofphase/staticvec

### Out of the Box Dynamic Dispatch

Llogiq 大佬新出的文章。摸索出了一种小技巧，可以不使用 Box 指针来实现同样的动态分派。值得学习。

https://llogiq.github.io/2020/03/14/ootb.html

### Rust for gopher

😊给 Gopher 的一段 Rust 简单介绍，相信你会喜欢上 Rust 的！

视频地址请戳：https://youtu.be/eQjPvsmfIts

Reddit 上参与讨论：https://www.reddit.com/r/golang/comments/fgy1fo/a_short_introduction_to_rust_for_programmers/

### 关于 Rust 复杂结构体初始化的讨论

Rust 的复杂结构的初始化是比较头疼的问题。 目前有几种流行的解决方法，例如  `pub fn new（）`约定和  `builder`模式。在这篇博文中，我们将对它们进行比较，并介绍一种新的模式 -  `Init Struct Pattern`。

查看博客原文：https://xaeroxe.github.io/init-struct-pattern/

### 【博客】我对 Rust 和 .NET 的探索

作者从事于用 Rust 促进 .NET 开发的工作，现在他们的项目有点快成形的意思了但还有很多问题，所以他决定和社区的人介绍一下他们的工作并交流一下.

项目现在还没取好名字，也暂不开源，主要两部分组成：

-   将 rustc 中的 LLVM bitcode 转化为 .NET 程序集（assembly）的编译器
-   为其他 .NET 程序集聚合 Rust bindings 的工具，这样就可以用 Rust 代码来调用 .NET 库了.

这样一来 Rust 代码就可以调用 .NET 代码了，反过来也一样.

下面是一个 Rust 函数，它将一串数字的字符串字面量转化为一个 .NET 字符，然后再对其调用  `System.Int32.TryParse()`

```
fn int_tryparse_out_parm() -> bool {
    let s = "30579";
    let s_clr = System::Text::Encoding::UTF8().GetString_1(s.as_bytes());
    let mut result = 0;
    let b = System::Int32::TryParse_2(&s_clr, &mut result);
    return b && (result == 30579);
}

```

详情：[https://ericsink.com/entries/dotnet_rust.html](https://ericsink.com/entries/dotnet_rust.html)

### Evokit ：用于搜索的神经网络进化机器学习系统

详情：[https://github.com/etsy/Evokit](https://github.com/etsy/Evokit)

### Bottlerocket ： 基于 Linux 的操作系统

这个操作系统几乎所有新部件都是用 Rust来构建的.

详情：[https://github.com/bottlerocket-os/bottlerocket](https://github.com/bottlerocket-os/bottlerocket)


### 【视频】Rust NYC: Jon Gjengset - 让 unsafe 代码变得简单易懂

详情：[https://www.youtube.com/watch?v=QAz-maaH0KM](https://www.youtube.com/watch?v=QAz-maaH0KM)

### 对 Rust build pattern 的新思考 - Init Struct Pattern

文章浅显易懂，有一些有价值的思考，推荐阅读：

https://xaeroxe.github.io/init-struct-pattern/

### 一本关于Rust初学者的书

[@snoyberg](https://twitter.com/snoyberg)和[Miriam Snoyman](https://twitter.com/LambdaMom/)正在写一本关于Rust的书，现在可以填写[表单](https://docs.google.com/forms/d/e/1FAIpQLSeBgnFFXK22-HqP9rub59oHI4pZ1rAdBdsxRAJ23GyEAAd6eQ/viewform)申请试读。

[Read More](https://docs.google.com/forms/d/e/1FAIpQLSeBgnFFXK22-HqP9rub59oHI4pZ1rAdBdsxRAJ23GyEAAd6eQ/viewform)

# Rust: Nannou 更新了 WebGPU

Nannou是Rust的開源，創新的程式框架。

自發布以來，今天標誌著該項目最大的里程碑之一-版本0.13的發布。

WebGPU是一個GPU使用的跨平台標準

WebGPU正在所有主要瀏覽器中實現

代表以後Nannou也可以利用瀏覽器來畫出各種畫面

文章裡有例子

http://bit.ly/32Squtg

### Serverless + Rust 的尝试

I lightly documented my experience with Rust serverless using Cloudflare Workers.

TL;DR There is a lot of promise, but the overall state of Rust on serverless is pretty immature. This is likely to change in the next 12 months.

UPDATE: For Cloudflare workers you can access the Workers KV API directly using wasm_bindgen. This improves performance significantly. A full example can be found here: https://github.com/jRiest/the-best-goats/

reddit: https://www.reddit.com/r/rust/comments/fdmzyh/serverless_rust_i_tried_it_with_cloudflare_workers/

### 半小时学习 Rust

Rust 学起来不是很难么？半小时怎么可能...让我们一起来看看这位小哥写的博客，30 分钟速览 Rust 语法的概要，博客地址：https://fasterthanli.me/blog/2020/a-half-hour-to-learn-rust/

## [【Rust每周一知】Rust为什么会有String和&str？](https://rust.cc/article?id=08bc71ca-7aa1-4fce-93aa-614712430c66)

[洋芋](https://rust.cc/blog_with_author?author_id=207704d2-4f5e-4219-a631-6ab4ab4d8929)  发表于  2020-03-05 09:38

Tags：rust，每周一知

本文是Amos博客文章[“Working with strings in Rust”](https://fasterthanli.me/blog/2020/working-with-strings-in-rust/)的翻译。

人们选择Rust编程语言时总会遇到一个问题：为什么会有两种字符串类型？为什么会出现String和&str？

Amos在其另一篇文章["declarative-memory-management"](https://fasterthanli.me/blog/2019/declarative-memory-management)中部分回答了这个问题。但是在本文中又进行了一些实验，看看是否可以为Rust的做法“辩护”。文章主要分为C和Rust两大部分。

首先是C语言部分：

-   print程序示例
-   UTF-8编码
-   print程序处理UTF-8编码
-   传递字符串

### C语言的print程序示例

让我们从简单C程序开始，打印参数。

```
// in `print.c`

#include <stdio.h> // for printf

int main(int argc, char **argv) {
    for (int i = 0; i < argc; i++) {
        char *arg = argv[i];
        printf("%s\n", arg);
    }

    return 0;
}

```

```
$ gcc print.c -o print
$ ./print "ready" "set" "go"
./print
ready
set
go

```

好的！很简单。程序使用的是标准的`C11`主函数签名，该签名用`int`定义参数个数（`argc`，参数计数），和用`char**`或`char *[]`“字符串数组”定义参数（`argv`，参数向量）。然后，使用`printf`格式说明符`%s`将每个参数打印为字符串，其后跟`\n`换行符。确实，它将每个参数打印在自己的行上。

在继续之前，请确保我们对正在发生的事情有正确的了解。修改以上的程序，使用`%p`格式说明符打印指针！

```
// in `print.c`

int main(int argc, char **argv) {
    printf("argv = %p\n", argv); // new!
    for (int i = 0; i < argc; i++) {
        char *arg = argv[i];
        printf("argv[%d] = %p\n", i, argv[i]); // new!
        printf("%s\n", arg);
    }

    return 0;
}

```

```
$ gcc print.c -o print
$ ./print "ready" "set" "go"
argv = 0x7ffcc35d84a8
argv[0] = 0x7ffcc35d9039
./print
argv[1] = 0x7ffcc35d9041
ready
argv[2] = 0x7ffcc35d9047
set
argv[3] = 0x7ffcc35d904b
go

```

好的，`argv`是一个地址数组，在这些地址上有字符串数据。像这样：

![rust-string-argv1](https://raw.githubusercontent.com/lesterli/blockchain/master/images/rust_string_argv1.png)

`printf`的`%s`格式符怎么知道什么时候停止打印？因为它只获得一个地址，而不是起始地址和结束地址，或者起始地址和长度。让我们尝试自己打印每个参数：

```
// in `print.c`

#include <stdio.h> // printf

int main(int argc, char **argv) {
    for (int i = 0; i < argc; i++) {
        char *arg = argv[i];
        // we don't know where to stop, so let's just print 15 characters.
        for (int j = 0; j < 15; j++) {
            char character = arg[j];
            // the %c specifier is for characters
            printf("%c", character);
        }
        printf("\n");
    }

    return 0;
}

```

```
$ gcc print.c -o print
$ ./print "ready" "set" "go"
./printreadys
readysetgoCD
setgoCDPATH=.
goCDPATH=.:/ho

```

哦哦～我们的命令行参数相互“渗入”。让我们尝试将我们的程序通过管道`xxd`传输到一个十六进制的转储程序中，以查看发生了什么事：

```
$ # note: "-g 1" means "show groups of one byte",
$ # xxd defaults to "-g 2".
$ ./print "ready" "set" "go" | xxd -g 1
00000000: 2e 2f 70 72 69 6e 74 00 72 65 61 64 79 00 73 0a  ./print.ready.s.
00000010: 72 65 61 64 79 00 73 65 74 00 67 6f 00 43 44 0a  ready.set.go.CD.
00000020: 73 65 74 00 67 6f 00 43 44 50 41 54 48 3d 2e 0a  set.go.CDPATH=..
00000030: 67 6f 00 43 44 50 41 54 48 3d 2e 3a 2f 68 6f 0a  go.CDPATH=.:/ho.

```

啊啊！它们确实彼此跟随，但是两者之间有一些区别：这是相同的输出，用`^^`进行注释的位置是分隔符：

```
00000000: 2e 2f 70 72 69 6e 74 00 72 65 61 64 79 00 73 0a  ./print.ready.s.
          .  /  p  r  i  n  t  ^^ r  e  a  d  y  ^^ 

```

似乎每个参数都由值0来终止。确实，C具有以null终止的字符串。因此，我们可以“修复”我们的打印程序：

```
#include <stdio.h> // printf

int main(int argc, char **argv) {
    for (int i = 0; i < argc; i++) {
        char *arg = argv[i];
        // note: the loop condition is gone, we just loop forever.
        // well, until a 'break' at least.
        for (int j = 0;; j++) {
            char character = arg[j];

            // technically, we ought to use '\0' rather than just 0,
            // but even `gcc -Wall -Wextra -Wpedantic` doesn't chastise
            // us, so let's just go with it.
            if (character == 0) {
                break;
            }
            printf("%c", character);
        }
        printf("\n");
    }

    return 0;
}

```

```
$ gcc print.c -o print
$ ./print "ready" "set" "go"
./print
ready
set
go

```

一切都更好！虽然，我们也需要修复图：

![rust-string-argv2](https://raw.githubusercontent.com/lesterli/blockchain/master/images/rust_string_argv2.png)

> 提示：可能已经注意到，当我们的打印程序超出参数范围时，`CDPATH=.:/ho`也会显示出来。那是（一部分）环境变量！这些都在GNU C库glibc中程序参数旁边。但是具体细节不在本文讨论范围之内，需要查看制作自己的可执行打包程序系列。

好的！现在我们完全了解发生了什么，让我们做一些更有趣的事情：将参数转换为大写。因此，如果我们运行`./print hello`，它应该打印HELLO。我们也将跳过第一个参数，因为它是程序的名称，现在对我们而言这并不是很有趣。

```
#include <stdio.h> // printf
#include <ctype.h> // toupper

int main(int argc, char **argv) {
    // start from 1, skips program name
    for (int i = 1; i < argc; i++) {
        char *arg = argv[i];
        for (int j = 0;; j++) {
            char character = arg[j];
            if (character == 0) {
                break;
            }
            printf("%c", toupper(character));
        }
        printf("\n");
    }

    return 0;
}

```

```
$ gcc print.c -o print
$ ./print "hello"
HELLO

```

好的！太好了！在我看来功能齐全，可以发货了。出于谨慎考虑，让我们运行最后一个测试：

```
$ gcc print.c -o print
$ ./print "élément"
éLéMENT

```

哦～我们真正想要的是“ÉLÉMENT”，但显然，我们还没有弄清正在发生的一切。好的，也许现在大写字母太复杂了，让我们做些简单的事情：打印每个字符并用空格隔开。

```
// in `print.c`

#include <stdio.h> // printf

int main(int argc, char **argv) {
    for (int i = 1; i < argc; i++) {
        char *arg = argv[i];
        for (int j = 0;; j++) {
            char character = arg[j];
            if (character == 0) {
                break;
            }
            // notice the space following `%c`
            printf("%c ", character);
        }
        printf("\n");
    }

    return 0;
}

```

```
$ gcc print.c -o print
$ ./print "élément"
  l   m e n t 

```

不好了。这不会做，根本不会做。让我们回到最后一个行为良好的版本，该版本仅打印每个字符，中间没有空格，并查看输出的实际内容。

```
// in main
// in for
// in second for
            printf("%c", character); // notice the lack of space after `%c`

```

```
$ gcc print.c -o print
$ ./print "élément" | xxd -g 1
00000000: c3 a9 6c c3 a9 6d 65 6e 74 0a                    ..l..ment.
          ^^^^^    ^^^^^

```

如果正确阅读此信息，则“é”不是一个`char`，实际上是2个`char`。好像...很奇怪。

让我们快速编写一个`JavaScript`程序，并使用`Node.js`运行它：

```
// in `print.js`

const { argv, stdout } = process;

// we have to skip *two* arguments: the path to node,
// and the path to our script
for (const arg of argv.slice(2)) {
    for (const character of arg) {
        stdout.write(character);
        stdout.write(" ");
    }
    stdout.write("\n");
}

```

```
$ node print.js "élément"
é l é m e n t

```

啊! 好多了！`Node.js`能正确转换为大写吗？

```
// in `print.js`

const { argv, stdout } = process;

for (const arg of argv.slice(2)) {
    stdout.write(arg.toUpperCase());
    stdout.write("\n");
}

```

```
$ node print.js "élément"
ÉLÉMENT

```

它可以。让我们看一下十六进制转储：

```
$ node print.js "élément" | xxd -g 1
00000000: c3 89 4c c3 89 4d 45 4e 54 0a                    ..L..MENT.
          ^^^^^    ^^^^^

```

虽然`Node.js`程序行为与预期相同，但我们可以看到，`É`也与其他字母不同，“c3 a9”的大写字母对应为“c3 89”。

C程序没有正常工作，因为它将“c3”和“a9”独立对待，它应将其看作一个单一的“Unicode值”。为什么将“é”编码为“c3 a9”？现在是时候进行快速的UTF-8编码入门了。

### 快速的UTF-8入门

“abcdefghijklmnopqrstuvwxyz”，“ABCDEFGHIJKLMNOPQRSTUVWXYZ”和“123456789”以及“!@＃$％^＆*()”等字符都有对应的数字值。例如，“A”的数字值是65。为什么会这样呢？这是个惯例，计算机只知道数字，而我们经常使用字节作为最小单位，因此很久以前人们决定，如果一个字节的值为65，则它表示字母“A”。

由于ASCII是7位编码，因此它具有128个可能的值：0到127（含0）。但是在现代机器上，一个字节为8位，因此还有“另外”128个可能的值。大家都以为。我们可以在其中填充“特殊字符”：

![rust-string-cp437](https://raw.githubusercontent.com/lesterli/blockchain/master/images/rust_string_cp437.png)

不只是ASCII，而是ASCII加我们选择的128个字符。当然有很多语言，因此并非每种语言的非ASCII字符都可以容纳这些额外的128个值，因此对于那些大于127的值，有几种替代的解释。这些解释被称为“代码页”。上面的图片是Codepage 437，也称为CP437，OEM-US，OEM 437，PC-8或DOS Latin US。

如果不关心大写字母，那么对于法语这样的语言来说已经足够了。但是对所有东欧语言，这是不够的，甚至一开始没覆盖亚洲语言。因此，日本想出了自己的办法，他们用日元符号代替了ASCII的反斜杠，并用上划线代替了波浪号，并引入了双字节字符，因为有128个额外的字符对他们来说还不够。

对于使用小字母的语言，人们使用诸如Windows-1252之类的代码页已有多年了，西方世界中的大多数文本仍然有点像ASCII，也称为“扩展ASCII”。但是最终，世界集体开始整理他们的事务，并决定采用UTF-8，该UTF-8：

-   看起来像ASCII字符的ASCII（未扩展），并且使用相同的空格。
-   允许更多的字符，多字节序列。

在这之前人们会问：两个字节还不够吗？（或者是两个双字节字符的序列？），当然也可以是四个字节，但是最终，由于诸如紧凑性之类的重要原因，并为使大多数C程序保持half-broken而不是完全不可用，采用了UTF-8。

除了微软。他们做了，但感觉太少，太迟了。内部一切仍然是UTF-16。RIP。

那么，ASCII加多字节字符序列，它如何工作？相同的基本原理，每个字符都有一个值，因此在Unicode中，“é”的数字是“e9”，我们通常这样写“U+00E9”。0xE9是十进制，其大于127，所以它不是ASCII 233，而我们需要做多字节编码。

UTF-8如何进行多字节编码？使用位序列！

-   如果一个字节以110开头，则意味着我们需要两个字节
-   如果一个字节以1110开头，则意味着我们需要三个字节
-   如果一个字节以11110开头，则意味着我们需要四个字节
-   如果一个字节以10开头，则表示它是多字节字符序列的延续。

因此，对于具有“U+00E9”的“é”，其二进制表示形式为“11101001”，并且我们知道我们将需要两个字节，因此我们应该具有以下内容：

![string-utf8-encoding1](https://raw.githubusercontent.com/lesterli/blockchain/master/images/string_utf8_encoding1.png)

我们可以看到两个字节的UTF-8序列为我们提供11位存储空间：第一个字节为5位，第二个字节为6位。我们只需要8位，因此我们从右到左填充它们，首先是最后6位：

![string-utf8-encoding2](https://raw.githubusercontent.com/lesterli/blockchain/master/images/string_utf8_encoding2.png)

然后是剩下的2位：

![string-utf8-encoding3](https://raw.githubusercontent.com/lesterli/blockchain/master/images/string_utf8_encoding3.png)

其余的位填充零：

![string-utf8-encoding4](https://raw.githubusercontent.com/lesterli/blockchain/master/images/string_utf8_encoding4.png)

大功告成！0b11000011是0xC3和0b10101001是0xA9。与我们之前看到的相对应：“é”是“c3 a9”。

### 返回C的print程序

所以C程序，如果要真正分离字符，则必须进行一些UTF-8解码。我们仍然可以尝试自己做。

```
// in `print.c`

#include <stdio.h> // printf
#include <stdint.h> // uint8_t

void print_spaced(char *s) {
    // start at the beginning
    int i = 0;

    while (1) {
        // we're going to be shifting bytes around,
        // so treat them like unsigned 8-bit values
        uint8_t c = s[i];
        if (c == 0) {
            // reached null terminator, stop printing
            break;
        }

        // length of the sequence, ie., number of bytes
        // that encode a single Unicode scalar value
        int len = 1;
        if (c >> 5 == 0b110) {
            len = 2;
        } else if (c >> 4 == 0b1110) {
            len = 3;
        } else if (c >> 3 == 0b11110) {
            len = 4;
        }

        // print the entire UTF-8-encoded Unicode scalar value
        for (; len > 0; len--) {
            printf("%c", s[i]);
            i++;
        }
        // print space separator
        printf(" ");
    }
}

int main(int argc, char **argv) {
    for (int i = 1; i < argc; i++) {
        print_spaced(argv[i]);
        printf("\n");
    }

    return 0;
}

```

没有讨论String和&str。关于Rust字符串处理的文章却没有Rust代码，而且已经花了大约十分钟！

程序有效吗？

```
$ gcc print.c -o print
$ ./print "eat the rich"
e a t   t h e   r i c h 

```

```
$ ./print "platée de rösti"
p l a t é e   d e   r ö s t i 

```

```
$ ./print "23€ ≈ ¥2731"
2 3 €   ≈   ¥ 2 7 3 1 

```

```
$ ./print "text 🤷 encoding"
t e x t   🤷   e n c o d i n g

```

好吧，我不知道每个人都在抱怨什么，UTF-8超易实现，只花了我们几分钟时间，而且100％正确，符合标准，永远适用于所有输入，并且始终做正确的事。是吗？反例来了，考虑以下字符串：

```
$ echo "noe\\u0308l"
noël

```

这只是法国的圣诞节！当然，我们的程序可以解决此问题，而且不会费力：

```
$ ./print $(echo "noe\\u0308l")
n o e ̈ l 

```

哦哦～事实上，U+0308是“组合解析”，是“仅在前一个字符上打两个点”。实际上，如果需要，我们可以打更多的东西（以增加圣诞节的欢呼声）：

> 提示：显示单个“形状”的多个标量值的组合被称为“字素簇”，了解更多有关内容阅读Henri Sivonen的文章  ["🤦🏼‍♂️".length == 7](https://hsivonen.fi/string-length/)。

> 另外，由于作者Amos是法国人，整篇文章都带有Latin-1偏爱。了解更多有关内容阅读Manish Goregaokar的文章[Breaking Our Latin-1 Assumptions](https://manishearth.github.io/blog/2017/01/15/breaking-our-latin-1-assumptions/)。

因此，也许我们的程序并未实现UTF-8编码的所有微妙之处，但是我们已经接近了。我们现在暂时不考虑字符的组合，而将重点放在Unicode标量值上。我们想要的是：

-   解码我们的输入，将其从UTF-8转换为一系列Unicode标量值（我们将选择uint32_t）
-   将标量值转换为大写对应值
-   重新编码为UTF-8
-   打印到控制台

因此，让我们从一个decode_utf8函数开始。我们将只处理2个字节的序列：

```
// in `upper.c`

#include <stdio.h> // printf
#include <stdint.h> // uint8_t, uint32_t
#include <stdlib.h> // exit

void decode_utf8(char *src, uint32_t *dst) {
    int i = 0;
    int j = 0;

    while (1) {
        uint8_t c = src[i];
        if (c == 0) {
            dst[j] = 0;
            break; // null terminator
        }

        uint32_t scalar;
        int len;

        if (c >> 3 == 0b11110) {
            fprintf(stderr, "decode_utf8: 4-byte sequences are not supported!\n");
            exit(1);
        } if (c >> 4 == 0b1110) {
            fprintf(stderr, "decode_utf8: 3-byte sequences are not supported!\n");
            exit(1);
        } else if (c >> 5 == 0b110) {
            // 2-byte sequence
            uint32_t b1 = (uint32_t) src[i];
            uint32_t b2 = (uint32_t) src[i + 1];
            uint32_t mask1 = 0b0000011111000000;
            uint32_t mask2 = 0b0000000000111111;

            scalar = ((b1 << 6) & mask1) | ((b2 << 0) & mask2);
            len = 2;
        } else {
            // 1-byte sequence
            scalar = (uint32_t) c;
            len = 1;
        }
        dst[j++] = scalar;
        i += len;
    }
}

int main(int argc, char **argv) {
    uint32_t scalars[1024]; // hopefully that's enough
    decode_utf8(argv[1], scalars);

    for (int i = 0;; i++) {
        if (scalars[i] == 0) {
            break;
        }
        printf("U+%04X ", scalars[i]);
    }
    printf("\n");

    return 0;
}

```

```
$ gcc upper.c -o upper
$ ./upper "noël"
U+006E U+006F U+00EB U+006C 

```

从逻辑上讲，U+00EB应该是“ë”的代码位置，确实是的！

它的全名是“带Diaeresis的拉丁文小写字母E”。因此，现在我们只需要进行反向转换即可！

```
// in `upper.c`

void encode_utf8(uint32_t *src, char *dst) {
    int i = 0;
    int j = 0;

    while (1) {
        uint32_t scalar = src[i];

        if (scalar == 0) {
            dst[j] = 0; // null terminator
            break;
        }

        if (scalar > 0b11111111111) {
            fprintf(stderr, "Can only encode codepoints <= 0x%x", 0b11111111111);
            exit(1);
        }

        if (scalar > 0b1111111) { // 7 bits
            // 2-byte sequence

            uint8_t b1 = 0b11000000 | ((uint8_t) ((scalar & 0b11111000000) >> 6));
            //           2-byte marker              first 5 of 11 bits

            uint8_t b2 = 0b10000000 | ((uint8_t) (scalar & 0b111111));
            //           continuation               last 6 of 11 bits  

            dst[j + 0] = b1;
            dst[j + 1] = b2;
            j += 2;
        } else {
            // 1-byte sequence
            dst[j] = (char) scalar;
            j++;
        }

        i++;
    }
}

// omitted: decode_utf8

int main(int argc, char **argv) {
    uint32_t scalars[1024]; // hopefully that's enough
    decode_utf8(argv[1], scalars);

    for (int i = 0;; i++) {
        if (scalars[i] == 0) {
            break;
        }
        printf("U+%04X ", scalars[i]);
    }
    printf("\n");

    uint8_t result[1024]; // yolo
    encode_utf8(scalars, result);

    printf("%s\n", result);

    return 0;
}

```

```
$ gcc upper.c -o upper
$ ./upper "noël"
U+006E U+006F U+00EB U+006C 
noël

```

太棒了！现在，我们需要的只是某种转换表！从小写的代码位置到大写的对应值。我们将编写足以支持法语的内容：

```
#include <ctype.h> // toupper

int main(int argc, char **argv) {
    uint32_t scalars[1024]; // hopefully that's enough
    decode_utf8(argv[1], scalars);

    for (int i = 0;; i++) {
        if (scalars[i] == 0) {
            break;
        }
        printf("U+%04X ", scalars[i]);
    }
    printf("\n");

    // this is the highest codepoint we can decode/encode successfully
    const size_t table_size = 0b11111111111;
    uint32_t lower_to_upper[table_size];
    // initialize the table to just return the codepoint unchanged
    for (uint32_t cp = 0; cp < table_size; cp++) {
        lower_to_upper[cp] = cp;
    }
    // set a-z => A-Z
    for (int c = 97; c <= 122; c++) { // ha.
        lower_to_upper[(uint32_t) c] = (uint32_t) toupper(c);
    }

    // note: nested functions is a GNU extension!
    void set(char *lower, char *upper) {
        uint32_t lower_s[1024];
        uint32_t upper_s[1024];
        decode_utf8(lower, lower_s);
        decode_utf8(upper, upper_s);
        for (int i = 0;; i++) {
            if (lower_s[i] == 0) {
                break;
            }
            lower_to_upper[lower_s[i]] = upper_s[i];
        }
    }
    // set a few more
    set(
        "éêèàâëüöïÿôîçæœ",
        "ÉÊÈÀÂËÜÖÏŸÔÎÇÆŒ"
    );

    // now convert our scalars to upper-case
    for (int i = 0;; i++) {
        if (scalars[i] == 0) {
            break;
        }
        scalars[i] = lower_to_upper[scalars[i]];
    }

    uint8_t result[1024]; // yolo
    encode_utf8(scalars, result);

    printf("%s\n", result);

    return 0;
}

```

```
$ gcc upper.c -o upper
$ ./upper "Voix ambiguë d'un cœur qui, au zéphyr, préfère les jattes de kiwis"
U+0056 U+006F U+0069 U+0078 U+0020 U+0061 U+006D U+0062 U+0069 U+0067 U+0075 U+00EB U+0020 U+0064 U+0027 U+0075 U+006E U+0020 U+0063 U+0153 U+0075 U+0072 U+0020 U+0071 U+0075 U+0069 U+002C U+0020 U+0061 U+0075 U+0020 U+007A U+00E9 U+0070 U+0068 U+0079 U+0072 U+002C U+0020 U+0070 U+0072 U+00E9 U+0066 U+00E8 U+0072 U+0065 U+0020 U+006C U+0065 U+0073 U+0020 U+006A U+0061 U+0074 U+0074 U+0065 U+0073 U+0020 U+0064 U+0065 U+0020 U+006B U+0069 U+0077 U+0069 U+0073 
VOIX AMBIGUË D'UN CŒUR QUI, AU ZÉPHYR, PRÉFÈRE LES JATTES DE KIWIS

```

### 传递字符串

首先，是C程序，C很容易！只需使用`char *`。

```
// in `woops.c`

#include <stdio.h>

int len(char *s) {
    int l = 0;
    while (s[l]) {
        l++;
    }
    return l;
}

int main(int argc, char **argv) {
    char *arg = argv[1];
    int l = len(arg);
    printf("length of \"%s\" = %d\n", arg, l);
}

```

```
$ # we're back into the parent of the "rustre" directory
$ # (in case you're following along)
$ gcc woops.c -o woops
$ ./woops "dog"
length of "dog" = 3

```

看到？简单！没什么`String/&str`。回到现实。首先，这实际上不是字符串的长度。它是使用UTF-8对其进行编码所需的字节数。因此，例如：

```
$ ./woops "née"
length of "née" = 4

$ ./woops "🐈"
length of "🐈" = 4

```

我们不会花费一半的文章来实现UTF-8解码器和编码器，只是感到惊讶的是，我们无法正确地计算字符数。而且，那不是现在困扰我的事情。现在困扰我的是，编译器没有采取任何措施阻止我们执行此操作：

```
#include <stdio.h>

int len(char *s) {
    s[0] = '\0';
    return 0;
}

int main(int argc, char **argv) {
    char *arg = argv[1];
    int l = len(arg);
    printf("length of \"%s\" = %d\n", arg, l);
}

```

```
$ gcc woops.c -o woops
$ ./woops "some user input"
length of "" = 0

```

`len()`是正确的，将通过单元测试。通过它执行完成时，字符串的长度是零。如果没有人愿意去看`len`函数本身，例如，如果它在第三方库中，或更糟的是在专有的第三方库中，那么调试将很有趣。当然，C有`const`：

```
int len(const char *s) {
    s[0] = '\0';
    return 0;
}

```

但它不会通过编译：

```
woops.c: In function ‘len’:
woops.c:4:10: error: assignment of read-only location ‘*s’
    4 |     s[0] = '\0';
      |    

```

修改下：

```
int len(const char *s) {
    char *S = (void *) s;
    S[0] = '\0';
    return 0;
}

```

现在它再次通过编译，运行它，它会默默地覆盖我们的输入字符串，就像之前一样。

接下来是Rust程序部分：

-   print程序
-   错误处理
-   迭代
-   传递字符串转换成大写
-   索引

### print程序

让我们看看实现打印参数，Rust程序是怎样实现的：

```
$ cargo new rustre
     Created binary (application) `rustre` package
$ cd rustre

```

```
fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");
    println!("{}", arg.to_uppercase());
}

```

以上内容的说明：`std::env::args()`返回一个`Iterator`字符串。`skip(1)`忽略程序名称（通常是第一个参数），`next()`获取迭代器中的下一个元素（第一个“实际”）参数。可能有下一个参数，也可能没有。如果没有，`.expect(msg)`通过停止程序打印`msg`。如果有，就有了一个`Option<String>`！

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rustre`
thread 'main' panicked at 'should have one argument', src/libcore/option.rs:1188:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

```

好的！因此，当我们不传递参数时，运行程序会有如上输出。让我们传递一些测试字符串：

```
$ cargo run --quiet -- "noël"
NOËL
$ cargo run --quiet -- "trans rights"
TRANS RIGHTS
$ cargo run --quiet -- "voix ambiguë d'un cœur qui, au zéphyr, préfère les jattes de kiwis"
VOIX AMBIGUË D'UN CŒUR QUI, AU ZÉPHYR, PRÉFÈRE LES JATTES DE KIWIS
$ cargo run --quiet -- "heinz große"
HEINZ GROSSE

```

一切都测试了！最后一个特别酷，用德语的“ß”，确实是“ss”的连字。好吧，这很复杂，但这就是要点。

### 错误处理

因此Rust的行为就像字符串是UTF-8一样，这意味着它必须在某个时刻解码我们的命令行参数，意味着这可能会失败。但是，只在没有参数的情况下看到错误处理，而对于参数无效的UTF-8则看不到错误处理。什么是无效的UTF-8？好吧，我们已经看到“é”被编码为“c3 e9”，所以可以这样工作：

```
$ cargo run --quiet -- $(printf "\\xC3\\xA9")
É

```

我们已经看到一个双字节的UTF-8序列具有：

-   在第一个字节中指示它是一个双字节的序列（前三个位，110）
-   在第二个字节中指示它是多字节序列的延续（前两个位10）

如果我们开始读取一个双字节的序列，然后突然停止怎么办？如果我们传入了C3，但未传入A9呢？

```
$ cargo run --quiet -- $(printf "\\xC3")
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "\xC3"', src/libcore/result.rs:1188:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

```

查看错误堆栈信息。

```
$ RUST_BACKTRACE=1 cargo run --quiet -- $(printf "\\xC3")
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "\xC3"', src/libcore/result.rs:1188:5                                                
stack backtrace:
(cut)
  13: core::result::unwrap_failed
             at src/libcore/result.rs:1188
  14: core::result::Result<T,E>::unwrap
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/result.rs:956
  15: <std::env::Args as core::iter::traits::iterator::Iterator>::next::{{closure}}
             at src/libstd/env.rs:789
  16: core::option::Option<T>::map
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/option.rs:450
  17: <std::env::Args as core::iter::traits::iterator::Iterator>::next
             at src/libstd/env.rs:789
  18: <&mut I as core::iter::traits::iterator::Iterator>::next
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/iter/traits/iterator.rs:2991
  19: core::iter::traits::iterator::Iterator::nth
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/iter/traits/iterator.rs:323
  20: <core::iter::adapters::Skip<I> as core::iter::traits::iterator::Iterator>::next
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/iter/adapters/mod.rs:1657
  21: rustre::main
             at src/main.rs:2
(cut)

```

基本上是这样：

-   在`main()`
-   我们调用`Iterator`的`.next()`
-   最后调用`Result`的`.unwrap()`
-   此时panicked

这意味着只有当我们尝试将参数作为String获取时，它才会出现panic。如果我们将其作为`OsString`，就不会panic：

```
fn main() {
    let arg = std::env::args_os()
        .skip(1)
        .next()
        .expect("should have one argument");
    println!("{:?}", arg)
}

```

```
$ cargo run --quiet -- hello
"hello"
$ cargo run --quiet $(printf "\\xC3")
"\xC3"

```

但是它没有`.to_uppercase()`方法。因为它是一个`OsString`，它是一系列字节。C程序如何处理无效的UTF-8输入？

```
$ ../upper $(printf "\\xC3")
U+00C0 U+0043 U+0044 U+0050 U+0041 U+0054 U+0048 U+003D U+002E U+003A U+002F U+0068 U+006F U+006D U+0065 U+002F U+0061 U+006D U+006F U+0073 U+002F U+0072 U+0075 U+0073 U+0074 U+003A U+002F U+0068 U+006F U+006D U+0065 U+002F U+0061 U+006D U+006F U+0073 U+002F U+0067 U+006F U+003A U+002F U+0068 U+006F U+006D U+0065 U+002F U+0061 U+006D U+006F U+0073 U+002F U+0066 U+0074 U+006C U+003A U+002F U+0068 U+006F U+006D U+0065 U+002F U+0061 U+006D U+006F U+0073 U+002F U+0070 U+0065 U+0072 U+0073 U+006F U+003A U+002F U+0068 U+006F U+006D U+0065 U+002F U+0061 U+006D U+006F U+0073 U+002F U+0077 U+006F U+0072 U+006B 
ÀCDPATH=.:/HOME/AMOS/RUST:/HOME/AMOS/GO:/HOME/AMOS/FTL:/HOME/AMOS/PERSO:/HOME/AMOS/WORK

```

答案是：不好。实际上一点也不好。UTF-8解码器首先读取C3，然后读取下一个字节（是空终止符），结果应为“à”。但它不再停下来，而是读完参数末尾，直接进入环境块，找到第一个环境变量。现在，在这种情况下，这似乎很温和。但是如果该C程序被用作Web服务器的一部分，并且其输出直接显示给用户怎么办？如果第一个环境变量不是`CDPATH`，而是  `SECRET_API_TOKEN`怎么办？那将是一场灾难。

但如果命令行参数是无效的UTF-8，Rust程序就会尽早panic。如果想优雅地处理这种情况怎么办？可以使用`OsStr::to_str`，它返回一个`Option`值。

```
fn main() {
    let arg = std::env::args_os()
        .skip(1)
        .next()
        .expect("should have one argument");

    match arg.to_str() {
        Some(arg) => println!("valid UTF-8: {}", arg),
        None => println!("not valid UTF-8: {:?}", arg),
    }
}

```

```
$ cargo run --quiet -- "é"
valid UTF-8: é
$ cargo run --quiet -- $(printf "\\xC3")
not valid UTF-8: "\xC3"

```

精彩。我们学到了什么？

在Rust中，只要你不明确地用`unsafe`，类型`String`的值永远是有效的UTF-8。如果尝试使用无效的UTF-8构建`String`，则会出现错误。一些程序，像`std::env::args()`会隐藏错误处理，因为错误的情况非常少。但它仍然会检查错误，并会检查是否发生错误，因为这样做是安全的。

相比之下，C没有字符串类型。它甚至没有真正的字符类型。char是一个ASCII字符加上一个附加位，实际上，它只是一个带符号的8位整数：`int8_t`。绝对不能保证`char *`其中的任何内容都是有效的UTF-8。没有与`char *`关联的编码，只是内存中的地址。也没有关联的长度，计算其长度涉及找到空终止符。空终止字符也是一个严重的安全问题，更不用说NUL是有效的Unicode字符，因此以空字符结尾的字符串不能表示所有有效的UTF-8字符串。

### 迭代 Iteration

我们将如何用空格分隔字符？

```
fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    for c in arg.chars() {
        print!("{} ", c);
    }
    println!()
}

```

```
$ cargo run --quiet -- "cup of tea"
c u p   o f   t e a 

```

很简单！让我们尝试使用非ASCII字符：

```
$ cargo run --quiet -- "23€ ≈ ¥2731"
2 3 €   ≈   ¥ 2 7 3 1 
$ cargo run --quiet -- "memory safety 🥺 please 🙏"
m e m o r y   s a f e t y   🥺   p l e a s e   🙏 

```

一切似乎都很好。如果我们要打印Unicode标量值的数字而不是它们的字形，该怎么办？

```
fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    for c in arg.chars() {
        print!("{} (U+{:04X}) ", c, c as u32);
    }
    println!()
}

```

```
$ cargo run --quiet -- "aimée"
a (U+0061) i (U+0069) m (U+006D) é (U+00E9) e (U+0065)

```

酷！如果我们想显示其为UTF-8编码怎么办？我的意思是打印单个字节？

```
fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    for b in arg.bytes() {
        print!("{:02X} ", b);
    }
    println!()
}

```

```
$ cargo run --quiet -- "aimée"
61 69 6D C3 A9 65 

```

有我们的"c3 a9"！很简单。目前为止，我们还没对类型的担心，在我们的Rust程序中还没有一个`String`或`&str`。所以，让我们去寻找麻烦。

### 传递字符串转换成大写

```
fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    println!("upp = {}", uppercase(arg));
    println!("arg = {}", arg);
}

fn uppercase(s: String) -> String {
    s.to_uppercase()
}

```

```
$ cargo build --quiet
error[E0382]: borrow of moved value: `arg`
 --> src/main.rs:8:26
  |
2 |     let arg = std::env::args()
  |         --- move occurs because `arg` has type `std::string::String`, which does not implement the `Copy` trait
...
7 |     println!("upp = {}", uppercase(arg));
  |                                    --- value moved here
8 |     println!("arg = {}", arg);
  |                          ^^^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustre`.

```

哦，上帝，编译器来了。问题在于我们将arg传入`uppercase()`，然后又再次使用它。我们可以先打印arg，然后再调用uppercase()。那行得通吗？可以。但是，假设我们就是需要先调用`uppercase`呢？

```
fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    println!("upp = {}", uppercase(arg.clone()));
    println!("arg = {}", arg);
}

fn uppercase(s: String) -> String {
    s.to_uppercase()
}

```

```
$ cargo run --quiet -- "dog"
upp = DOG
arg = dog

```

但是这有点愚蠢。为什么我们需要克隆arg？只是传入`uppercase`，我们不需要在内存中有第二个拷贝。现在在内存中，我们有：

-   arg（“dog”）
-   arg的拷贝，我们传入`uppercase()`（“dog”）
-   uppercase()返回值（“DOG”）

我猜这是`&str`存在的意义吧？让我们尝试一下：

```
fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    println!("upp = {}", uppercase(arg));
    println!("arg = {}", arg);
}

fn uppercase(s: &str) -> String {
    s.to_uppercase()
}

```

```
cargo run --quiet -- "dog"
error[E0308]: mismatched types
 --> src/main.rs:7:36
  |
7 |     println!("upp = {}", uppercase(arg));
  |                                    ^^^
  |                                    |
  |                                    expected `&str`, found struct `std::string::String`
  |                                    help: consider borrowing here: `&arg`

```

根据编译器的提示修改：

```
println!("upp = {}", uppercase(&arg));

```

```
$ cargo run --quiet -- "dog"
upp = DOG
arg = dog

```

为了使其更接近于C代码，我们应该：

-   分配一个“目标”
-   传递“目标”到`uppercase()`
-   `uppercase()`遍历每个字符，将其转换为大写，并将其附加到"目标"

```
fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    let mut upp = String::new();
    println!("upp = {}", uppercase(&arg, upp));
    println!("arg = {}", arg);
}

fn uppercase(src: &str, dst: String) -> String {
    for c in src.chars() {
        dst.push(c.to_uppercase());
    }
    dst
}

```

```
$ cargo run --quiet -- "dog"
error[E0308]: mismatched types
  --> src/main.rs:14:18
   |
14 |         dst.push(c.to_uppercase());
   |                  ^^^^^^^^^^^^^^^^ expected `char`, found struct `std::char::ToUppercase`

```

> ToUppercase，该结构由char上的to_uppercase方法创建，返回一个迭代器，该迭代器生成char的大写等效项。

迭代器，知道这一点，我们可以使用`for x in y`：

```
fn uppercase(src: &str, dst: String) -> String {
    for c in src.chars() {
        for c in c.to_uppercase() {
            dst.push(c);
        }
    }
    dst
}

```

```
$ error[E0596]: cannot borrow `dst` as mutable, as it is not declared as mutable
  --> src/main.rs:15:13
   |
12 | fn uppercase(src: &str, dst: String) -> String {
   |                         --- help: consider changing this to be mutable: `mut dst`
...
15 |             dst.push(c);
   |             ^^^ cannot borrow as mutable

```

让我们看一下`String::push`的声明：

```
pub fn push(&mut self, ch: char)

```

因此`dst.push(c)`与`String::push(&mut dst, c)`完全相同。根据编译器建议修改：

```
fn uppercase(src: &str, mut dst: String) -> String {
	...
}

```

```
$ cargo run --quiet -- "dog"
upp = DOG
arg = dog

```

`uppercase`没有返回值呢？

```
fn uppercase(src: &str, mut dst: String) {
    for c in src.chars() {
        for c in c.to_uppercase() {
            dst.push(c);
        }
    }
}

```

```
cargo run --quiet -- "dog"
error[E0382]: borrow of moved value: `upp`
  --> src/main.rs:10:26
   |
7  |     let upp = String::new();
   |         --- move occurs because `upp` has type `std::string::String`, which does not implement the `Copy` trait
8  |     uppercase(&arg, upp);
   |                     --- value moved here
9  | 
10 |     println!("upp = {}", upp);
   |                          ^^^ value borrowed here after move

```

我们需要让upp可变地借用。

```
fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    let mut upp = String::new();
    // was just `upp`
    uppercase(&arg, &mut upp);

    println!("upp = {}", upp);
    println!("arg = {}", arg);
}

// was `mut dst: String`
fn uppercase(src: &str, dst: &mut String) {
    for c in src.chars() {
        for c in c.to_uppercase() {
            dst.push(c);
        }
    }
}

```

```
$ cargo run --quiet -- "dog"
upp = DOG
arg = dog

```

现在又可以使用了！可增长的字符串，这是否意味着我们可以预分配合理大小的String，然后将其重新用于多个uppercase 调用？

### 索引

C允许我们直接索引，Rust允许我们这样做吗？

```
fn main() {
    for arg in std::env::args().skip(1) {
        for i in 0..arg.len() {
            println!("arg[{}] = {}", i, arg[i]);
        }
    }
}

```

```
$ cargo run --quiet -- "dog"
error[E0277]: the type `std::string::String` cannot be indexed by `usize`
 --> src/main.rs:4:41
  |
4 |             println!("arg[{}] = {}", i, arg[i]);
  |                                         ^^^^^^ `std::string::String` cannot be indexed by `usize`
  |
  = help: the trait `std::ops::Index<usize>` is not implemented for `std::string::String`

```

我们不可以。我们可以先将其转换为Unicode标量值数组，然后对其进行索引：

```
fn main() {
    for arg in std::env::args().skip(1) {
        let scalars: Vec<char> = arg.chars().collect();
        for i in 0..scalars.len() {
            println!("arg[{}] = {}", i, scalars[i]);
        }
    }
}

```

```
$ cargo run --quiet -- "dog"
arg[0] = d
arg[1] = o
arg[2] = g

```

是的，行得通！老实说，这样比较好，因为我们只需要解码一次UTF-8字符串，然后我们就可以进行随机访问了。这可能就是为什么它没有实现`Index<usize>`的原因。

有趣的事情：`Index<Range<usize>>`。

```
fn main() {
    for arg in std::env::args().skip(1) {
        let mut stripped = &arg[..];
        while stripped.starts_with(" ") {
            stripped = &stripped[1..]
        }
        while stripped.ends_with(" ") {
            stripped = &stripped[..stripped.len() - 1]
        }
        println!("     arg = {:?}", arg);
        println!("stripped = {:?}", stripped);
    }
}

```

```
$ cargo run --quiet -- "  floating in space   "
     arg = "  floating in space   "
stripped = "floating in space"

```

> String是堆分配的，因为它是可增长的。而&str可以从任何地方引用数据：堆，栈，甚至程序的数据段。

`&str`，它是不同的，它指向相同的内存区域，只是在不同的偏移量处开始和结束。实际上，我们可以使其成为一个函数：

```
fn main() {
    for arg in std::env::args().skip(1) {
        let stripped = strip(&arg);
        println!("     arg = {:?}", arg);
        println!("stripped = {:?}", stripped);
    }
}

fn strip(src: &str) -> &str {
    let mut dst = &src[..];
    while dst.starts_with(" ") {
        dst = &dst[1..]
    }
    while dst.ends_with(" ") {
        dst = &dst[..dst.len() - 1]
    }
    dst
}

```

而且效果也一样。不过，这似乎很危险。如果原始字符串的内存被释放怎么办？

```
fn main() {
    let stripped;
    {
        let original = String::from("  floating in space  ");
        stripped = strip(&original);
    }
    println!("stripped = {:?}", stripped);
}

```

```
$ cargo run --quiet -- "  floating in space   "
error[E0597]: `original` does not live long enough
 --> src/main.rs:5:26
  |
5 |         stripped = strip(&original);
  |                          ^^^^^^^^^ borrowed value does not live long enough
6 |     }
  |     - `original` dropped here while still borrowed
7 |     println!("stripped = {:?}", stripped);
  |                                 -------- borrow later used here

```

在Rust中？编译器将检查所有的"恶作剧"。

最后，String用范围索引，很酷，但是`..`是字符范围吗？

```
fn main() {
    for arg in std::env::args().skip(1) {
        println!("first four = {:?}", &arg[..4]);
    }
}

```

```
$ cargo run --quiet -- "want safety?"
first four = "want"
$ cargo run --quiet -- "🙈🙉🙊💥"
first four = "🙈"

```

字节范围。我以为所有Rust字符串都是UTF-8？但是使用切片，我们可以得到部分多字节序列，或无效的UTF-8？假如：

```
fn main() {
    for arg in std::env::args().skip(1) {
        println!("first two = {:?}", &arg[..2]);
    }
}

```

```
$ cargo run --quiet -- "🙈🙉🙊💥"
thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside '🙈' (bytes 0..4) of `🙈🙉🙊💥`', src/libcore/str/mod.rs:2069:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

```

那太好了。它会panic，这是安全的事情。

### 结束语

无论如何，这篇文章已经很长了。希望它对Rust中的字符串处理有足够的介绍，以及Rust为什么同时具有String和&str。

答案当然依旧是安全性，正确性和性能。

在我们编写的最后一个Rust字符串操作程序时，确实遇到了障碍，但是它们主要是编译时错误或panic。我们没有一次：

-   从无效地址读取
-   写入无效的地址
-   忘了释放东西
-   覆盖了其他一些数据
-   需要一个额外的工具来告诉我们问题出在哪里

而且，加上令人惊叹的编译器信息以及社区，这就是Rust的美。

### 几个小技巧让你的 Rust 代码性能

不用改动代码，只通过几个技巧就能提高你的 Rust 项目运行速度，比如在 `Cargo.tom` 文件中  `[profile.release]`  下根据情况更改一些字段或许就可以提升你的项目性能：

-   `lto = "fat"`
-   `codegen-units = 1`
-   `target-cpu = "native"`
-   ...

详细介绍：[https://deterministic.space/high-performance-rust.html](https://deterministic.space/high-performance-rust.html)

### Rust blog：近期以及未来的模式匹配改进

Rust 官方博客介绍了即将了即将应用于stable Rust 的模式匹配新特性

#### Subslice 模式匹配，[head, tail @ ..]

`..`  意味着可变间隔，例如

```
fn recover_attrs_no_item(&mut self, attrs: &[Attribute]) -> PResult<'a, ()> {
    let (start, end) = match attrs {
        [] => return Ok(()),
        [x0] => (x0, x0),
        [x0, .., xn] => (x0, xn),
    };
    let msg = if end.is_doc_comment() {
        "expected item after doc comment"
    } else {
        "expected item after attributes"
    };
    let mut err = self.struct_span_err(end.span, msg);
    if end.is_doc_comment() {
        err.span_label(end.span, "this doc comment doesn't document anything");
    }
    if let [.., penultimate, _] = attrs {
        err.span_label(start.span.to(penultimate.span), "other attributes here");
    }
    Err(err)
}

```

其中  `[x0, .., xn]`  就表示匹配第一个以及最后一个元素而忽略中间的所有元素.

另一种用法是可以将subslice约束为一个变量，比如如果我们希望某个函数除了最后一个参数之外的参数不能为  `...`  那么可以这样写：

```
match &*fn_decl.inputs {
    ... // other arms
    [ps @ .., _] => {
        for Param { ty, span, .. } in ps {
            if let TyKind::CVarArgs = ty.kind {
                self.err_handler().span_err(
                    *span,
                    "`...` must be the last argument of a C-variadic function",
                );
            }
        }
    }
}

```

这里的  `ps @ ..`  就表示忽略参数的最后的一个元素而将剩下的元素转化为 变量  `ps`

其他还有

-   Nested OR-patterns
-   Bindings after @
-   Combining by-move and by-ref bindings

详情：  [https://blog.rust-lang.org/inside-rust/2020/03/04/recent-future-pattern-matching-improvements.html](https://blog.rust-lang.org/inside-rust/2020/03/04/recent-future-pattern-matching-improvements.html)

## April --202004


### Rust 的 Type-level 编程

Typestate 是在编程语言的类型系统中对状态机进行编码的概念。尽管不是特定于 Rust，但在 Rust 编程的中也有很多地方探讨了 typestate。

Typestate可以归结为四个想法：

-   每个状态都表示为唯一类型。
-   状态转换仅可用作相应状态类型的方法。
-   进行状态转换将返回新状态类型的状态机。
-   状态转换会使旧状态无效。

如下，这是一个  `send-then-receive channel`  状态机：

```
// Each state is a unique type
struct Receiving;
struct Sending;

// The state machine is parameterized by the state
struct Channel<State> {
  chan: ...,
  _state: PhantomData<State>
}


// Methods for the state are uniquely associated with only the state
impl Channel<Receiving> {
  // recv consumes ownership, ensuring old state is invalidated
  fn recv(mut self) -> (Channel<Sending>, String) {
    let msg = self.chan.recv();
    // The state type changes after executing a transition
    (unsafe { transmute(self) }, msg)
  }
}

impl Channel<Sending> {
  fn send(mut self, msg: String) -> Channel<Receiving> {
    self.chan.send(msg);
    unsafe { transmute(self) }
  }
}

#[test]
fn channel_test() {
  let c: Channel<Sending> = Channel::new();
  let c: Channel<Receiving> = c.send("hi");
  let (c, msg) = c.recv();
  // and so on
}

```

这种模式对于简单的有限状态机有效，其中确定下一个状态的逻辑很简单。本篇博文中，我将探讨确定下一个状态不是那么简单的情况。在此过程中，我们将讨论类型级编程，或者如何使用 Rust 的类型系统对类型的计算进行编码。

本篇博文中的部分目标是在实践中显示类型级编程的价值。这些相同的机制已经用于更深奥的目的，例如表明 Rust 的类型系统已经实现  `Turing`，但我认为类型级编程确实可以帮助我们设计更好的系统！

更多请查看原文：http://willcrichton.net/notes/type-level-programming/

##  -【博客】减小 Rust GStreamer 插件的体积

受困于 Rust 项目编译后二进制包过大，尤其是对于嵌入式开发就更是一个问题了。作者受到  [Tiny Rocket](https://jamesmunns.com/blog/tinyrocket/)以及[Minimizing Rust Binary Size](https://github.com/johnthagen/min-sized-rust)的启发，在这篇博客中介绍了他是如何给[GStreamer](https://gstreamer.freedesktop.org/)  压缩体积的。

[https://www.collabora.com/news-and-blog/blog/2020/04/28/reducing-size-rust-gstreamer-plugin/](https://www.collabora.com/news-and-blog/blog/2020/04/28/reducing-size-rust-gstreamer-plugin/)


## Rust语言gRPC入门

> Intro to gRPC with Rust
> 
> [https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o](https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o)

这个小哥Anshul Goyal写了一个关于gRPC入门的教程，有兴趣学习的同学们可以看看。很不错！

内容提纲：

-   Introduction
-   Protocol Buffer
-   Rust and gRPC
-   Creating a Server
-   Creating a Client
-   Streaming in gRPC
-   Authentication
-   Conclusion

## Cargo Bloat Action: 跟踪`Github`上进行了`cross builds/pull`操作的`Rust`二进制大小

> Cargo Bloat Action: Track Rust binary sizes across builds/pull requests using Github Actions
> 
> [https://github.com/orf/cargo-bloat-action/](https://github.com/orf/cargo-bloat-action/)

跟踪`Github`上进行了`cross builds/pull`操作的`Rust`二进制大小


## (几乎) 不用锁的`Stream Buffering`编程

> (Almost) Lockless Stream Buffering  
> [https://mcfelix.me/blog/shared-buffers/](https://mcfelix.me/blog/shared-buffers/)
> 
> Tags: Rust Discord Audio Concurrency

(几乎) 不用锁的`Stream Buffering`编程，文章讲述如何高效的分享（或保持）bytestreams。

## Rust语言辅助学习网络直播达人清单

> List of Rust streamers
> 
> [https://github.com/jamesmunns/awesome-rust-streaming](https://github.com/jamesmunns/awesome-rust-streaming)

Rust语言辅助学习网络直播清单，网络上比较出名的streamers包括：

-   David Pedersen
    
    -   YouTube Channel:[https://www.youtube.com/channel/UCDmSWx6SK0zCU2NqPJ0VmDQ](https://www.youtube.com/channel/UCDmSWx6SK0zCU2NqPJ0VmDQ)
    -   Twitch Channel:  [https://www.twitch.tv/davidpdrsn](https://www.twitch.tv/davidpdrsn)
    -   [GitHub](https://github.com/davidpdrsn),
    -   [Twitter](https://twitter.com/davidpdrsn)
    -   CLI Tools, Teaching
-   Ferris Streams Stuff
    
    -   [YouTube Channel - https://www.youtube.com/channel/UC4mpLlHn0FOekNg05yCnkzQ)](https://www.youtube.com/channel/UC4mpLlHn0FOekNg05yCnkzQ)
    -   [Twitch Channel - https://www.twitch.tv/ferrisstreamsstuff](https://www.twitch.tv/ferrisstreamsstuff)
    -   [GitHub - https://github.com/yupferris](https://github.com/yupferris)
    -   [Twitter - https://twitter.com/ferristweetsnow](https://twitter.com/ferristweetsnow)
    -   Emulators, Demoscene
-   Ferrous Systems
    
    -   [YouTube Channel - https://www.youtube.com/c/FerrousSystemsGmbH](https://www.youtube.com/c/FerrousSystemsGmbH)
        
    -   [GitHub - https://github.com/FerrousSystems](https://github.com/FerrousSystems),
        
    -   [Twitter - https://twitter.com/ferroussystems](https://twitter.com/ferroussystems),
        
    -   [Website - https://ferrous-systems.com/](https://ferrous-systems.com/)
        
    -   Q&A sessions, Embedded, Compiler contributions, Tooling
        
-   James' Office Hours
    
    -   YouTube Channel:  [https://www.youtube.com/channel/UCb48C4qqcXQpRugPbdwigZQ](https://www.youtube.com/channel/UCb48C4qqcXQpRugPbdwigZQ)
    -   Old YouTube Channel:  [https://www.youtube.com/c/JamesMunns/](https://www.youtube.com/c/JamesMunns/)
    -   GitHub:  [https://github.com/jamesmunns](https://github.com/jamesmunns)
    -   Twitter:  [https://twitter.com/bitshiftmask](https://twitter.com/bitshiftmask)
    -   Blog:  [https://jamesmunns.com/](https://jamesmunns.com/)
    -   Embedded, CLI tools
-   Jon Gjengset
    
    -   YouTube Channel:  [https://www.youtube.com/c/JonGjengset/](https://www.youtube.com/c/JonGjengset/)
    -   Twitch Channel:  [https://www.twitch.tv/jonhoo](https://www.twitch.tv/jonhoo)
    -   GitHub:  [https://github.com/Jonhoo](https://github.com/Jonhoo)
    -   Twitter:  [https://twitter.com/jonhoo](https://twitter.com/jonhoo)
    -   Website:  [https://thesquareplanet.com/](https://thesquareplanet.com/)
    -   Teaching, Databases, Concurrency
-   Ryan Levick
    
    -   YouTube Channel:  [https://www.youtube.com/channel/UCpeX4D-ArTrsqvhLapAHprQ](https://www.youtube.com/channel/UCpeX4D-ArTrsqvhLapAHprQ)
    -   Twitch Channel:  [https://github.com/jamesmunns/awesome-rust-streaming/blob/master/twitch.tv/ryanlevick](https://github.com/jamesmunns/awesome-rust-streaming/blob/master/twitch.tv/ryanlevick)
    -   GitHub:  [https://github.com/rylev](https://github.com/rylev)
    -   Twitter:  [https://twitter.com/ryan_levick](https://twitter.com/ryan_levick)
    
    Teaching, Web Assembly
    
-   Yoshua Wuyts
    
    -   YouTube Channel:  [https://www.youtube.com/yoshuawuyts](https://www.youtube.com/yoshuawuyts)
    -   Twitch Channel:  [https://www.twitch.tv/yoshuawuyts](https://www.twitch.tv/yoshuawuyts)
    -   GitHub:  [https://github.com/yoshuawuyts/](https://github.com/yoshuawuyts/)
    -   Twitter:  [https://twitter.com/yoshuawuyts](https://twitter.com/yoshuawuyts)
    -   Blog:  [https://blog.yoshuawuyts.com/](https://blog.yoshuawuyts.com/)
    -   Async, Web, API Design
-   Brandon Falk
    
    -   YouTube Channel:  [https://www.youtube.com/user/gamozolabs](https://www.youtube.com/user/gamozolabs)
    -   Twitch Channel:  [https://www.twitch.tv/gamozo](https://www.twitch.tv/gamozo)
    -   GitHub:  [https://github.com/gamozolabs](https://github.com/gamozolabs)
    -   Twitter:  [https://twitter.com/gamozolabs](https://twitter.com/gamozolabs)
    -   Blog:  [https://gamozolabs.github.io/](https://gamozolabs.github.io/)
    -   OSdev, Hypervisors, Fuzzers
-   Stefano Casillo - Jaxx Vane Studio Live
    
    -   YouTube Channel:  [https://www.youtube.com/channel/UC7n_g2xDySrmKRaf41rSwlg](https://www.youtube.com/channel/UC7n_g2xDySrmKRaf41rSwlg)
    -   Twitch Channel:  [https://www.twitch.tv/kunosstefano](https://www.twitch.tv/kunosstefano)
    -   Twitter:  [https://twitter.com/KunosStefano](https://twitter.com/KunosStefano)
    -   Gamedev
	
## Rust Notebooks：Anaconda, Jupyter, and Rust 设置教程

> Setup Anaconda, Jupyter, and Rust for Rust Notebooks
> 
> [https://shahinrostami.com/posts/programming/rust-notebooks/setup-anaconda-jupyter-and-rust/](https://shahinrostami.com/posts/programming/rust-notebooks/setup-anaconda-jupyter-and-rust/)

Anaconda, Jupyter, and Rust 设置教程

## 细思极恐的`Teleforking`  - 在另外一个计算机"远程启动"一个计算进程！

> Teleforking a process onto a different computer!
> 
> [https://thume.ca/2020/04/18/telefork-forking-a-process-onto-a-different-computer/](https://thume.ca/2020/04/18/telefork-forking-a-process-onto-a-different-computer/)

一个用rust语言实现的细思极恐的`Teleforking`远程启动进程功能。大家围观，不要拿来做坏事哈！

## 用`rust`语言的`Traits`数据结构实现高阶排序"欺骗"！

> Cheating Higher Ranks with Traits。
> 
> [https://leshow.github.io/post/cheat_rank_n/](https://leshow.github.io/post/cheat_rank_n/)

### [视频] 关于所有权，闭包和线程

[YouTube 视频](https://www.youtube.com/watch?v=2mwwYbBRJSo)，https://www.youtube.com/watch?v=2mwwYbBRJSo

### 一个关于3D图形、Rust、Vulkan、ash的教程

[教程在这里](https://hoj-senna.github.io/ashen-aetna/)，https://hoj-senna.github.io/ashen-aetna/

# 嵌入式Rust模式-零空間參考

文章提出一種參考方式可以在嵌入式系統使用 讓你可以在嵌入式系統中節省記憶體的使用

[Read more](https://ferrous-systems.com/blog/zero-sized-references/)

### CS-3210 课程推荐

大家可能对 stanford 的 cs140e 课程还有印象，现在他的“高阶版”来了。佐治亚理工学院 OS lab 开设了 CS-3210 课程，主要内容是设计和实现操作系统的核心组件。地址：https://tc.gts3.org/cs3210/2020/spring/info.html

### - 佐治亚理工学院 CS-3210 课程实验：用 Rust 为树莓派写一个操作系统

[Read More](https://tc.gts3.org/cs3210/2020/spring/lab.html)

### - Multiversion 0.5.0, 多版本函数宏，现在已经"可以跑生产系统"了。

> Multiversion 0.5.0, now "production ready"

[`Multiversion`  - 是`Rust`语言支持多版本函数的属性宏.](https://crates.io/crates/multiversion)

什么是`function multiversioning`?

> 大部分的CPU架构都有自己独特的指令集支持一些额外的功能。最常见的例子包括`x86/x86-64`  上的`SSE & AVX`，`NEON`上的`ARM/AArch64`指令集扩展`Single Instruction, Multiple Data(SIMD)`。 这些指令集扩展可以给某些特殊的函数提升大量的运行速度。这些特殊的功能是不能胡乱的编译到一个 不支持这些特殊功能CPU的可执行文件里去的，那样往往会造成系统崩溃。

`Function multiversioning`是一种特殊的编译方法，通过编译包含特殊功能支持的不同版本的函数 能够在运行时`runtime`检测到这些特殊的功能并匹配不同的版本的可执行函数。

Function multiversioning功能：

-   动态调控，启用运行时CPU功能检测
-   静态调控，避免嵌套式的重复功能检测（但允许行内嵌套）
-   支持所有类型的函数，包括generic和async类型的函数

例子： 用`clone`属性宏来实现多版本函数，类似GCC的`target_clones`

```
use multiversion::multiversion;

#[multiversion]
#[clone(target = "[x86|x86_64]+avx")]
#[clone(target = "x86+sse")]
fn square(x: &mut [f32]) {
   for v in x {
       *v *= *v;
   }
}

```

用`multiversion`和`target`属性宏来实现多版本函数.

```
use multiversion::{multiversion, target};

#[target("[x86|x86_64]+avx")]
unsafe fn square_avx(x: &mut [f32]) {
   for v in x {
       *v *= *v;
   }
}

#[target("x86+sse")]
unsafe fn square_sse(x: &mut [f32]) {
   for v in x {
       *v *= *v;
   }
}

#[multiversion]
#[specialize(target = "[x86|x86_64]+avx", fn = "square_avx", unsafe = true)]
#[specialize(target = "x86+sse", fn = "square_sse", unsafe = true)]
fn square(x: &mut [f32]) {
   for v in x {
       *v *= *v;
   }
}

```

[更多信息请点击crates官网说明-Read More](https://crates.io/crates/multiversion)

### - 如何在Windows 10系统环境安装原生Rust编程环境

> [How to install rust on Windows 10 (native)](https://estada.ch/2020/4/19/installing-rust-on-windows-10-native/)

下面是快速安装`Windows 10 2004`的步骤：

1.  下载并运行[rustup.rs](https://rustup.rs/)
2.  下载[`Build Tools for Visual Studio 2019`](https://visualstudio.microsoft.com/downloads/)，一般这个下载隐藏在微软下载链接的`"Tools for Visual Studio 2019"`下面。
3.  运行`Build Tools for Visual Studio 2019 Installer`并选择:
    -   `C++ Tools`
4.  `C++ Tools`中还必须同时选择安装`"Windows 10 SDK"`，安装程序提供多个版本，选最新的版本安装就好。

测试看看是否安装成功：

1.  打开PowerShell或命令行窗口，输入下面的命令并保证没有错误。
2.  切换到临时文件夹：`cd %TEMP%`
3.  创建一个测试项目：`cargo new toolchain_test`
4.  进入项目所在目录：`cd toolchain_test`
5.  编译并运行"Hello, world!"程序：`cargo run`

然后你应该可以得到一个编译的过程并看到结果显示`"Hello, world!"`

如果遇到类似`cargo command not found`的错误，你需要检查一下你的`%PATH%`看看是否设置好。

### 基于gfx-hal的Rust图形学教程-第三部分

#graphics

[part 3](https://www.falseidolfactory.com/2020/04/16/intro-to-gfx-hal-part-3-vertex-buffers.html)  [part 2](https://www.falseidolfactory.com/2020/04/01/intro-to-gfx-hal-part-2-push-constants.html)  [part 1](https://www.falseidolfactory.com/2020/04/01/intro-to-gfx-hal-part-1-drawing-a-triangle.html)


### 系列轻教程：在 Rust 中写 Python 代码

作者实现了一个 python! 宏，结合pyo3（一个流行的python api的rust ffi 绑定），可以让你在 Rust中编写python代码。

教程是他如何编写该库。 学习宏、FFi 可以参考该库。

[文章](https://blog.m-ou.se/writing-python-inside-rust-1/)，https://blog.m-ou.se/writing-python-inside-rust-1/  [代码](https://github.com/fusion-engineering/inline-python)，https://github.com/fusion-engineering/inline-python

# Hash 查找不用分配記憶體的方法

```
struct OwnedKey {
    s: String,
    bytes: Vec<u8>,
}

struct BorrowedKey<'a> {
    s: &'a str,
    bytes: &'a [u8],
}

//教你用BorrowedKey在 HashSet<OwnedKey> or BTreeSet<OwnedKey>容器中做查找

```

[Read more](https://github.com/sunshowers/borrow-complex-key-example/blob/master/README.md)

### μfmt 项目

`μfmt`  是替代  `core::fmt`  的更小，更快，更轻松的选择，项目地址：https://github.com/japaric/ufmt

![](https://github.com/japaric/ufmt/raw/master/cg.png)

##### 使用 Actix 和 Juniper 构建简单的 GraphQL API

#graphql

油管视频教程，该up主还做了一系列actix相关的视频教程，虽然看视频学的比较慢，但是很适合初学者。

[Read More](https://www.youtube.com/watch?v=aEAz5DHhpLo&feature=youtu.be)

### 如何在正确性至关重要的Rust项目中进行错误处理

#rust #error_handing

[Read More](http://sled.rs/errors)

# 分析rust的三種回傳包裝

```
// Ok-Wrapping
fn foo() -> Result<PathBuf, io::Error> {
    let base = env::current_dir()?;
    Ok(base.join("foo"))
}
// use exception
fn foo() -> PathBuf throws io::Error {
    let base = env::current_dir()?;
    base.join("foo")
}
//Try Functions
#![feature(try_blocks)]
fn foo() -> Result<PathBuf, io::Error> {
    try {
        let base = env::current_dir()?;
        base.join("foo")
    }
}

```

[Read more](https://yaah.dev/try-blocks)

# Ok(match thing { ... }) 不好嗎？

有人在boats最近的blog發現他不建議大家用

他建議除了作為返回值以外不要使用Ok-Wrapping

可以讓程式碼更清楚更容易看懂

[Read more](https://boats.gitlab.io/blog/post/why-ok-wrapping/)

# Ok-Wrapping的心理模型

這幾天大家瘋狂的在討論Ok-Wrapping

本文只是希望以一些分析性的方式來說明

為什麼我個人不喜歡Ok-wrapping的一些原因。

[Read more](https://vorner.github.io/2020/04/09/wrapping-mental-models.html)

### 200行代码讲透 Rust Futures

这是一个比较长的博客，主要是用一个例子驱动的方法来解释Rust中的Futures，探索为什么他们被设计成这样，以及他们如何工作，此外还介绍在编程中处理并发性时的一些替代方案。

原文地址：https://cfsamson.github.io/books-futures-explained/introduction.html，同时国内的大佬 白振轩的个人博客已经做了翻译，请看：https://stevenbai.top/rust/futures_explained_in_200_lines_of_rust/

### Rust 是 k8s 的不错选择

前些天，我们日报小组介绍了 Krustlet，这是 Rust 中一个基于 WebAssembly 的 Kubelet 实现。 我们选择使用Rust的原因有两个：1、Rust对WebAssembly编译提供了一些最好的支持（稍后会详细介绍），1、我们想证明 Rust 的优势可以应用于 Kubernetes 生态系统。 这篇文章旨在表明我们学到了什么以及为什么我们认为 Rust 是编写 Kubernetes 重点应用程序的绝佳选择（有时更好）【来自（DeisLabs）的博客】。

原文请看：https://deislabs.io/posts/kubernetes-a-rusty-friendship/

### proc-macro-error

proc-macro-error 的目标是使过程宏中的错误报告变得轻松便捷。

使用实例速览：

```
use proc_macro_error::*;
use proc_macro::TokenStream;
use syn::{spanned::Spanned, DeriveInput, ItemStruct, Fields, Attribute , parse_macro_input};
use quote::quote;

fn process_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter_map(|attr| match process_attr(attr) {
            Ok(res) => Some(res),
            Err(msg) => {
                emit_error!(attr, "Invalid attribute: {}", msg);
                None
            }
        })
        .collect()
}

fn process_fields(_attrs: &Fields) -> Vec<TokenStream> {
    // processing fields in pretty much the same way as attributes
    unimplemented!()
}

#[proc_macro]
#[proc_macro_error]
pub fn make_answer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let attrs = process_attrs(&input.attrs);

    // abort right now if some errors were encountered
    // at the attributes processing stage
    abort_if_dirty();

    let fields = process_fields(&input.fields);

    // no need to think about emitted errors
    // #[proc_macro_error] will handle them for you
    //
    // just return a TokenStream as you normally would
    quote!(/* stuff */).into()
}

```

### 【博客】Rust 项目中的错误处理

[http://sled.rs/errors](http://sled.rs/errors)

#### [Voik - 一个试验性的分布式流平台](https://github.com/marceloboeira/voik)

项目目的

-   学习
-   实现类似Kinesis一样的流服务
-   单一可执行文件
-   轻松托管，执行和运营

已经发布的博文：

-   [Building a Distributed Log from Scratch, Part 1: Storage Mechanics](https://bravenewgeek.com/building-a-distributed-log-from-scratch-part-1-storage-mechanics/)
-   [Building a Distributed Log from Scratch, Part 2: Data Replication](https://bravenewgeek.com/building-a-distributed-log-from-scratch-part-2-data-replication)
-   [Building a Distributed Log from Scratch, Part 3: Scaling Message Delivery](https://bravenewgeek.com/building-a-distributed-log-from-scratch-part-3-scaling-message-delivery/)
-   [Building a Distributed Log from Scratch, Part 4: Trade-Offs and Lessons Learned](https://bravenewgeek.com/building-a-distributed-log-from-scratch-part-4-trade-offs-and-lessons-learned/)
-   [Building a Distributed Log from Scratch, Part 5: Sketching a New System](https://bravenewgeek.com/building-a-distributed-log-from-scratch-part-5-sketching-a-new-system/)
-   [The Log: What every software engineer should know about real-time data's unifying abstraction](https://engineering.linkedin.com/distributed-systems/log-what-every-software-engineer-should-know-about-real-time-datas-unifying)
-   [How Kafka's Storage Internals Work](https://thehoard.blog/how-kafkas-storage-internals-work-3a29b02e026)

#### Servo浏览器编程: Service服务的Worker脚本进程.

描述Service workers网页服务后台脚本进程在整个Servo浏览器大架构里的地位，这些后台脚本都是用Rust语言来实现的并行Web引擎。

https://medium.com/programming-servo/programming-servo-workers-at-your-service-db71e5943511

### gfx-hal介绍第一部分-画三角形

#rust #webgl

这篇文章是rust图形编程教程系列的第一篇，使用的是gfx-hal这个库，介绍了通过这个库使用Rust实现一个webgl三角形。

[Read More](https://www.falseidolfactory.com/2020/04/01/intro-to-gfx-hal-part-1-drawing-a-triangle.html)

### Stjepan：为什么我要建立一个新的异步运行时？

Stjepan Glavina是Rust流行库Crossbeam的作者，最近一年专注于async-std的开发。

[博客文章](https://stjepang.github.io/2020/04/03/why-im-building-a-new-async-runtime.html)，https://stjepang.github.io/2020/04/03/why-im-building-a-new-async-runtime.html

# 寫一個Lambda演算解析器

有點lex, yacc的感覺

[Read more](https://christianpoveda.github.io/blog/parsing/)

### 【译文】Rust中的`String`和`&str`

当你开始 Rust 的学习之旅后，很可能遇到需要使用字符串的场景，但是编译器却无法让你的代码通过编译，因为有一部分代码，看起来像字符串，事实上却又不是。看下这个文章或许能给你解惑：https://zhuanlan.zhihu.com/p/123278299，内容来源：知乎-Praying。

原文地址：https://link.zhihu.com/?target=https%3A//blog.thoughtram.io/string-vs-str-in-rust/

### 【演讲】新机器的灵魂:重新思考计算机

分享者： 布莱恩·坎特里（Bryan Cantrill） - 氧化物（Oxide）计算机公司 2020年2月26日 活动：斯坦福研讨会 (Stanford Seminar )

尽管我们的软件系统变得越来越有弹性，但可用于运行该软件的物理基础（即计算机！）仍然停留在PC架构的旧时代。 超大型基础设施提供商早就意识到了这一点，制造出适合用途的机器，但这些进步却被大众市场所拒绝。

在本次演讲中，讨论对新型机架式服务器端机器的愿景，以及开放固件、RISC-V和Rust等技术进步将如何在实现这一愿景中发挥核心作用。

详情：[https://youtu.be/vvZA9n3e5pc](https://youtu.be/vvZA9n3e5pc)

## May --202005


### 通过拆解Rust来学习

#rust

[@MGDev91](https://twitter.com/MGDev91)通过拆解Rust来学习，通过了解Rust结构如果转化为指令，观察哪里需要注意内存安全，点击链接查看作者的第一篇文章。

[Read More](https://giordi91.github.io/post/disassemlbyrust1/)

# Rust二进制解析

这篇文章比较了三种RUST現在主流的二进制解析库

nom, byteorder, binread

也介紹了为什么要用binread因为简单直接

[Read more](https://jam1.re/blog/binread-a-declarative-rust-binary-parsing-library)

# 覆盖测试

有個crate叫cov-mark

可以手动标注覆盖标记能更轻松地共同开发测试和代码，有助于维护。

可以用 cov_mark::hit! 标注

这篇文章也讲解了這個hit宏的实战

[Read more](https://ferrous-systems.com/blog/coverage-marks/)

# Fuchsia Rust 庫

Fuchsia 操作系统也支持rust开发

这里举例了一些相关的库

[Read more](https://fuchsia.dev/fuchsia-src/development/languages/rust/crates)

### Rust 中较新指令集的自动向量化

如何利用硬件上所有可能的 SIMD 指令获得最佳性能。

[文章链接](https://www.nickwilcox.com/blog/autovec2/)，https://www.nickwilcox.com/blog/autovec2/

### Tour of Rust 中文版

[Tour of Rust 链接](https://tourofrust.com/index_zh-cn.html)，https://tourofrust.com/index_zh-cn.html

**BTW**：GitHub 搜索时，Rust 已成为语言过滤的选项之一。

### 关于 Rust runtime

如果你对 Rust runtime 了解不多，可以查阅这个博文看一下运行时代码的样子，并逐一分解它的工作方式，地址：https://blog.mgattozzi.dev/rusts-runtime/

### ESP32 上的 Bluetooth Heartrate 示例

这是嵌入式 Rust 的示例，作者从调研、选型到实践作了详细介绍，文章地址：https://specific.solutions.limited/projects/hanging-plotter/oxidized-bluetooth.md

### 书：从 0 到产品（利用 Rust 语言描述）

讲云原生应用开发从 0 到产品的一本书，连载。惊喜的是是用 Rust 语言来描述。强烈推荐。

https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/

这是前言。

### CRDT = Conflict-free Replicated Data Type

这个概念很有新意。可以用在实时协同编辑doc之类的应用上面。新知识太多，小编根本学不过来了。

这是一个示例：https://github.com/anchpop/crdts

这是 CRDT 的一个简单教程：http://jtfmumm.com/blog/2015/11/17/crdt-primer-1-defanging-order-theory/

### Tour of Rust 更新第4章-范型

[Tour of Rust 链接](https://tourofrust.com/index.html)，https://tourofrust.com/index.html

### [视频] C++20，Rust 和 Zig 中的零成本抽象

[Youtube 链接](https://www.youtube.com/watch?v=43X9ia-qpds&feature=share)，https://www.youtube.com/watch?v=43X9ia-qpds&feature=share

# CHIP-8 直译器 Rust

目前还缺很多功能，算是作者练习的玩具。

[Read more](https://github.com/RyanHope97/CHIP-8)

# 对Rust生命周期的误解

解答新手对Rust生命周期的误解，高手也可以查表复习。

[Read more](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)

# 如何组织Rust的测试

測試是发现bug的一种廉价且简便的方法。单元测试的伟大之处在于成本低廉。

[Read more](https://blog.logrocket.com/how-to-organize-your-rust-tests/)

### 使用 Rust 和 React 构建实时聊天室应用

详细教程请看：https://outcrawl.com/rust-react-realtime-chat,教程中附带了项目源码地址。

![](https://github.com/tinrab/rusty-chat/raw/master/cover.jpg)

### `iiiD6`

`iiiD6`  - A hydrogen atom orbital wave function renderer based on random sampling.

github:https://github.com/cbeuw/iiiD6

![](https://github.com/cbeuw/iiiD6/raw/master/render/200.png)

### Oxidizing the technical interview

以讲故事的方式来讲 Rust 技术面试的一些东西

[https://blog.mgattozzi.dev/oxidizing-the-technical-interview/](https://blog.mgattozzi.dev/oxidizing-the-technical-interview/)

## json objects索引和查询软件包

> A crate for indexing and quering json objects
> 
> [https://github.com/mambisi/json_indexer](https://github.com/mambisi/json_indexer)

这是一个多数值json objects检索器。这个软件包可以创建基于点路径（dot path)的json objects的排序表， 这个有点类似mongodb那样根据你给的路径建立检索。这个软件包主要是针对NoSQL编程，已经在项目  [escanordb](https://github.com/mambisi/escanor)中用于创建检索结构了。

[https://github.com/mambisi/escanor](https://github.com/mambisi/escanor)

## Rust语言全局数据应用指南

> A Guide to Global Data in Rust
> 
> [https://github.com/paulkernfeld/global-data-in-rust](https://github.com/paulkernfeld/global-data-in-rust)

这个应用指南介绍如果在rust语言中使用"全局数据"，"全局数据"（Global Data)的意思就是你在一个程序加载了的数据，可以在别的程序中调用和使用。 "全局数据"潜在的应用场景包括：

-   配置，App configuration, e.g. weapon characteristics for a game
-   参数，Making data available everywhere without needing to pass it as an argument through all functions (apply this carefully!)
-   代码生成，Generating Rust code from external data
-   数据库连接，Database connections... or other network resources?
-   日志，A logger, maybe

## Rust的循环; 从嵌套循环中中断退出

> Loops in Rust; Breaking From Nested Loops
> 
> [https://qvault.io/2020/05/14/loops-in-rust-breaking-from-nested-loops/](https://qvault.io/2020/05/14/loops-in-rust-breaking-from-nested-loops/)

Rust语言的循环和标准的C语言方式是不一样的。语法不一样，而且有一些强有力的循环选项 是的Rust语言的循环用起来非常简单。首先我们学习一些最基本的循环，然后我们再学习一下 如果在嵌套循环里中断退出和继续执行。

标准的For循环:

```
fn main() {
    for x in 0..10 {
        println!("{}", x);
    }
}

```

这个循环打印：

```
0
1
2
3
4
5
6
7
8
9

```

0..10的[iterator](https://doc.rust-lang.org/1.2.0/book/iterators.html)  在循环语法中包含了下限但是不包含上限。更通用的语法：

```
for var in iterator {
    // do stuff
}

```

我个人认为，所有的编程语言都应该采用iterators单一语法来实现循环。 这样的简单明了的方式使得Rust语言通俗易懂，这样就可以通过定制iterator 来让循环变得更加强大，甚至可以达到Go语言的复杂格式的循环：

```
for i := 0; i < 10; i++ {
	fmt.Println(i)
}

```

Rust语言的循环没有说明iteration(i++)的每一个指针步增时候的情况， 也没有说明如何在（i<10)前如果步进到下一步，仅仅是让iterator走完。

循环继续和中断：

```
for x in 0..10 {
    if x > 5 && x < 7 {
        continue
    }
    println!("{}", x);
}

```

关键词`continue`都是跟所有语言一样，在这个例子中当`x>5`并且`x<7`  的时候在循环中`continue`到下一个iteration中而不打印，这样结果是：

```
0
1
2
3
4
5
7
8
9

```

中断循环也是类似：

```
for x in 0..10 {
    if x > 5{
        break
    }
    println!("{}", x);
}
打印结果：
0
1
2
3
4
5

```

嵌套循环：

在很多编程语言，嵌套循环都非常诡异，比如，我们如何在一个嵌套循环中让外循环继续的条件建立在内循环里呢？ 让我们看看下面的例子：

```
'outer: for x in 0..5 {
    for y in 0..5 {
        if y > 2{
            break 'outer
        }
        println!("x: {}, y: {}", x, y);
    }
}
打印：
x: 0, y: 0
x: 0, y: 1
x: 0, y: 2

```

利用``outer`标签使得直接控制从哪个循环中断出来非常清晰。 缺省的中断就是直接从内循环中断出来，利用标签的方式，也可以用来实现循环继续。

### 利用 Rust 的自动向量化

在 Rust 代码使用 SIMD CPU 指令优化，学习如何快速检查编译器的汇编程序输出，以及修改 Rust 代码以生成更快程序的方法。其中 SIMD 是单指令多数据 Single Instruction Multiple Data 的缩写。

[博客文章](https://nickwilcox.github.io/blog/autovec/)，https://nickwilcox.github.io/blog/autovec/

### async-graphql

async-graphql，是用 Rust 语言实现的 GraphQL 服务端库。

[教程](https://async-graphql.github.io/async-graphql/zh-CN/introduction.html)，https://async-graphql.github.io/async-graphql/zh-CN/introduction.html

[Github 链接](https://github.com/async-graphql/async-graphql)，https://github.com/async-graphql/async-graphql

[示例](https://github.com/async-graphql/examples)，https：//github.com/async-graphql/examples

[基准测试](https://github.com/async-graphql/benchmark)，https://github.com/async-graphql/benchmark

### [视频] Rust 基础知识

这个视频是 Jonathan Turner 教 Jason Turner 有关 Rust 基础知识的直播视频。Jonathan 是 NuShell 的作者，而 Jason 是一位 C++ 专家。

[视频链接](https://www.youtube.com/watch?v=EzQ7YIIo1rY)，https://www.youtube.com/watch?v=EzQ7YIIo1rY

### Rust全域變數教學

裡面討論了 let, const, std::include_str!

等等不同關鍵字的用法

[Read more](https://github.com/paulkernfeld/global-data-in-rust)

### 如何使用 WASM 作为抽象平台

已经有几种通用运行时可用于在 Rust 程序中运行 Wasm【如：https://github.com/bytecodealliance/wasmtime】。 这些运行时为您提供了一个可以运行任意代码的虚拟机，这与细节区分应用程序运行于哪个平台是有区别的。详细请看博文：http://adventures.michaelfbryan.com/posts/wasm-as-a-platform-for-abstraction/?utm_source=users-forum&utm_medium=social&utm_campaign=wasm-platform-for-abstraction

### 使用 Tokio-Serde 实现作业队列

这是一个系列博客，在作者的上一个博客中，他使用 tmq（https://github.com/cetra3/tmq） 实现了作业队列。 本篇博客作者将使用 tokio-serde 而不是 tmq 来构建具有较小库占用空间的作业队列。

详情请看：https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/

### 在 Rust 中使用全局数据

This guide explains how you can use "global data" in Rust. When I say "global data," I mean data that is loaded near the start of the program and is accessible in almost all of the program.

Possible use cases for global data:

-   App configuration, e.g. weapon characteristics for a game
-   Making data available everywhere without needing to pass it as an argument through all functions (apply this carefully!)
-   Generating Rust code from external data
-   Database connections... or other network resources?
-   A logger, maybe

地址：https://github.com/paulkernfeld/global-data-in-rust

### 【视频】用 Actix 和 Juniper 开发一个简单的 GraphQL API

[https://youtu.be/7v7ERnrC4fo](https://youtu.be/7v7ERnrC4fo)

### 【教程】2020年学习Rust

[https://github.com/pretzelhammer/rust-blog/blob/master/posts/learning-rust-in-2020.md#Practical-Rust-Resource-Reviews](https://github.com/pretzelhammer/rust-blog/blob/master/posts/learning-rust-in-2020.md#Practical-Rust-Resource-Reviews)

### 【教程】用 Rust 开发 RISCV 操作系统之文件系统

[https://www.reddit.com/r/rust/comments/gieoc1/filesystems_riscv_os_in_rust/](https://www.reddit.com/r/rust/comments/gieoc1/filesystems_riscv_os_in_rust/)

### 【博客】Rust Traits: Iterator

[https://dpbriggs.ca/blog/Rust-Traits-Iterator](https://dpbriggs.ca/blog/Rust-Traits-Iterator)

### 【博客】为 Rust-Analyzer 贡献代码，我学会了什么

[https://dev.to/bnjjj/what-i-learned-contributing-to-rust-analyzer-4c7e](https://dev.to/bnjjj/what-i-learned-contributing-to-rust-analyzer-4c7e)

### 清华大学操作系统课程推荐

清华大学操作系统课(RISC-V)以 RISC-V 为 CPU 平台实例，讲解操作系统中如何管理和协调应用程序对计算机系统中软硬件资源的使用。其中第 21 章讲异步编程，包含 Rust 异步编程的优秀设计。

![](https://qn-next.xuetangx.com/15676613232012.png)

学堂在线课程主页：https://next.xuetangx.com/course/thu08091002729/3175284

### 【博客】关于 io-uring 的笔记

[https://boats.gitlab.io/blog/post/io-uring/](https://boats.gitlab.io/blog/post/io-uring/)

## 学习笔记：用Rust来写编译程序

> Notes on Parsing in Rust
> 
> [https://blog.wesleyac.com/posts/rust-parsing](https://blog.wesleyac.com/posts/rust-parsing)

学习笔记：用Rust来写编译程序。

## slip 0.1.0: 保护你程序的错误代码字符串被逆向工程获取。

> slip 0.1.0: protect your error strings against reverse-engineering
> 
> [https://hub.docker.com/r/michaelfbryan/mdbook-docker-image](https://hub.docker.com/r/michaelfbryan/mdbook-docker-image)

slip 0.1.0: 保护你程序的错误代码字符串被逆向工程获取。

安装：

```
cargo install --path unslip

```

使用，直接产生秘密字符串：

```
unslip key

```

系统环境变量设置：

```
#### Linux
$ export SLIP_KEY=<your key, without quotations>   
 
#### Windows    
$ set SLIP_KEY=<your key, without quotations>            

```

## Type-level Rust编程

> Type-level Programming in Rust
> 
> [http://willcrichton.net/notes/type-level-programming/](http://willcrichton.net/notes/type-level-programming/)

Type-level Rust编程。英文的学习笔记，大家自己欣赏哈～

## Github自动编译Rust二进制代码

> Github Actions to build Rust Binaries Automatically
> 
> [https://github.com/zackify/flydb/blob/master/.github/workflows/build-binary.yml](https://github.com/zackify/flydb/blob/master/.github/workflows/build-binary.yml)

Github自动编译Rust二进制代码的配置文件的一个例子：

```
name: Build Binaries
on: 
  release:
    types: # This configuration does not affect the page_build event above
      - created
jobs:
  build_for_mac:
    name: MacOS
    runs-on: macos-10.15
    steps:
      - uses: actions/checkout@master
      - uses: actions-rs/cargo@v1
        with:
          command: build
          args: --release
      - name: Rename binary
        run: mv target/release/flydb flydb-macos64
      - name: Upload to release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          curl \
            -f \
            -sSL \
            -XPOST \
            -H "Authorization: token $GITHUB_TOKEN" \
            -H "Content-Length: $(stat -f%z flydb-macos64)" \
            -H "Content-Type: application/octet-stream" \
            --upload-file "flydb-macos64" \
            "https://uploads.github.com/repos/$GITHUB_REPOSITORY/releases/$(jq --raw-output '.release.id' $GITHUB_EVENT_PATH)/assets?name=flydb-macos64"
  build_for_pi:
    name: Raspberry Pi
    runs-on: ubuntu-18.04
    steps:
      - uses: actions/checkout@master
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: armv7-unknown-linux-gnueabihf
          override: true
      - uses: actions-rs/cargo@v1
        with:
          use-cross: true
          command: build
          args: --release --target armv7-unknown-linux-gnueabihf
      - name: Rename binary
        run: mv target/armv7-unknown-linux-gnueabihf/release/flydb flydb-armv7-pi
      - name: Upload to release
        uses: JasonEtco/upload-to-release@d648f1babf776de9cad881320bd9e9818fc3b262
        with:
          args: flydb-armv7-pi application/octet-stream
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  
  build_for_android:
    name: Android
    runs-on: ubuntu-18.04
    steps:
      - uses: actions/checkout@master
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: aarch64-linux-android
          override: true
      - uses: actions-rs/cargo@v1
        with:
          use-cross: true
          command: build
          args: --release --target aarch64-linux-android
      - name: Rename binary
        run: mv target/aarch64-linux-android/release/flydb flydb-linux-android
      - name: Upload to release
        uses: JasonEtco/upload-to-release@d648f1babf776de9cad881320bd9e9818fc3b262
        with:
          args: flydb-linux-android application/octet-stream
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  build_for_linux:
    name: Linux
    runs-on: ubuntu-18.04
    steps:
      - uses: actions/checkout@master
      - uses: actions-rs/cargo@v1
        with:
          command: build
          args: --release
      - name: Rename binary
        run: mv target/release/flydb flydb-linux-amd64
      - name: Upload to release
        uses: JasonEtco/upload-to-release@d648f1babf776de9cad881320bd9e9818fc3b262
        with:
          args: flydb-linux-amd64 application/octet-stream
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

```

## `rust`语言学习笔记：理解`structs`

> Understanding rust lang - structs notes
> 
> [https://www.staszewski.me/rust-struct-notes/](https://www.staszewski.me/rust-struct-notes/)

波兰小伙子[`Kamil Staszewski`](https://github.com/staszewski)  的Rust语言学习笔记。

什么是`struct`：

`struct`可以让我们创建有内容的数据结构，`struct`数据结构的写法有点类似`Typescript`语言中的`interface`接口， 当然很多别的编程语言的接口的实现也很类似：

```
struct Player {
    name: String,
    health: u32,
    mana: u32,
}

```

要实现一个`struct`我们就要给它创建一个实例，我们依照它的定义用K/V方式给它赋值：

```
let player1 = Player {
    name: String::from("Kamil"),
    health: 100,
    mana: 100,
};

```

至此，我们还不能直接用`println!`宏来打印出来，否则会遇到`Player doesn't implement std::fmt::Display: Player cannot be formatted with the default formatte`  这样的错误。因为这里`struct`还没有实现它的`trait`，这就需要我们参照[官方文档](https://doc.rust-lang.org/std/fmt/trait.Display.html#examples)自己来实现一个。 我们可以加这样的注释`#[derive(Debug)]`也可以用缺省的格式`:?`，或者在`println!`宏里面用花括号来表示缺省格式。 然后我们可以试一试：

```
#[derive(Debug)] // annotation
struct Player {
    name: String,
    health: u32,
    mana: u32,
}

let player1 = Player {
    name: String::from("Kamil"),
    health: 100,
    mana: 100,
};

println!("Players data {:?}, player1);

```

怎么更新数据：

```
let player1 = Player {
    name: String::from("Kamil"),
    health: 100,
    mana: 100,
};

let player2 = Player {
    name: String::from("Gustav"),
    ..player1
}; // player2 has the same health and mana values as player1

```

也可以直接给实例的某个键赋值，不过要记得加上`mut`关键字保证数据结构的实例可以更改：

```
let mut player1 = Player {
    name: String::from("Kamil"),
    health: 100,
    mana: 100,
};

player1.name = String::from("Some other name");

```

给数据结构`struct`添加函数（方法）：

rust语言添加函数非常方便，首先它又一个可以自引用的语法`&self`，这个跟JS的`this`和Python的`self`非常类似。 我们必须用`impl`关键字并加上数据结构的名字来'实现'实例，记住我们在函数内部用`&self`来应用数据机构本身：

```
struct Player {
    name: String,
    health: u32,
    mana: u32,
}

impl Player {
    fn multiply_by(&self, n: u32) -> u32 {
      self.health * n
    }
}

let player1 = Player {
    name: String::from("Kamil"),
    health: 100,
    mana: 100,
};

println!("Players multipied health {:?}", player1.multiply_by(3));

```

作者的GitHub：[https://www.github.com/staszewski](https://www.github.com/staszewski)

## kibi 0.2.0: 一个用≤1024行Rust代码编写的文本编辑器, 现在也兼容Windows啦！

> kibi 0.2.0: a text editor in ≤1024 lines of Rust, now compatible with Windows
> 
> [https://github.com/ilai-deutel/kibi](https://github.com/ilai-deutel/kibi)

这个可配置的文本编辑器支持UTF-8，步进搜索，语法加亮，行数显示等很多功能。只用了不到1024行Rust语言代码，最小化的依赖库。

Kibi现在兼容Linux，MacOS和Windows 10(beta).

这个项目受一个用C语言写成的[Kilo](https://github.com/antirez/kilo)文本编辑器的启发， 可以在网上找到[两者比较](https://github.com/ilai-deutel/kibi#comparison-with-kilo)发现更多的功能。欢迎开源参与，不过要记住1024行代码的限制。

安装Cargo就可以完成：

```
$ cargo install kibi

```

应用：

```
kibi              # Start an new text buffer
kibi <file path>  # Open a file
kibi --version    # Print version information and exit

```

一些快捷键：

```
Ctrl-F	Incremental search; use arrows to navigate
Ctrl-S	Save the buffer to the current file, or specify the file path
Ctrl-G	Go to <line number>[:<column number>] position
Ctrl-Q	Quit
Ctrl-D	Duplicate the current row

```

### minisudo

minisudo，一个类 Unix 操作系统的小型 sudo 式特权提升（Privilege escalation）。

用户可以在 TOML 文件`/etc/minisudo-rules.toml`中指定运行哪些程序的规则。示例：

```
＃用户“ben”可以运行“ls”，但不能执行其他操作。
[[rule]]
user = “ben”
program = “/bin/ls”

＃“staff”组的所有成员都可以执行“whoami”，但其他操作不行。
[[rule]]
group = “staff”
program = “/usr/bin/whoami”

```

[Github](https://github.com/ogham/minisudo)，https://github.com/ogham/minisudo

###【视频】了解 Rust 字符串

该视频适用于Rust的“高级初学者”，熟悉所有权和字符串的概念，但尚未牢牢掌握它们。

[Youtube 链接](https://www.youtube.com/watch?v=7I11degAElQ&feature=youtu.be)，https://www.youtube.com/watch?v=7I11degAElQ&feature=youtu.be

### 发布：RustCrypto:  `p256`  和  `k256`  v0.2.0: 纯 Rust NIST P-256 和 secp256k1 曲线算法

这个版本用 Rust 实现了如下两个曲线算法。

• p256: NIST P-256

```
  □ GitHub: https://github.com/RustCrypto/elliptic-curves/tree/master/p256

  □ crates.io: https://crates.io/crates/p256

  □ docs.rs: https://docs.rs/p256/

```

• k256: secp256k1 (as used by Bitcoin, Ethereum, etc)

```
  □ GitHub: https://github.com/RustCrypto/elliptic-curves/tree/master/k256

  □ crates.io: https://crates.io/crates/k256

  □ docs.rs: https://docs.rs/k256/

```

### tco: 用过程宏实现的尾递归优化能力

这个思路很清奇——用属性宏来实现尾递归调用优化。类似于下面这个样子：

```
#[tco::rewrite]
fn fac_with_acc(n: u128, acc: u128) -> u128 {
    if n > 1 {
        fac_with_acc(n - 1, acc * n)
    } else {
        acc
    }
}

```

优化后的代码为：

```
fn fac_with_acc(n: u128, acc: u128) -> u128 {
    let mut n = n;
    let mut acc = acc;
    '__tco_loop: loop {
        return {
            if n > 1 {
                {
                    let __tco_0 = (n - 1, acc * n);
                    n = __tco_0.0;
                    acc = __tco_0.1;
                    continue '__tco_loop;
                }
            } else {
                acc
            }
        };
    }
}

```

仓库：https://github.com/samsieber/tco

## June --202006


## 动态链接库统计分析

> Statistics on dynamic linking
> 
> [https://drewdevault.com/dynlib.html](https://drewdevault.com/dynlib.html)

你开发部署的程序比如一个普通的系统有经常用到动态链接库吗？统计表明很少。 一般系统使用动态链接库的检测可以通过通过一个脚本来统计出来：

```
libs.awk
--------------------------
    /\t.*\.so.*/ {
        n=split($1, p, "/")
        split(p[n], l, ".")
        lib=l[1]
        if (libs[lib] == "") {
            libs[lib] = 0
        }
        libs[lib] += 1
    }
    END {
        for (lib in libs) {
            print libs[lib] "\t" lib
        }
    }

Usage 用法：
-------------------------
    $ find /usr/bin -type f -executable -print \
      | xargs ldd 2>/dev/null \
      | awk -f libs.awk \
      | sort -rn > results.txt
    $ awk '{ print NR "\t" $1 }' < results.txt > nresults.txt
    $ gnuplot
    gnuplot> plot 'nresults.txt'

my results，我的测试结果：
--------------------------
    $ find /usr/bin -type f -executable -print | wc -l
    5688
    $ head -n20 < results.txt
    4496	libc
    4484	linux-vdso
    4483	ld-linux-x86-64
    2654	libm
    2301	libdl
    2216	libpthread
    1419	libgcc_s
    1301	libz
    1144	libstdc++
    805	liblzma
    785	librt
    771	libXdmcp
    771	libxcb
    771	libXau
    755	libX11
    703	libpcre
    667	libglib-2
    658	libffi
    578	libresolv
    559	libXext


```

动态链接库会很快吗？ 答案是不，还是静态链接库快很多，结果：

```
    Linkage	Avg. startup time
    Dynamic	137263 ns
    Static	64048 ns

```

更多的结果和统计大家亲自看原文吧。

## Rust + Actix + CosmosDB (MongoDB) API入门教程.

> Rust + Actix + CosmosDB (MongoDB) tutorial API.
> 
> [https://dev.to/jbarszczewski/rust-actix-cosmosdb-mongodb-tutorial-api-17i5](https://dev.to/jbarszczewski/rust-actix-cosmosdb-mongodb-tutorial-api-17i5)

在做一个开发项目的时候需要一个简单的API的后台，就打算自己搞一个，Rust是最佳选择，同时也打算以此新学习一门全新的技术。 现在微软的Azure CosmosDB也有免费的VM可以用，所以简直就是为学习并快速使用Rust语言做小型项目最完美的环境。

完成后的项目代码放在github上：[https://github.com/jbarszczewski/plant-server](https://github.com/jbarszczewski/plant-server)

小编注：我很欣赏国外工程师这种学习精神，学习的时候同时就从代码开始原创一个idea并分享出来给社区。利人利己皆大欢喜。

# electron rust

一个使用rust开发native程式的范例

[Read more](https://github.com/rousan/electron-with-rust)

### 通过 Cloud 文档类比可视化 Rust 的所有权和借用

对于初学 Rust 的朋友来说，所有权和借用是一个棘手的概念。 作者尝试为此提供简单的视觉类比。原文地址：https://webminal.org/rust_ownership_borrowing/

![](https://webminal.org/static/rust_ownership_borrowing.jpg)


### 用 Rust 和 Rocket 构建一个智能书签工具

一个比较实用的教程：https://developers.facebook.com/blog/post/2020/06/03/build-smart-bookmarking-tool-rust-rocket/

## Rust搞定WebAssembly: 一个好例子让你一文读懂。

> Rust with WebAssembly: Simple explanation with a nice example
> 
> [https://blog.knoldus.com/rust-with-webassembly-simple-explanation-with-a-nice-example/](https://blog.knoldus.com/rust-with-webassembly-simple-explanation-with-a-nice-example/)

Rust搞定WebAssembly: 一个好例子让你一文读懂。

## Rust语言缩短编译时间技巧

> Tips for Faster Rust Compile Times
> 
> [https://endler.dev/2020/rust-compile-times/](https://endler.dev/2020/rust-compile-times/)

Rust语言的运行效率，一直都是可以更C/C++比肩，但是如果谈到编译时间？哈哈，不一定了，是真慢，因为大量的编译检测和工作。所以需要看看这些技巧。

## 从零学习Rust语言到能用于生产系统应用 #2: 学习创建电邮新闻应用

> Zero To Production In Rust #2: Learn By Building An Email Newsletter
> 
> [https://www.lpalmieri.com/posts/2020-06-21-zero-to-production-2-learn-by-building-an-email-newsletter/](https://www.lpalmieri.com/posts/2020-06-21-zero-to-production-2-learn-by-building-an-email-newsletter/)

从零开始学习Rust语言到精通用于生产系统级别水平，计划写成一本开放的，自由的每次一篇博客文章的形式出版的开放书。 用户可以通过提供的链接的网站订阅更新和进展。这篇是系列第二章《学习创建邮件新闻》。

## 用NDArray实现多维数组及其操作。

> Multidimensional Arrays and Operations with NDArray
> 
> [https://shahinrostami.com/posts/programming/rust-notebooks/multidimensional-arrays-and-operations-with-ndarray/](https://shahinrostami.com/posts/programming/rust-notebooks/multidimensional-arrays-and-operations-with-ndarray/)

这篇文章介绍了`ndarray`这个crate，用来作多维数组的容器和操作。演示如何创建多维数组，如何找到数组的维度，检索以及如何对多维数组作基本的数学操作。

## 如何在Rust语言中应用C++ polymorphism多态性。

> How to use C++ polymorphism in Rust
> 
> [https://medium.com/swlh/how-to-use-c-polymorphism-in-rust-76e1d1a88ed1](https://medium.com/swlh/how-to-use-c-polymorphism-in-rust-76e1d1a88ed1)

C++一个非常普遍的设计范式就是可以先写一个抽象类，然后用户可以继承这个抽象类并实现具体的商业逻辑。 具体的实体类就可以具体实现这个抽象类的函数并传递给别的类库。这个文章告诉大家如何在Rust语言中实现和调用C++多态性。

代码Github:  [https://github.com/sakex/Rust-CPP-FFI-Polymorphism](https://rustcc.cn/article?id=52602e3b-81b7-4dd0-ace5-f59648c9703d)

### .zip() 方法迭代两个数组

#rust #track

```
fn main() {
    let x_coordinates: [f32; 4] = [0., 1., 2., 3.];
    let y_coordinates: [f32; 4] = [0., 1., 4., 9.];
    
    for (x, y) in x_coordinates.iter().zip(y_coordinates.iter()) {
        
        if *x <= 0.0 || *y <= 1.0 {
            continue
        }
        
        println!("x = {}; ln(y) = {}", x, y.ln());
    }
}

```

### Wgpu-rs examples 特性矩阵

#rust #graphics

在学习使用一个新的crate时，我们可能想看一些满足我们需求的示例代码，但是通常我们要自己去浏览所有的示例才能找到我们需要的那个，wgpu-rs制作了一个特性矩阵，列举了每个example都是用了哪些特性，非常人性化，建议开源项目作者可以尝试用这种方式指引开发者。

[Read More](https://github.com/gfx-rs/wgpu-rs/tree/master/examples#feature-matrix)

### Rust 视频教程

-   CS196，这个系列视频教程是 UIUC 的课程。

[视频集链接](https://www.youtube.com/channel/UCRA18QWPzB7FYVyg0WFKC6g/videos)，https://www.youtube.com/channel/UCRA18QWPzB7FYVyg0WFKC6g/videos

-   学习 Rust，这个系列是针对 C++ 程序员和游戏开发人员

[视频链接](https://www.youtube.com/watch?v=1asufxmZW5A)，https://www.youtube.com/watch?v=1asufxmZW5A

### 理解 Rust 中的字符串

有助于 Rust 新手理解基础知识的文章：

-   解释 String 和 ＆str 区别的[文章](https://blog.thoughtram.io/string-vs-str-in-rust/)：https://blog.thoughtram.io/string-vs-str-in-rust/
-   在 Rust 中使用字符串的[文章](https://fasterthanli.me/blog/2020/working-with-strings-in-rust/)：https://fasterthanli.me/blog/2020/working-with-strings-in-rust/
-   在 Rust 中使用字符串的[文章的中文翻译](https://mp.weixin.qq.com/s/ZX_0G6JcNMusLz6JJOkNSg)：https://mp.weixin.qq.com/s/ZX_0G6JcNMusLz6JJOkNSg


### 以Rust为例讲解消息传输的常见bug

[https://medium.com/@polyglot_factotum/rust-concurrency-the-archetype-of-a-message-passing-bug-817b60efd8f8](https://medium.com/@polyglot_factotum/rust-concurrency-the-archetype-of-a-message-passing-bug-817b60efd8f8)

### fortraith，实现于 Rust 的 trait 系统

Rust 的 trait 系统是图灵完备的，[crate fortraith](https://docs.rs/fortraith/0.1.3/fortraith/)（https://docs.rs/fortraith/0.1.3/fortraith/）使用 trait-eval 中的原则使用户可以根据需要充分利用 trait，并提供`forth!`宏产生  [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language))  语法的 trait 表达式。

[Github 链接](https://github.com/Ashymad/fortraith)，https://github.com/Ashymad/fortraith

### `derive_aktor`

`derive_aktor`  是一个宏库，使用起来还是挺方便的，项目地址：https://github.com/insanitybit/derive_aktor

使用示例:

```
pub struct KeyValueStore<U>
    where U: Hash + Eq + Send + 'static
{
    inner_store: HashMap<U, String>,
    self_actor: Option<KeyValueStoreActor<U>>,
}

impl<U: Hash + Eq + Send + 'static> KeyValueStore<U> {
    pub fn new() -> Self {
        Self {
            inner_store: HashMap::new(),
            self_actor: None,
        }
    }
}

// All methods in this block form our Actor's API
#[derive_actor]
impl<U: Hash + Eq + Send + 'static> KeyValueStore<U> {
    pub fn query(&self, key: U, f: Box<dyn Fn(Option<String>) + Send + 'static>) {
        println!("query");
        f(self.inner_store.get(&key).map(String::from))
    }

    pub fn set(&mut self, key: U, value: String) {
        println!("set");
        self.inner_store.insert(key, value);
    }
}


#[tokio::main]
async fn main() {

    let (kv_store, handle) = KeyValueStoreActor::new(KeyValueStore::new()).await;
    
    // We can use an async API that's typed and nominal
    kv_store.query("foo", Box::new(|value| println!("before {:?}", value))).await;
    kv_store.set("foo", "bar".to_owned()).await;
    kv_store.query("foo", Box::new(|value| println!("after {:?}", value))).await;

    // We must drop any references to kv_store before we await the handle, or it will leak!

    drop(kv_store);
    handle.await;
}

```

### 进一步理解 Rust 错误处理

本文 Rust 的初学者和对 Rust 感兴趣却还没能很好上手的朋友更加友好。阅读和理解大约需要15分钟，仅需一些基本的编程知识，你就能很好的理解本文。

在文章中，作者会介绍 Rust 中错误的基本知识，并分析为什么会这样处理，然后解释一些更高级的错误处理概念，同时分享一些实用的技巧。原文地址：https://www.halcyon.hr/posts/error-handling-in-rust/

reddit 上参与讨论：https://www.reddit.com/r/rust/comments/h12itt/error_handling_in_rust_a_beginners_guide/

### Rust/WinRT快速入门

> Getting started with Rust/WinRT
> 
> [https://kennykerr.ca/2020/06/05/getting-started-with-rust-winrt/](https://kennykerr.ca/2020/06/05/getting-started-with-rust-winrt/)

加拿大小伙子Kenny Kerr写的Rust/WinRT编程快速入门。

Rust/WinRT编程快速入门已经非常简单，这得益于程序员喜欢的Rust语言编程工具链提供了大量的便利。如果你想不需要额外的帮助直接入门，下面是一些有用的链接

-   GitHub:  [https://github.com/microsoft/winrt-rs](https://github.com/microsoft/winrt-rs)
-   Docs.rs:  [https://docs.rs/winrt/](https://docs.rs/winrt/)
-   Crates.io:  [https://crates.io/crates/winrt](https://crates.io/crates/winrt)

下面会给一些个人的心得和小技巧：

安装先决条件和工具：

-   [Visual Studio 2019](https://visualstudio.microsoft.com/downloads/)  – be sure to install the C++ tools as this is required by the Rust compiler (only the linker is required).
-   [Visual Studio Code](https://code.visualstudio.com/Download)  – this is the default IDE used for Rust.
-   [Python](https://www.python.org/downloads/)  – be sure to install the x64 version as this is required for debugging support.
-   [Git](https://git-scm.com/downloads)  – Rust has deep support for Git.
-   [Rust](https://rustup.rs/)  – this installs  `rustup`  which is a tool for installing Rust toolchains and common Rust related tooling.

打开VS Code然后键入`Ctrl+Shift+X`打开扩展页安装下面的extensions:

-   rust-analyzer – there are others, but this is the only Rust extension that I’ve tried that actually works reliably most of the time.
-   CodeLLDB – you can also use the Microsoft C++ extension for debugging, but this one does a better job of integrating with the IDE.
-   C/C++ – the Microsoft C++ extension doesn’t integrate as well with the IDE, but provides superior debugging information, so you may want to have that on hand for an emergency.

提示下载并安装Rust language server，确认安装，然后重新启动IDE。然后我们开始用新的cargo包创建例子：

```
C:\>cargo new sample
     Created binary (application) `sample` package

C:\>cd sample
C:\sample>code .

```

新创建的项目目录下修改Cargo.toml配置文件，并添加WinRT的依赖库包：

```
[dependencies]
winrt = "0.7.0"

```

确认所有的库是最新的，然后开始编译项目：

```
C:\sample>cargo build
    Updating crates.io index
   Compiling proc-macro2 v1.0.18
   Compiling unicode-xid v0.2.0
   ...
   Compiling winrt_gen_macros v0.7.0
   Compiling winrt_gen v0.7.0
   Compiling winrt_macros v0.7.0
   Compiling winrt v0.7.0
   Compiling sample v0.1.0 (C:\sample)
    Finished dev [unoptimized + debuginfo] target(s) in 19.65s

C:\sample>cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target\debug\sample.exe`
Hello, world!

```

在项目文件夹里找到写hello world的源文件main.rs，我们用winrt::import macro来生成Rust bindings for WinRT的APIs：

```
winrt::import!(
    dependencies
        os
    types
        windows::data::xml::dom::*
);

```

其实你在main.rs里面任何位置放置上面的代码都可以，这个导入的宏分成两个部分：一类是你的项目中需要标识WinRT组件，另一类是特别需要相应的类型子集。 这里用了`os`表示需要运行的操作系统，也可以指定特定版本的Windows SDK。然后指定了官方文档中的一些类型`windows::data::xml::dom`  下面还有用了`XmlDocument`，具体的细节可以参考[官方文档](https://docs.microsoft.com/en-us/uwp/api/windows.data.xml.dom.xmldocument):

```
fn main() -> winrt::Result<()> {
    use windows::data::xml::dom::*;
 
    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;
 
    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    assert!(root.inner_text()? == "hello world");
 
    Ok(())
}

```

编译运行的结果：

```
C:\sample>cargo run
   Compiling sample v0.1.0 (C:\sample)
    Finished dev [unoptimized + debuginfo] target(s) in 8.71s
     Running `target\debug\sample.exe`

```

这样，import宏导进来的库就可以开始调用指定的`Windows API`了。

### 在MacOS上交叉编译到树莓派上的开发

这篇博客记录了其交叉编译过程。

https://amritrathie.now.sh/posts/2020/03/06/cross-compiling-rust-from-macos-to-raspberry-pi/

### 剖析C程序和Rust程序在程序启动和停止过程中的系统调用

这篇大神级的文章，分析比较了C程序和Rust程序在程序启动和停止过程中的系统调用。

https://gist.github.com/Ben-PH/539fd19911cae424cc9f9484fd6b0da9

### 使用 parcel 和 rust 进行 wasm 开发

想研究wasm的，可以顺便读一下这篇博客。

https://dev.p.ota.to/post/parcel-and-rust-a-wasm-romcom-4kgsjnrdm5t/

### 记录 servo 浏览器集成 流 的设计和开发过程

这篇博文，详尽记录了 servo 浏览器集成 流（一种通用的数据传输抽象） 的设计和开发过程。大神级文章。

https://medium.com/@polyglot_factotum/programming-servo-integrating-readablestream-1a7faebeeed7?source=friends_link&sk=e297efa0a9e9d59d59233f3ec7038b1c

## July --202007


### `fixed_vec`一个减少数组冗余边界检查的库

rust的`Vec`在使用索引的时候总会触发边界检查，在某些时候降低了程序的性能。通常解决方法是尽可能使用迭代器来处理数组。

本文通过[Ghosts of Departed Proofs](https://kataskeue.com/gdp.pdf)这篇论文中讨论的技术来减少Rust中冗余边界检查。

[具体内容](https://github.com/Torrencem/fixed_vec/blob/master/post.md)

在文末有提到了性能差距，如果只想看看效果如何的，可以直接跳到文末

代码示例

```
use fixed_vec::{name, FixedVec};

let v = vec![0u32; 10];
let v = name!(v);
let mut v = FixedVec::fix(v);

// Perform the two index checks here:
let index_a = v.check_index(...).unwrap();
let index_b = v.check_index(...).unwrap();

for _ in 0..100 {
    // These do *not* perform bounds checks!
    // At compile time, v and index_a must match
    *v.get_mut(index_a) += 5;
    *v.get_mut(index_b) += 10;
}

let v = v.unfix();

// continue using v...

```

[Github仓库](https://github.com/Torrencem/fixed_vec)

### 在油管发现一个专门更新Rust实战相关视频的博主

视频内容大概有actix、rocket等一系列web线管的实战视频，挺有意思的。

油管博主主页：[Genus-v Programming](https://www.youtube.com/channel/UCSkHbGjrjJmuAbDPhIQ5T0A)

B站有人搬运：[Rust web框架教程](https://www.bilibili.com/video/BV1sD4y1S7QL?p=1)

### 为Linux系统打包Rust项目

文章分为两部分，第二部分还没有发出来

[Read More](https://ebbflow.io/blog/vending-linux-1)

### rusty days活动

[活动地址](https://rusty-days.org/agenda/)

[部分录播地址](https://www.youtube.com/playlist?list=PLf3u8NhoEikhTC5radGrmmqdkOK-xMDoZ)the rust borrow checker 大概在2:24左右开始

#### 关于2021-edition：

这两天刚举办的rusty days线上 Rust 大会，steve 讲了一个Topic，需要有2021-edition吗？ 这里做一些关键点梳理：

1.  里面提到 Edition 的作用范围： 允许增加新的关键字，改进语法，但不允许去对语言一致性、标准库等进行根本性破坏。
    
2.  回顾编译器的编译过程： a. rustc 是多道编译器，从源码到 AST -> HIR(大部分检查、类型检查、方法查找) -> MIR（借用检查） -> LLVM IR b. rustc 是 基于查询 (query-based) 的编译器
    
3.  对于 编译器来说，所有 Edition 的代码，在 MIR 层面都不允许存在差异，即， MIR 是多个Edition的通用语言。 对于 人类开发者来说，Edition 之间会有差异，但不会太多。生态系统也不会搞的像主版本变化那样分裂。
    
4.  Rust 的发布周期是比较固定的，Nightly是每晚，beta和stable是每六周发布一版。但是 Edition现在还没有确定。那什么时候用 Edition呢? 回顾过去的Rust 2018，可以说是即成功又不成功。 成功是说，Rust 团队达成了既定目标，并且完成了一个艰巨的任务。 不成功的地方在于，发布的东西其实并不是计划的全部，并且团队成员长期工作带来了巨大的疲劳和怠倦。 Rust 团队想做的太多，但是他们低估了投入成本。
    
5.  不过，steve表示， 我们应该有一个 Rust 2021 Edition。但它应该比Rust 2018更小的版本，小版本优点会大于缺点。并且在未来保持一个「发行列车」，即便三年内没有什么大的特性，也会坚持发布一个Edition。 对于不使用 Rust 的人，不应该来频繁地关注Rust Edtion的发布信息，他们只需要知道 Rust 已经很稳定就够了！ 对于使用Rust的人来说，每三年的 Edition，其实就是一个 「总结」。
    
6.  2021 edition 应该有什么特性呢？ steve表示其实他并不在意，不必要非得刻意规定什么特性来证明Edition的合理性，哪怕有一个特性错过了发布，那么三年后发布就可以了。 但是，Rust 官方会在 10月份以后发布一个 RFC 的，目前主要是 Niko 和 Steve 在做这个，当然，Niko 还是主力担当。
    
7.  为什么是三年呢？ 一年一次有点过，五年一次太长，三年节奏刚刚好，这一点 c++ 已经证明了。
    

视频[地址](https://www.youtube.com/watch?v=XFhrb-qLX_8&list=PLf3u8NhoEikhTC5radGrmmqdkOK-xMDoZ&index=2&t=0s)  注意这个视频是 33 分钟以后才开始的

### 首届 Rusty Day 线上大会散记： 深入借用检查器

分享者：Nell Shamrell-Harrington ，Mozilla员工

该Topic议程分为两部分： Rust编译器概述 和 深入借用检查器

#### Rust 编译器概述：

介绍了编译阶段：词法、解析、语义分析、优化、代码生成

Token -> AST -> HIR -> MIR -> LLVM IR

在 AST 阶段主要做的工作：宏展开、去糖、处理各种模块导入 HIR的数据结构： Crate(CrateNum) < Definition(DeId) < Node (HirId) > > MIR 的数据结构： Control Flow Graph < bb0(statement -> statement -> terminator ) -> bb1(statement -> terminator ) -> bb2 ((statement > terminator )... >

详细内容可以参阅： https://rustc-dev-guide.rust-lang.org/

优化主要在 MIR 和 LLVM IR 阶段完成，最终由 LLVM IR 通过 LLVM 生成机器码。

深入借用检查：

借用检查器的工作：

1.  跟踪变量的初始化和move
2.  生命周期推导（Lifetime inference） a. 变量的生命周期 b. 引用的生命周期。如果引用一个值，则该引用的生命周期不能超过该值的作用域。
3.  还有很多其他功能，继续参阅rustc dev guide。 （比如 检查不可变和可变借用等）。 https://rustc-dev-guide.rust-lang.org/borrow_check.html

（看完后发现，好像也没有很深入）

视频[地址](https://www.youtube.com/watch?v=XFhrb-qLX_8&list=PLf3u8NhoEikhTC5radGrmmqdkOK-xMDoZ&index=2&t=0s)  注意这个Topic 在 2小时以后

### Stackoverflow里有人探索Rust中的函数指针魔法，写出了一个奇怪的东西

从代码看上去似乎是通过函数空指针调用了函数

[代码Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5ecc01ed5c00f707a9d2d53224db6c9a)

```
fn foo() {
    println!("This is really weird...");
}

fn caller<F>() where F: FnMut() {
    let closure_ptr = 0 as *mut F;
    let closure = unsafe { &mut *closure_ptr };
    closure();
}

fn create<F>(_: F) where F: FnMut() {
    let func_ptr = caller::<F> as fn();
    func_ptr();
}

fn main() {
    create(foo);
    
    create(|| println!("Okay..."));
    
    let val = 42;
    create(|| println!("This will seg fault: {}", val));
}

```

作者本人对这部分代码能够正常运行感到疑惑，特别是为什么foo函数能够被`caller<F>()`函数里强制转化成`nullptr`调用。

已经有大神对这个问题做出了解释，太长了我直接贴地址。中文社区的大佬如果对这个问题感兴趣，可以帮忙在这里回复一下。

可能有小伙伴访问StackOverflow比较缓慢，这里我复制高赞回复的原文：

This program never actually constructs a function pointer to anything but  `caller`- it always invokes  `foo`  and those two closures directly. Every Rust function, whether it's a closure or a  `fn`  item, has a unique, anonymous type. This type implements the  `Fn`/`FnMut`/`FnOnce`  traits, as appropriate. The anonymous type of a  `fn`  item is zero-sized, just like the type of a closure with no captures. Thus, the expression  `create(foo)`  instantiates  `create`'s parameter  `F`  with  `foo`'s type- this is not the function pointer type  `fn()`, but an anonymous, zero-sized type just for  `foo`. In error messages, rustc calls this type  `fn() {foo}`, as you can see  [this error message](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=2c30794e639b53cb5f59212d1fabd174). Inside  `create::<fn() {foo}>`  (using the name from the error message), the expression  `caller::<F> as fn()`  constructs a function pointer to  `caller::<fn() {foo}>`. Invoking this function pointer is the same as calling  `caller::<F>()`  directly, and this is also the only function pointer in the whole program. Finally, in  `caller::<fn() {foo}>`  the expression  `closure()`  desugars to  `FnMut::call_mut(closure)`. Because  `closure`  has type  `&mut F`  where  `F`  is just the zero-sized type  `fn() {foo}`, the  `0`  value of  `closure`  itself is simply never used(注释1), and the program calls  `foo`  directly. The same logic applies to the closure  `|| println!("Okay...")`, which like  `foo`  has an anonymous zero-sized type, this time called something like  [closure@src/main.rs:2:14: 2:36](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7d4f09ea0eb0cb82c57c3e3cb0519c29). The second closure is not so lucky- its type is not zero-sized, because it must contain a reference to the variable  `val`. This time,  `FnMut::call_mut(closure)`  actually needs to dereference  `closure`  to do its job. So it crashes(注释2)

注释1 Constructing a null reference like this is technically undefined behavior, so the compiler makes no promises about this program's overall behavior. However, replacing 0 with some other "address" with the alignment of F avoids that problem for zero-sized types like fn() {foo}, and gives the same behavior!) 注释2 Again, constructing a null (or dangling) reference is the operation that actually takes the blame here- after that, anything goes. A segfault is just one possibility- a future version of rustc, or the same version when run on a slightly different program, might do something else entirely!

后续还有不少回复，能直接访问StackOverflow的建议查看原文

[Read More](https://stackoverflow.com/questions/63164973/wat-code-rust-allows-calling-functions-via-null-pointers)


### 博客：用 Rust 写一个简单的查询系统

[https://pnevyk.github.io/posts/query-system-in-rust/](https://pnevyk.github.io/posts/query-system-in-rust/)

### 教程：用Rust创建一个简单的CRUD

这个教程叫你如何用 rocket.rs 和 diesel.rs 构建一个CRUD程序，其中用 rocket.rs 作为 web 服务器，用 diesel.rs 作为 PostgreSQL 的 ORM 。

[https://medium.com/@luis_50157/a-simple-crud-on-rust-with-rocket-rs-and-diesel-rs-e885672cb23d](https://medium.com/@luis_50157/a-simple-crud-on-rust-with-rocket-rs-and-diesel-rs-e885672cb23d)

## 用Rust从头写一个文件系统

[这篇文章](https://blog.carlosgaldino.com/writing-a-file-system-from-scratch-in-rust.html)  教你如何用Rust从头写一个文件系统。

文章中介绍了文件系统中用到的概念，以及在实现自己的文件系统的过程中这些概念是如何运用的。

代码看这里：[carlosgaldino/gotenksfs](https://github.com/carlosgaldino/gotenksfs)


## Tokio官方新教程

手把手带你构建一个Redis的客户端和服务端来学习Tokio。

[Read more](https://tokio.rs/tokio/tutorial)

### Rust项目测试小技巧

在Rust项目中，如果你要对私有方法做测试，你该怎么办呢？临时将方法改为public？别再这样做了。你可以不创建tests文件夹，而是添加test模块，然后将方法改为pub(crate)。

```
#[cfg(test)]
mod tests;

```

### Easy Rust 让Rust文档更容易理解

Rust的文档已经很全面了，但是很多非英语母语的人理解起来可能比较困难，easy rust用比较容易理解的表述方式组织了Rust文档。

能降低文档理解难度，确实能让一部分人受益，如果你想通过这个文档学习，最好以官方文档为主，将这个文档作为参考，否则可能会走上偏路。

[Repo](https://github.com/Dhghomon/easy_rust)

### 用rust重写web博客

你正在阅读的这篇文章就是这个web站点用rust重新实现的结果，这归功于大量的努力、调研和咨询。如果有人想要引用go的原始代码实现，也可以从这里得到。

如果你发现关于`RSSFeed`、`AtomFeed`和`JSONFeed`的任何问题，请尽快告诉我让我可以修复这些。

[文章链接](https://christine.website/blog/site-update-2020-07-16)

### 响应式IDE的三种架构

`rust-analyzer`是Rust编程语言的一个新IDE后端。在这篇文章中，我们将用三种方式学习如何制作一个简洁的IDE。

它的灵感来自于一篇关于使用`datalog`进行语义分析的优秀文章。这篇文章只描述了最高级别的架构，要实现一个成熟的IDE还需要做更多的工作。

[文章链接](https://rust-analyzer.github.io/blog/2020/07/20/three-architectures-for-responsive-ide.html)


### 在CUDA设备的编译期用Rust进行检查

> **Compile time CUDA device checking in Rust**
> 
> 对于本实验，我将使用Rust编程语言。具体来说，我将使用[tch-rs](https://github.com/LaurentMazare/tch-rs)绑定到libtorch。这允许编写与PyTorch非常相似的代码，但使用Rust。

[Read More](https://m-decoster.github.io/2020/07/24/compile-time-cuda/)

[Read More](https://github.com/LaurentMazare/tch-rs)

### 【经验贴】通过将Python转换为Rust学习Rust

Rust是一种静态和强类型的系统编程语言。Rust适用于渴望使用某种语言的速度和稳定性的人。

[Read More](https://towardsdatascience.com/learning-rust-by-converting-python-to-rust-259e735591c6#3cb0)


### 单一所有权和RAII

从 2011年的 C++ 智能指针开始,讲述了单一所有权和 RAII

[https://vale.dev/blog/raii-next-steps](https://vale.dev/blog/raii-next-steps)

## Rust 模块系统的超清晰解释

Rust 的模块系统可能对于新人来说有些困惑,这篇文章通过几个循序渐进的小例子,让你快速且清晰的对 Rust 模块系统有一个基本的认识.

[Read more](http://www.sheshbabu.com/posts/rust-module-system/)

## 使用 Rust 实现 boids 算法 (Game)

这是 Rust 来实现经典的 Boids 算法的一系列文章的 Part1. 做游戏或者对该算法感兴趣,并且希望来学习 Rust 的同学也可以参考一下.有比较详细的过程和代码.

[Read more](https://blog.bitsacm.in/a-fistful-of-boids/), 关于[Boids](http://www.red3d.com/cwr/boids/)

### Trait 工作组 2020 Sprint 3 总结

Trait 工作组的目标是完善 Chalk ，以便将其最终用于 rustc 中解决 trait的各种问题。 看上去，工作组的一个Sprint周期是6周左右。

Chalk 由多个独立crate组成，其结构：

-   chalk-derive : 定义了派生宏（derive proc macros)
-   chalk-ir : 基本的「类型库」，用于在 rustc、rust-analyzer和Chalk之间共享
-   chalk-solve： 定义 chalk-ir 中类型的Rust 语义
-   chalk-engine ： 实现SLG 求解器
-   chalk-recursive ： 实现递归求解器
-   chalk-parse : 用于测试，将 类似Rust的语法解析为chalk-ir 和 chalk-solve 的类型
-   chalk-integration ： 用于为测试提供有用的类型
-   chalk： 用于测试，提供REPL

关于具体什么是 Chalk，可以看看 Chalk Book： http://rust-lang.github.io/chalk/book/what_is_chalk/crates.html

目前状态：

-   进一步完善 Chalk 对 GAT 的支持。GAT相关 Issues： https://github.com/rust-lang/rust/issues/44265
-   提取表示类型的共享库
-   支持 .chalk 文件用来debug
-   上一个sprint支持了 impl Trait，这个sprint 继续提升对impl Trait的支持
-   完善Chalk对Rust语义的支持
-   处理生命周期约束

感觉目前Chalk还有很多工作要做，接下来 官方宣布成员们要休假，可能到9月份再继续工作。。。

[Read More](https://blog.rust-lang.org/inside-rust/2020/07/17/traits-sprint-3.html)

### 一些底层的学习 Booting to 'Hello Rust!'

作者在通过Rust学习一些操作系统底层的东西，这篇文章是第一篇，记录了他如何在x86_64上引导至裸机Rust。

[Read More](https://micouy.github.io/posts/low-level-pt-1/)

### Rust 中创建基准测试

一篇在Rust中创建基准测试的指南。

[文章链接](https://nbsoftsolutions.com/blog/guidelines-on-benchmarking-and-rust)，https://nbsoftsolutions.com/blog/guidelines-on-benchmarking-and-rust

# 无船同志新博客：Shipping Const Generics in 2020

链接：[https://without.boats/blog/shipping-const-generics/](https://without.boats/blog/shipping-const-generics/)

### Rust 中的函数重载

作者提供了一些思路在 Rust 中实现了函数重载，一个示例的代码：

```
#![feature(unboxed_closures, fn_traits, type_alias_impl_trait, impl_trait_in_bindings)]
use functionate::functionate;
use std::ops::Fn;

#[allow(non_upper_case_globals)]
static foo: impl Fn() -> &'static str + Fn(i32) -> i32 = {
    struct Foo;
    #[functionate]
    impl Foo {
        fn a(&self) -> &'static str { "bar" }
        fn b(&self, x: i32) -> i32 { x * 2 }
    }
    Foo
};

fn main() {
    println!("{}", foo());
    println!("{}", foo(5));
}

```

![运行结果](https://miro.medium.com/max/875/1*MNG-BA3xFECy05PQdA-p5g.png)

具体请查看[原文](https://medium.com/@nrabulinski/function-overloading-in-rust-d591aff64a03)：https://medium.com/@nrabulinski/function-overloading-in-rust-d591aff64a03

### 用 Rust 编写内核驱动

作者在使用 Rust 重写了他的部分 Windows 内核驱动程序，并分享了一些经验，再次过程中的总结，详情请[查看博客原文](https://out.reddit.com/t3_hrwyl8?url=https%3A%2F%2Fnot-matthias.github.io%2Fkernel-driver-with-rust%2F&token=AQAAQmIQX-R0HIdkl3yL2W3c0VUgs6R3cmuRwPkthSrNtCS84p_1&app_name=desktop2x&user_id=134987470566)

[查看reddit讨论区](https://www.reddit.com/r/rust/comments/hrwyl8/writing_a_kernel_driver_with_rust/)：https://www.reddit.com/r/rust/comments/hrwyl8/writing_a_kernel_driver_with_rust/

### 一个使用了三维拟合算法用来解决讲小盒子放入大盒子里的问题的一个 Crate

[crates.io](https://crates.io/crates/bin_packer_3d/)：https://crates.io/crates/bin_packer_3d/

[doc](https://docs.rs/bin_packer_3d/1.0.0/bin_packer_3d/)：https://docs.rs/bin_packer_3d/1.0.0/bin_packer_3d/

### 用Rust重写FORTRAN科学软件

博客地址：  [https://mckeogh.tech/post/shallow-water/](https://mckeogh.tech/post/shallow-water/)

项目地址：  [https://github.com/rse-standrewscs/shallow-water](https://github.com/rse-standrewscs/shallow-water)


### 欢迎来到《Rust之旅》

《Rust之旅》（Tour of Rust）是一个Rust入门教程系列，它将从例子开始，引导初学者尝试使用Rust的各个特性。整个系列包括9个章节，109篇文章，每篇之后都有单独的练习，涵盖Rust语言的大量知识，并带领初学者初步熟悉Rust语言的生态。比较新的语言特性也在计划的开发过程中。这个项目可能是从Tour of Go中获得灵感的，已经发展出自己的网站，并且拥有包含简体中文在内的十种语言翻译，受到广大Rust爱好者的欢迎。

[官方网站](https://tourofrust.com/)



### Rust是一门超棒的服务端开发语言

本篇文章中，作者通过改写他的博客网站，体会了Rust开发网页服务端的优势和不足。作者选择了rocket作为开发框架。

作者认为Rust非常好的地方在于，很少需要亲自处理生命周期和借用检查器的问题。相比Flask，Rocket充分发挥了Rust过程宏灵活性的优势，这给作者留下了深刻的印象。稍微糟糕一些的部分，可能在于一些trait实现的错误非常长，不便于阅读，还有一些第三方库API的设计可能有小问题。

当Rust最终发展为一门生态成熟的语言，作者认为这时候相比Python，Rust在网页开发上会有更多的优势。作者注意到最终成果的资源占用，Rust网页服务器的CPU占用非常低，性能十分良好。

[文章链接](https://stu2b50.dev/posts/rust-is-surpris76171)

### nnnoiseless：把多媒体音频代码重写为Rust

本篇文章中，作者用Rust重写了知名的音频去噪库RNNoise。本次作者重写的动机出于原来的C语言项目缺乏维护，随着时间的推移，已经无法在部分平台上编译。由于这个库可能被其它C语言项目依赖，在FFI部分必须保持兼容，尤其是部分结构体的内存布局必须相同，作者认为这是移植工作的痛点之一。

探针效应指的是添加测试部分后软件的行为会变得不同，作者本次也遇到了类似的头疼问题，不过重新审阅unsafe代码后，作者找到了问题；作者提到这也是不得不和C语言兼容后，Rust编译器无法帮它找到这个问题，如果只考虑safe代码，编译器是能找到问题的。

最终作者测试了库的性能，处理同一段音频，作者用Rust编写的nnnoiseless库需要大约14.5毫秒，原来C语言编写的RNNoise则需要至少17毫秒，重写后性能有了部分的提升。

在文章末尾，作者特别感谢了Rust的包管理器cargo，成熟的包管理器帮了他很大的忙。

[博客链接](https://jneem.github.io/nnnoiseless/)

### 重新思考使用Rust开发web应用

#rust #web

作者尝试了很多个用于开发前端的Rust库，它们确实实现了它们所展示的功能，但缺点也是很明显的。比如API难以理解，复杂度随着代码量逐渐升高等等，如果从JS的解决方式来看这些问题，就会变的很容易，但作者是个Rust死忠粉，所以他尝试解决Rust开发前端的几个问题：

1.  自然的组件使用方式
2.  构建可组合的UI
3.  状态管理

作者按他理想的方式用rust写了伪代码，然后开发了Valerie这个库实现这种方式，一个简单的表单绑定可以这样实现，已经非常接近React了。

```
use valerie::prelude::components::*;
use valerie::prelude::*;

fn ui() -> Node {
    let string = StateMutex::new(String::new());
    let length = StateAtomic::from(&string, |x| x.len());

    div!(
        h3!(string.clone()),
        h3!(length),
        input!("text").bind(string)
    )
    .into()
}

#[valerie(start)]
pub fn run() {
    App::render_single(ui());
}

```

[Read More](https://dev.to/emmanuelantony2000/valerie-rethinking-web-apps-in-rust-4cl3)

### 使用Rocket 0.4开发服务器的感受

#rust #web

作者之前的博客是静态网站生成器实现的，使用一段时间后发现各种麻烦，比如修改内容要SSH，用SFTP上传照片等等（其实还是有更方便的办法的），后来作者重写博客使用SSR渲染的方式，在选择语言、选择框架方面做了一些考虑，最终他选择使用Rocket，但是随着开发中发现的一些问题，比如处理multipart/form-data繁琐，数据库ORM设计不合理的地方。

[Read More](https://stu2b50.dev/posts/rust-is-surpris76171)

### Rust 机器学习应用：K-means 聚类和可视化

虽然机器学习的通用语言是 Python， 但是 Python 本身在计算密集型算法中实际上相当慢的。而 Rust 可能比 Python 快 25 倍。

对于基于 web 的机器学习应用程序来说，基于 Rust、WebAssembly 和 Node.js 的精简软件栈很有意义。Rust 允许我们编写高性能的机器学习函数。为了运行时安全性、跨平台可移植性和基于功能的安全性，可以将这些 Rust 函数编译成 WebAssembly 字节码。然后，开发人员可以从 Node.js 环境中易用的 JavaScript API 访问这些 Rust 函数。本博文作者向我们展示如何将 Rust 函数编译成 WebAssembly，并从 Node.js 应用程序中调用它们。在这个例子中，我们将演示Rust中两个重要的机器学习任务：K-means 聚类和可视化。

详情请看原文：https://www.secondstate.io/articles/machine-learning/

### 嵌入式小组：创建你自己的Rust编译目标

如果您需要支持新的指令集架构、新的操作系统，您应当创建一个新的编译目标。创建新目标并不是轻松的事情，思路可能很复杂。本次Rust嵌入式小组发布的文章，为我们梳理了支持新目标的步骤和技术途径。创建自己的编译目标，包含决定目标三元组、填写目标配置、使用目标配置等三个步骤。填写目标配置时，您应当提供链接器配置、编译器底层架构功能，以及CPU的特性。构建过程中，您可以选择只编译core、alloc库，或者包含自己定义的std库。本篇文章还给出部分技术解答，有助于排除创建过程中遇到常见的问题。

[嵌入式小组博客](https://rust-embedded.github.io/embedonomicon/custom-target.html)

### Rust与短字符串

@killercup撰写了文章《Rust与短字符串》，为我们测试、比较了字符串较短时，Rust最高效处理字符串的方式。字符串常常涉及内存的分配，短字符串的分配和分配器的实现有较大关联。文章从只基于操作系统的简单封装开始，包装自己的全局分配器，以便探测内存使用的情况，包括峰值、操作数、尺寸等等，还给出一个简单的图形化呈现。随后，文章比较了`smol_str`、`smartstring`和标准库的实现，以比较各个情况下实现短字符串的好处和不足。文章补充了在长度一定的前提下，From转换、复制、Into转换所需的时间，给出了详细的折线图，以供比较参考。

[博客文章](https://fasterthanli.me/articles/small-strings-in-rust)

#rust

通过解析1000行JSON字符串到String，SmolStr和SmartString，看看他们的性能和内存特征。干货文章

[Read More](https://fasterthanli.me/articles/small-strings-in-rust)

### 使用Rust和GLSL进行pixel shader创作

#rust #graphics

作者分享他使用Rust和GLSL进行pixel share创作的获奖项目，用了不到4k的代码。

[Read More](https://www.codeslow.com/2020/07/writing-winning-4k-intro-in-rust.html)

### Canrun，逻辑编程库

Canrun 是 Rust 开发的逻辑编程库，其受到  [miniKanren](http://minikanren.org/)  的启发。

示例：

```
use canrun::{Goal, both, unify, var, example::I32};

let x = var();
let y = var();
let goal: Goal<I32> = both(unify(x, y), unify(1, x));
let result: Vec<_> = goal.query(y).collect();
assert_eq!(result, vec![1])

```

相关链接：

-   [介绍文章](https://esimmler.com/announcing-canrun/)，https://esimmler.com/announcing-canrun/
-   [Github](https://github.com/tgecho/canrun_rs)，https://github.com/tgecho/canrun_rs
-   [Crates.io](https://crates.io/crates/canrun)，https://crates.io/crates/canrun
-   [Docs.rs](https://docs.rs/crate/canrun)，https://docs.rs/crate/canrun



### 把C密码学库移植为Rust语言版，我学到了这七件事

[https://sharpend.io/7-things-I-learned-from-porting-a-c-crypto-library-to-rust/](https://sharpend.io/7-things-I-learned-from-porting-a-c-crypto-library-to-rust/)

## August --202008


### 构建一个Rust运行时反射系统（第一篇）

作者的公司在做一款叫oso的产品，旨在通过提供一门叫Polar的申明式语言，方便用户编写策略文件和用户自身应用逻辑解耦来解决authorization的问题。官方提供了不同语言的库来解析这些策略，作者打算写三篇文章来分享他们在用Rust解析的时候涉及到的动态反射的问题，以及他们的解决方案。

为了方便大家了解它们的Polar语言，我在他们官网摘了一小段：

```
allow(actor: String, "GET", _expense: Expense) if
    actor.endswith("@example.com");

```

链接：[https://www.osohq.com/post/rust-reflection-pt-1](https://www.osohq.com/post/rust-reflection-pt-1)

###  Building even faster interpreters in Rust

作者在Cloudflare实习，其中一部分工作是用Rust给防火墙规则（Firewall Rules ）编写匹配引擎，这篇文章里面作者分享了他在这一方面的优化经验。

链接：[https://blog.cloudflare.com/how-we-made-firewall-rules/](https://blog.cloudflare.com/how-we-made-firewall-rules/)

### 用Rust学习网络编程TCP/IP基本原理

Low-Level Academy是一个致力于使底层编程更容易的网站。目前只有第一章节的内容。

[Read More](https://lowlvl.org/): https://lowlvl.org/

### Neovim 加 Rust：高效的开发体验

Neovim Rust 开发环境搭建教程

[https://sharksforarms.dev/posts/neovim-rust/](https://sharksforarms.dev/posts/neovim-rust/)

### 【博客】并发迭代语义

这篇博客中介绍了作者遇到过关于并行异步迭代的挑战。

[https://blog.yoshuawuyts.com/async-iteration/](https://blog.yoshuawuyts.com/async-iteration/)

### 为Rust构建体积小的镜像

#rust #docker

-   分层构建，避免重复构建依赖项
-   为linux:alpine构建，减少镜像体积
-   给予最小运行权限

[Read More](https://shaneutt.com/blog/rust-fast-small-docker-image-builds/)

#rust

[Jean Manguy](https://jean.manguy.eu/)是一位博士后，目前就职于基因组学和宏基因组学的项目，平时的工作都围绕使用Nextflow为现有的命令行工具使用和编写管道而展开，工作中使用最多的是R语言，R语言是一门用于数据探索，统计和数据可视化的解释语言，随着工作变成常态，他发现自己的应用开发能力已经逐渐退步，虽然之前也用C语言做过一些项目，现在他想尝试一门新的语言，于是在疫情期间开始学习Rust。

作者在文中介绍了他学习Rust的目的——开发一款游戏，介绍了他对开发环境、学习资源的整理，以及他做过的一些几个项目，详细介绍了他初学习Rust的历程。

[ReadMore](https://jean.manguy.eu/post/i-started-to-learn-rust/#some-cons-of-rust)

### 基于原生AST的linting的吐槽

第一篇博客！在这篇博客文章中，我想谈谈关于linters的各种问题，它只使用ast(抽象语法树)来linting源代码，以及一种您可能不知道的树表示。在rust的一个名为rslint的JavaScript linter上工作了好几个月，我遇到了很多问题，也探索了很多概念，我觉得我应该分享这些概念来提供一些见解，来解释为什么我认为纯粹基于AST的linting不适用于复杂的linters。

[文章链接](https://rdambrosio016.github.io/rust/2020/09/18/pure-ast-based-linting-sucks.html)，https://rdambrosio016.github.io/rust/2020/09/18/pure-ast-based-linting-sucks.html

## rustdoc文档内链接接近stable

文档内链接是rustdoc的一项特性，可让你通过其名称而不是硬编码的URL链接到“项”（函数，类型等）。即使你的类型重新导出到其他模块或crate中，这也可以使你获得准确的链接。

自2017年以来，文档内链接已经存在了一段时间了。它们在nightly版本可用（因此在docs.rs上可用），因此你可能听说它们还不稳定时可能会感到惊讶。

“现在发生的变化是，它们将在stable版本上可用，这也意味着我们对实现更加有信心，并将大力鼓励使用它们。我们建议你将库切换为使用文档内链接，这样可以修复重新导出的类型的损坏链接以及指向不同crate的链接。我们希望将来增加对通过cargo fix自动执行此过程的支持。”

原文地址:  [https://blog.rust-lang.org/inside-rust/2020/09/17/stabilizing-intra-doc-links.html](https://blog.rust-lang.org/inside-rust/2020/09/17/stabilizing-intra-doc-links.html)

## 使用 Rust + Wasm 探索系统编程

作者在  [Rust playground](https://play.rust-lang.org/)(https://play.rust-lang.org/) 新建了一个新课程，它就是“khan系统编程学院”。它使用 Rust 和 WebAssembly 在浏览器中运行一个虚拟网络(它是一个基于 smoltcp 的真正的 TCP/IP 堆栈实现)。作者从 Bret Victor 的 Learnable Programming 中获得了很多灵感，后续将继续扩展它以涵盖更多的主题，包括更多可探索的演示。

reddit讨论：https://www.reddit.com/r/rust/comments/itzhzl/lowlevel_academy_an_explorable_systems/

课程地址：https://lowlvl.org/lesson1.html

### awak：简单的Rust异步运行时

是由开发者御坂知惠（@cssivision）编写的练习项目。代码风格非常清爽，非常简单。阅读项目的代码，能初步理解Rust语言异步编程的方法。

[GitHub主页](https://github.com/cssivision/awak/)

## [2020年Rust社区调查开始了](https://rustcc.cn/article?id=26836ff0-f3ac-462c-b1b0-430fae9946b7)

[Mike Tang](https://rustcc.cn/blog_with_author?author_id=09e42b7c-c2bc-410a-9079-8ad0370d2603)  发表于  2020-09-11 22:25

2020年Rust社区调查开始了，

希望大家填写以帮助Rust团队了解Rust当前的状况。

链接：

简体中文 (  [https://wj.qq.com/s2/7111747/269f](https://wj.qq.com/s2/7111747/269f)  )

正体中文 (  [https://docs.google.com/forms/d/e/1FAIpQLSdExtEatN0UOsjmadXcGcHyQpwuhsgkLCQb-VLoRzL9P1K5iw/viewform?usp=sf_link](https://docs.google.com/forms/d/e/1FAIpQLSdExtEatN0UOsjmadXcGcHyQpwuhsgkLCQb-VLoRzL9P1K5iw/viewform?usp=sf_link)  )

英文 (  [https://docs.google.com/forms/d/e/1FAIpQLSf__XKjS2xa55jUOi78ONvjG0elG5ZWqOz0MYdX6sgmcjb5pw/viewform?usp=sf_link](https://docs.google.com/forms/d/e/1FAIpQLSf__XKjS2xa55jUOi78ONvjG0elG5ZWqOz0MYdX6sgmcjb5pw/viewform?usp=sf_link)  )

（与往年一样，考虑到大陆地区访问，简体中文版使用了腾讯的问卷系统，对隐私有忧虑的可以考虑填写正体中文或英文版，题目是一样的）

### 你不会相信这个奇怪的技巧可以加速和提高你的测试

标题是直译原文的，虽然看上去有种标题党的嫌疑，但是就如原文所说的，这种方法在之前已经有不少人用过了，但是从来没有谁把它放到台面上分享给大家，这是一种实用的方法。

作者在github上维护一个[命令行工具  `Intermodal`](https://github.com/casey/intermodal/)，需要大量测试用户输入与程序输出是否符合预期，因此用到了这个怪异的方法。

使用起来大概时这样的：

```
  #[test]
  fn quiet() {
    let mut env = test_env! {
      args: [
        "--quiet",
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "udp:bar.com",
        "--announce-tier",
        "foo",
      ],
      tree: {
        foo: "",
      }
    };
    env.status().ok();
    assert_eq!(env.err(), "");
    assert_eq!(env.out(), "");
  }
  // 代码直接摘自Intermodal的测试用例

```

可以看到这个宏用起来还是挺方便的，具体原理原文以及源代码都有。

作者目前还没有将这个方法提炼成一个库，如果你想用的话可以去作者的命令行工具源代码里面复制这部分的相关代码，然后在你的项目里面使用。作者说以后肯定时要和现在它写的项目分离出来作为一个独立的项目的，但是暂时没有动力。作者鼓励大家去维护这样的一个库。

[Read More](https://www.reddit.com/r/rust/comments/ippyeg/you_wont_believe_this_one_weird_trick_to_speed_up/): https://www.reddit.com/r/rust/comments/ippyeg/you_wont_believe_this_one_weird_trick_to_speed_up/

[`Intermodal`  github仓库](https://github.com/casey/intermodal/): https://github.com/casey/intermodal/

### 用`BPF`拦截Zoom的加密数据

本来是三月份的时候就已经写的这么文章，但由于当时Zoom的一些风波，作者便没有选择发布这篇blog。

现在好像风波过去了，作者整理了这篇blog，选择重新发布出来。

[Read More](https://confused.ai/posts/intercepting-zoom-tls-encryption-bpf-uprobes): https://confused.ai/posts/intercepting-zoom-tls-encryption-bpf-uprobes

### 编译时字符串格式化

需要Rust 1.46

[Crate](https://crates.io/crates/const_format/0.2.5): https://crates.io/crates/const_format/0.2.5

### 使用Knurling-rs学习嵌入式Rust

[Read More](https://ferrous-systems.com/blog/knurling-sessions-introduction/)：https://ferrous-systems.com/blog/knurling-sessions-introduction/

### `Twistrs`  ー Rust 中的域名枚举库

这是作者的第一个Rust库，blog介绍了作者写这个库的一些历程。

[Read More](https://blog.digital-horror.com/twistrs/): https://blog.digital-horror.com/twistrs/

### 用 Rust 创建一门编程语言

系列教程：用 Rust 开发一个名叫 Eldiro 的编程语言

[https://arzg.github.io/lang/](https://arzg.github.io/lang/)

### 是什么让 Actix 比其他框架更快？

reddit 提问，蹲回答  [https://www.reddit.com/r/rust/comments/ip88wb/what_makes_actix_faster_than_other_frameworks/](https://www.reddit.com/r/rust/comments/ip88wb/what_makes_actix_faster_than_other_frameworks/)

## vopono 项目回顾

vopono 是文章作者自己的第一个有用的 Rust 项目. vopono 是一个linux 的 VPN 工具, 可以让不同的程序在临时的 network namespace 上启动, 以便每个程序都使用各自独立的 VPN 连接,而不影响系统上的其他程序.

作者从 项目背景, Rust带来的好处, 困难点, 目前存在的问题 这几方面来回顾这个项目的开发,这一篇干货满满的分享,下面为部分节选:

Rust 带来的益处:

1.  `Enums`
    
    Rust对 enum 的原生支持,让推导和 debug 更加的简单和方便(例如在为OpenVPN协议上选择 TCP 还是 UDP), Rust 编译器会强制我们处理 enum 的所有可能情况来减少 bug.
    
2.  `StructOpt`
    
    StructOpt是一个通过  `derive`宏来处理命令行参数 非常棒的 crate.
    
3.  `Result`和`anyhow`
    
    Rust 的  `Result`  和  `?`  操作符让我们可以非常优雅的处理错误. 结合  `anyhow`  这个 crate 会非常容易的处理错误.
    
4.  `Serde`
    
    `Serde`  可以让你非常方便的序列化和反序列化你的结构体.
    
5.  `Drop`
    
    `Drop`  trait可以允许我们再在 struct 失去作用范围的时候运行desctructor. 当程序退出的时候, vopono用他来自动销毁一些临时的 network namespace.
    
6.  `Cargo`
    
    Cargo 作为 crate 管理器, 本身就是使用 Rust 的一大好处.
    
7.  `include_str`宏
    
    `include_str`宏可以在编译期间将硬盘上的文件作为静态 string 加载编译到 binary 中.
    
8.  `Rustls`
    
    `Rustls`是一个TLS库,可以用来替代 OpenSSL.
    
9.  `musl`和静态链接
    
    `x86_64-unknown-linux-musl`  target可以用来做(交叉)编译,静态的连接到`musl`而不是默认的`glibc`.
    

原文链接:  [http://jamesmcm.github.io/blog/2020/09/05/vopono/#en](http://jamesmcm.github.io/blog/2020/09/05/vopono/#en)


### Rust 序列化相关库综合分析

#rust #crate

干货文章，作者对现在用于生产环境的Rust序列化相关的库做了benchmark，细致的分析了每个库具体适用的场景，总结来说：

-   如果那你需要快速序列化和反序列化，bincode是你最好的选择
-   如果被序列化的内容很小，可是试试MessagePack，但你可能会在反序列化牺牲一些运行时间
-   在嵌入式场景中Postcard提供了很好的折中方案
-   FlatBuffer很复杂，而且占用的空间比应有的多，除非您以多种语言使用Schema定义，否则没有理由使用它。即使这样做了，JSON也是更好的选择
-   JSON是三种可读格式中最快的，这是有道理的，因为它在业界得到了广泛的使用，并受益于SIMD优化

[ReadMore](https://blog.logrocket.com/rust-serialization-whats-ready-for-production-today/)

### easy_rust 正式完成了

Rust 已经有了很多不错的教科书来帮助大家学习, 但是有时候阅读这些教科书都比较困难, 因为他们大多数是为母语是英语的人准备的. 作者使用更简单的英语来写这本书, 目的是为了让那些母语不是英语的人能够更快的学习 Rust.

现在  `easy_rust`  已经正式完成了,大家可以查看下面链接来观摩学习.

[https://github.com/Dhghomon/easy_rust/blob/master/README.md](https://github.com/Dhghomon/easy_rust/blob/master/README.md)


### 【从零到生产】 3.5 HTML表单处理, 数据库操作 和 集成测试

【从零到生产】是一系列聚焦于使用 Rust 来开发云原生应用的教程. 本篇为第三章后半部分, 讲述如何处理 HTML 的表单, 数据库连接的建立和使用, 以及如何进行集成测试.

感兴趣的同学可以从第一章开始看起.

[Read more](https://www.lpalmieri.com/posts/2020-08-31-zero-to-production-3-5-html-forms-databases-integration-tests/)

### 生产环境选哪个 Rust 序列化库？

序列化一直是 Rust 的强项，序列化库既成熟又快速。

特别是 Serde 在 Rust 1.0.0 发布之前就已经可用，其背后的理念是使用  `trait`  解耦对象，并从序列化格式中进行序列化/反序列化，这是一个非常强大的思想。格式编写者只需实现 Serde 的  `trait`  即可，而用户通过`#[derive(Serialize，Deserialize)]`对其对象进行序列化，而无需考虑格式。

当然有各种特定格式的库，这篇博客文章中，将在考虑 API 可用性和性能的情况下比较一些库。

博客文章[链接](https://blog.logrocket.com/rust-serialization-whats-ready-for-production-today/)，https://blog.logrocket.com/rust-serialization-whats-ready-for-production-today/

### 学一点Rust内存模型会发生什么呢(2)

这是CrLF0710在知乎连载的一系列讨论Rust内存模型的专栏，讲得浅显易懂，推荐一看。

链接：[https://zhuanlan.zhihu.com/p/201220495](https://zhuanlan.zhihu.com/p/201220495)


### Contributing to the Intellij-Rust plugin: writing an intention from scratch

作者在自己博客连载他给Intellij-Rust插件贡献的经历，这是第二篇文章。

链接：[https://kobzol.github.io/rust/intellij/2020/08/25/contributing-2-subst-assoc-type-int.html](https://kobzol.github.io/rust/intellij/2020/08/25/contributing-2-subst-assoc-type-int.html)

### `Constany`：将任何rust函数转换为const函数的crate

库的介绍文档有提到了这个库的一些原理。

地址：https://github.com/moelife-coder/constany

### `Bevy`源码学习笔记-001

最近在看Bevy的源码，有些有趣的地方分享给大家。

看的比较少，所以只找到一些简短的地方。

因为对Bevy的资产处理不是太熟悉，所以优先看了这部分的源码。

```
// Properties这个宏的部分没看，不过当初看Bevy引擎介绍的时候
// 有说到这个是为了在Rust中实现类似其他语言中反射这种功能而特
// 意设计的
#[derive(Properties)]
pub struct Handle<T>
where
    T: 'static,
{
	// pub struct HandleId(pub Uuid);
    pub id: HandleId,
    #[property(ignore)]
    marker: PhantomData<T>,
}

```

这部分源码我觉得是比较有意思的，因为除了Handle结构体之外，在这部分源码中还实现了另一个结构体：

```
#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub struct HandleUntyped {
    pub id: HandleId,
    pub type_id: TypeId,
}

```

大部分对Handle的操作方法都是实现在了Handle里的，那实现HandleUntyped的意义在哪里呢？源码自带的注释很好的解释了这一点： This allows handles to be mingled in a cross asset context. For example, storing  `Handle<A>`  and  `Handle<B>`  in the same  `HashSet<HandleUntyped>`.

目前我也就只看了资产的部分，比较复杂的处理应该是AssetServer这部分，不过基本都是正常的代码，没有上面这种取巧的地方了。

### 推介一个B站Up主

前几天无聊的时候刷到B站一个用Rust刷力扣题目的Up主，视频质量杠杠的。

但是一个视频的播放量低的可怜，有兴趣的观众可以关注一波

地址：https://space.bilibili.com/202107274/

建议从力扣刷题的部分看，我个人认为这是他系列里做的最好的。

# 理解 Rust 的切片

在迁移一些 C/C++ 代码到 Rust 过程中, 你是否也对`切片`的`所有权`如何从  `Rust`  中传递给  `C`  感到困惑 ? 亦或是对`切片`的内存布局感到困惑, 从而担心是否会造成内存泄露 ?

作者开始有同样的不解, 在深入了解以后,写下该篇文章帮助有同样疑惑的人.

[Read more](https://codecrash.me/understanding-rust-slices)

# Julia 的 Rust 绑定库

`jlrs`  的目标是对  `Julia C API`  提供一个简单安全的接口. 目前该 crate 只在  `Linux`和`Windows`上测试通过, 且只支持  `Julia 1.5.0`, 不兼容之前版本的  `Julia`.

有兴趣同学可以访问其  [Github](https://github.com/Taaitaaiger/jlrs)

### Easy_Rust 文档进入进入审校阶段

Rust的官方文档对于非英语母语的开发者来说理解起来有点难，因此作者用更加通俗易懂的语言解释了Rust官方文档，目前已经完成内容进入审校阶段。

[Read More](https://github.com/Dhghomon/easy_rust/blob/master/README.md)

### Rust 内存容器备忘录

#rust #memory

![](https://github.com/usagi/rust-memory-container-cs/raw/master/3840x2160/rust-memory-container-cs-3840x2160-dark-back.png)

[Read More](https://github.com/usagi/rust-memory-container-cs)

### 使用rust在内核print

print是很重要的。如果某些东西不工作，您想知道为什么(例如，通过查看控制台输出)。当我第一次为内核驱动程序编写日志宏时，我没有考虑太多安全性问题。我只是想:“肯定没有人会用错误的格式说明符或错误的参数数量调用它，因为它的用法简单而直接”。

[文章链接](https://not-matthias.github.io/kernel-printing-with-rust/)，https://not-matthias.github.io/kernel-printing-with-rust/


### 视频系列: 使用Rust和Rocket构建一个智能书签工具

我在6月份写的文章2的基础上创建了一个视频系列2，在文章2中，我们使用Rust和Rocket创建了一个智能书签工具。它是初学者友好的，并且在结尾你将有一个生产rust应用部署到Heroku!

[文章链接](https://users.rust-lang.org/t/video-series-build-a-smart-bookmarking-tool-with-rust-and-rocket/47601)，https://users.rust-lang.org/t/video-series-build-a-smart-bookmarking-tool-with-rust-and-rocket/47601


### Async 项目比较

### 【博客】并发迭代语义

这篇博客中介绍了作者遇到过关于并行异步迭代的挑战。

[https://blog.yoshuawuyts.com/async-iteration/](https://blog.yoshuawuyts.com/async-iteration/)

## 小编私货

昨天的日报我们报道了 Bevy 发布 v0.2 的消息，小编关注里面的异步任务系统提高，今天在搜索的时候发现相关特性也是出自社区的异步大佬 stjepang 之手（发现了async-executor 需要紧急升级到 v1.3.0版本），详情可以看看 PR： https://github.com/bevyengine/bevy/pull/526

### Rust中的多线程池

作者用`Rayon`库写了一篇在rust中线程池的一篇教程。

Read More：https://pkolaczk.github.io/multiple-threadpools-rust/


Rust 的将异步代码引入语言的方法比较新颖，它没有将异步系统与语言打包在一起，例如 Golang 提供的内置 goroutine 的方法，而是提供了一个接口，供独立的库开发人员使用以实现异步给定进程的运行时。所以博文对 Futures、Tokio、async-std、smol、Actix、embrio、Bastion 这些 async 生态的库做了个总结，[更多请看原文](https://runrust.miraheze.org/wiki/Async_crate_comparison#Comparison_of_Async_Ecosystems)：https://runrust.miraheze.org/wiki/Async_crate_comparison#Comparison_of_Async_Ecosystems

### Rust exa源码阅读分析系列 文章

一共九篇，作者分享了他阅读 exa 源码的一些心得，值得一看。

> 作为Rust新手，本着学习的态度，对Rust 实现的命令行工具exa源码进行阅读分析。由于Rust经验不足，分析是基于Rust基础的语法功能以及简单用法。一些高级的用法没有涉及到，主要还是以学习为目的，了解一个Rust项目的构建流程以及模块划分。

> 第一篇主要是介绍exa的功能以及体验效果，中间是对源码进行分析，介绍如何根据命令行参数渲染最后的结果。最后一篇，介绍exa的编译脚本以及cargo.toml文件，学习一个项目是如何进行编译配置。

详情请见:[https://zhuanlan.zhihu.com/c_1279213952211898368](https://zhuanlan.zhihu.com/c_1279213952211898368)


### 如何为 Rust 项目创建 Linux 安装包(2/3) - 使用 Github Actions 和 Docker 进行构建

这篇文章描述了如何为 Linux 流行发行版创建软件包!

1.  Github Actions
2.  计划:Docker镜像 -> Actions -> 工作流程
3.  Docker 镜像
4.  Github Actions 的装箱魔术
5.  构建镜像
6.  建立和托管
7.  创建Github Actions
8.  最终的 Github Action 工作流程
9.  最后:我们所有的公共资源

详情请见:[https://ebbflow.io/blog/vending-linux-2](https://ebbflow.io/blog/vending-linux-2)

### 用于Clang/LLVM 和 Rust的控制流防护机制(CFG)

作为我们对更安全的系统编程的持续努力的一部分,我们高兴地宣布 Clang C/C++ 编译器和 Rust 现在已经支持 Windows Control Flow Guard。

-   什么是安全防护流机制?
-   安全防护流在 LLVM 和 Clang 中
-   安全防护流在 Rust 中
-   怎样在 Rust 中启用 CFG
-   CFG 的开销

详情请见:[https://msrc-blog.microsoft.com/2020/08/17/control-flow-guard-for-clang-llvm-and-rust/](https://msrc-blog.microsoft.com/2020/08/17/control-flow-guard-for-clang-llvm-and-rust/)

### kosmonaut 从头开发的浏览器引擎

kosmonaut 是一个从头开发的浏览器引擎,Rust编写.目前来说能做的事情还比较基础.仅一部分 CSS 被支持,大部分的网页应该是不能正常渲染的.

https://github.com/twilco/kosmonaut

### 【经验分享】Rust's Unsafe 是如何起作用的？

作者在文中概述了Rust的`unsafe`关键字为何起作用，而C / C ++中的类似措施却无效的缘由。

[Read More](https://jam1.re/blog/why-rusts-unsafe-works)

### 【经验分享】如何在VS Code中更改`rust-analyzer`的语法高亮显示？

将以下内容添加到[settings.json](https://code.visualstudio.com/docs/getstarted/settings#_settings-file-locations)文件中（假设使用的是默认主题）

```
{
    "editor.semanticTokenColorCustomizations": {
        "[Default Dark+]": {
            "enabled": true,
            "rules": {
                "parameter.mutable": {
                    "bold": true,
                    "underline": false
                },
                "selfKeyword.mutable": {
                    "bold": true,
                    "underline": false
                },
                "variable.mutable": {
                    "bold": true,
                    "underline": false
                }
            }
        }
    }
}

```

[Read More](https://www.reddit.com/r/rust/comments/iak2hk/how_can_i_change_syntax_highlighting_of_rust/)

### 现在 Rust Nightly 标准库文档已涵盖了每个关键字

截止到今天，Rust 中每个关键字的初始文档已经完成并且内容充实，为了完整性，这些文档还包括 unsafe 以及原始类型的关键字。详细可查看这三个链接：

-   [标准库](https://doc.rust-lang.org/stable/std/#keywords)：https://doc.rust-lang.org/stable/std/#keywords
-   [unsafe 关键字](https://github.com/rust-lang/rust/pull/73943)：https://github.com/rust-lang/rust/pull/73943
-   [原始类型关键字](https://github.com/rust-lang/rust/issues/34601)：https://github.com/rust-lang/rust/issues/34601

### 学习Rust，沮丧？

还记得之前关于Rust中字符串的长文吗？这是 Amos 最新的又一篇长文，内容包括Rust的特性，生命周期等。

更多参见[文章](https://fasterthanli.me/articles/frustrated-its-not-you-its-rust)，https://fasterthanli.me/articles/frustrated-its-not-you-its-rust

### docs.rs 已经更新了文档覆盖功能

docs.rs更新了文档覆盖功能

在这个位置可以看到：

![](https://github.com/YRainbbbb/pictures/blob/master/8.13/0.jpg?raw=true)

### 关于构建者模式的一个简短介绍

一个关于构建者模式的简短介绍。

以及提供了三个实际项目中的用例。

[Read More](https://matklad.github.io/2020/08/12/who-builds-the-builder.html)：https://matklad.github.io/2020/08/12/who-builds-the-builder.html


### Sana: 更简单的词法生成器

Sana是Rust的词法生成器，提供了一种简单的方法来为语言创建词法分析器。

[https://github.com/suhr/sana](https://github.com/suhr/sana)

### 初创公司Meili投入150万欧元，探究Rust语言编写的开源搜索引擎

Meili SAS是2018年11月成立的初创公司，希望能开发一套搜索引擎接口，替代当前的托管式云搜索引擎竞争品。本次他们从LocalGlobe、Seedcamp、Kima & Tiny.vc几个投资机构融资150万欧元，希望能建设开源搜索引擎MeiliSearch的社区，以吸纳更多的用户和贡献者。MeiliSearch是一款高效开源的全文搜索引擎，值得一提的是，它也使用社区的汉字分词库“结巴”，对汉字词语的搜索支持有特殊的设计。

与竞品Algolia等等不同的是，MeiliSearch是开源项目。为了举例子，这个引擎在官网给出了一个快速的crates.io搜索引擎，它的搜索和呈现过程加起来仅需数毫秒，几乎可以做到输入字母的同时就已经搜索完毕。这个项目最初是路易威登公司在巴黎赞助的黑客松活动诞生的。没错，就是卖时尚手包的那家路易威登公司——那次活动结束后，项目的三位作者分别获得了一个路易威登的包包作为奖品。

MeiliSearch社区已经公开了后续开发的路线图。他们希望扩展更多的功能，而且能在任何地方快速搭建开发环境。这个社区也非常欢迎开发者的贡献，重视开源社区的价值。

[https://blog.meilisearch.com/meili-fundraise/](https://blog.meilisearch.com/meili-fundraise/)


### 在 Rust 中存储连续数据?

作者都帮你整理好了:

1.  使用 Rust 中的数组  `[T; N]`.
2.  Slice  `&[T]`  or  `&mut [T]`, 可以方便的 split.
3.  Boxed slice  `Box<[T]>`.
4.  `Vec`. 长度和内容都可以变化,这可能是我们最常用的方式.
5.  `smallvec`, 第三方 crate, 当元素较少时可以存储在 stack 上.
6.  `arrayvec`, 第三方 crate, 如名字所述, 底层是用 Array 来存储的,因此长度不能动态增长. 但是数据可以存储在 data segment, stack 或 heap上.
7.  `tinyvec`, 第三方 crete, 可以100%替代  `smallvec`和`arrayvec`的一个crate.
8.  `VecDeque`, 标准库中的  `std::collections::VecDeque`, 是一个可增长的ring buffer实现的双端队列.可以高效的  `pop`和`push`.
9.  `bytes`, 第三方 crate,提供了  `Bytes`,可以非常方便高效的存储和操作连续的内存.不过他只能存储  `u8`,常用于网络.

更多详细内容  [Read more](https://github.com/paulkernfeld/contiguous-data-in-rust)

### Rust 实现 Bloom Filter

图文并茂的讲述 并且 使用 Rust 来实现一个 Bloom Filter.

[Read more](https://onatm.dev/2020/08/10/let-s-implement-a-bloom-filter/)

### 从零到生产: 使用 Rust 进行 API开发系列

`从零到生产`  是一个使用 Rust 进行 Web API 开发的系列文章. 目前更新到第三章,使用  `actix-web`  来实现邮件订阅的 API 部分.

[Read more](https://www.lpalmieri.com/posts/2020-08-09-zero-to-production-3-how-to-bootstrap-a-new-rust-web-api-from-scratch/)

### 为什么 Rust 作为第二门语言非常棒

[Read more](https://beyondtheloop.dev/rust-second-language/)

### Rust books

想找本书学习一下 Rust,这里有非常齐全的 Rust books, 从新手到进阶. 不过基本上都是英文哒, 英文好的同学可以参考一下.

[Read more](https://github.com/sger/RustBooks)


### 教程：Rust练习题

#rust #learn

一个学习Rust的项目，通过不同的练习，学习Rust Book中提到的相关概念，从类型基础到泛型，线程都有详细的介绍

[Read More](https://egghead.io/playlists/learning-rust-by-solving-the-rustlings-exercises-a722)

### 视频：使用Rust无惧安全问题

#security

RustLab 2019的分享的topic，众所周知Rust是一门安全的语言，似乎耳朵已经起茧子了，但开发者如何才能享受到Rust的安全呢？Rust在安全方面到了多了哪些事情？来这个视频一探究竟。关注RustLab油管频道，还有其他的精彩topic：

-   Make life easier with Derive and other tools
-   Writing a Nintendo 64 emulator in Rust
-   Safe and Simple Rust Smart Contract in Near
-   ...

[Read More](https://m.youtube.com/watch?index=3&list=PLkgYzo_ZZ7KACuiy-AW_ClsEzGMKdkDu7&v=4Y9aK8bSJrEv)

### 微软是如何使用Rust的？

#rust

微软正在内部推广Rust，包括一些service和原生的windows应用，可能还会有操作系统的一些核心组件，更多关于Rust在微软推广的文章请看。

[Read More](https://medium.com/the-innovation/how-microsoft-is-adopting-rust-e0f8816566ba)

### rust的第一印象

一段时间以来，我一直想用Rust编写一个大项目作为学习练习，实际在2018年末开始了一个项目(FUSE服务器实现)。但后来生活发生了变化，我变得很忙，从来没有带着它去任何地方。

由于某些特定的世界环境，我目前花了很多时间在室内，所以rust-fuse (docs)现在已经存在，足以编写基本的hello-world文件系统。

[文章链接](https://john-millikin.com/first-impressions-of-rust)

# Propane: an experimental generator syntax for Rust

众所周知，Rust的Generator一直没有稳定，主要原因是Generator仍然有许多设计上的问题没有明确，所以无船同志写了一个名字叫**Propane**的新crate，旨在nightly上实验性的探索Rust Generator未来语法的可能性。

> Propane中文翻译：丙烷，无船同志取名向来看起来比较奇怪，不知道是否有其他用意

目前nightly的generator只能写成闭包的形式（官方称作generator literal），比如这样：

```
#![feature(generators, generator_trait)]

fn main() {
    let mut generator = || {
        yield 1;
        return "foo"
    };
}

```

如果你写成这样的话：

```
#![feature(generators, generator_trait)]

fn fake_generator() -> &'static str {
    yield 1;
    return "foo"
}

fn main() {
    let mut generator = fake_generator;
}

```

编译器会报[E0627 A yield expression was used outside of the generator literal.](https://doc.rust-lang.org/nightly/error-index.html#E0627)这个错。也就是目前的generator不支持以函数的方式写。

前面说了，generator很多语法没有稳定甚至不支持，主要原因还是很多设计理念没有明确，所以Propane这个库先迈出了第一步。

```
#![feature(generators, generator_trait, try_trait)]

#[propane::generator]
fn fizz_buzz() -> String {
   for x in 1..101 {
      match (x % 3 == 0, x % 5 == 0) {
          (true, true)  => yield String::from("FizzBuzz"),
          (true, false) => yield String::from("Fizz"),
          (false, true) => yield String::from("Buzz"),
          (..)          => yield x.to_string(),
      }
   }
}

fn main() {
    let mut fizz_buzz = fizz_buzz();
    assert_eq!(&fizz_buzz.next().unwrap()[..], "1");
    assert_eq!(&fizz_buzz.next().unwrap()[..], "2");
    assert_eq!(&fizz_buzz.next().unwrap()[..], "Fizz");
    assert_eq!(&fizz_buzz.next().unwrap()[..], "4");
    assert_eq!(&fizz_buzz.next().unwrap()[..], "Buzz");
    assert_eq!(&fizz_buzz.next().unwrap()[..], "Fizz");
    assert_eq!(&fizz_buzz.next().unwrap()[..], "7");

    // yada yada yada
    let mut fizz_buzz = fizz_buzz.skip(90);

    assert_eq!(&fizz_buzz.next().unwrap()[..], "98");
    assert_eq!(&fizz_buzz.next().unwrap()[..], "Fizz");
    assert_eq!(&fizz_buzz.next().unwrap()[..], "Buzz");
    assert!(fizz_buzz.next().is_none());
}

```

Propane提供了一个`generator`宏，可以让我们以函数的方式写Generator。当然Propane的主要目的是为了验证两个主要的设计理念是否合理：


### 1) 默认返回Iterator；return关键字可以终止generator，但只支持返回`()`; generator中的`?`表达式的默认行为和普通函数有差别

用Propane的`generator`宏标记的函数是一个返回`impl Iterator`的生成器，生成器中依然可以使用`return`关键字来终止，但是不能返回其他类型的值，只支持返回`()`。

生成器中支持`?`表达式，但是与普通函数中不同的是，如果生成器yield是一个`Result`类型，当`?`表达式碰到错误情况时会把错误yield出去，而不是return出去。然后在下一次resume直接退出生成器。

### 2) 不支持自引用（Self-referential）

async/await语法稳定的时候为了解决自引用的问题花了很大的心思设计Pin和Unpin等概念。如果Generator默认返回迭代器Iterator的话，我们依然会碰到了自引用的问题。因为`Iterator::next`在1.0就稳定了，我们不可能再去修改它的API来让迭代器支持自引用。如果不考虑性能，目前最简单粗暴的方法是可以把Generator的每个state装箱到堆上。

当然如果我们不支持自引用，就可以让generator支持零开销（zero cost），而且无船同志也大胆的推测（hypothesis）：也许我们确实不需要一个支持自引用的Generator。

最后，无船同志强调，这几个理念仅仅是实验性的，而且也有可能是一次失败的尝试。

以上是我了解完Propane之后结合无船的博客整理的文章，Propane的代码也很简洁，大家可以去Github查看。

来源我的博客文章：[https://folyd.com/blog/rust-generator-experimental/](https://folyd.com/blog/rust-generator-experimental/)

无船的博客文章：[Propane: an experimental generator syntax for Rust](https://without.boats/blog/propane/)


# Building Canrun: A statically typed logic programming library for Rust (part 1)

作者在用Rust写一个名叫Canrun的逻辑编程(logic programming)库，这是他这是这一系列连载博客的第一篇。

链接:  [https://esimmler.com/building-canrun-part-1/](https://esimmler.com/building-canrun-part-1/)

# Parallel stream processing with Rayon

作者写了一遍博客分享他使用Rayon的并发流式处理体验。

链接：[https://morestina.net/blog/1432/parallel-stream-processing-with-rayon](https://morestina.net/blog/1432/parallel-stream-processing-with-rayon)

### 使用 Rust 和 SDL2 构建一个 iOS 项目

[博客原文](https://blog.aclysma.com/rust-on-ios-with-sdl2/)：https://blog.aclysma.com/rust-on-ios-with-sdl2/

### Crust of Rust 系列视频

在这个（第五个）Crust of Rust 视频中，我们通过重新实现标准库中的某些  `std :: sync :: mpsc`  类型，介绍了多产品/单消费者（mpsc）channels。 作为其中的一部分，我们将介绍 channel 的用途，它们在更高级别上的工作方式，不同的通用 channels 的变体以及通用 channels 实现。 在此过程中，我们介绍了一些常见的 Rust 并发原语，例如 Mutex 和 Condvar。

YouTube地址：https://www.youtube.com/watch?v=b4mS5UPHh20

### 教程：使用 WebAssembly, Rust 和 WASI 编写 Deno 应用

教程中，我们将用 Rust 编写高性能函数，将它们编译成 WebAssembly，并在你的 Deno 应用程序中运行它们。更多详情请看原文：https://www.secondstate.io/articles/deno-webassembly-rust-wasi/

### 如何为 Rust 项目创建 Linux 安装包(1/2)

这篇文章描述了如何为 Linux 流行发行版创建软件包!

-   1.后台守护程序
    
-   2.无依赖
    
-   3.用发行版
    
-   4.自动化构建 (下篇文章讲)
    

详情请见:[https://ebbflow.io/blog/vending-linux-1](https://ebbflow.io/blog/vending-linux-1)

## 预防优于治疗 — 来自张汉东老师对于内存泄漏问题的观点

Rust 与 内存泄漏

学习 Rust 语言的一个很大的收获就是： 「预防大于治疗」。

内存泄漏并不是 Rust 语言要解决的问题，但是 Rust 语言也不大容易发生内存泄漏。

写 Rust 代码如何避免内存泄漏呢？ 两点：

1.  做好预防
2.  及早发现

如何做好预防：

1.  你可以增加一些内存泄露基准测试代码。在高并发访问下前后内存变化是否超过了一个额定值。
2.  测试服上降低 OOM 限额。让代码尽早出问题。
3.  在写代码的时候对于有可能出现内存泄露的情况进行样板化检查，比如循环引用、forgot调用、FFi边界之类。

及早发现：

4.  上线的项目里加上性能监控，及早发现泄露问题。

一旦如果发生了泄露，那就只能借助日常性能检测工具发现问题了，但是在这之前你可以先检查下刚提到的样板化检查区域的代码，再用一些性能测试工具逐步收集信息，缩小范围，逐渐排查。

5.  检测工具推荐：Valgrind、gperftools、ltrace、rr、gdb
    
6.  回放线上流量，对流量做二分找到有内存泄漏的部分请求，加压重放加速问题显露，然后二分注释代码找最小的泄露区域人肉排查。（此条经验 via DCjanus）
    

## Go VS Rust — 一个cli 写两遍

作者写了一个 CLI, 用 go 和 rust 各自写了一遍, 还附带总结.：https://cuchi.me/posts/go-vs-rust


## 【小编私货】Rust异步书翻译更新

小编周末的时候重梳理了一下去年对异步书的翻译，病重新更新翻译到最新的版本，请各位读者大人多多指教，欢迎 issue 和 PR 翻译文档链接： https://huangjj27.github.io/async-book/index.html 仓库链接：https://github.com/huangjj27/async-book

# Rust新手的错误处理指南

这是一篇对 Rust 新手非常友好的 error handling 指南. 从基本的小例子开始,循序渐进,由浅入深地引导出 Rust 中是如何处理错误的.

[新手的错误处理指南](http://www.sheshbabu.com/posts/rust-error-handling/)

如果读完还有疑问, 还可以结合另外一篇  [Rust 2020 错误处理](https://nick.groenen.me/posts/rust-error-handling/).


# What is "Memory Safety", really?

Rust 的一项核心特性就是 内存安全. 这篇文章清晰的告诉你 Rust 在内存安全方面能够做什么, 也同时提醒我们哪些地方是 Rust 做不到的而需要我们自己来注意的.

> reddit 观众高度评价: 最近读到的最好的分析之一.

[Read more](https://tiemoko.com/blog/blue-team-rust/)

### 通过构建RISC-V驱动的机器人来学习嵌入式Rust

在带有SiFive FE310 RISC-V微控制器的[HiFive1板](https://www.sifive.com/boards/hifive1)上学习嵌入式Rust

![](https://k155la3.blog/media/2020/allbot_rust_part1/blinking-leds.gif)

RISC-V Rust快速启动项目：

[Repo](https://github.com/riscv-rust/riscv-rust-quickstart)

[Read More](https://k155la3.blog/posts/)

### Rusty Days 2020 演讲录播

了解更多有关  _Webference Rusty Days 2020_  的信息

> Is it safe to use unsafe?
> 
> Is Rust ready for mainstream usage in backend development?
> 
> Leveraging Rust to build cross-platform mobile libraries
> 
> etc, YouTube

[Read More](https://www.youtube.com/playlist?list=PLf3u8NhoEikhTC5radGrmmqdkOK-xMDoZ)

### Rust+VS Code+WSL2+LLDB 安装/配置/调试指南

一篇来自【思否社区 SF】的帖子，适合初学者学习。

[Read More](https://segmentfault.com/a/1190000023464786)

## September --202009

### 《Rust中应该尝试的12个杀手级库（下）》

写了一个bug公众号添加了此篇博文。文章中，作者介绍了Rust开发中常常见到的解决痛点的库。这些库包括重量级的网页服务器引擎Hyper、应用开发框架Actix，包括Python语言运行时PyO3，也包括简单的动态链接库加载器libloading等等。作者通过代码例子，快速地介绍了它们的使用方法和特点。

[微信公众号“写了一个bug”](https://mp.weixin.qq.com/s/StDEAc-qXSfz8hu2yoD0nA)

### awak：简单的Rust异步运行时

是由开发者御坂知惠（@cssivision）编写的练习项目。代码风格非常清爽，非常简单。阅读项目的代码，能初步理解Rust语言异步编程的方法。

[GitHub主页](https://github.com/cssivision/awak/)

### 你不会相信这个奇怪的技巧可以加速和提高你的测试

标题是直译原文的，虽然看上去有种标题党的嫌疑，但是就如原文所说的，这种方法在之前已经有不少人用过了，但是从来没有谁把它放到台面上分享给大家，这是一种实用的方法。

作者在github上维护一个[命令行工具  `Intermodal`](https://github.com/casey/intermodal/)，需要大量测试用户输入与程序输出是否符合预期，因此用到了这个怪异的方法。

使用起来大概时这样的：

```
  #[test]
  fn quiet() {
    let mut env = test_env! {
      args: [
        "--quiet",
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "udp:bar.com",
        "--announce-tier",
        "foo",
      ],
      tree: {
        foo: "",
      }
    };
    env.status().ok();
    assert_eq!(env.err(), "");
    assert_eq!(env.out(), "");
  }
  // 代码直接摘自Intermodal的测试用例

```

可以看到这个宏用起来还是挺方便的，具体原理原文以及源代码都有。

作者目前还没有将这个方法提炼成一个库，如果你想用的话可以去作者的命令行工具源代码里面复制这部分的相关代码，然后在你的项目里面使用。作者说以后肯定时要和现在它写的项目分离出来作为一个独立的项目的，但是暂时没有动力。作者鼓励大家去维护这样的一个库。

[Read More](https://www.reddit.com/r/rust/comments/ippyeg/you_wont_believe_this_one_weird_trick_to_speed_up/): https://www.reddit.com/r/rust/comments/ippyeg/you_wont_believe_this_one_weird_trick_to_speed_up/

[`Intermodal`  github仓库](https://github.com/casey/intermodal/): https://github.com/casey/intermodal/

### 用`BPF`拦截Zoom的加密数据

本来是三月份的时候就已经写的这么文章，但由于当时Zoom的一些风波，作者便没有选择发布这篇blog。

现在好像风波过去了，作者整理了这篇blog，选择重新发布出来。

[Read More](https://confused.ai/posts/intercepting-zoom-tls-encryption-bpf-uprobes): https://confused.ai/posts/intercepting-zoom-tls-encryption-bpf-uprobes


### 编译时字符串格式化

需要Rust 1.46

[Crate](https://crates.io/crates/const_format/0.2.5): https://crates.io/crates/const_format/0.2.5

### 使用Knurling-rs学习嵌入式Rust

[Read More](https://ferrous-systems.com/blog/knurling-sessions-introduction/)：https://ferrous-systems.com/blog/knurling-sessions-introduction/

### `Twistrs`  ー Rust 中的域名枚举库

这是作者的第一个Rust库，blog介绍了作者写这个库的一些历程。

[Read More](https://blog.digital-horror.com/twistrs/): https://blog.digital-horror.com/twistrs/

### 作为一个 Rust 业余爱好者，我不想考虑太多错误处理

作者给 2021 roadmap 提的建议

[https://mbuffett.com/posts/rust-less-error-handling/](https://mbuffett.com/posts/rust-less-error-handling/)

### inline_tweak: 在项目运行中改变源代码中的数字或布尔值

例如下列代码可以在代码运行时，改变打印的值

```
use inline_tweak::*;

fn main() {
    loop {
        println!("{}", tweak!(3.14)); // Try changing the value while the application is running
    }
}

```

[https://crates.io/crates/inline_tweak](https://crates.io/crates/inline_tweak)

### 引入“auditable”：审计Rust二进制文件中的已知错误或生产中的漏洞

由于具有内存安全保证，Rust 在对安全性要求较高的应用程序开发中比较有前景。但是，尽管Rust crates 中的漏洞很少见，但它们仍然存在，Rust当前缺少处理它们的工具。

例如，如果您正在运行易受攻击的版本，则Linux发行版会警告您，甚至可以选择自动安全更新。Cargo不仅没有安全更新基础结构，甚至都不知道编译某个二进制文件时要使用哪个库或库版本，因此无法检查您的系统是否易受攻击。

[rust-audit](https://github.com/Shnatsel/rust-audit)  这个仓库已经在着手解决这个问题

[https://github.com/Shnatsel/rust-audit](https://github.com/Shnatsel/rust-audit)

### vopono 项目回顾

vopono 是文章作者自己的第一个有用的 Rust 项目. vopono 是一个linux 的 VPN 工具, 可以让不同的程序在临时的 network namespace 上启动, 以便每个程序都使用各自独立的 VPN 连接,而不影响系统上的其他程序.

作者从 项目背景, Rust带来的好处, 困难点, 目前存在的问题 这几方面来回顾这个项目的开发,这一篇干货满满的分享,下面为部分节选:

Rust 带来的益处:

1.  `Enums`
    
    Rust对 enum 的原生支持,让推导和 debug 更加的简单和方便(例如在为OpenVPN协议上选择 TCP 还是 UDP), Rust 编译器会强制我们处理 enum 的所有可能情况来减少 bug.
    
2.  `StructOpt`
    
    StructOpt是一个通过  `derive`宏来处理命令行参数 非常棒的 crate.
    
3.  `Result`和`anyhow`
    
    Rust 的  `Result`  和  `?`  操作符让我们可以非常优雅的处理错误. 结合  `anyhow`  这个 crate 会非常容易的处理错误.
    
4.  `Serde`
    
    `Serde`  可以让你非常方便的序列化和反序列化你的结构体.
    
5.  `Drop`
    
    `Drop`  trait可以允许我们再在 struct 失去作用范围的时候运行desctructor. 当程序退出的时候, vopono用他来自动销毁一些临时的 network namespace.
    
6.  `Cargo`
    
    Cargo 作为 crate 管理器, 本身就是使用 Rust 的一大好处.
    
7.  `include_str`宏
    
    `include_str`宏可以在编译期间将硬盘上的文件作为静态 string 加载编译到 binary 中.
    
8.  `Rustls`
    
    `Rustls`是一个TLS库,可以用来替代 OpenSSL.
    
9.  `musl`和静态链接
    
    `x86_64-unknown-linux-musl`  target可以用来做(交叉)编译,静态的连接到`musl`而不是默认的`glibc`.
    

原文链接:  [http://jamesmcm.github.io/blog/2020/09/05/vopono/#en](http://jamesmcm.github.io/blog/2020/09/05/vopono/#en)

### serde_query

serde_query是一个  `Serde`  数据模型下的一个 query language.

```
use serde_query::{DeserializeQuery, Query};

#[derive(DeserializeQuery)]
struct Data {
    #[query(".commit.authors.[0]")] // query 部分数据
    first_author: String,
    #[query(".hash")]
    hash_value: u64,
}

let document = serde_json::to_string(&serde_json::json!({
    "commit": {
        "authors": ["Kou", "Kasumi", "Masaru"],
        "date": "2020-09-10",
    },
    "hash": 0xabcd,
}))?;

let data: Data = serde_json::from_str::<Query<Data>>(&document)?.into();

assert_eq!(data.first_author, "Kou");
assert_eq!(data.hash_value, 0xabcd);

```

[https://github.com/pandaman64/serde-query/](https://github.com/pandaman64/serde-query/)

### Evil_DLL 用来测试注入方法的DLL

#dll

用Rust实现用来测试注入方法的DLL。下面时往 "c:\pwned" 写入一个名为 pwned_{pid}.txt 的demo。

-   {pid} = pid of process injected.

文件内容如下：

```
[*]          Pid: "7204"
[*]      Process: "C:\\Windows\\system32\\regsvr32.exe"
[*]         Args: [".\\evil_dll.dll"]
[*]         User: "user"
[*]       Domain: "DOMAIN"
[*] Created file: "c:\\pwned\\pwned_7204.txt"

```

要编译所有依赖到DLL，需要在项目根目录创建`.cargo/config.toml`文件，并写入以下内容：

```
[target.x86_64-pc-windows-msvc]
rustflags = ["-Ctarget-feature=+crt-static"]

```

[Repo](https://github.com/theflakes/Evil_DLL)

### 基于原生AST的linting的吐槽

第一篇博客！在这篇博客文章中，我想谈谈关于linters的各种问题，它只使用ast(抽象语法树)来linting源代码，以及一种您可能不知道的树表示。在rust的一个名为rslint的JavaScript linter上工作了好几个月，我遇到了很多问题，也探索了很多概念，我觉得我应该分享这些概念来提供一些见解，来解释为什么我认为纯粹基于AST的linting不适用于复杂的linters。

[文章链接](https://rdambrosio016.github.io/rust/2020/09/18/pure-ast-based-linting-sucks.html)，https://rdambrosio016.github.io/rust/2020/09/18/pure-ast-based-linting-sucks.html

### 为Rust构建体积小的镜像

#rust #docker

-   分层构建，避免重复构建依赖项
-   为linux:alpine构建，减少镜像体积
-   给予最小运行权限

[Read More](https://shaneutt.com/blog/rust-fast-small-docker-image-builds/)

### Rust初学者经验分享

#rust

[Jean Manguy](https://jean.manguy.eu/)是一位博士后，目前就职于基因组学和宏基因组学的项目，平时的工作都围绕使用Nextflow为现有的命令行工具使用和编写管道而展开，工作中使用最多的是R语言，R语言是一门用于数据探索，统计和数据可视化的解释语言，随着工作变成常态，他发现自己的应用开发能力已经逐渐退步，虽然之前也用C语言做过一些项目，现在他想尝试一门新的语言，于是在疫情期间开始学习Rust。

作者在文中介绍了他学习Rust的目的——开发一款游戏，介绍了他对开发环境、学习资源的整理，以及他做过的一些几个项目，详细介绍了他初学习Rust的历程。

[ReadMore](https://jean.manguy.eu/post/i-started-to-learn-rust/#some-cons-of-rust)

### ouroboros 简单易用的 自我引用struct 生成器

ouroboros 可以让你非常简单地就能创建复杂的自我引用 struct. 简单用法参考下面例子.

```
use ouroboros::self_referencing;

#[self_referencing]
struct MyStruct {
    int_data: Box<i32>,
    float_data: Box<f32>,
    #[borrows(int_data)]
    int_reference: &'this i32,
    #[borrows(mut float_data)]
    float_reference: &'this mut f32,
}

fn main() {
    let mut my_value = MyStructBuilder {
        int_data: Box::new(42),
        float_data: Box::new(3.14),
        int_reference_builder: |int_data: &i32| int_data,
        float_reference_builder: |float_data: &mut f32| float_data,
    }.build();

    // Prints 42
    println!("{:?}", my_value.with_int_data_contents(|int_data| *int_data));
    // Prints 3.14
    println!("{:?}", my_value.with_float_reference(|float_reference| **float_reference));
    // Sets the value of float_data to 84.0
    my_value.with_mut(|fields| {
        **fields.float_reference = (**fields.int_reference as f32) * 2.0;
    });

    // We can hold on to this reference...
    let int_ref = my_value.with_int_reference(|int_ref| *int_ref);
    println!("{:?}", *int_ref);
    // As long as the struct is still alive.
    drop(my_value);
    // This will cause an error!
    // println!("{:?}", *int_ref);
}

```

[Github 地址](https://github.com/joshua-maros/ouroboros)

### 用Rust学习网络编程TCP/IP基本原理

Low-Level Academy是一个致力于使底层编程更容易的网站。目前只有第一章节的内容。

[Read More](https://lowlvl.org/): https://lowlvl.org/

### juner_os

Rust 开发的OS 基于blog_os 尝试一个运行在内核态的repl环境 lisp 基于mal实现进行开发。

项目是国人开发的，对这个项目感兴趣的小伙伴可以一起去贡献。

[Github](https://github.com/zzhgithub/juner_os): https://github.com/zzhgithub/juner_os

###  Building even faster interpreters in Rust

作者在Cloudflare实习，其中一部分工作是用Rust给防火墙规则（Firewall Rules ）编写匹配引擎，这篇文章里面作者分享了他在这一方面的优化经验。

链接：[https://blog.cloudflare.com/how-we-made-firewall-rules/](https://blog.cloudflare.com/how-we-made-firewall-rules/)

###  robotstxt -- Google的robotstxt Rust版

Robotstxt 是小编自己把Google的robotstxt解析库port到rust了，并且用FFI的方式跑通了C++的测试用例。

链接：[https://github.com/Folyd/robotstxt](https://github.com/Folyd/robotstxt)

### 构建一个Rust运行时反射系统（第一篇）

作者的公司在做一款叫oso的产品，旨在通过提供一门叫Polar的申明式语言，方便用户编写策略文件和用户自身应用逻辑解耦来解决authorization的问题。官方提供了不同语言的库来解析这些策略，作者打算写三篇文章来分享他们在用Rust解析的时候涉及到的动态反射的问题，以及他们的解决方案。

为了方便大家了解它们的Polar语言，我在他们官网摘了一小段：

```
allow(actor: String, "GET", _expense: Expense) if
    actor.endswith("@example.com");

```

链接：[https://www.osohq.com/post/rust-reflection-pt-1](https://www.osohq.com/post/rust-reflection-pt-1)

### 用Rust增强您的Electron应用程序

Electron已成为创建跨平台桌面应用程序的主要技术。它用于Atom，VS Code，Spotify，Slack，Discord等知名和流行的平台。

JavaScript是一门"缓慢的语言"，但是在本教程中将演示如何使用Rust来加速Electron应用程序来获得更好的用户体验。Rust是具有高级人体工程学的"低级语言"。它速度超快而且内存占用极低。

详情请见-blog:[https://blog.logrocket.com/supercharge-your-electron-apps-with-rust/](https://blog.logrocket.com/supercharge-your-electron-apps-with-rust/)

### 《ESP32、ESP8266与Rust语言：建造生态圈》

博主斯科特·玛彬发布了这篇博客文章，阐述他和社区成员一起工作，为ESP32、ESP8266芯片的Rust支持做贡献工作的经历。ESP8266芯片，是全球化的芯片生产商乐鑫公司生产的明星产品，它是性价比的无线连接单片机，能提供适用于物联网、安防领域的解决方案。作者从今年一月就启动了对该系列芯片的支持工作，大量近期的技术更新，也让作者有更进一步的进展。

ESP8266拥有较为特殊的Xtensa指令集，这也决定了编译器rustc需要有进一步的工作。作者做了代码的清扫和整理工作，来更快地更新Xtensa指令集所需的编译器模块。目前的Rust编译器rustc和基础设施LLVM的更新有紧密的关系，作者也在推动这个指令集更新到LLVM的主分支里去。作者特别提到，乐鑫公司非常乐于解答开源社区的问题，经常帮助修复软件漏洞和解决问题。作者给出了一个在Linux平台可以使用的rustc编译版，它包括了Xtensa指令集的支持。

和许多公司的芯片相似，ESP系列的芯片有特定的烧录流程。厂家给出的流程是以Python脚本的形式提供的。@icewind1991和@jessebraham两位社区成员帮助了作者，分别整理Python脚本到Rust项目，以及提供cargo适用的接口，这有助于搭建完备的烧录工具链。现在，用户只需要一行命令，就可以向ESP系列的芯片里，烧录编译好的Rust程序。这对敏捷开发是非常有好处的，作者也更新了更多的功能，做了提升烧录速度的诸多优化过程。

Rust嵌入式社区提供了完整的裸机开发架构：这由外设访问库、硬件中间层库和运行时库组成。通常情况下，外设访问库是由SVD文件生成的。乐鑫公司提供了自己的称为IDF的外设索引格式；作者开发了idf2svd项目，将它转换为SVD格式，以便更快适应Rust嵌入式的生态。作者提到，有了SVD文件，用户就可以调试ESP8266上运行的程序了。因为Xtensa指令集特殊的特点，作者专门开发了适用于它的运行时库，和其它库一样，都把功能归纳到标签宏，这样能精简很多开发流程，包括时钟在内的外设就可以使用了。

如果说外设访问库可以机器生成，那么硬件中间层库，便是支持库开发中最需要开发者理解、花功夫的部分。作者在这段时间里，开发了ESP32、ESP8266两种芯片的支持库。社区成员@arjanmels做了一些开拓性的工作，作者随后完成了大量ESP32芯片的开发例子，这些例子都是可以实际运行的。ESP8266的支持库也是由社区成员完成的，和ESP32不同，乐鑫只提供编译好的二进制代码，所以需要由这些二进制代码逆向得到。“虽然这挺有趣的，”作者提到，“但是比从源代码翻阅着写会难很多。”作者完成了大量外设的支持工作，已经开源到esp-rs社区里面了。

为了引导新手快速开发，作者很早就编写了一系列入门文章。随着编译器、支持库的修改和优化，在随后的时间里，作者不厌其烦地修订文章，以便满足最新版本的开发需求。这些修改包括LLVM的条件变多了，从自己编译到能在rustup里安装，也包括能同时在两款芯片上编译运行的程序样例。作者希望未来，Xtensa能作为广泛支持的目标，他的工作能用在未来可能出现的新芯片上。

相比今年初的工作，作者得到了更多的成果。作者在文章最后，感谢了所有的社区志愿者。作者希望，未来WiFi和蓝牙的支持能继续完善，最后能适配到Tock、RTIC等许多社区成熟的裸机运行环境里去。

[博客文章](https://mabez.dev/blog/posts/esp-rust-ecosystem/)

### 《所以……你想要热更新Rust代码？》

博主fasterthanlime发布了这篇文章。他希望解决文件会被修改的前提下，动态重载链接库的问题。链接库不能单独工作，应该和具体的应用程序一起工作，我们需要导入符号，由操作系统寻找函数的调用位置。

文章中，博主详细阐述了动态链接库的创建和应用方法，并用C和Rust两门编程语言具体举例。Rust语言中，要链接静态链接库，需要给编译器提供编译参数。为了链接动态链接库，需要找到操作系统提供的函数，然后调用这些函数完成过程。需要先打开链接库，传入一些操作系统能读取的字符串作为参数。这样能把返回的值转换为函数指针，以便后续的调用过程。所有的都结束后，应当使用特殊的函数关闭这个链接库。

作者给出了Linux系统的例子，很显然我们想把它推广到各大主流的操作系统。社区已经为我们造好了轮子——libloading，适用于常见的操作系统。用这款库之后，我们能轻松运用Rust语言的所有权特性，帮助我们自动打开关闭链接库，还能方便地把函数从库里导出，以便调用。包括返回的值也是Result类型，这将帮助我们处理文件、符号不存在等等问题。

动态链接库可以用C语言写，当然Rust也能写。将Rust函数使用extern关键字导出，就能在其它语言里访问这个函数。要配置为动态链接库，还需要修改Cargo.toml配置文件里的部分内容，包括它的包名称和包类型。这样Rust写的静态库就能被Rust程序访问了。你想的没错——它也能被C语言程序访问，这是Rust语言在兼容性上非常有优势的一点。

在加载、使用链接库之后，还有一个释放过程。我们很容易联想到，释放之后，这个库是否再能被使用一次。作者在这里花费了大量的调试时间，需要注意的是，打开链接库的函数可能被调用多次；这之后的关闭过程，只会降低引用计数。如果要真正关闭这个库，需要引用计数为零，加载时的参数需要满足条件，而且不能被其它链接库同时使用。

作者做的工程设计是，先暂停所有的线程，遍历正在使用这个库的线程，取消它们，就能关闭这个库了。这需要可执行文件的代码挂钩一个特殊的函数，它在线程结束时需要完成一些工作。这之后，就可以再加载一次需要的Rust链接库了。作者通过分析工具，找到了这个过程中的内存泄漏问题，最终就能完成手动的重载流程。

以上的还是手动的过程，要自动重载代码，需要使用特别的notify库。这个库能跟踪系统中依赖库的变化情况，及时给出反应；这样就能实现自动重载代码了。作者把这些功能包装到了专门的插件系统，最终我们就能为代码编写插件，而且在插件变化时，自动重载这个插件。基于这样的成果，作者实现了有趣的例子，能实时更新图形卡渲染程序的一部分，来在屏幕上画出不同的图形。

作者用两周的时间完成了这篇文章，完成在Linux下的整个流程。作者把在Windows和macOS下的软件支持留给读者，这两者和Linux都有一定的区别和独特之处。作者最后感谢了所有支持他的粉丝和捐助者们，没有他们，作者就不能完成这篇文章。

[fasterthanlime的博客](https://fasterthanli.me/articles/so-you-want-to-live-reload-rust)

### 在 Rust 中实时重载动态库

重新加载动态库可以让我们在程序不退出的情况下达到升级的效果.

想在 Rust 中实时重新加载动态库 ? 这篇文章详细到令人发指, 绝对包教包会.

[原文链接](https://fasterthanli.me/articles/so-you-want-to-live-reload-rust)


### 我们可以观测了吗 ? Rust 的遥测介绍

这是  `<<从零到生产>>`  系列的第四章.

首先介绍了程序的观测性的重要性, 然后从  `log`  到  `tracing`等 crate 都有深入的介绍和演示.

[原文链接](https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/)

### go 标准库在 rust 中的对应 crates

如果你是从 golang 转到 rust, 这篇文章会有所帮助.

文章列出了 go 中常用的一些标准库中所对应的 rust 中的 crates.

例如:

-   go 中的 log 对应 rust 中的 log.
-   go 中的 flag 对应 rust 中的 structopt 等.
-   go 中的 erorr 对应 rust 中的 thiserror.
-   go 中的 encoding/json 对应 rust 中的 serde 等.
-   ...等等

[原文链接](https://christine.website/blog/rust-crates-go-stdlib-2020-09-27)

### In Rust We Trust

Igor Aleksanov，做为一名具有强大C/C ++背景的专业 Rust 开发人员，撰文描述了希望Rust在2021年实现哪些功能，大致有如下内容：

-   [范型关联类型 GAT](https://github.com/rust-lang/rust/issues/44265)
-   [proc_macro_diagnostics](https://github.com/rust-lang/rust/issues/54140)
-   [std::str::pattern::Pattern](https://github.com/rust-lang/rust/issues/27721)
-   [内置基准测试](https://doc.rust-lang.org/test/bench/index.html)
-   [trait aliases](https://github.com/rust-lang/rust/issues/41517)

具体可参见[文章](https://popzxc.github.io/rust-2021)，https://popzxc.github.io/rust-2021

### 窥视Rust枚举（enum）的内部

在我的twitch频道最近的Rust问答环节中，有人问了一个看起来很简单的问题:为什么像SmartString或SmolStr这样的小字符串类型和string一样大小，而像SmallVec这样的小vec类型却比vec大?

我知道我刚刚用了形容词simple，但事实是:为了理解这个问题，我们需要一些背景知识。

[文章链接](https://fasterthanli.me/articles/peeking-inside-a-rust-enum)

### Rust 核心团队成员 RalfJung 的PhD 毕业论文

Ralf 说：

「 完成了！ 我的论文终于完成了。 因此，如果您一直想深入了解我对Rust的研究（以及更多），可以阅读我的论文。 它有近300页，应该会让您忙一阵子。 ;）

这也意味着，经过六年多的学习，我成为博士生的时间已经结束了。实际上，我当了十多年的学生了，是时候结束了。 真是奇怪的感觉。

接下来肯定会继续做很多 Rusty 的事情，从事更多技术性的工作，研究迄今为止还没有研究过的东西。 」

论文简介：

论文提出了两个项目，这些项目为Rust的正式基础奠定了基础，使我们能够更好地理解和发展这一重要语言：RustBelt和Stacked Borrows。

RustBelt是Rust类型系统的形式化模型，并具有健全的内存和线程安全性证明。 该模型旨在验证Rust标准库中许多复杂API的安全性，尽管这些API的实现使用了Unsafe的语言功能。

Stacked Borrows 是对Rust规范的建议扩展，它使编译器可以使用Rust类型的强别名信息来更好地分析和优化其正在编译的代码。 不仅可以正式评估该规范的适当性，还可以在实现了Stacked Borrows语义的Rust的Miri解释器的实际版本中运行真实的Rust代码。

RustBelt建立在Iris（语言无关的框架）之上，该框架在Coq proof assistant中实现，用于构建更高阶的并发分离（ higher-order concurrent separation）逻辑。 本文首先介绍了Iris，并解释了Iris 如何从一些简单的成分中衍生出复杂的高级推理原理。 在RustBelt中，该技术被关键地用来引入生命周期逻辑，该逻辑提供了新颖的借用分离逻辑说明，这是Rust类型系统的关键突出特性。

链接：[https://www.ralfj.de/blog/2020/09/03/phd.html](https://www.ralfj.de/blog/2020/09/03/phd.html)

论文下载地址：  [https://people.mpi-sws.org/~jung/thesis.html](https://people.mpi-sws.org/~jung/thesis.html)

### 关于 1.46.0 stable 的 "const fn"

关于`const fn`已经有很多讨论了,这篇文章只是作者个人对问题的所答

那为什么`const fn`有用呢？一个简单的例子

```
    // a normal function
    fn foo(n: usize) -> usize {
        n + 1
    }

    fn main() {
        const BAR: usize = foo(5);
        let array = [0_u8; foo(7)];
    }

```

```
   // a constant function
    const fn foo(n: usize) -> usize {
        n + 1
    }


```

更多关于`const fn`的讨论:[https://old.reddit.com/r/rust/comments/ihnnnz/announcing_rust_1460_rust_blog/](https://old.reddit.com/r/rust/comments/ihnnnz/announcing_rust_1460_rust_blog/)

详情请见:[https://www.reddit.com/r/rust/comments/iksmgk/psa_what_is_const_fn/](https://www.reddit.com/r/rust/comments/iksmgk/psa_what_is_const_fn/)


### 试试用常量函数画曼德布洛特图

一位Reddit贴主使用全新的常量函数，回顾了作者两年前编写曼德布洛特图小项目。如今的Rust常量函数已经支持while、loop等语句，可以编写较为复杂的函数，并直接将输出结果字符串保存到以const语句定义的全局常量中。相比作者两年前使用类型系统较麻烦的方法，新的方法快捷、便利，思考量更少，开发效率更高。评论认为，相比竞品C++目前的灵活性，Rust语言拥有的编译期保障更强。

代码中展示了在常量函数里定义可变变量、使用while循环语句、访问数组下标和调用其它常量函数的方法。需要注意的是，作者使用的全局常量是`[u8; SSIZE]`类型，其中类型里的常量类型`SSIZE`也是通过常量运算得到的。我们可以直接把这个类型转换为`&str`类型；如果我们能确定它是合法的UTF-8串，那么这个转换将是一个零开销函数。

[贴文链接](https://www.reddit.com/r/rust/comments/ijpxz2/const_fn_makes_it_too_easy_to_do_mandelbrots/)

## October --202010


### Rust可测性设计：一个调查

在设计Rust代码时，我们能做什么来让它更容易测试？这是我所能找到的关于Rust测试的所有调查，特别关注为可测试性和正确性而设计。有些文章展示了在一个工作示例中需要做的多种事情，有些文章更侧重于一个特定的技巧。

[文章链接](https://alastairreid.github.io/rust-testability/)，https://alastairreid.github.io/rust-testability/

### For Complex Applications, Rust is as Productive as Kotlin

作者是Intellij Rust（使用Kotlin开发）和rust-analyzer（使用rust开发）两个项目的核心开发者之一，作者学习曲线、构建工具、生态系统、并发性、性能和安全性等方面对比Rust和Kotlin，认为Rust在复杂的应用开发中，开发效率丝毫不输Kotlin。

链接：[https://ferrous-systems.com/blog/rust-as-productive-as-kotlin/](https://ferrous-systems.com/blog/rust-as-productive-as-kotlin/)

### move/copy/drop 语义和 zeroing data 的一个小陷阱.

Rust 的 value 被  `drop`  之后,默认并不会变成零值的. 但是,在有一些高度安全的设备和行业中, 我们是不希望把一些敏感的信息在内存中停留时间超过他本该停留的时间, 由于上面特性, 会导致即使变量  `drop`  了, 数据仍然是在内存中的, 直到该内存重新被覆盖.

crate  `zeroize`  可以帮助我们来做到安全的设置数据为零值, 而且可以防止编译优化.

本文主要介绍了  `stack`  变量和  `heap`  变量在使用  `zeorize`  的时候一个小陷阱.

[原文链接](https://benma.github.io/2020/10/16/rust-zeroize-move.html)

### Ungrammar : 描述 concrete syntax tree 的一种新形式

Ungrammer 例子如下

```
Module =
  Attr* Visibility?
  'mod' Name
  (ItemList | ';')

```

他和 EBNF 很像, 区别在于, Ungrammer 描述的是 concrete syntax tree, 是一系列数据或者一系列 trees.

[原文链接](https://rust-analyzer.github.io/blog/2020/10/24/introducing-ungrammar.html)

### 与Rust类型系统做斗争：一个OOP程序员学习FP

作者是一个很有经验的程序员，初次接触rust时，发现自己之前的经验并不足以很快适应rust，因此将这个过程记录了下来，并且分享了一些自己的感悟。

[Read More](https://thefuntastic.com/blog/fighting-rusts-type-system): https://thefuntastic.com/blog/fighting-rusts-type-system

### 给nightly通道用户的小提示：在更新rust之后使用`cargo-sweep`来帮助你清理垃圾

对于nightly通道的用户来说，通常在使用过程中会伴随着频繁的升级你的rust版本，而对于日常维护的项目，如果你升级了rust版本之后，`target`编译文件夹里面会生成多个版本的编译文件。这个时候就是使用`cargo sweep`的时候了，它会帮你清理掉除了当前版本以外的`target`目录下多余的文件。

用例：  `cargo sweep -i -r -v ~/src`

-   `-i`  是开启保留`~/src`目录下`target`文件夹内当前电脑上已安装rust版本的编译文件。
-   `-r`  是开启递归（recursively）搜索
-   `-v`  是开启详细（啰嗦模式，开启之后会告诉你它干了啥。）

如果你的电脑上没有cargo sweep，可以用以下命令安装：  `cargo install cargo-sweep`

[Read More](https://www.reddit.com/r/rust/comments/jfdiao/tip_for_nightly_users_use_cargosweep_after/): https://www.reddit.com/r/rust/comments/jfdiao/tip_for_nightly_users_use_cargosweep_after/


### 博客：Rust 生命周期

[https://blog.thoughtram.io/lifetimes-in-rust/](https://blog.thoughtram.io/lifetimes-in-rust/)

### 视频：STM32 Rust 系列教程

[https://www.youtube.com/playlist?list=PLDWmoWFf46givBRQmh5DyE27OsXMJPfag](https://www.youtube.com/playlist?list=PLDWmoWFf46givBRQmh5DyE27OsXMJPfag)

### Rust 玩具神经网络

用 Rust 实现的玩具神经网络，灵感来自于  [11行Python代码实现玩具神经网络](https://iamtrask.github.io/2015/07/12/basic-python-network/)

```
// Original python code
// https://iamtrask.github.io/2015/07/12/basic-python-network/
// Neural Network = 3 inputs, 4 hidden, 1 output with sigmoid activation

use ndarray::{array, Array};
use ndarray_rand::{rand::SeedableRng, rand_distr::Uniform, RandomExt}; // random matrix generation
use rand_isaac::isaac64::Isaac64Rng; // for setting seed

fn main() -> std::io::Result<()> {
    // set seeds
    let seed = 42;
    let mut rng = Isaac64Rng::seed_from_u64(seed);

    // dataset
    let x = array![[0., 0., 1.], [0., 1., 1.], [1., 0., 1.], [1., 1., 1.]];
    let y = array![[0., 1., 1., 0.]].reversed_axes();

    // initializing random weights
    let mut w0 = Array::random_using((3, 4), Uniform::new(0., 1.), &mut rng); // syn0
    let mut w1 = Array::random_using((4, 1), Uniform::new(0., 1.), &mut rng); // syn1

    for i in 0..1001 {
        // forward propagation
        let hidden = -x.dot(&w0); // l1
        let hidden = 1. / (1. + (hidden.mapv(f64::exp)));
        let output = -hidden.dot(&w1); // l2
        let output = 1. / (1. + (output.mapv(f64::exp)));
        
        // gradient calculation
        let output_delta = (&y - &output) * (output.clone() * (1. - &output)); // l2_delta
        let hidden_delta = output_delta.dot(&w1.t()) * (hidden.clone() * (1. - &hidden)); // l1_delta

        // update weights
        w1 = &w1 + &hidden.t().dot(&output_delta);
        w0 = &w0 + &x.t().dot(&hidden_delta);

        if i % 100 == 0 {
            println!("Epoch: {} \n {:?}\n", i,output);
        }
        
    }

    Ok(())
}

```

原文链接：  [https://www.reddit.com/r/rust/comments/jf6t5v/toy_neural_network_in_rust/](https://www.reddit.com/r/rust/comments/jf6t5v/toy_neural_network_in_rust/)

### 通过`本福特定律`来学习 Rust

> 本福特定律，也称为本福特法则，说明一堆从实际生活得出的数据中，以1为首位数字的数的出现概率约为总数的三成，接近直觉得出之期望值1/9的3倍。推广来说，越大的数，以它为首几位的数出现的概率就越低。它可用于检查各种数据是否有造假。(来自百度百科)

这篇文章通过验证`本福特定律`的方式来学习 Rust (入门级).

大纲如下:

-   本福特定律 介绍
-   Installing Rust
-   Setting up the Project
-   Reading the Dataset
-   Logging
-   Parsing the Dataset
-   Comparing Results
-   Error Handling
-   Command Line Arguments
-   总结

[原文链接](https://gliderkite.github.io/posts/learn-rust-with-benford/)

### 在Rust中证明 1 +1 = 2

有趣~

> Proving that 1 + 1 = 2 in Rust

[Gist](https://gist.github.com/gretingz/bc194c20a2de2c7bcc0f457282ba2662)

### Rust的move/copy/drop语义和数据归零的陷阱

我们在BitBox02硬件的固件中广泛使用rust。在这样的安全设备中，您不希望将敏感材料留在内存中超过必要的时间。特别是，当删除该值时，应该用零安全地覆盖内存，以减少内存泄漏的风险。zeroize是一个包，被设计使这项任务容易和安全。

[文章链接](https://benma.github.io/2020/10/16/rust-zeroize-move.html)，https://benma.github.io/2020/10/16/rust-zeroize-move.html

### 如何编写健康的rust宏

健康宏(Macro hygiene)是宏一个概念，支持在所有的上下文中工作。他们不影响周围，也不受周围任何事物的影响。理想情况下，所有的宏都是完全健康的，但是有很多陷阱使得很容易意外地编写不健康的宏。

了解module模块。

首先，稍微讲一下Rust模块系统的细节，特别是路径;以后了解这个很重要。

[文章链接](https://gist.github.com/Koxiaet/8c05ebd4e0e9347eb05f265dfb7252e1)，https://gist.github.com/Koxiaet/8c05ebd4e0e9347eb05f265dfb7252e1

### rust 树莓派交叉编译，并部署web服务

-   树莓派交叉编译
-   systemd 服务管理

内容非常详细！

链接：[https://tiziano88.medium.com/compiling-rust-for-raspberry-pi-arm-922b55dbb050](https://tiziano88.medium.com/compiling-rust-for-raspberry-pi-arm-922b55dbb050)

链接：[https://tokio.rs/blog/2020-10-tokio-0-3](https://tokio.rs/blog/2020-10-tokio-0-3)

### 对  `std::io::Error`  的学习

博客系列，首先要求对Rust错误处理有基本的了解。在本文中，我们将从 Rust 的标准库中剖析  `std::io::Error`  类型的实现。 源码在这里：[library/std/src/io/error.rs](https://github.com/rust-lang/rust/blob/5565241f65cf402c3dbcb55dd492f172c473d4ce/library/std/src/io/error.rs)【https://github.com/rust-lang/rust/blob/5565241f65cf402c3dbcb55dd492f172c473d4ce/library/std/src/io/error.rs】。

你可以通过以下任一方式理解这些代码：

-   对特定标准库的研究。
-   高级错误管理指南。
-   一个漂亮的API设计案例。

[更多请看原文：](https://matklad.github.io/2020/10/15/study-of-std-io-error.html)https://matklad.github.io/2020/10/15/study-of-std-io-error.html

## Rust 中的可选参数

在 Rust 中, 可选参数并不在语法层面上被支持.

这篇文章列出了下面多种方式来支持可选参数, 并且附上了优缺点:

-   `Option<T>`
-   `Into<Option<T>>`
-   自定义的 struct.
-   使用`builder`模式.
-   `Endpoint-oriented 接口`
-   `Hybrid derive pattern`
-   `Grouping up endpoints`
-   `Macros`

[原文链接](https://vidify.org/blog/rust-parameters/)

### 关于 curl 的讨论

最近，ISRG（Internet Security Research Group，Internet 安全研究小组）宣布通过一系列的开发确保 curl 代码库关键部分的内存安全。包括：资助 curl 的主要作者 Daniel，增加对 Hyper 的支持，使用它作为 curl 的 HTTP 后端；同时，ISRG 工程师会增加对 Rustls 的支持，将其作为 curl 的 TLS 后端。

小编整理了一些相关的链接，供大家了解更多信息。

ISRG 博客中关于[内存安全 curl 的文章](https://www.abetterinternet.org/post/memory-safe-curl/)，https://www.abetterinternet.org/post/memory-safe-curl/

curl is C  [旧文](https://daniel.haxx.se/blog/2017/03/27/curl-is-c/)，https://daniel.haxx.se/blog/2017/03/27/curl-is-c/

hyper C API的pr  [#2278](https://github.com/hyperium/hyper/pull/2278)， https://github.com/hyperium/hyper/pull/2278

curl 作者在 hyper 上提的 issue  [#2265](https://github.com/hyperium/hyper/issues/2265)，https://github.com/hyperium/hyper/issues/2265

Reddit 上的[讨论](https://www.reddit.com/r/rust/comments/j7yegb/memory_safe_curl_for_a_more_secure_internet/)，https://www.reddit.com/r/rust/comments/j7yegb/memory_safe_curl_for_a_more_secure_internet/


### Cranelift 的新后端系列文章

该系列是作者 Chris Fallin 在 Mozilla 关于 Cranelift 上的工作介绍。这篇文章是由三部分组成的系列文章的第一篇，文章中作者介绍了一些背景并描述了指令选择问题，尤其是讨论了过去九个月左右对指令选择器和后端框架的总体改进。

Cranelift 是什么？该项目是用 Rust 编写的一个编译器框架，该框架专门设计用于即时编译（JIT，just-in-time）。

Mozilla Hacks 的[文章](https://hacks.mozilla.org/2020/10/a-new-backend-for-cranelift-part-1-instruction-selection/)，https://hacks.mozilla.org/2020/10/a-new-backend-for-cranelift-part-1-instruction-selection/

**BTW**，今天是作者在 Mozilla 的最后一天，所以其计划的其它 Cranelift 系列文章都将放在其博客上。

阅读作者的博客[原文](https://cfallin.org/blog/2020/09/18/cranelift-isel-1/)，https://cfallin.org/blog/2020/09/18/cranelift-isel-1/


### Dancing links 算法的Rust实现

本篇博客讲解的细致入微，甚至连变量命名都讲解了。

是一篇十分适合rust初学者的博客，如果你对rust中的链表等数据结构如何实现不了解，本篇也十分适合你。

[Read More](https://ferrous-systems.com/blog/dlx-in-rust/): https://ferrous-systems.com/blog/dlx-in-rust/

### Rust中的迭代器

作者学习rust中迭代器的笔记，也是一篇适合初学者的博客。

[Read More](https://blog.thoughtram.io/iterators-in-rust/): https://blog.thoughtram.io/iterators-in-rust/

### Rust中的迭代器与scala的比较

本篇讲解一个函数式编程中的笑话：每段scala代码都能用traverse函数写出来（感谢洛佳的总结概括）

同时也将rust中迭代器的一些用法，与scala做了一些比较。

对函数式编程感兴趣的可以看看。

[Read More](https://www.fpcomplete.com/blog/collect-rust-traverse-haskell-scala/): https://www.fpcomplete.com/blog/collect-rust-traverse-haskell-scala/

### Rust异步基准测试

本篇看点是`Analysis`部分。

作者对自己的测试结果说明了rust异步是否是零成本抽象。

[Read More](https://github.com/jkarneges/rust-async-bench)  :https://github.com/jkarneges/rust-async-bench


### 斯坦福大学新开的rust相关的课程

![](https://reberhardt.com/blog/images/designing-cs-110l/security-overlap.png)

[Read More](https://reberhardt.com/blog/2020/10/05/designing-a-new-class-at-stanford-safety-in-systems-programming.html): https://reberhardt.com/blog/2020/10/05/designing-a-new-class-at-stanford-safety-in-systems-programming.html

[课程链接](https://reberhardt.com/cs110l/spring-2020/): https://reberhardt.com/cs110l/spring-2020/

## 使用 Rust 创造一门新语言: Part 5 绑定的用法

这是使用 Rust 创造一门新语言的第五篇文章.

这篇文章主要讲述了如何支持类似  `let a = 10`  这种绑定的支持. 感兴趣的小伙伴可以查看原文.

[原文链接](https://arzg.github.io/lang/5/)

### Rust如何使我们监控30k/min的API调用

在Bearer，我们是一个多语言的工程师团队。无论是交流的语言，还是变成的语言。我们的技术栈是由Node.js，Ruby，Elixir，以及我们代理库支持的所有语言之外的其他语言写的服务组成。想大多数团队一样，我们再使用正确的工具工作和使用正确的工具节省时间中取得平衡。

[文章链接](https://blog.bearer.sh/how-rust-lets-us-monitor-30k-api-calls-min/)，https://blog.bearer.sh/how-rust-lets-us-monitor-30k-api-calls-min/

### Formicarium(蚁群)

这篇文章是关于群体智能的，以及我们如何尝试模拟它来展示集体行为如何能够为可能无法解决的问题找到更好的解决方案。从维基百科中，我们可以将群体智能(SI)定义为:

> SI系统通常由一个简单的代理或人口与本地交互彼此和他们的环境。这些代理遵循非常简单的规则，尽管没有集中控制结构来规定个体代理应该如何行为，局部的，并且在一定程度上是随机的，这些代理之间的交互导致了“智能的”全局行为的出现，而个体代理不知道。

蚁群智能系统最常见的例子之一就是蚁群，这就是formicarium试图模拟的。

[文章链接](https://gliderkite.github.io/posts/formicarium/)，https://gliderkite.github.io/posts/formicarium/

[Github 链接](https://github.com/gliderkite/formicarium)，https://github.com/gliderkite/formicarium

### 深入了解Ringbahn的drivers

Ringbahn是无船同志(without boats)基于io-uring 的 安全Rust 绑定。

> 柏林环线（ringbanhn， Berlin Ringbahn）是一条双轨通勤铁路，环绕柏林市中心形成一个完整的环形。类似地，IO -uring是一个新的异步IO接口，Linux内核构建在双环缓冲区数据结构上。基于async/await开发，100%内存安全。

这是无船一系列解读Ringbahn原理文章中的第三篇。

链接：[https://without.boats/blog/ringbahn-iii/](https://without.boats/blog/ringbahn-iii/)

## November --202011


### Monad interface

`monad`  主要是函数式编程中的概念. 本文通过代码例子展示了如何在 rust 中来实现 moands.

[原文链接](https://medium.com/swlh/monad-interface-rust-edition-bd6486b93607)


### 编写 Rust 的 23 条基本规则和建议

`Seed`  是一个优秀的 Rust 前端框架, 但是, 这里我们不讨论  `Seed`  本身, 而是隐藏在这个优秀项目里的一些优秀的建议.

在他的文档中,`Seed`给出了 如何编写 Rust 的 23 条基本规则和建议, 无论新手老手都会有所收获.

[原文链接](https://seed-rs.org/0.8.0/rust)  请查看`How to write in Rust`章节.

### 为什么  `BTreeMap`  没有  `with_capacity()`  方法 ?

Rust 中的  `HashMap`  (`Hash?Set`,  `Vec`)等都提供了一个初始化的方法  `with_capacity(capacity: usize)`. 但是为什么  `BTreeMap`(以及  `BTreeSet`) 没有这个方法呢?

本文讲述了这两种数据结构的区别,并且解答了上述的问题.

[原文链接](https://www.nicolas-hahn.com/2020/11/30/btreemap-with-capacity/)

### 如何在rust的路上前进

这是我的背景，我以前用c开发嵌入式系统项目；C++/Qt/Python用来开发桌面应用；javascript和Node.js来做Web开发。当我知道了rust可以做到所有这些，尤其是嵌入式系统。我开始学习rust...

[文章链接](https://www.reddit.com/r/rust/comments/k2nige/how_could_i_get_advanced_on_rust_road/)，https://www.reddit.com/r/rust/comments/k2nige/how_could_i_get_advanced_on_rust_road/

### Rust类型状态

这项工作是我关于行为类型的论文的一部分，任何类型的反馈都是受欢迎的，并且可以作为问题提交给rustype组织中的任何存储库。如需进一步讨论，请通过Twitter或Keybase与我联系。

本系列的目标是成为一种开发日志，我在其中探索类型状态（可能还有其他类型）及其使用Rust类型系统的实现。

[文章链接](https://rustype.github.io/notes/notes/rust-typestate-series/rust-typestate-index)，https://rustype.github.io/notes/notes/rust-typestate-series/rust-typestate-index


## 用一个周末学习光线追踪

这是一篇用 Rust 来学习 《Ray Tracing in One Weekend》系列的仓库，目前已经实现了基本类型与初步的并行化渲染等需求，对光追感兴趣的朋友可以看看~

[repo](https://github.com/7sDream/remda)

## 基于 WASM 在浏览器里运行MNIST推理（转自 Rust 视界）

同样是基于 Bevy 编写的 demo,这次借助于 Rust -> WASM 的能力，在 Bevy 框架基础上将 MNIST 搬到了浏览器中

[repo](https://github.com/vleue/bevmnist)

### A Cool Generic Concurrency Primitive in Rust

这是  `Crust of Rust`  作者最新一期视频, 在此视频中, 作者会演示如何抽象出一个可复用的泛型版本的高性能无锁并发基础类型.

[油管视频链接](https://www.youtube.com/watch?v=eLNAMEoKAAc)

### WebAssembly Three Ways

作者对比了三种可以编译成 WebAssembly 的方式:

-   Rust
-   Go
-   AssemblyScript

文中提供了各个语言的简单 demo 代码, 并且分别从 binary size, 执行速度等进行了简单的对比测试.

[原文链接](https://www.ianmccall.codes/post/2020/10/11/webassembly-three-ways.html)

### 使用 Enums 来减少内存使用

Enum 在不存储真正数据的时候,仅仅需要`1byte`就可以代表多种状态. 根据这个特性, 作者把一些特殊场景下的内存使用大大降低了.

[原文链接](https://dom.events/2020/11/21/decrease-memory-using-structs-in-rust.html)


### 使用 const 泛型的物理单位

`const_unit_poc`就像一个具有更好的错误消息的  [uom](https://github.com/iliekturtles/uom)！

```
#![feature(const_generics, const_evaluatable_checked)]
use const_unit_poc::values::{m, kg, s, N};

let distance = 1.0 * m;
let mass = 18.0 * kg;
let force = distance * mass / (1.8 * s * 2.0 * s);
assert_eq!(force, 5.0 * N);

let mut mutable_distance = 3.2 * m;
mutable_distance -= 0.2 * m;
mutable_distance += 2.0 * m;

assert_eq!(mutable_distance, 5.0 * m);

```

`const_unit_poc`  crate  [链接](https://docs.rs/const_unit_poc/1.0.0/const_unit_poc/)，https://docs.rs/const_unit_poc/1.0.0/const_unit_poc/

### Lunatic

Lunatic，用于构建使用 WebAssembly 实例作为 actors 的 Actor 系统平台。它在很大程度上受到 Erlang 的启发，可以从可编译为 WebAssembly 的任何语言中进行定位。当前只有 Rust 的绑定可用。

示例：

```
use lunatic::{Channel, Process};

fn main() {
    let channel = Channel::new(0);
    let vec: Vec<i32> = (0..1_000).collect();

    for i in vec.iter() {
        Process::spawn((*i, vec.clone(), channel.clone()), child).unwrap();
    }

    for _ in vec.iter() {
        let (i, sum) = channel.receive();
        println!("Sum until {}: {}", i, sum);
    }
}

// Child process calculates the sum of numbers of context.1 until context.0 index.
fn child(context: (i32, Vec<i32>, Channel<(i32, i32)>)) {
    let i = context.0;
    let vec = context.1;
    let channel = context.2;
    let sum_until_i: i32 = vec[..=i as usize].iter().sum();
    channel.send((i, sum_until_i));
}

```

该应用程序产生1000个子进程，并在每个子进程中计算从0到i的数字总和，然后将结果发送回父进程并进行打印。

Github[链接](https://github.com/lunatic-lang/lunatic)，https://github.com/lunatic-lang/lunatic


### 轻松编写rust事件

这是之前日报提到过的[rust自定义事件(https://medium.com/better-programming/custom-events-in-rust-c4e534b6b8cb)](https://medium.com/better-programming/custom-events-in-rust-c4e534b6b8cb)的后续。

[Read More（需科学上网）](https://rossketeer.medium.com/rust-events-revisited-926486721e3f): https://rossketeer.medium.com/rust-events-revisited-926486721e3f

### Rust错误处理中的匿名(Anonymous)类型与和(Sum)类型

一篇讨论rust中错误处理的返回类型的文章。

[Read More](https://jam1.re/blog/anonymous-sum-types-for-rust-errors): https://jam1.re/blog/anonymous-sum-types-for-rust-errors

[相关阅读](https://zhuanlan.zhihu.com/p/225808164?utm_source=ZHShareTargetIDMore): https://zhuanlan.zhihu.com/p/225808164?utm_source=ZHShareTargetIDMore

之前日报里有提到过reddit上有次讨论错误处理在rust和java中的区别的帖子，于是最近社区里就有人按照帖子中提到的痛点实现了这个库：  [Crates.io](https://crates.io/crates/polyerror): https://crates.io/crates/polyerror

[Read More](https://users.rust-lang.org/t/errors-in-rust-can-now-be-handled-more-ergonomically-cleanly-and-simply-introducing-a-new-error-crate/51527): https://users.rust-lang.org/t/errors-in-rust-can-now-be-handled-more-ergonomically-cleanly-and-simply-introducing-a-new-error-crate/51527


### 所有权语义的可用性

一篇讨论所有权可用性的论文。

[Read More](https://arxiv.org/abs/2011.06171): https://arxiv.org/abs/2011.06171

### Rust性能之书

转自 Rust视界

rustc-perf 2号贡献者 Nicholas 写了一本关于 提升 Rust 程序性能的经验之书。

在线阅读：https://nnethercote.github.io/perf-book/ 源码：https://github.com/nnethercote/perf-book


### @xxy1413 的 Rust编程实战课程笔记

到目前为止，课程已经讲完了基本语法以及包管理系统  `Cargo`  的用法，此仓库是学习群群友的笔记，还在刷 trpl 的朋友可以参考

github： https://github.com/xxg1413/inviting-rust-note


### Crust of Rust: Sorting Algorithms

这是  `Crust of Rust`最新的一期 Rust 视频:  `Sorting Algorithms`.  
`Crust of Rust`  是一系列质量比较高的 Rust 直播编码视频. 强烈推荐给各位小伙伴.  
需要科学上网.

[油管连接](https://www.youtube.com/watch?v=h4RkCyJyXmM&feature=youtu.be)

### Rust 实现 Custom Events

本文设计了一个比较巧妙的 Events 模式, 不由地让人想起  `Qt`的  `signal`.

下面是核心代码以及使用例子. 具体更多细节可以参阅原文.

```
trait Sig {
    type Data;
    type Receiver: Rec;

    fn emit(&self, data: Self::Data);
    fn conn(&mut self) -> Self::Receiver;
    fn disc(&mut self, i: usize);
}

trait Rec {
    type Data;

    fn on_emit(self, data: Self::Data);
    fn get_id(&self) -> usize;
}

macro_rules! def_signal{
    // ...
}

// 使用例子
struct MySigData {
    num: i32,
}
fn main() {
    def_signal!(
        MySig, // Signal 名字
        MyRec, // Receiver 名字
        NySigData, // 预定义的数据类型
        |this: MyRec, data: MySigData| { // 逻辑
            println!("MySig receriver R{} - num: {}", this.id, data.num);
        }
    )
    let mut ms2 = MySig::new();
    let r1 = ms2.conn();
    let r2 = ms2.conn();

    ms2.emit(MySigData{ num: 3});
    ms2.disc(r1.id);
    ms2.emit(MySigData{ num: 9});
    ms2.disc(r2.id);
}

```

输出结果

```
MySig receriver R1 - num: 3
MySig receriver R2 - num: 3
Removing Signal R1
MySig receriver R2 - num: 9
Removing Signal R2

```

[原文链接](https://rossketeer.medium.com/custom-events-in-rust-c4e534b6b8cb)



### 【学习资料】Rust模块系统说明

![Snipaste_2020-11-16_21-53-21](https://blog-imghost.oss-cn-shanghai.aliyuncs.com/img/20201116215404.png)

这是面向初学者的视频，拥抱Rust吧🎉～

[YouTube](https://www.youtube.com/watch?v=4KsAsGhFo4U&feature=emb_logo&ab_channel=RustCast)

### 实体组件系统调度器设计

`ECS (entity-component-system)`  是一种广泛应用于游戏引擎中的设计理念.

本文主要描述的是  `ECS`  相关的概念中的  `Scheduler`.

-   什么是调度器?
-   动态调度.
-   静态调度.
-   实际情况考量, 例如 thread local 等.
-   调度器实际例子. 如  `bevy_ecs`,  `yaks`等.

[原文链接](https://ratysz.github.io/article/scheduling-1/)

### rustq

在rest API中表达数据操作的最简单的办法。

使用适当的http方法、url、csv的数据格式组合实。

[Github 链接](https://github.com/ivanceras/restq)，https://github.com/ivanceras/restq

### PossibleRust.com

PossibleRust.com是上线不久的专注于分享Rust的个人博客网站，作者从2013年0.8版开始学习Rust，并声称到现在他也一直在学新的Rust的东西。

比如最新的一篇博文[Non-Generic Inner Functions](https://www.possiblerust.com/pattern/non-generic-inner-functions)讲到如何使用内部非泛型函数来避免静态分发导致的编译速度下降和二进制体积膨胀的问题，强烈推荐一看。（占个坑，有时间我翻译一下）

链接：[https://www.possiblerust.com/pattern/non-generic-inner-functions](https://www.possiblerust.com/pattern/non-generic-inner-functions)


### Are out parameters idiomatic in Rust?

Out parameter中文应该翻译成传出参数？其实很好理解，大家都碰到过或用过。

举个例子，如下第二种即Out parameter：

```
fn foo() -> i32 {
    // body elided
}

let x = foo();

```

```
fn foo(out: &mut i32) {
    // body elided
}

let mut x = 0;

foo(&mut x);

```

作者这篇文章提供了在Rust中Out parameter的最佳实践：

-   **能不使用传出参数的情况下尽量不要使用**

有几个原因：1) 返回值的方式语义上更明确，更清晰。2) Rust有非常丰富的数据类型，如果需要返回多个参数可以用tuple包裹起来，没必要作为out parameter来返回。3) Rust编译器大都数情况下对返回值的拷贝有优化，几乎能达到和out parameter同样的效果；

-   **只有一种情况推荐使用Out parameter: 调用方传递给函数的是某种形式的buffer**

比如标准库的`Read`  trait:

```
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

```

这里的read()比较适合使用out parameter。

链接：[https://steveklabnik.com/writing/are-out-parameters-idiomatic-in-rust](https://steveklabnik.com/writing/are-out-parameters-idiomatic-in-rust)


### Writing BPF code in Rust

BPF是Berkeley Packet Filter（伯克利包过滤器）的缩写，这是一个用于过滤(filter)网络报文(packet)的架构。其实 BPF 可谓是名气不大，作用不小的典范。BPF 即为 tcpdump 抑或 wireshark 乃至网络监控(Network Monitoring)领域的基石。以 tcpdump 为例：熟悉网络监控(network monitoring)的读者大抵都知道 tcpdump 依赖于 pcap 库，tcpdump 中的诸多核心功能都经由后者实现。而pcap就是基于内核中的BPF模块。

这篇文章作者介绍了自己公司提供了几个方便使用Rust编写BPF/eBPF程序的crate。

文章链接：[https://blog.redsift.com/labs/writing-bpf-code-in-rust/](https://blog.redsift.com/labs/writing-bpf-code-in-rust/)

> 更多参考资料了解BPF
> 
> eBPF 简史:  [https://www.ibm.com/developerworks/cn/linux/l-lo-eBPF-history/index.html](https://www.ibm.com/developerworks/cn/linux/l-lo-eBPF-history/index.html)
> 
> BPF: A New Type of Software：[http://www.brendangregg.com/blog/2019-12-02/bpf-a-new-type-of-software.html](http://www.brendangregg.com/blog/2019-12-02/bpf-a-new-type-of-software.html)

### 通过用 Rust 编写大量的Brainfuck编译器来学习汇编

你是否想成为 CPU Whisperer？从逻辑上讲，Brainfuck 是最著名的深奥编程语言。它之所以成名，主要是因为它的名称中带有“ fuck”一词，但爱好者编译器开发人员喜欢它，因为它是一种很小的语言，可以很容易地为其编写编译器。有趣的事实是：人们编写的 Brainfuck 编译器比实际的 Brainfuck 程序更多。详情请看博客原文：[https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md](https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md)

### reacty_yew

作者一直是 React 和它的 JSX 语法的忠实拥护者。同时倾向于使用旨在提供类似语法的 Rust 前端框架，并且当 Yew 成为可行的选择时，作者试图将两者结合起来，通过 Typescript 类型定义从 React 组件生成 Yew 组件。更多请看原文：[https://www.hobofan.com/blog/2020-11-10-reacty_yew/](https://www.hobofan.com/blog/2020-11-10-reacty_yew/)

### 关于 Rust 的数学运算

昨天有个同学说 rust 没有基础数学运算。emmmm

```
今天写小玩具的时候发现标准库都不支持基础数学运算，都要自己造轮子，
三方库里也只有unsafe的linux数学库，愿rust越来越强！

```

洛佳同学的回复:

```
数字运算在类型里面，标准库是有的，是支持的

```

Attila的灵魂一击:

```
说找不到库的试试这个的分类？

https://lib.rs/science/math

```

点击发现更多科学计算库:[https://lib.rs/science/math](https://lib.rs/science/math)

最后说一句:多 Google ,几个关键字一拼什么都有了

### [linux kernel] 在Rust中编写BPF代码

BPF是一种虚拟机，当Linux系统上发生某些事件时，它允许在内核中运行用户定义的程序。例如，您要监视可疑文件活动，记录网络响应延迟甚至跟踪用户空间应用程序–您可以编写小型BPF程序，请求将它们附加到内核中的正确位置，并实施必要的检测。

BPF VM使用其自己的指令集。您可以直接编写字节码，但是人们通常使用  [bpftrace](https://github.com/iovisor/bpftrace)  或编写C代码并使用  [BPF编译器集合（BCC）进行编译](https://github.com/iovisor/bcc)。

从原理上讲，开发BPF程序的过程可以归纳为以下步骤：

1.  用C编写BPF代码
2.  编译BPF VM的代码
3.  编写一个将第2步的输出加载到BPF VM的用户空间组件
4.  使用BPF API在用户空间组件和BPF代码之间交换数据

RedBPF包括用于实现上述所有步骤（步骤1）除外的API和工具。使用RedBPF，步骤1变为：

1.  在Rust中编写BPF代码

文中实现了一个简单的http trace, 有兴趣的可以仔细阅读

[详情](https://blog.redsift.com/labs/writing-bpf-code-in-rust/):[https://blog.redsift.com/labs/writing-bpf-code-in-rust/](https://blog.redsift.com/labs/writing-bpf-code-in-rust/)


### KRust:Rust形式化可执行语义

论文摘要：Rust是新兴的系统级编程语言，旨在提供内存安全的同时保证极高的性能。Rust形式化语义是用来证明其内存安全和开发Rust程序分析工具的基础。鉴于目前没有直接描述Rust的形式化语义，提出了针对Rust语言的形式化可执行语义KRust。为了确保语义的可执行性和应用性，使用了K框架进行语义的开发。KRust目前涵盖了Rust常见的语法和语义，包括了Rust的3个核心特性：所有权、借用和生命周期。KRust通过了191个测试样例，其中157个都是来自Rust官方的测试集。语义对比测试实验发现了Rust编译器的缺陷。此外，KRust的语义还可以被应用于开发Rust程序分析工具。

论文链接： http://fcst.ceaj.org/CN/abstract/abstract2041.shtml

### Rust for gophers

这是为 golang 使用者打造的一系列学习 Rust 的指南. 可以让 gophers 快速找到 golang 在 Rust 中对应的知识点.

目前已经更新到了第六章. 从 golang 转过来的小伙伴可以看看.

[原文链接](https://levpaul.com/posts/rust-lesson-5-and-6/)

### 使用 Rust 创造一门新语言: Part 9, Function calls

这是`<<使用 Rust 创造一门新语言>>`的最新一章,  `函数调用`.

[原文链接](https://arzg.github.io/lang/9/)

## 构建可测试性的 Rust 工程

最近  `reddit`  上有不少人对  `Rust`  的工程如何进行组织, 以及如何才能进行更加完善的测试进行了一些讨论.

下面的文章给出了一个  `web`  服务的例子, 演示了如何来组织  `Rust`  中的模块, 以及如何让代码更加具有测试性.

[原文链接](https://medium.com/better-programming/structuring-rust-project-for-testability-18207b5d0243)

### 使用Rust进行嵌入式开发

![ewg-logo-blue-white-on-transparent-256x256](https://blog-imghost.oss-cn-shanghai.aliyuncs.com/img/20201108194618.png)

在树莓派（Raspberry Pi）上用Rust做嵌入式系统开发

[Repo](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)

### 茶余饭后的一点儿小谈：Rust 安全规范

C 安全规范：171条建议，106条规则。

C++ 安全规范：101条准则，92条风格指南。

Rust 安全规范： 所有权模型 + 编译器检查。👍

> 顺便提一嘴：
> 
> Rust can be thought of as a combination of two programming languages:  _Safe Rust_  and  _Unsafe Rust_.
> 
> 更多请阅读如下链接：

[Doc](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html)

# Rust Reference中文版

经过两个月的奋战，作者把 Rust 的 Reference 给翻译完成了。

现在诚邀各位围观，希望大家积极提交修改意见，以及后续的维护建议。

在线预览地址：[https://minstrel1977.gitee.io/rust-reference/](https://minstrel1977.gitee.io/rust-reference/)

# 清华大学《R:Z 从零开始的RustOS编写体验指南》项目邀请

> 各位同学大家好，我是本学期操作系统专题训练课程“R:Z RustOS Learning Guide”小组的组长马川。

> 我们小组在本学期试着为进行 rCore 实验的同学们编纂一部面向进行 rCore 实验但没有Rust基础的同学们的学习指南。我们现在已经有了整个指南的大致框架以及部分内容，现在它已经成为了 GitHub 上的一个开源项目，欢迎各位同学前去围观，给出你的意见与看法，说出你希望指南中加入什么内容，还可以加入我们一起进行项目的维护。我们欢迎所有人加入这个项目，一起帮助在 rCore 学习中遇到困难的同学们度过难关。

**参考资料与链接**

-   网页端 gitbook：[https://simonkorl.gitbook.io/r-z-rustos-guide/](https://simonkorl.gitbook.io/r-z-rustos-guide/)
    
-   GitHub 仓库：[https://github.com/simonkorl/rustOS_learning_guide](https://github.com/simonkorl/rustOS_learning_guide)
    
-   本项目的操作系统 wiki 页：[http://os.cs.tsinghua.edu.cn/oscourse/OsTrain2020/g7](http://os.cs.tsinghua.edu.cn/oscourse/OsTrain2020/g7)
    
### 为什么Dark语言不选择Rust

本来只是一篇简单的为什么不选择Rust的blog，但是在reddit上还是引起了不少讨论。

总的来说rust是一门低级语言，就像这篇blog中作者所说的一样，有时候能畅快表达比性能更重要（ Maybe that costs performance, but I need the ability to quickly write code a lot more than I need the extra performance.）。

虽然如此，但是就像之前日报报道过一篇关于ra的文章一样，rust在足够复杂的项目中，开发效率是能够和kotlin这类高级语言相媲美的。

[Read More](https://www.reddit.com/r/rust/comments/jo2hco/why_dark_didnt_choose_rust/): https://www.reddit.com/r/rust/comments/jo2hco/why_dark_didnt_choose_rust/

[Blog 原文](https://blog.darklang.com/why-dark-didnt-choose-rust/): https://blog.darklang.com/why-dark-didnt-choose-rust/

### 在rust中实现链表

本文作者以尽可能符合rust风格的方式来完成链表这一数据结构。

[Read More](https://rossketeer.medium.com/implementing-a-linked-list-in-rust-c25e460c3676): https://rossketeer.medium.com/implementing-a-linked-list-in-rust-c25e460c3676

### 所有权语义的致命缺陷

一篇讨论所有权的blog，正如此篇blog发布的reddit贴评论所说的一样，尽管标题的骗点击率的嫌疑和一些毫无根据的结论，但实际上，这篇文章是围绕某些编程范例的哲学而写的。

[Read More](http://www.gingerbill.org/article/2020/06/21/the-ownership-semantics-flaw/): http://www.gingerbill.org/article/2020/06/21/the-ownership-semantics-flaw/


### `Bevy`贪吃蛇教程更新了

`Bevy`最近发布了0.3的版本，带来了很多令人激动的提升，同时社区学习资料较少，之前的贪吃蛇教程也随着`Bevy`新版本的发布而更新了。

[Read More](https://mbuffett.com/posts/bevy-snake-tutorial/#0.3)：https://mbuffett.com/posts/bevy-snake-tutorial/#0.3


### 《蜜月后的Rust语言》

作者德雷克·摩尔分享了它两年来的Rust开发经历。作者主要关注Rust在嵌入式处理器的运用，从开发到调试，包括宏、格式化到内联汇编，分享了很多Rust语言改进为开发带来的便利之处。Rust提供有别于传统C语言的思路，是丰富而功能强大的语言。作者认为，提升与Rust的关系将是他事业中最重要的部分。

[博客链接](http://dtrace.org/blogs/bmc/2020/10/11/rust-after-the-honeymoon/)

### Rust 程序的 CD (Continuous Deployment)

这是`<<从零到生产>>`的新一章.

这一章主要讲述如何发布我们的程序到生产环境中. 文中以发布到  `DigitalOcean`  为例, 讲述如何编写  `Dockerfile`, 打包镜像, 以及最终发布到  `DigitalOcean`的过程.

感兴趣的后端同学可以看看.

[原文链接](https://www.lpalmieri.com/posts/2020-11-01-zero-to-production-5-how-to-deploy-a-rust-application/)

### Rust Quiz: Rust 小测试

这是  `dtolnay`的一个小玩具, 可以通过小测试的方式了解一下 Rust 的一些好玩的知识.

`dtolnay`  是  `anyhow`,  `thiserror`,  `cxx`,  `paste`  等一系列 crates 的作者.

[rust quiz地址](https://dtolnay.github.io/rust-quiz)

[github地址](https://github.com/dtolnay/rust-quiz)

### 《Zero to production in Rust》

#rust

这是一本为后端开发写的书，书中介绍了完整实现一个Rust服务端应用的过程，包括前期技术调研，应用结构设计，可扩展性设计，如何编写测试，如何收集日志和跟踪信息，搭建一个鲁棒性强的持续集成和持续部署应用。

[Read More](https://www.zero2prod.com/)

## December --202012


### 《Tokio 1.0 接口概览》

本篇博客中，作者分析了Tokio 1.0版本和0.2版本的接口代码差异，以及对代码风格的影响。在新版本下，许多Tokio结构体的操作不再要求自身的可变借用，这能为并发代码减少包装的层数。另外，作者还分析了异步询问函数的改变，以及给出了它对未来接口代码修改的期望。

[博客文章](https://leshow.github.io/post/udp_tokio_1_0/)

### Rust中的闭包

这是一篇详细讲解 rust 中闭包的文章. 不仅从闭包的使用,更是从闭包的原理来更深入的理解闭包.

![img](https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/closures.svg)

[原文链接](https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/)

### Rust 代码仓库十年之旅

这是 Rust 代码仓库通过可视化的方式展示这十年变化的视频. 3382 为 contributors, 交织出 Rust 的辉煌.

[视频链接](https://www.visualsource.net/repo/github.com/rust-lang/rust.git)

### Rust中 Clone 一个引用以及方法调用语法

本来通过几个例子讲述 Rust 中的一些工效学概念. 例如我们日常使用的  `str.clone()`  调用过程中, Rust 是如何帮助我们自动借用或者解引用的.

[原文链接](https://www.fpcomplete.com/blog/cloning-reference-method-calls/)


### Rust no-std FAQ

为什么要写这个

大部分rustaceans（包括我）从我们的第一个hello world程序开始使用std库。不过，在某些情况下，我们需要将代码部署到裸机环境中，这也是Rust的一个非常重要的特性。没有POSIX操作系统的支持，我们无法使用std库，而且它通常会让人们恐慌，所以我写这篇文章来澄清对Rust no-std的误解。

[文章链接](https://justjjy.com/Rust-no-std)，https://justjjy.com/Rust-no-std

### 为什么mongodump很快

出于好奇，我编写了一个mongo数据库同步器，它可以从一个数据库同步到另一个数据库。

当我完成这项工作时，我尝试用mongodump/mongorestore对对它进行基准测试。令人惊讶的是，mongodump比我的假设要快得多。

[文章链接](https://windsoilder.github.io/why_mongodump_is_fast.html)，https://windsoilder.github.io/why_mongodump_is_fast.html

### 2020年在Rust中构建后端应用程序的问题

文章的作者描述了想写一个爬虫,从开源库的选择到解决问题的历程。

[ReadMore](https://blog.0xfa.be/building-a-backend-app-in-rust/):[https://blog.0xfa.be/building-a-backend-app-in-rust/](https://blog.0xfa.be/building-a-backend-app-in-rust/)

### 使用 Rust 创造一门语言, Part 18: 错误

这是  `使用 Rust 创造一门语言`的最新一期. Part 18:  `错误`.

[原文链接](https://arzg.github.io/lang/18/)

### Book: Rust and WebAssembly

这是一本描述如何使用  `Rust`  和  `WebAssembly`的书.

[原文链接](https://rustwasm.github.io/book/)

### Rust in Action MEAP v15发布

《 Rust in Action》是一本针对任何语言背景的中级程序员的书籍，这些程序员对通过项目工作来学习Rust和/或系统编程感兴趣。

[Book](https://www.manning.com/books/rust-in-action)

### wasm-2048

Wasm-2048，是用Rust（Yew）实现了著名的2048游戏，并编译为WASM。

![图片](https://cloud.githubusercontent.com/assets/1175750/8614312/280e5dc2-26f1-11e5-9f1f-5891c3ca8b26.png)

Github[链接](https://github.com/dev-family/wasm-2048)，https://github.com/dev-family/wasm-2048

在线 Demo  [链接](https://2048.dev.family/)，https://2048.dev.family/

### 用 Rust 实现的一些算法和数据结构

一个Github仓库，一些常用算法和数据结构的Rust实现，其中大多数基于William Fiset的 Java 实现：https://github.com/williamfiset/Algorithms。

Github[链接](https://github.com/TianyiShi2001/Algorithms)，https://github.com/TianyiShi2001/Algorithms


### Build your own async primitive

这篇博客介绍了如何在no_std环境下构建自己的异步Mutex和单管道Oneshot，注释非常详细，值得学习。

链接：[https://tweedegolf.nl/blog/50/build-your-own-async-primitive](https://tweedegolf.nl/blog/50/build-your-own-async-primitive)

### 用rust编写一门编程语言系列又更新了

这是第十六部分，已经讲到了重构。

[Read More](https://arzg.github.io/lang/16/): https://arzg.github.io/lang/16/

### Raph Levien 大神最新的blog

Raph Levien也就是piet-gpu的作者，最新的blog里有提到对rust的一些看法。

[Read More](https://raphlinus.github.io/personal/2020/12/16/an-old-new-adventure.html): https://raphlinus.github.io/personal/2020/12/16/an-old-new-adventure.html

### 【视频】OSO 如何为 Rust 构建运行时反射系统

[https://youtu.be/J7Aosp1Uauo](https://youtu.be/J7Aosp1Uauo)

### Rust异步爬虫实战

@原子之音 带来的用 async-std + surf 以及 tokio + reqwest 库来教大家写一个爬虫~ 小编也许可以试着用这个抓一下 彩票的分析数据然后来深度一下~

[bilibili](https://www.bilibili.com/video/BV1tX4y1u7wg?from=search&seid=9375060030658381444)

### 类型标记了之后，反而可以通过编译？

这是今天在 QQ Rust 社区 1 群里面发出的问题，代码如下：

```
fn fn1(s: &mut String) {}
fn main() {
    let mut s = "".to_string();
    let s1 = &mut s;
    fn1(s1);  // s1 没有失效
    s1.len();
    {
        let s2: &mut String = s1;
    }
    s1.len();  // s1 没有失效

    {
         let s2 = s1;
    }
    s1.len(); // s1 失效了
}

```

[play 一下](https://play.rust-lang.org/)

有群友帮忙追溯了一下，原来这个问题[历史还挺久远的](https://github.com/rust-lang/rust/issues/35919), 原来，我们在声明  `s2`  的时候，当我们标记成  `&mut String`  类型的时候，rust 编译器知道  `s2`  也需要一个独占的可变引用，于是就触发了  [“reborrow coercion”机制](https://github.com/rust-lang/rust/issues/35919#issuecomment-304130115)，把  `s1`  可变引用赋予给  `s2`, 同时在第一个大括号里面冻结了  `s1`  变量，而在第一个大括号结束了之后，  `s2`  生命周期结束，  `s1`  也就被解冻了。 而在第二个大括号里面，因为 rust 编译器不知道应当给  `s2`  推断成什么类型，于是干脆就转移（move）  `s1`  到  `s2`, 使得原本的  `s1`在转移之后失效，只剩下  `s2`, 所以 不能在  `let s2 = s1;`  之后使用  `s1`了。

### 使用 VScode 和 GDB 在 PineCone BL602 上进行调试

这是一片非常详尽的调试指南, 但是目标不是我们笔记本或者服务器, 而是  `PineCone BL602`.

![img](https://lupyuen.github.io/images/debug-title.jpg)

[原文链接](https://lupyuen.github.io/articles/debug)

### `deps.rs`  服务恢复了

`deps.rs`  是一个可以检测你 crate 中有多少过期依赖的服务. 你可以在你的项目中加入 Badges, 以达到下面的效果.

![img](https://camo.githubusercontent.com/3a876f044e76aa23af6f6d852b2a6be6abb512ae9233525459a7893a3537d241/68747470733a2f2f646570732e72732f7265706f2f6769746875622f646570732d72732f646570732e72732f7374617475732e737667)

其次,你还可以访问他们的网站,来查看具体一个 crate 是否有过期的依赖.

![img](https://github.com/deps-rs/deps.rs/raw/main/resources/banner.png)

[github地址](https://github.com/deps-rs/deps.rs)

### The Unsafe Chronicles: Exhibit A: Aliasing Boxes

这是`Crust of Rust`作者的最新一系列视频. 在这一系列中, 作者会带领大家深入到  `Unsafe Rust`中.  
本视频是这一系列的第一章, 附该视频的关键时间节点:

-   `0:00:00`  About this new series
-   `0:01:48`  left-right and evmap
-   `0:06:31`  Expressing the problem in code
-   `0:15:28`  Why is aliasing a Box not okay?
-   `0:19:09`  Aliasing Box with MaybeUninit
-   `0:39:48`  Dropping aliases
-   `0:55:15`  The cast is unsound
-   `1:00:45`  Making the cast sound
-   `1:10:47`  Q&A
-   `1:26:43`  Unsoundness from non-determinism
-   `1:28:25`  More Q&A
-   `1:35:14`  Outroduction

[油管地址](https://www.youtube.com/watch?v=EY7Wi9fV5bk)

### chromiumoxide: 通过 DevTools Protocal 控制 chrome/chromium

chromiumoxide 提供了 high-level 控制 Chorme 或者 Chromium 的能力, 几乎支持所有的  `DevTools Protocal`  协议中的类型. 通过该  `crate`, 可以轻松启动或者连接到到一个  `headless`  或者  `non-headless`  的 Chrome 或者 Chromium 实例.

可以非常方便的结合 Chrome 的 DevTools 来做一些 Rust 的相关工具, 如性能调试工具等.

[github地址](https://github.com/mattsse/chromiumoxide)


### MYOX: Javascript打包器

你听过/用过Webpack，Bable么？想了解他们背后的基本思想吗？

在这篇博客文章中，我将创建Javascript bundler，并演示如何在这里利用像Bable这样的东西。但我们不会用"Bable"。在Rust生态系统中，我们有我们自己的，叫做swc。

[Github 链接](https://kakoc.blog/blog/myox-js-bundler/)，https://kakoc.blog/blog/myox-js-bundler/

### 《Rust Servers, Services, and Apps》- Manning新出的关于Rust web开发方面的书

Manning新出了一本Rust书，最近在MEAP阶段，原价$47.99，现折扣价$23.99。感兴趣的朋友可以关注一下。

简介：

> `Rust Servers, Services, and Apps`  is a hands-on guide to developing modern distributed web applications with Rust. You’ll learn how to build efficient services, write custom web servers, and even build full-stack applications end-to-end in Rust.

链接：[https://www.manning.com/books/rust-servers-services-and-apps](https://www.manning.com/books/rust-servers-services-and-apps)

# 嵌入式中的Rust async体验

链接：[https://blog.drogue.io/rust-and-async/](https://blog.drogue.io/rust-and-async/)

### dhat - 监控你的Rust程序堆内存使用情况的crate

dhat是《Rust性能之书》作者新发布的crate，帮助大家只用几行代码就可以监控rust程序的堆内存使用情况。

它提供了两种监控方式：

-   heap profiling

```
use dhat::{Dhat, DhatAlloc};

// 使用dhat提供的DhatAlloc
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    // 在main函数最开头调用这个方法
    let _dhat = Dhat::start_heap_profiling();
}

```

-   ad hoc profiling

```
use dhat::Dhat;

fn main() {
    let _dhat = Dhat::start_ad_hoc_profiling();

}

```

```
    dhat::ad_hoc_event(100);

```

运行之后分别会生成dhat-heap.json 或 dhat-ad-hoc.json两个json文件，需要使用Valgrind来查看。

链接：[https://docs.rs/dhat/0.2.0/dhat/](https://docs.rs/dhat/0.2.0/dhat/)

### Measuring Memory Usage in Rust

rust-analyzer的作者写了一篇文章讲了如何监控Rust程序内存使用情况的几种方法。

链接：[https://rust-analyzer.github.io/blog/2020/12/04/measuring-memory-usage-in-rust.html](https://rust-analyzer.github.io/blog/2020/12/04/measuring-memory-usage-in-rust.html)

### DeisLabs 使用 Rust 一年后总结：Still Rusting

微软Azure的 DeisLabs 团队在去年使用 Rust 开发了 Krustlet 概念验证型项目，基于 wasmtime 实现的 wasm-wasi 版的 Kubelet，实现了在K8S中运行 wasm 负载。

一年后该团队写这篇文章，总结了 Rust 的Good、Bad和Ugly 三方面。

Good：

-   traits： 帮助你设计可扩展的API
-   第三方库 serde：超优秀
-   错误处理、Option 和 迭代器
-   Enum的表现力和便利性
-   宏
-   Cargo
-   强制性的 Unsafe 标记不安全的边界

Bad：

-   生态系统中crate的文档，虽然指明了功能使用方法，但不够清晰，还必须查看源码才能发现是否真的是零成本抽象，或者有没有副作用。库作者在文档上还需要多花点心思。
-   生态系统中很多crate 功能并不完善。
-   操作实现同一个trait的集合类型有点烦人，比如这篇文章里的例子：https://deislabs.io/posts/a-fistful-of-states/
-   发现Rust 学习曲线有两种：上手 Rust 、 设计合适的 API （特别是使用泛型和trait的时候）
-   宏有时候会导致奇怪或难以解释的错误，编译器标记发生错误的地方和实际发生错误的地方并不一致
-   有些trait可能被使用过度（个人偏好）
-   Cargo.toml 中 dependency features可以配置的更具体更易于调试

Ugly：

-   异步 （async/await）

该团队对Rust异步不满意的地方在于：

-   异步运行时生态有竞争性问题（async-std、tokio等）
-   到处都是复杂的不透明的返回类型，比如impl Future<Item = ...>
-   async trait 有持续的需求，但是没有内置到语言中
-   为手动实现诸如AsyncRead之类编写的大量样板代码

认为Rust异步目前对于新手不是很友好，在开发体验上还有更大的改进空间。

总结：

Rust 提升了工作乐趣，强烈推荐将 Rust 用于云原生应用。

附录：

DeisLabs 团队 对 Go 的看法：

「对于需要快速，轻松编写的小型项目，Go是一个很好的工具，但是由于大型项目的维护难度，我们通常避免使用它。 在因err！= nil而导致的膨胀，缺少集合功能（如映射和过滤），缺少泛型以及低于标准的依赖管理解决方案之间，随着项目规模的增加，维护变得越来越困难。」

链接：[https://deislabs.io/posts/still-rusting-one-year-later/](https://deislabs.io/posts/still-rusting-one-year-later/)

### 更快的集成测试

本文记录了一个可以加快集成测试的一个方法. 如果你也有很多集成测试需要跑,可以参考一下.

[原文链接](https://rune-rs.github.io/posts/faster-tests/)

### Monads and GATs in nightly Rust

本文是受 reddit 的  `GATs on Nightly`  这篇 POST 的启发而写. 但是作者更加深入的讨论一些关于 GATs 的可能性和非可能性.

[原文链接](https://www.fpcomplete.com/blog/monads-gats-nightly-rust/)


### rust-analyzer 测量 Rust 的内存使用情况

这篇文章记录了在 rust-analyzer 中用于测量内存消耗的一些有趣技巧。通常，有两种广泛的方法可以分析程序的内存使用情况。第一种方法是基于堆的解析（heap parsing），第二种方法是基于对分配和释放进程的调用检测。详细可以参见原文。

文章[链接](https://rust-analyzer.github.io/blog/2020/12/04/measuring-memory-usage-in-rust.html)，https://rust-analyzer.github.io/blog/2020/12/04/measuring-memory-usage-in-rust.html


### RAUI，将 UI 视为另一种数据

RAUI，受到 React 声明式 UI 组成和 UE4 Slate 小部件组件系统的极大启发。RAUI 体系结构的主要思想是将 UI 视为另一种数据，你可以将其转换为目标呈现引擎选择的目标可呈现数据格式。

Application 的示例：

```
let mut application = Application::new();
let tree = widget! {
    (app {
        title = (title_bar: {"Hello".to_owned()})
        content = (vertical_box [
            (#{"hi"} button: {"Say hi!".to_owned()})
            (#{"exit"} button: {"Close".to_owned()})
        ])
    })
};
let mut renderer = HtmlRenderer::default();
application.apply(tree);
if let Ok(output) = application.render(&mut renderer) {
    println!("{}", output);
}

```

Github  [链接](https://github.com/PsichiX/raui)，https://github.com/PsichiX/raui

### 200 行 Rust 代码解释 Futures

本书旨在通过示例驱动的方法来解释 Rust 中的 Futures，探索为什么它们被设计成这样，以及它们是如何工作的。还介绍了一些在编程中处理并发时的替代方案。

英文原版[链接](https://cfsamson.github.io/books-futures-explained/introduction.html)，https://cfsamson.github.io/books-futures-explained/introduction.html

中文版本[链接](https://stevenbai.top/rust/futures_explained_in_200_lines_of_rust/)，https://stevenbai.top/rust/futures_explained_in_200_lines_of_rust/


### 最后，小编私货

【翻译】编写Rust的23条通用规则及建议

[Read More](https://rustcc.cn/article?id=b9c63f24-4aab-4d16-976b-30146efdfa48): https://rustcc.cn/article?id=b9c63f24-4aab-4d16-976b-30146efdfa48


### GATs(generic_associated_types)已经可以在 nightly 中使用

例子如下：

```
#![feature(generic_associated_types)]

trait Monad /* : Applicative (for pure/return, doesn't matter for this example) */ {
    // Self is like the "f a" in haskell

    /// extract the "a" from "f a"
    type Unplug;

    /// exchange the "a" in "f a" in the type of Self with B
    type Plug<B>: Monad;

    fn bind<B, F>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unplug) -> Self::Plug<B>;
}

impl<A> Monad for Option<A> {
    type Unplug = A;
    type Plug<B> = Option<B>;
    fn bind<B, F>(self, f: F) -> Option<B>
    where
        F: Fn(A) -> Option<B>,
    {
        self.and_then(f)
    }
}

fn main() {
    let x = Some(1).bind(|x| Some(x * 2));
    println!("{:?}", x);
}


```

[https://www.reddit.com/r/rust/comments/k4vzvp/gats_on_nightly/](https://www.reddit.com/r/rust/comments/k4vzvp/gats_on_nightly/)

### Bevy cookbook

要先煮菜，先要会切菜—— Cookbook 里面的就是教会大家如何用 Bevy 这把菜刀来切菜:

-   键盘输出的处理
-   应用 Esc 事件响应
-   2D/3D 世界坐标与屏幕坐标的转换
-   鼠标转向跟进
-   自定义相机映射
-   平面/轨道摄像机

[repo](https://github.com/jamadazi/bevy-cookbook)

### awesome-bevy

另外一些和 bevy相关的资料与demo集合下一篇新闻的原文也有收录哦

[repo](https://github.com/bevyengine/awesome-bevy)

### 用 Bevy 游戏引擎编写贪吃蛇（译）

不过小编建议中英文对照着看，毕竟中文全是意译，可能有些说法通顺但是没有表达清楚原文的意思.

[read](https://huangjj27.github.io/snake-with-bevy.html)

### Bevy 国际象棋

又是另外一篇用 Bevy 来做出来的小游戏，还是 3D的棋子哦~

[read](https://caballerocoll.com/blog/bevy-chess-tutorial/)
