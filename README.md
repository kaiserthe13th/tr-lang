# tr-lang

tr-lang is a language that aims to bring programming language syntax closer to Turkish.
tr-lang is a stack based language and uses reverse-polish notation for maths.

# What has been Implemented?

#### 🕘 tr-lang lexer is in progress [Issue #1](https://github.com/kaiserthe13th/tr-lang/issues/1#issue-1027652152)<br>
#### ❌ tr-lang parser is not implemented [Issue #2](https://github.com/kaiserthe13th/tr-lang/issues/2#issue-1027660436)<br>
#### ❌ tr-lang bytecode is not implemented [Issue #3](https://github.com/kaiserthe13th/tr-lang/issues/3#issue-1027661753)<br>
#### ❌ tr-lang bytecode reader is not implemented [Issue #4](https://github.com/kaiserthe13th/tr-lang/issues/4#issue-1027663331)<br>
#### ❌ tr-lang runtime is not implemented [Issue #5](https://github.com/kaiserthe13th/tr-lang/issues/5#issue-1027665033)<br>

# Specification

#### ❕ All of the below have not yet been implemented

## Datatypes

### Strings

```py
"This is a string\n"
'Hello, World!\n'
```

### Number

```py
10328
142.32
```

### Boolean

```py
doğru
yanlış
```
## Keywords

### de
used to display datatypes to the screen
```py
"Hello, World!\n" de
1232 de ' ' de
doğru de
```
#### Stdout
```stdout
Hello, World!
1232 doğru
```

### ise
if statement in tr-lang
check conditions!
```py
10 0 > ise
  "Hello\n" de
son
```

### yoksa
else statement in tr-lang
What dould you do if `if` was wrong?
```py
10 0 < ise
  "How did we get here?\n" de
yoksa
  "The universe still works!\n" de
son
```

### son
ends an if block

### Unplanned...
