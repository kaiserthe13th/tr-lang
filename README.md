# ![tr-lang](./img/logo/logo.png)
#### <center>Fast and Easy</center>
<hr style="width: 50%;">

### Made with ❤️ in 🇹🇷

tr-lang is a language that aims to bring programming language syntax closer to Turkish.
tr-lang is a stack based language and uses reverse-polish notation for maths.

# :triangular_flag_on_post: İçindekiler
- [What Has Been Implemented?](#what-has-been-implemented)
- [Installation](#installation)
- [See the Wiki](#see-the-wiki)
- [Contributing](#contributing)
- [Thanks](#thanks)

# What Has Been Implemented?

## ✔️ All parts of the language seems like they are done!

#### ✔️ tr-lang lexer seems like its done [Issue #1](https://github.com/kaiserthe13th/tr-lang/issues/1#issue-1027652152)<br>
#### ✔️ tr-lang parser is in progress [Issue #2](https://github.com/kaiserthe13th/tr-lang/issues/2#issue-1027660436)<br>
#### ✔️ tr-lang bytecode seems like its done [Issue #3](https://github.com/kaiserthe13th/tr-lang/issues/3#issue-1027661753)<br>
#### ✔️ tr-lang bytecode reader seems like its done [Issue #4](https://github.com/kaiserthe13th/tr-lang/issues/4#issue-1027663331)<br>
#### ✔️ tr-lang runtime seems like its done [Issue #5](https://github.com/kaiserthe13th/tr-lang/issues/5#issue-1027665033)<br>

# Installation

## Homebrew
```sh
brew tap kaiserthe13th/tr-lang
brew install tr-lang
```

## Cargo
```sh
cargo install tr-lang
```

## Build from source
dependencies: `rust, cargo`

```sh
git clone https://github.com/kaiserthe13th/tr-lang
cd tr-lang
cargo build --release
```
your file will be in target/release/tr-lang

# [See the Wiki](https://github.com/kaiserthe13th/tr-lang/wiki)
- [Specification](https://github.com/kaiserthe13th/tr-lang/wiki/Specification-EN)

# Contributing
To report bugs, suggest new features or update documentation use the [issue tracker](https://github.com/kaiserthe13th/tr-lang/issues)

for features use <span class="tag">`(enhancement | yükseltme)`</span> tag, for bugs use <span class="tag">`(bug)`</span> tag and for documentation updates use <span class="tag">`(documentation | dökümantasyon)`</span> tag

:+1: Bugfix PR's are welcome!

# Thanks

I give my thanks to user [Netwave](https://stackoverflow.com/users/1695172/netwave) from stackoverflow.com for helping fix a [bug](https://stackoverflow.com/questions/69635458/pattern-matching-does-not-allow-me-to-change-values/69636181#69636181) during the creation of the parser.
