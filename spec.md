# RAINBOW ASSEMBLY (RASM) SPECIFICATION

## INSTRUCTIONS

```
[ ]   NOP
[ ]   PUSH    [imm/var]
[ ]   POP     [var]
[ ]   LDARG   [imm/var]
[ ]   CALL    [func/var]
[ ]   ADD     [imm/var]   [imm/var]   [var]
[ ]   SUB     [imm/var]   [imm/var]   [var]
[ ]   MUL     [imm/var]   [imm/var]   [var]
[ ]   DIV     [imm/var]   [imm/var]   [var]
[ ]   JMP     [imm/var]
[ ]   JNE     [imm/var]   [imm/var]   [imm/var]
[ ]   JE      [imm/var]   [imm/var]   [imm/var]
[ ]   JGE     [imm/var]   [imm/var]   [imm/var]
[ ]   JG      [imm/var]   [imm/var]   [imm/var]
[ ]   JLE     [imm/var]   [imm/var]   [imm/var]
[ ]   JL      [imm/var]   [imm/var]   [imm/var]
[ ]   MOV     [imm/var]   [var]
[ ]   AND     [imm/var]   [imm/var]   [var]
[ ]   OR      [imm/var]   [imm/var]   [var]
[ ]   XOR     [imm/var]   [imm/var]   [var]
[ ]   NOT     [imm/var]   [var]
[ ]   LSH     [imm/var]   [imm/var]   [var]
[ ]   RSH     [imm/var]   [imm/var]   [var]
[ ]   VAR     [type/var]  [name]
[ ]   RET     [imm/var]
[ ]   DEREF   [ptr]       [var]
[ ]   REF     [var]       [ptr]
```

## TYPES

```
void
i8
i16
i32
i64
u8/char
u16
u32
u64
f16
f32
f64
pointer
type
struct
bytecode string (used for variable names, function names, etc.) (also is a function pointer)
```

## DATA SECTION
This is a section of the assembly where all constants (i.e. string, arrays) are stored for use in the program.
This section is placed at the end of the file.
The format is as follow
```
.data
(name) (type) (data)
(name) (type) (data)
...
```
An example data section may look like this
```
.data
str_1 char* "Hello, World!"
arr_1 u32* [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
```

## FUNCTIONS
Defining functions in RASM is much like defining functions in other languages.
The format is as follows
```
(return type) (name) (args) {
    (code)
}
```
An example function is
```c
void foo() {
    LDARG str_1
    CALL io.print
}

.data
str_1 char* "Hello, World!"
```
An example function with arguments is
```c
void bar(i32 x u64 y) {
    LDARG x
    CALL io.print
    LDARG y
    CALL io.print
}
```

## LABELS
Labels can be placed anywhere in a function, and used in combination with jump instructions to jump around inside of a function.
```c
void baz() { ; this creates an infinite loop
    :label
    JMP :label
}
```

## MACROS
todo: add description of macros
The format is as follows
```
.macro (name) (args) {
    (code)
}
```
An example macro is as follows
```
.macro CALL1ARG func a {
    LDARG a
    CALL func
}
```

## IMPORTING
You can import other files to use functions and macros from them.
To import other files, all you need to do is as follows
```
---- foo.rasm ----
.macro MACRO a b c {
    ; macro code
}

---- bar.rasm ----
.import foo.rasm

MACRO 0 1 2
```

## STRUCTS
Structs are custom data structures that contain variables.
Their format is as follows
```
.struct (name) {
    (var)
    (var)
    ...
}
```
An example struct would look like this
```rust
.struct Foo {
    i32 a
    f32 b
    char* txt
}
```

## ERRORS
Error handling is currently undefined in RASM.

## OPTIMIZATIONS
Optimizations are currently not implemented for RASM.