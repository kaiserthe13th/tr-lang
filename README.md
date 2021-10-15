# tr-lang

tr-lang is a language that aims to bring programming language syntax closer to Turkish.
tr-lang is a stack based language and uses reverse-polish notation for maths.

# What has been Implemented?

#### 🕘 tr-lang lexer is in progress [#1](https://github.com/kaiserthe13th/tr-lang/issues/1#issue-1027652152)<br>
#### ❌ tr-lang parser is not implemented<br>
#### ❌ tr-lang bytecode is not implemented<br>
#### ❌ tr-lang bytecode reader is not implemented<br>
#### ❌ tr-lang runtime is not implemented<br>

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
