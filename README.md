# ![tr-lang](./img/logo/logo.png)
#### <center>Fast and Easy</center>
<hr style="width: 50%;">

### Made with ❤️ in 🇹🇷

tr-lang is a language that aims to bring programming language syntax closer to Turkish.
tr-lang is a stack based language and uses reverse-polish notation for maths.

View in [Turkish](README-TR.md)

# 🚩 Table of Contents
- [🏆 What Has Been Implemented?](#-what-has-been-implemented)
- [🚀 Installation](#-installation)
- [📖 See the Docs](#-see-the-docs)
- [🤝 Contributing](#-contributing)
- [📜 Thanks](#-thanks)

# 🏆 What Has Been Implemented?

## ✔️ All parts of the language seems like they are done!

#### ✔️ tr-lang lexer seems like its done [Issue #1](https://github.com/kaiserthe13th/tr-lang/issues/1#issue-1027652152)<br>
#### ✔️ tr-lang parser is in progress [Issue #2](https://github.com/kaiserthe13th/tr-lang/issues/2#issue-1027660436)<br>
#### ✔️ tr-lang bytecode seems like its done [Issue #3](https://github.com/kaiserthe13th/tr-lang/issues/3#issue-1027661753)<br>
#### ✔️ tr-lang bytecode reader seems like its done [Issue #4](https://github.com/kaiserthe13th/tr-lang/issues/4#issue-1027663331)<br>
#### ✔️ tr-lang runtime seems like its done [Issue #5](https://github.com/kaiserthe13th/tr-lang/issues/5#issue-1027665033)<br>

# 🚀 Installation

## 🪟 Windows

### 📇 tr-lang_Setup.exe
For windows there is a setup program.
Just download it and run it.
Follow the instructions and you are ready to go!

### 📇 Pre-Compiled Executable
For windows there is a pre-compiled binary.
Just download it, extract the zip and you are ready to go!
> Note: This binary won't be in the PATH environment variable by default
> meaning it won't be globally usable across the system with 'tr-lang'
>
> If you want to add tr-lang to your PATH environment variable you can follow this [tutorial](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/) by Ryan Hoffman

### 📦 Cargo
```console
$ cargo install tr-lang
```

### ⚙️ Build from source
dependencies: `rust, cargo`

```console
$ git clone https://github.com/kaiserthe13th/tr-lang
$ cd tr-lang
$ cargo install --path .
```
> Note: If you just want to play you can change the last command to `cargo build --release`
> your file will be in target/release/tr-lang

## 🍎 MacOS

### 🍺 Homebrew
```console
$ brew tap kaiserthe13th/tr-lang
$ brew install tr-lang
```

### 📦 Cargo
```console
$ cargo install tr-lang
```

### ⚙️ Build from source
dependencies: `rust, cargo`

```console
$ git clone https://github.com/kaiserthe13th/tr-lang
$ cd tr-lang
$ cargo install --path .
```
> Note: If you just want to play you can change the last command to `cargo build --release`
> your file will be in target/release/tr-lang

## 🐧 Linux

### 🗃️ Debian Package
1. Go to the Releases tab and download tr-lang_<x.x.x>_amd64.deb
2. In the Terminal
```console
$ dpkg -i tr-lang_<x.x.x>_amd64.deb
```
> Note: On some linux systems just clicking or doubke clicking on the file would start install

### 🎩 RPM Package
1. Go to the Releases tab and download tr-lang_<x.x.x>.x86_64.rpm
2. In the Terminal
```console
$ rpm -i tr-lang_<x.x.x>.x86_64.rpm
```
> Note: On some linux systems just clicking or doubke clicking on the file would start install

### 🍺 Homebrew
```console
$ brew tap kaiserthe13th/tr-lang
$ brew install tr-lang
```

### 📦 Cargo
```console
$ cargo install tr-lang
```

### ⚙️ Build from source
dependencies: `rust, cargo`

```console
$ git clone https://github.com/kaiserthe13th/tr-lang
$ cd tr-lang
$ cargo install --path .
```
> Note: If you just want to play you can change the last command to `cargo build --release`
> your file will be in target/release/tr-lang

# [📖 See the Docs](https://tr-lang-docs.netlify.app/english/)

# 🤝 Contributing
To report bugs, suggest new features or update documentation use the [issue tracker](https://github.com/kaiserthe13th/tr-lang/issues)

for features use <span class="tag">`(enhancement | yükseltme)`</span> tag, for bugs use <span class="tag">`(bug)`</span> tag and for documentation updates use <span class="tag">`(documentation | dökümantasyon)`</span> tag

👍 Bugfix PR's are welcome!

# 📜 Thanks

- I give my thanks to user [Netwave](https://stackoverflow.com/users/1695172/netwave) from stackoverflow.com for helping fix a [bug](https://stackoverflow.com/questions/69635458/pattern-matching-does-not-allow-me-to-change-values/69636181#69636181) during the creation of the parser.
- I give my thanks to user [Chayim Friedman](https://stackoverflow.com/users/7884305/chayim-friedman) from stackoverflow.com for helping me with [closure magic](https://stackoverflow.com/questions/70053866/rust-cloning-hashmapstring-object-without-moving-into-closure-solved) during the making of UnknownIdentifier error message.
