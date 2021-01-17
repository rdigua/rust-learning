
# [Druid](https://linebender.org/druid/#druid)

Druid is a framework for building simple graphical applications.

Druid is composed of a number of related projects.  [`druid-shell`](https://docs.rs/druid-shell)  is a low-level library that provides a common abstraction for interacting with the current OS & window manager.  [`piet`](https://docs.rs/piet)  is an abstraction for doing 2D graphics;  [`kurbo`](https://docs.rs/kurbo)  is a library for 2D geometry; and  [`druid`](https://docs.rs/druid)  itself is an opinionated set of high-level APIs for building cross-platform desktop applications.

Druid is  _data oriented_. It shares many ideas (and is directly inspired by) contemporary declarative UI frameworks such as  [Flutter](https://flutter.dev/),  [Jetpack Compose](https://developer.android.com/jetpack/compose), and  [SwiftUI](https://developer.apple.com/documentation/swiftui), while also attempting to be conceptually simple and largely  _non-magical_. A programmer familiar with Rust should be able to understand how Druid works without special difficulty.

## [Goals and Status](https://linebender.org/druid/#goals-and-status)

The current goal of Druid is to make it easy to write a program in Rust that can present a GUI and accept user input. Running your program should be as simple as  `cargo run`.

## [Key Concepts](https://linebender.org/druid/#key-concepts)

-   **[the  `Data`  trait](https://linebender.org/druid/data.html)**: How you represent your application model.
-   **[the  `Widget`  trait](https://linebender.org/druid/widget.html)**: How you represent your UI.
-   **[the  `Lens`  trait](https://linebender.org/druid/lens.html)**: How you associate parts of your model with parts of your UI.
---
# druid

druid 图形应用程序的框架。

druid 相关项目组成。
druid-shell 一个低级库， 它提供了与当前 Os 和窗口管理器交互的常见抽象。
piet 是用于进行 2D 图形的抽象;
kurbo 是 2D 几何的库;
druid本身就是一组***有意见***的用于构建跨平台桌面应用程序的高 API。

druid面向数据。
它分享了许多想法（并直接受到）当代声明性UI框架的启发，如 Flutter、Jetpack Compose 和 SwiftUI，同时还尝试在概念上简单且基本上非魔法。熟悉 Rust 的程序员应该能够理解druid如何工作，没有特别困难。

目标和状态
Druid 的当前目标是在 Rust 中轻松编写一个可以呈现 GUI 并接受用户输入的程序 程序运行 `Cargo run` 

关键概念
数据特征：如何表示应用程序模型。
小部件特征：如何表示 UI。
镜头特征：如何将模型的某些部分与 UI 的某些部分关联。

---


## [Set up Druid](https://linebender.org/druid/setup.html#set-up-druid)

This tutorial assumes basic familliarity with Rust and a working setup with the basic tooling like Rustup and Cargo. This tutorial will use stable Rust (v1.39.0 at the time of writing) and the latest released version of Druid.

This tutorial will first walk you through setting up the dependencies for developing a Druid application, then it will show you how to set up a basic application, build it and run it.

### [Setting up Druid dependencies](https://linebender.org/druid/setup.html#setting-up-druid-dependencies)

In addition to including the  `druid`  library in your project

#### [Linux](https://linebender.org/druid/setup.html#linux)

On Linux, Druid requires gtk+3.

On Ubuntu this can be installed with

`sudo apt-get install libgtk-3-dev` 

On Fedora

`sudo dnf install gtk3-devel glib2-devel` 

See  [GTK installation page](https://www.gtk.org/docs/installations/linux/)  for more installation instructions.

### [Starting a project](https://linebender.org/druid/setup.html#starting-a-project)

Starting a project is as easy as creating an empty application with

`cargo new my-application` 

and adding the druid dependency to your Cargo.toml

`[dependencies]
druid = "0.7.0"
// or to be on the bleeding edge:
druid = { git = "https://github.com/linebender/druid.git" }`

---

##[设置德鲁伊]（https：//linebender.org/德鲁伊/设置.html#设置德鲁伊）

本教程假设与 Rust 的基本家庭性和工作设置与基本工具，如 Rustup 和货物。本教程将使用rust stable （v1.39.0 ） 最新版本的druid.

本教程将首先引导您完成开发 Druid 应用程序的依赖项设置，然后介绍如何设置基本应用程序、生成并运行它。

##[设置德鲁伊依赖关系]（https：//linebender.org/德鲁伊/设置.html#设置德鲁伊-德鲁伊依赖项）

除了在项目中包括"德鲁伊"库

###Linux]（https：//linebender.org/德鲁伊/设置.html#linux）

Linux 需要 gtk+3。

Ubuntu 安装

```
sudo apt - get 安装 libgtk - 3 - dev
```

Fedora 安装

```
sudo dnf 安装 gtk3 - devel glib2 - devel
```

有关更多安装说明，请参阅 [GTK 安装页面]（https：//www.gtk.org/文档/安装/linux/）。

###[启动项目]（https：//linebender.org/德鲁伊/设置.html#启动项目）

启动项目就像创建空应用程序一样简单。

```
cargo new my-application
```

并添加druid依赖到 Cargo. toml

```
druid = "0.7.0"
或在出血边缘：
druid [git] "https://github.com/linebender/druid.git""
```