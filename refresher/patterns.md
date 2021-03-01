# patterns


## 


---
[patterns](https://rust-unofficial.github.io/patterns/intro.html)
[more github](https://github.com/rust-unofficial/patterns)


GoF 的 23 种设计模式的分类，现在对各个模式的功能进行介绍。

1.  单例（Singleton）模式：某个类只能生成一个实例，该类提供了一个全局访问点供外部获取该实例，其拓展是有限多例模式。
2.  原型（Prototype）模式：将一个对象作为原型，通过对其进行复制而克隆出多个和原型类似的新实例。
3.  工厂方法（Factory Method）模式：定义一个用于创建产品的接口，由子类决定生产什么产品。
4.  抽象工厂（AbstractFactory）模式：提供一个创建产品族的接口，其每个子类可以生产一系列相关的产品。
5.  建造者（Builder）模式：将一个复杂对象分解成多个相对简单的部分，然后根据不同需要分别创建它们，最后构建成该复杂对象。
6.  代理（Proxy）模式：为某对象提供一种代理以控制对该对象的访问。即客户端通过代理间接地访问该对象，从而限制、增强或修改该对象的一些特性。
7.  适配器（Adapter）模式：将一个类的接口转换成客户希望的另外一个接口，使得原本由于接口不兼容而不能一起工作的那些类能一起工作。
8.  桥接（Bridge）模式：将抽象与实现分离，使它们可以独立变化。它是用组合关系代替继承关系来实现，从而降低了抽象和实现这两个可变维度的耦合度。
9.  装饰（Decorator）模式：动态的给对象增加一些职责，即增加其额外的功能。
10.  外观（Facade）模式：为多个复杂的子系统提供一个一致的接口，使这些子系统更加容易被访问。
11.  享元（Flyweight）模式：运用共享技术来有效地支持大量细粒度对象的复用。
12.  组合（Composite）模式：将对象组合成树状层次结构，使用户对单个对象和组合对象具有一致的访问性。
13.  模板方法（TemplateMethod）模式：定义一个操作中的算法骨架，而将算法的一些步骤延迟到子类中，使得子类可以不改变该算法结构的情况下重定义该算法的某些特定步骤。
14.  策略（Strategy）模式：定义了一系列算法，并将每个算法封装起来，使它们可以相互替换，且算法的改变不会影响使用算法的客户。
15.  命令（Command）模式：将一个请求封装为一个对象，使发出请求的责任和执行请求的责任分割开。
16.  职责链（Chain of Responsibility）模式：把请求从链中的一个对象传到下一个对象，直到请求被响应为止。通过这种方式去除对象之间的耦合。
17.  状态（State）模式：允许一个对象在其内部状态发生改变时改变其行为能力。
18.  观察者（Observer）模式：多个对象间存在一对多关系，当一个对象发生改变时，把这种改变通知给其他多个对象，从而影响其他对象的行为。
19.  中介者（Mediator）模式：定义一个中介对象来简化原有对象之间的交互关系，降低系统中对象间的耦合度，使原有对象之间不必相互了解。
20.  迭代器（Iterator）模式：提供一种方法来顺序访问聚合对象中的一系列数据，而不暴露聚合对象的内部表示。
21.  访问者（Visitor）模式：在不改变集合元素的前提下，为一个集合中的每个元素提供多种访问方式，即每个元素有多个访问者对象访问。
22.  备忘录（Memento）模式：在不破坏封装性的前提下，获取并保存一个对象的内部状态，以便以后恢复它。
23.  解释器（Interpreter）模式：提供如何定义语言的文法，以及对语言句子的解释方法，即解释器。


---


# [建造者模式（Builder)](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E5%BB%BA%E9%80%A0%E8%80%85%E6%A8%A1%E5%BC%8Fbuilder)

## [概述](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E6%A6%82%E8%BF%B0)

构建者模式是一种设计模式，提供一种灵活的解决方案，已解决面向对象程序设计中的各种对象创建问题。Builder设计模式的目的是将复杂对象的构造与其表示分离开来。是"是四人帮"设计模式之一[wiki]。建造者模式是一种创建型设计模式，使你能够分步骤创建复杂对象。该模式允许你使用相同的创建代码生成不同类型和形式的对象。

定义：Builder设计模式的目的是将复杂对象的构造与其表示分离开来。通过这样做，同样的构造过程可以创建不同的表示。

## [历史](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E5%8E%86%E5%8F%B2)

假如有一个复杂的对象，需要对其进行构造时需要对诸多成员变量和嵌套对象进行繁杂的初始化工作。有时这些初始化代码通常深藏于一个包含众多参数且让人看不懂的构造函数中；或者这些代码散落在客户端代码的多个位置。

1.  例如，创建一个房子，不同种类的房子有不同的风格，为每一种类型的房子创建一个子类，这可能会导致程序变得过于复杂。
2.  或者无需生成子类，但是需要创建一个包括所有可能参数的超级构造函数，并用它来控制房屋对象的创建。这样虽然可以避免生成子类，但是会造成当拥有大量输入参数的构造函数不是每次都要全部用上。通常情况下，绝大部分的参数都没有使用，这使得对于构造函数的调用十分不简洁。

## [建造者模式 的使用](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E5%BB%BA%E9%80%A0%E8%80%85%E6%A8%A1%E5%BC%8F-%E7%9A%84%E4%BD%BF%E7%94%A8)

建造者模式建议将对象构造的代码从产品类中抽取出来，并将其放在一个名为生成器的独立对象中。每次创建对象时，都需要通过生成器对象执行一系列步骤。重点在于无需调用所有步骤，而只需调用创建特定对象配置所需的那些步骤。

## [适用场景](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E9%80%82%E7%94%A8%E5%9C%BA%E6%99%AF)

-   使用建造者设计模式可以避免“重叠构造函数”的出现。
    -   假设复杂函数中有十几个可选参数，那么调用这些函数会非常不方便，因此需要重载这个构造函数，新建几个只有较少参数的简化版本。
    -   建造者设计模式让你可以分步骤生成对象，而且允许你仅适用必须的步骤。
-   当使用代码创建不同形式的产品时，可使用生成器模式
    -   如果你需要创建各种形式的产品，他们的制造过程相似且仅有细节上的差异，此时可使用生成器模式。
    -   基本生成器接口中定义了所有可能的制造步骤，具体生成器将实现这些步骤来制造特定形式的产品。
-   使用构造者模式构造其他复杂对象
    -   构造者模式让你能分步骤构造产品，你可以延迟执行某些步骤而不会影响最终产品。

## [优点](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E4%BC%98%E7%82%B9)

-   可以分步骤创建对象，暂缓创建步骤或者递归运行创建步骤。
-   生成不同形式的产品，你可以复用相同的制造代码
-   单一职责原则，可以将复杂构造代码从产品的业务逻辑中分离出来。

## [缺点](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E7%BC%BA%E7%82%B9)

由于该模式需要新增多个类，因此代码整体复杂程度会有所增加。

## [描述](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E6%8F%8F%E8%BF%B0)

通过使用构建者助手创建一个对象。

## [例子](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E4%BE%8B%E5%AD%90)

```rust
fn main() {
    let foo = Foo {
        bar: String::from("Y"),
    };
    let foo_from_builder = FooBuilder::new().name(String::from("Y")).build();
    println!("foo = {:?}", foo);
    println!("foo from builfer = {:?}", foo_from_builder);
}

#[derive(Debug, PartialEq)]
pub struct Foo {
    // lots of complicated fields
    bar : String,
}

pub struct FooBuilder {
    // Probably lots of optional fields.
    bar: String,
}

impl FooBuilder {
    pub fn new() -> Self {
        // set the minimally required fields of Foo.
        Self {
            bar: String::from("x"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        // set the name on the builder iteself,
        // and return the builder by value.
        self.bar = bar;
        self 
    }
    // if we can get away with not consuming the builder here, that is an 
    // advantage. It means we can use the FooBuilder as a template for constructing many Foo.
    pub fn build(self) -> Foo {
        // Create a Foo from Foo the FooBuilder, applying all settings in FooBuilder to Foo. 
        Foo { bar: self.bar }
    }
}

```

```rust
// Rust 编程之道. P234
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0, y: 0.0, radius: 1.0,
        }
    }
}

impl CircleBuilder {
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }
    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }
    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn build(&self) -> Circle {
        Circle {
            x: self.x, y: self.y, radius: self.radius,
        }
    }
}

fn main() {
  let c = Circle::new().x(1.0).y(2.0).radius(2.0).build();
  println!("area = {:?}", c.area());
  println!("c.x = {:?}", c.x);
  println!("c.y = {:?}", c.y);
}

```

## [动机](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E5%8A%A8%E6%9C%BA)

当你需要许多不同的构造函数或者当构造有副作用时，这种方法有用。

## [优点](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E4%BC%98%E7%82%B9-1)

将构造方法与其他方法分离。

防止构造函数的扩散

可用于单次初始化以及更加复杂的构造。

## [缺点](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E7%BC%BA%E7%82%B9-1)

比直接创建结构对象或简单的的构造函数更复杂。

## [讨论](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E8%AE%A8%E8%AE%BA)

这种模式在Rust（以及简单对象）中比在其他许多语言中更常见，这是因为Rust缺乏重载。由于你只能使用给定名称的单个方法，因此在Rust中使用多个构造函数要比C++、Java或其他语言好。

这种模式通常用于构建器对象本身就很有用的地方，而不仅仅是一个构建器。例如：std::process::Command 是Child的构建器。在这种情况下，不使用T和TBuilder的命名模式。

该示例通过值获取并返回生成器。接受并返回构建器作为可变引用通常更符合人体工程学（并且更有效）。

```rust

#![allow(unused)]
fn main() {
let mut fb = FooBuilder::new();
fb.a();
fb.b();
let f = fb.builder();
}

```

以及FooBuilder::new().a().b().builder()样式。

## [参见](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E5%8F%82%E8%A7%81)

-   [Description in the style guide](https://web.archive.org/web/20210104103100/https://doc.rust-lang.org/1.12.0/style/ownership/builders.html)
-   [derive_builder](https://crates.io/crates/derive_builder), a crate for automatically implementing this pattern while avoiding the boilerplate.
-   [Constructor pattern](https://rust-unofficial.github.io/patterns/idioms/ctor.html)  for when construction is simpler.
-   [Builder pattern (wikipedia)](https://en.wikipedia.org/wiki/Builder_pattern)
-   [Construction of complex values](https://web.archive.org/web/20210104103000/https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder)
-   Rust编程之道 ch7,p234

## [项目中的使用](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#%E9%A1%B9%E7%9B%AE%E4%B8%AD%E7%9A%84%E4%BD%BF%E7%94%A8)

### [Tokio 中的建造者模式 Struct](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#tokio-%E4%B8%AD%E7%9A%84%E5%BB%BA%E9%80%A0%E8%80%85%E6%A8%A1%E5%BC%8F-struct-a-hrefhttpsdocsrstokio110tokioindexhtmltokioaa-hrefhttpsdocsrstokio110tokioruntimeindexhtmlruntimeaa-hrefhttpsdocsrstokio110tokioruntimestructbuilderhtmlbuildera) [tokio](https://docs.rs/tokio/1.1.0/tokio/index.html)::[runtime](https://docs.rs/tokio/1.1.0/tokio/runtime/index.html)::[Builder](https://docs.rs/tokio/1.1.0/tokio/runtime/struct.Builder.html)

```rust

#![allow(unused)]
fn main() {
pub struct Builder {
    /// Runtime type
    kind: Kind,

    /// Whether or not to enable the I/O driver
    enable_io: bool,

    /// Whether or not to enable the time driver
    enable_time: bool,

    /// The number of worker threads, used by Runtime.
    ///
    /// Only used when not using the current-thread executor.
    worker_threads: Option<usize>,

    /// Cap on thread usage.
    max_blocking_threads: usize,

    /// Name fn used for threads spawned by the runtime.
    pub(super) thread_name: ThreadNameFn,

    /// Stack size used for threads spawned by the runtime.
    pub(super) thread_stack_size: Option<usize>,

    /// Callback to run after each thread starts.
    pub(super) after_start: Option<Callback>,

    /// To run before each worker thread stops
    pub(super) before_stop: Option<Callback>,

    /// Customizable keep alive timeout for BlockingPool
    pub(super) keep_alive: Option<Duration>,
}

pub fn new_current_thread() -> Builder // 设置current thread 类型
//Returns a new builder with the current thread scheduler selected.
//Configuration methods can be chained on the return value.

pub fn new_multi_thread() -> Builder // 设置 multi thread 类型
//This is supported on crate feature rt-multi-thread only.
//Returns a new builder with the multi thread scheduler selected.
//Configuration methods can be chained on the return value.

pub fn enable_all(&mut self) -> &mut Self
// Enables both I/O and time drivers.
// Doing this is a shorthand for calling enable_io and enable_time individually. If additional components are added to Tokio in the future, enable_all will include these future components.

pub fn worker_threads(&mut self, val: usize) -> &mut Self // 设置的runtime 用于工作的线程数
// Sets the number of worker threads the Runtime will use.
// This should be a number between 0 and 32,768 though it is advised to keep this value on the smaller side.

pub fn max_blocking_threads(&mut self, val: usize) -> &mut Self // 设置生成的用于阻塞操作的线程最大数
//Specifies limit for threads spawned by the Runtime used for blocking operations.
//Similarly to the worker_threads, this number should be between 1 and 32,768.
//The default value is 512.
//Otherwise as worker_threads are always active, it limits additional threads (e.g. for blocking annotations).

pub fn thread_name(&mut self, val: impl Into<String>) -> &mut Self // 设置线程的名字
//Sets name of threads spawned by the Runtime's thread pool.
//The default name is "tokio-runtime-worker".

// .....

pub fn build(&mut self) -> Result<Runtime> // 构造出tokio中的runtime结构
//Creates the configured Runtime.
//The returned Runtime instance is ready to spawn tasks.

//etc..

//example
// build runtime
let runtime = Builder::new_multi_thread()
                .worker_threads(4)
                .thread_name("my-custom-name")
                .thread_stack_size(3 * 1024 * 1024)
                .build()
                .unwrap();
}

```

从Builder的build函数可以知道Builder结构是Runtime的辅助结构体用来帮助构造Runtime的。

## [Futures 中的建造者设计模式 Struct](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#futures-%E4%B8%AD%E7%9A%84%E5%BB%BA%E9%80%A0%E8%80%85%E8%AE%BE%E8%AE%A1%E6%A8%A1%E5%BC%8F-struct-a-hrefhttpsdocsrsfutures0312futuresindexhtmlfuturesaa-hrefhttpsdocsrsfutures0312futuresexecutorindexhtmlexecutoraa-hrefhttpsdocsrsfutures0312futuresexecutorstructthreadpoolbuilderhtmlthreadpoolbuildera) [futures](https://docs.rs/futures/0.3.12/futures/index.html)::[executor](https://docs.rs/futures/0.3.12/futures/executor/index.html)::[ThreadPoolBuilder](https://docs.rs/futures/0.3.12/futures/executor/struct.ThreadPoolBuilder.html)

```rust

#![allow(unused)]
fn main() {
/// A general-purpose thread pool for scheduling tasks that poll futures to
/// completion.
///
/// The thread pool multiplexes any number of tasks onto a fixed number of
/// worker threads.
///
/// This type is a clonable handle to the threadpool itself.
/// Cloning it will only create a new reference, not a new threadpool.
///
/// This type is only available when the `thread-pool` feature of this
/// library is activated.
#[cfg_attr(docsrs, doc(cfg(feature = "thread-pool")))]
pub struct ThreadPool {
    state: Arc<PoolState>,
}

/// Thread pool configuration object.
///
/// This type is only available when the `thread-pool` feature of this
/// library is activated.
#[cfg_attr(docsrs, doc(cfg(feature = "thread-pool")))]
pub struct ThreadPoolBuilder {
    pool_size: usize,
    stack_size: usize,
    name_prefix: Option<String>,
    after_start: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    before_stop: Option<Arc<dyn Fn(usize) + Send + Sync>>,
}


struct PoolState {
    tx: Mutex<mpsc::Sender<Message>>,
    rx: Mutex<mpsc::Receiver<Message>>,
    cnt: AtomicUsize,
    size: usize,
}

enum Message {
    Run(Task),
    Close,
}

impl ThreadPoolBuilder {
    /// Create a default thread pool configuration.
    ///
    /// See the other methods on this type for details on the defaults.
    pub fn new() -> Self {
        Self {
            pool_size: cmp::max(1, num_cpus::get()),
            stack_size: 0,
            name_prefix: None,
            after_start: None,
            before_stop: None,
        }
    }

    /// Set size of a future ThreadPool
    ///
    /// The size of a thread pool is the number of worker threads spawned. By
    /// default, this is equal to the number of CPU cores.
    ///
    /// # Panics
    ///
    /// Panics if `pool_size == 0`.
    pub fn pool_size(&mut self, size: usize) -> &mut Self {
        assert!(size > 0);
        self.pool_size = size;
        self
    }

    /// Set stack size of threads in the pool, in bytes.
    ///
    /// By default, worker threads use Rust's standard stack size.
    pub fn stack_size(&mut self, stack_size: usize) -> &mut Self {
        self.stack_size = stack_size;
        self
    }

    /// Set thread name prefix of a future ThreadPool.
    ///
    /// Thread name prefix is used for generating thread names. For example, if prefix is
    /// `my-pool-`, then threads in the pool will get names like `my-pool-1` etc.
    ///
    /// By default, worker threads are assigned Rust's standard thread name.
    pub fn name_prefix<S: Into<String>>(&mut self, name_prefix: S) -> &mut Self {
        self.name_prefix = Some(name_prefix.into());
        self
    }

    /// Execute the closure `f` immediately after each worker thread is started,
    /// but before running any tasks on it.
    ///
    /// This hook is intended for bookkeeping and monitoring.
    /// The closure `f` will be dropped after the `builder` is dropped
    /// and all worker threads in the pool have executed it.
    ///
    /// The closure provided will receive an index corresponding to the worker
    /// thread it's running on.
    pub fn after_start<F>(&mut self, f: F) -> &mut Self
        where F: Fn(usize) + Send + Sync + 'static
    {
        self.after_start = Some(Arc::new(f));
        self
    }

    /// Execute closure `f` just prior to shutting down each worker thread.
    ///
    /// This hook is intended for bookkeeping and monitoring.
    /// The closure `f` will be dropped after the `builder` is droppped
    /// and all threads in the pool have executed it.
    ///
    /// The closure provided will receive an index corresponding to the worker
    /// thread it's running on.
    pub fn before_stop<F>(&mut self, f: F) -> &mut Self
        where F: Fn(usize) + Send + Sync + 'static
    {
        self.before_stop = Some(Arc::new(f));
        self
    }
	
  	// 从ThreadBuilder的create函数可以看到ThreadPoolBuilder根据配置采纳数创建ThreadPool， 是ThreadPool的辅助结构体 
    /// Create a [`ThreadPool`](ThreadPool) with the given configuration.
    pub fn create(&mut self) -> Result<ThreadPool, io::Error> {
        let (tx, rx) = mpsc::channel();
        let pool = ThreadPool {
            state: Arc::new(PoolState {
                tx: Mutex::new(tx),
                rx: Mutex::new(rx),
                cnt: AtomicUsize::new(1),
                size: self.pool_size,
            }),
        };

        for counter in 0..self.pool_size {
            let state = pool.state.clone();
            let after_start = self.after_start.clone();
            let before_stop = self.before_stop.clone();
            let mut thread_builder = thread::Builder::new();
            if let Some(ref name_prefix) = self.name_prefix {
                thread_builder = thread_builder.name(format!("{}{}", name_prefix, counter));
            }
            if self.stack_size > 0 {
                thread_builder = thread_builder.stack_size(self.stack_size);
            }
            thread_builder.spawn(move || state.work(counter, after_start, before_stop))?;
        }
        Ok(pool)
    }
}
}

```

从ThreadBuilder的create函数可以看到ThreadPoolBuilder根据配置采纳数创建ThreadPool， 是ThreadPool的辅助结构体

## [Surf中的建造者设计模式](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#surf%E4%B8%AD%E7%9A%84%E5%BB%BA%E9%80%A0%E8%80%85%E8%AE%BE%E8%AE%A1%E6%A8%A1%E5%BC%8F)

```rust
/// Request Builder
///
/// Provides an ergonomic way to chain the creation of a request.
/// This is generally accessed as the return value from `surf::{method}()`,
/// however [`Request::builder`](crate::Request::builder) is also provided.
///
/// # Examples
///
/// ```rust
/// use surf::http::{Method, mime::HTML, Url};
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let mut request = surf::post("https://httpbin.org/post")
///     .body("<html>hi</html>")
///     .header("custom-header", "value")
///     .content_type(HTML)
///     .build();
///
/// assert_eq!(request.take_body().into_string().await.unwrap(), "<html>hi</html>");
/// assert_eq!(request.method(), Method::Post);
/// assert_eq!(request.url(), &Url::parse("https://httpbin.org/post")?);
/// assert_eq!(request["custom-header"], "value");
/// assert_eq!(request["content-type"], "text/html;charset=utf-8");
/// # Ok(())
/// # }
/// ```
///
/// ```rust
/// use surf::http::{Method, Url};
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let url = Url::parse("https://httpbin.org/post")?;
/// let request = surf::Request::builder(Method::Post, url).build();
/// # Ok(())
/// # }
/// ```

pub struct RequestBuilder {
    /// Holds the state of the request.
    req: Option<Request>,
    /// Hold an optional Client.
    client: Option<Client>,
    /// Holds the state of the `impl Future`.
    fut: Option<BoxFuture<'static, Result<Response>>>,
}

impl RequestBuilder {
    /// Create a new instance.
    ///
    /// This method is particularly useful when input URLs might be passed by third parties, and
    /// you don't want to panic if they're malformed. If URLs are statically encoded, it might be
    /// easier to use one of the shorthand methods instead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// use surf::http::{Method, Url};
    ///
    /// let url = Url::parse("https://httpbin.org/get")?;
    /// let req = surf::RequestBuilder::new(Method::Get, url).build();
    /// # Ok(()) }
    /// ```
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            req: Some(Request::new(method, url)),
            client: None,
            fut: None,
        }
    }

    pub(crate) fn with_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Sets a header on the request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = surf::get("https://httpbin.org/get").header("header-name", "header-value").build();
    /// assert_eq!(req["header-name"], "header-value");
    /// ```
    pub fn header(mut self, key: impl Into<HeaderName>, value: impl ToHeaderValues) -> Self {
        self.req.as_mut().unwrap().insert_header(key, value);
        self
    }

    /// Sets the Content-Type header on the request.
    ///
    /// # Examples
    ///
    /// ```
    /// # use surf::http::mime;
    /// let req = surf::post("https://httpbin.org/post").content_type(mime::HTML).build();
    /// assert_eq!(req["content-type"], "text/html;charset=utf-8");
    /// ```
    pub fn content_type(mut self, content_type: impl Into<Mime>) -> Self {
        self.req
            .as_mut()
            .unwrap()
            .set_content_type(content_type.into());
        self
    }

    /// Sets the body of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// use serde_json::json;
    /// let mut req = surf::post("https://httpbin.org/post").body(json!({ "any": "Into<Body>"})).build();
    /// assert_eq!(req.take_body().into_string().await.unwrap(), "{\"any\":\"Into<Body>\"}");
    /// # Ok(())
    /// # }
    /// ```
    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.req.as_mut().unwrap().set_body(body);
        self
    }

    /// Set the URL querystring.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// #[derive(Serialize, Deserialize)]
    /// struct Index {
    ///     page: u32
    /// }
    ///
    /// let query = Index { page: 2 };
    /// let mut req = surf::get("https://httpbin.org/get").query(&query)?.build();
    /// assert_eq!(req.url().query(), Some("page=2"));
    /// assert_eq!(req.url().as_str(), "https://httpbin.org/get?page=2");
    /// # Ok(()) }
    /// ```
    pub fn query(mut self, query: &impl Serialize) -> std::result::Result<Self, Error> {
        self.req.as_mut().unwrap().set_query(query)?;

        Ok(self)
    }

    /// Submit the request and get the response body as bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// let bytes = surf::get("https://httpbin.org/get").recv_bytes().await?;
    /// assert!(bytes.len() > 0);
    /// # Ok(()) }
    /// ```
    pub async fn recv_bytes(self) -> Result<Vec<u8>> {
        let mut res = self.send().await?;
        Ok(res.body_bytes().await?)
    }

    /// Submit the request and get the response body as a string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// let string = surf::get("https://httpbin.org/get").recv_string().await?;
    /// assert!(string.len() > 0);
    /// # Ok(()) }
    /// ```
    pub async fn recv_string(self) -> Result<String> {
        let mut res = self.send().await?;
        Ok(res.body_string().await?)
    }

    /// Submit the request and decode the response body from json into a struct.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// #[derive(Deserialize, Serialize)]
    /// struct Ip {
    ///     ip: String
    /// }
    ///
    /// let uri = "https://api.ipify.org?format=json";
    /// let Ip { ip } = surf::get(uri).recv_json().await?;
    /// assert!(ip.len() > 10);
    /// # Ok(()) }
    /// ```
    pub async fn recv_json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let mut res = self.send().await?;
        Ok(res.body_json::<T>().await?)
    }

    /// Submit the request and decode the response body from form encoding into a struct.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// If the body cannot be interpreted as valid json for the target type `T`,
    /// an `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// #[derive(Deserialize, Serialize)]
    /// struct Body {
    ///     apples: u32
    /// }
    ///
    /// let url = "https://api.example.com/v1/response";
    /// let Body { apples } = surf::get(url).recv_form().await?;
    /// # Ok(()) }
    /// ```
    pub async fn recv_form<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let mut res = self.send().await?;
        Ok(res.body_form::<T>().await?)
    }
		
  	// 从build函数可以知道最后RequestBuilder是Request的辅助结构体，用来构造返回Request
  	// 这个函数返回的是Request
    /// Return the constructed `Request`.
    pub fn build(self) -> Request {
        self.req.unwrap()
    }

    /// Create a `Client` and send the constructed `Request` from it.
    pub async fn send(mut self) -> Result<Response> {
        self.client
            .take()
            .unwrap_or_else(Client::new_shared_or_panic)
            .send(self.build())
            .await
    }
}

```

从build函数可以知道最后RequestBuilder是Request的辅助结构体，用来构造返回Request

## [Reqwest中的建造者设计模式](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust-design-patterns/builder.html#reqwest%E4%B8%AD%E7%9A%84%E5%BB%BA%E9%80%A0%E8%80%85%E8%AE%BE%E8%AE%A1%E6%A8%A1%E5%BC%8F)

```rust

#![allow(unused)]
fn main() {
/// A request which can be executed with `Client::execute()`.
pub struct Request {
    method: Method,
    url: Url,
    headers: HeaderMap,
    body: Option<Body>,
    timeout: Option<Duration>,
}

/// A builder to construct the properties of a `Request`.
///
/// To construct a `RequestBuilder`, refer to the `Client` documentation.
#[must_use = "RequestBuilder does nothing until you 'send' it"]
pub struct RequestBuilder {
    client: Client,
    request: crate::Result<Request>,
}

impl Request {
    /// Constructs a new request.
    #[inline]
    pub fn new(method: Method, url: Url) -> Self {
        Request {
            method,
            url,
            headers: HeaderMap::new(),
            body: None,
            timeout: None
        }
    }

    /// Get the method.
    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get a mutable reference to the method.
    #[inline]
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    /// Get the url.
    #[inline]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get a mutable reference to the url.
    #[inline]
    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }

    /// Get the headers.
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get a mutable reference to the headers.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Get the body.
    #[inline]
    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    /// Get a mutable reference to the body.
    #[inline]
    pub fn body_mut(&mut self) -> &mut Option<Body> {
        &mut self.body
    }

    /// Get the timeout.
    #[inline]
    pub fn timeout(&self) -> Option<&Duration> {
        self.timeout.as_ref()
    }

    /// Get a mutable reference to the timeout.
    #[inline]
    pub fn timeout_mut(&mut self) -> &mut Option<Duration> {
        &mut self.timeout
    }

    /// Attempt to clone the request.
    ///
    /// `None` is returned if the request can not be cloned, i.e. if the body is a stream.
    pub fn try_clone(&self) -> Option<Request> {
        let body = match self.body.as_ref() {
            Some(ref body) => Some(body.try_clone()?),
            None => None,
        };
        let mut req = Request::new(self.method().clone(), self.url().clone());
        *req.timeout_mut() = self.timeout().cloned();
        *req.headers_mut() = self.headers().clone();
        req.body = body;
        Some(req)
    }

    pub(super) fn pieces(self) -> (Method, Url, HeaderMap, Option<Body>, Option<Duration>) {
        (self.method, self.url, self.headers, self.body, self.timeout)
    }
}

impl RequestBuilder {
    pub(super) fn new(client: Client, request: crate::Result<Request>) -> RequestBuilder {
        let mut builder = RequestBuilder { client, request };

        let auth = builder
            .request
            .as_mut()
            .ok()
            .and_then(|req| extract_authority(&mut req.url));

        if let Some((username, password)) = auth {
            builder.basic_auth(username, password)
        } else {
            builder
        }
    }

    /// Add a `Header` to this Request.
    pub fn header<K, V>(self, key: K, value: V) -> RequestBuilder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>, 
    {
        self.header_sensitive(key, value, false)
    }

    /// Add a `Header` to this Request with ability to define if header_value is sensitive.
    fn header_sensitive<K, V>(mut self, key: K, value: V, sensitive: bool) -> RequestBuilder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        let mut error = None;
        if let Ok(ref mut req) = self.request {
            match <HeaderName as TryFrom<K>>::try_from(key) {
                Ok(key) => match <HeaderValue as TryFrom<V>>::try_from(value) {
                    Ok(mut value) => {
                        value.set_sensitive(sensitive);
                        req.headers_mut().append(key, value);
                    }
                    Err(e) => error = Some(crate::error::builder(e.into())),
                },
                Err(e) => error = Some(crate::error::builder(e.into())),
            };
        }
        if let Some(err) = error {
            self.request = Err(err);
        }
        self
    }

    /// Add a set of Headers to the existing ones on this Request.
    ///
    /// The headers will be merged in to any already set.
    pub fn headers(mut self, headers: crate::header::HeaderMap) -> RequestBuilder {
        if let Ok(ref mut req) = self.request {
            crate::util::replace_headers(req.headers_mut(), headers);
        }
        self
    }

    /// Enable HTTP basic authentication.
    pub fn basic_auth<U, P>(self, username: U, password: Option<P>) -> RequestBuilder
    where
        U: fmt::Display,
        P: fmt::Display,
    {
        let mut header_value = b"Basic ".to_vec();
        {
            let mut encoder = Base64Encoder::new(&mut header_value, base64::STANDARD);
            // The unwraps here are fine because Vec::write* is infallible.
            write!(encoder, "{}:", username).unwrap();
            if let Some(password) = password {
                write!(encoder, "{}", password).unwrap();
            }
        }

        self.header_sensitive(crate::header::AUTHORIZATION, header_value, true)
    }

    /// Enable HTTP bearer authentication.
    pub fn bearer_auth<T>(self, token: T) -> RequestBuilder
    where
        T: fmt::Display,
    {
        let header_value = format!("Bearer {}", token);
        self.header_sensitive(crate::header::AUTHORIZATION, header_value, true)
    }

    /// Set the request body.
    pub fn body<T: Into<Body>>(mut self, body: T) -> RequestBuilder {
        if let Ok(ref mut req) = self.request {
            *req.body_mut() = Some(body.into());
        }
        self
    }

    /// Enables a request timeout.
    ///
    /// The timeout is applied from when the request starts connecting until the
    /// response body has finished. It affects only this request and overrides
    /// the timeout configured using `ClientBuilder::timeout()`.
    pub fn timeout(mut self, timeout: Duration) -> RequestBuilder {
        if let Ok(ref mut req) = self.request {
            *req.timeout_mut() = Some(timeout);
        }
        self
    }

    /// Sends a multipart/form-data body.
    ///
    /// ```
    /// # use reqwest::Error;
    ///
    /// # async fn run() -> Result<(), Error> {
    /// let client = reqwest::Client::new();
    /// let form = reqwest::multipart::Form::new()
    ///     .text("key3", "value3")
    ///     .text("key4", "value4");
    ///
    ///
    /// let response = client.post("your url")
    ///     .multipart(form)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "multipart")]
    pub fn multipart(self, mut multipart: multipart::Form) -> RequestBuilder {
        let mut builder = self.header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", multipart.boundary()).as_str(),
        );

        builder = match multipart.compute_length() {
            Some(length) => builder.header(CONTENT_LENGTH, length),
            None => builder,
        };

        if let Ok(ref mut req) = builder.request {
            *req.body_mut() = Some(multipart.stream())
        }
        builder
    }

    /// Modify the query string of the URL.
    ///
    /// Modifies the URL of this request, adding the parameters provided.
    /// This method appends and does not overwrite. This means that it can
    /// be called multiple times and that existing query parameters are not
    /// overwritten if the same key is used. The key will simply show up
    /// twice in the query string.
    /// Calling `.query([("foo", "a"), ("foo", "b")])` gives `"foo=a&foo=b"`.
    ///
    /// # Note
    /// This method does not support serializing a single key-value
    /// pair. Instead of using `.query(("key", "val"))`, use a sequence, such
    /// as `.query(&[("key", "val")])`. It's also possible to serialize structs
    /// and maps into a key-value pair.
    ///
    /// # Errors
    /// This method will fail if the object you provide cannot be serialized
    /// into a query string.
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> RequestBuilder {
        let mut error = None;
        if let Ok(ref mut req) = self.request {
            let url = req.url_mut();
            let mut pairs = url.query_pairs_mut();
            let serializer = serde_urlencoded::Serializer::new(&mut pairs);

            if let Err(err) = query.serialize(serializer) {
                error = Some(crate::error::builder(err));
            }
        }
        if let Ok(ref mut req) = self.request {
            if let Some("") = req.url().query() {
                req.url_mut().set_query(None);
            }
        }
        if let Some(err) = error {
            self.request = Err(err);
        }
        self
    }

    /// Send a form body.
    pub fn form<T: Serialize + ?Sized>(mut self, form: &T) -> RequestBuilder {
        let mut error = None;
        if let Ok(ref mut req) = self.request {
            match serde_urlencoded::to_string(form) {
                Ok(body) => {
                    req.headers_mut().insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/x-www-form-urlencoded"),
                    );
                    *req.body_mut() = Some(body.into());
                }
                Err(err) => error = Some(crate::error::builder(err)),
            }
        }
        if let Some(err) = error {
            self.request = Err(err);
        }
        self
    }

    /// Send a JSON body.
    ///
    /// # Optional
    ///
    /// This requires the optional `json` feature enabled.
    ///
    /// # Errors
    ///
    /// Serialization can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    #[cfg(feature = "json")]
    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> RequestBuilder {
        let mut error = None;
        if let Ok(ref mut req) = self.request {
            match serde_json::to_vec(json) {
                Ok(body) => {
                    req.headers_mut()
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                    *req.body_mut() = Some(body.into());
                }
                Err(err) => error = Some(crate::error::builder(err)),
            }
        }
        if let Some(err) = error {
            self.request = Err(err);
        }
        self
    }

    /// Disable CORS on fetching the request.
    ///
    /// # WASM
    ///
    /// This option is only effective with WebAssembly target.
    ///
    /// The [request mode][mdn] will be set to 'no-cors'.
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Request/mode
    pub fn fetch_mode_no_cors(self) -> RequestBuilder {
        self
    }
  
 
		// 从RequestBuilder的build函数可以知道，RequestBuilder是用来帮助构造Request的辅助结构体
    /// Build a `Request`, which can be inspected, modified and executed with
    /// `Client::execute()`.
    pub fn build(self) -> crate::Result<Request> {
        self.request
    }

    /// Constructs the Request and sends it to the target URL, returning a
    /// future Response.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while sending request,
    /// redirect loop was detected or redirect limit was exhausted.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use reqwest::Error;
    /// #
    /// # async fn run() -> Result<(), Error> {
    /// let response = reqwest::Client::new()
    ///     .get("https://hyper.rs")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn send(self) -> impl Future<Output = Result<Response, crate::Error>> {
        match self.request {
            Ok(req) => self.client.execute_request(req),
            Err(err) => Pending::new_err(err),
        }
    }

    /// Attempt to clone the RequestBuilder.
    ///
    /// `None` is returned if the RequestBuilder can not be cloned,
    /// i.e. if the request body is a stream.
    ///
    /// # Examples
    ///
    /// ```
    /// # use reqwest::Error;
    /// #
    /// # fn run() -> Result<(), Error> {
    /// let client = reqwest::Client::new();
    /// let builder = client.post("http://httpbin.org/post")
    ///     .body("from a &str!");
    /// let clone = builder.try_clone();
    /// assert!(clone.is_some());
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_clone(&self) -> Option<RequestBuilder> {
        self.request
            .as_ref()
            .ok()
            .and_then(|req| req.try_clone())
            .map(|req| RequestBuilder {
                client: self.client.clone(),
                request: Ok(req),
            })
    }
}
}

```

从RequestBuilder的build函数可以知道，RequestBuilder是用来帮助构造Request的辅助结构体。

参考链接：

https://docs.rs/tokio/1.1.0/tokio/runtime/struct.Builder.html

https://docs.rs/reqwest/0.11.0/src/reqwest/async_impl/request.rs.html#36-39

https://github.com/http-rs/surf/blob/31315743b91ff003231183c1ec5a3cd2b698c58a/src/request_builder.rs

https://docs.rs/futures/0.3.12/futures/executor/struct.ThreadPoolBuilder.html

---

