# this is so outdated

# RAINBOW ASSEMBLY (RASM) SPECIFICATION

## INSTRUCTIONS

```
[x]   NOP                                           Does nothing
[x]   PUSH    [imm/var]                             Pushes a value onto the stack
[x]   POP     [var]                                 Pops a value off of the stack and stores it in a variable
[x]   PEEK    [imm/var]   [var]                     Peeks a value from the stack and stores it in a variable
[x]   CALL    [func/var]                            Calls a function
[x]   ADD     [imm/var]   [imm/var]   [var]         Add two numbers and store in a variable
[x]   SUB     [imm/var]   [imm/var]   [var]         Subtract two numbers and store in a variable
[x]   MUL     [imm/var]   [imm/var]   [var]         Multiply two numbers and store in a variable
[x]   DIV     [imm/var]   [imm/var]   [var]         Divide two numbers and store in a variable
[x]   JMP     [imm/var]                             Jump to a location within the current scope
[x]   JNE     [imm/var]   [imm/var]   [imm/var]     Jump to a location within the current scope if the given values are not equal
[x]   JE      [imm/var]   [imm/var]   [imm/var]     Jump to a location within the current scope if the given values are equal
[x]   JGE     [imm/var]   [imm/var]   [imm/var]     Jump to a location within the current scope if value A is greater than or equal to B
[x]   JG      [imm/var]   [imm/var]   [imm/var]     Jump to a location within the current scope if value A is greater than to B
[x]   JLE     [imm/var]   [imm/var]   [imm/var]     Jump to a location within the current scope if value A is less than or equal to B
[x]   JL      [imm/var]   [imm/var]   [imm/var]     Jump to a location within the current scope if value A is less than to B
[x]   MOV     [imm/var*]  [var*]                    Move a value into a variable
[x]   AND     [imm/var]   [imm/var]   [var]         Perform bitwise AND on two values and store in a variable
[x]   OR      [imm/var]   [imm/var]   [var]         Perform bitwise OR on two values and store in a variable
[x]   XOR     [imm/var]   [imm/var]   [var]         Perform bitwise XOR on two values and store in a variable
[x]   NOT     [imm/var]   [var]                     Perform bitwise NOT on a value and store in a variable
[x]   LSH     [imm/var]   [imm/var]   [var]         Left shift value A value B bits
[x]   RSH     [imm/var]   [imm/var]   [var]         Right shift value A value B bits
[x]   VAR     [type/var]  [name/var]                Create a variable with the given type and name
[x]   RET     {imm/var}                             Return from a function (functions with void type do not need to include arguments)
[x]   DEREF   [imm/ptr]   [var]                     Dereference a pointer and store in a variable
[x]   REF     [imm/var]   [ptr var]                 Create a reference to a variable and store in another variable
[x]   INST    [name/var]  [var]                     Instantiate a struct with default values
[x]   MOD     [imm/var]   [imm/var]   [var]         Perform modulus on two values and store in a variable
[x]   PMOV    [imm/var]   [ptr var]   [imm/var]     Moves the value into the pointer with the offset
[x]   ALLOC   [type/var]  [imm/var]   [ptr var]     Allocates a given pointer with a type and size
[x]   FREE    [imm/ptr]   {imm/var}                 Frees the given pointer with the given size
[x]   CALLC   [imm/var]   [type/var]  [imm/var]     Calls the function at the given address with the given arguments and return type.
```

To specify a dynamic variable for the MOV instruction use the @ character before the variable name.
To specify an imported function for the CALL instruction use the @ character before the function name.

When declaring a variable, do not use $ before the variable name.
When using a variable, use $ before the variable name.

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
(name) (type) (length type) (length) (data)
(name) (type) (length type) (length) (data)
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
    CALL io_print
}

.data
str_1 char* "Hello, World!"
```
An example function with arguments is
```c
void bar(i32 x u64 y) {
    LDARG x
    CALL io_print
    LDARG y
    CALL io_print
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
TODO READD MACROS
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
The values within structs are accessed through the normal instructions used for variables, with the format of
```
(struct instance name).(field)
```

## CONDITIONAL PARSING
Conditional parsing allows you to toggle any part of your code based off of constant variables. These varaibles are provided by either the runtime or the user.
```c#
.if PLATFORM == PLATFORM_WINDOWS
    {code}
.elseif PLATFORM == PLATFORM_LINUX
    {code}
.end
```

## ERRORS
Error handling is currently undefined in RASM.

## OPTIMIZATIONS
Optimizations are currently not implemented for RASM.