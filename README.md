# tr-lang

tr-lang is a language that aims to bring programming language syntax closer to Turkish.
tr-lang is a stack based language and uses reverse-polish notation for maths.

# What has been Implemented?

#### :x: tr-lang lexer is not implemented<br>
#### :x: tr-lang parser is not implemented<br>
#### :x: tr-lang bytecode is not implemented<br>
#### :x: tr-lang bytecode reader is not implemented<br>
#### :x: tr-lang runtime is not implemented<br>

# Specification

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
```py
10 0 > ise
  "Hello" de
son
```

### yoksa
else statement in tr-lang
```py
10 0 < ise
  "How did we get here?" de
yoksa
  "The universe still works!" de
son
```

### son
ends an if block

### Unplanned...
