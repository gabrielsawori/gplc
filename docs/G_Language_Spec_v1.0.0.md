# G Language Specification
**Version**: 1.0.0-draft  
**File Extensions**: `.gpl` (source), `.hdr` (header)  
**Paradigm**: Compiled, statically typed, imperative/procedural with OOP support  
**Design Philosophy**: C-level control + Python-level readability

---

## Table of Contents

1. [File Structure](#1-file-structure)
2. [Header Files (.hdr)](#2-header-files-hdr)
3. [Comments & Documentation](#3-comments--documentation)
4. [Data Types](#4-data-types)
5. [Variables & Constants](#5-variables--constants)
6. [Operators](#6-operators)
7. [Control Flow](#7-control-flow)
8. [Functions](#8-functions)
9. [Pointers & References](#9-pointers--references)
10. [Arrays & Slices](#10-arrays--slices)
11. [Strings](#11-strings)
12. [Structs](#12-structs)
13. [Enums](#13-enums)
14. [Union Types](#14-union-types)
15. [Interfaces & Traits](#15-interfaces--traits)
16. [Operator Overloading](#16-operator-overloading)
17. [Error Handling](#17-error-handling)
18. [Memory Management](#18-memory-management)
19. [RAII & Destructors](#19-raii--destructors)
20. [Modules & Imports](#20-modules--imports)
21. [Generics](#21-generics)
22. [Type System Extensions](#22-type-system-extensions)
23. [Concurrency](#23-concurrency)
24. [Macros & Compile-time](#24-macros--compile-time)
25. [Inline Assembly](#25-inline-assembly)
26. [Type Casting](#26-type-casting)
27. [Attributes & Annotations](#27-attributes--annotations)
28. [Testing & Benchmarks](#28-testing--benchmarks)
29. [Standard Operators Summary](#29-standard-operators-summary)
30. [Appendix A — Full .gpl Example](#appendix-a--full-gpl-example)
31. [Appendix B — Full .hdr Example](#appendix-b--full-hdr-example)
32. [Appendix C — Design Decisions](#appendix-c--design-decisions)
33. [Numeric Literals](#30-numeric-literals)
34. [Operator Precedence](#31-operator-precedence)
35. [Iterator Protocol & Functional Methods](#32-iterator-protocol--functional-methods)
36. [Lifetime & Borrow Semantics](#33-lifetime--borrow-semantics)
37. [FFI — C Interoperability](#34-ffi--c-interoperability)
38. [Async / Await](#35-async--await)
39. [Build System — gpl.toml](#36-build-system--gpltoml)
40. [CLI Toolchain — gpl](#37-cli-toolchain--gpl)
41. [SIMD Intrinsics](#38-simd-intrinsics)
42. [Reflection & Runtime Type Info](#39-reflection--runtime-type-info)
43. [Standard Library Overview](#40-standard-library-overview)
44. [std/io — Input & Output](#41-stdio--input--output)
45. [std/fs — Filesystem](#42-stdfs--filesystem)
46. [std/path — Path Manipulation](#43-stdpath--path-manipulation)
47. [std/os — Operating System Interface](#44-stdos--operating-system-interface)
48. [std/time — Date & Time](#45-stdtime--date--time)
49. [std/fmt — Formatting](#46-stdfmt--formatting)
50. [std/net — Networking](#47-stdnet--networking)
51. [std/json — JSON Encoding & Decoding](#48-stdjson--json-encoding--decoding)
52. [std/log — Structured Logging](#49-stdlog--structured-logging)
53. [std/regex — Regular Expressions](#50-stdregex--regular-expressions)
54. [std/hash & std/crypto](#51-stdhash--stdcrypto)
55. [std/collections — Data Structures](#52-stdcollections--data-structures)
56. [std/rand — Random Numbers](#53-stdrand--random-numbers)
57. [Freestanding Mode (no_std)](#54-freestanding-mode-no_std)
58. [Volatile Memory Access](#55-volatile-memory-access)
59. [Memory-Mapped I/O Patterns](#56-memory-mapped-io-patterns)
60. [Calling Conventions](#57-calling-conventions)
61. [Interrupt Handlers](#58-interrupt-handlers)
62. [Linker Script Integration](#59-linker-script-integration)
63. [CPU Control Instructions](#60-cpu-control-instructions)
64. [Atomic Operations with Memory Ordering](#61-atomic-operations-with-memory-ordering)
65. [Virtual Memory & Paging](#62-virtual-memory--paging)
66. [Physical Memory Manager](#63-physical-memory-manager)
67. [Stack & Context Switching](#64-stack--context-switching)
68. [Kernel Address Space Types](#65-kernel-address-space-types)
69. [core/mem — No-alloc Memory Operations](#66-coremem--no-alloc-memory-operations)
70. [Formal Grammar (EBNF)](#67-formal-grammar-ebnf)
71. [Behavior Specification](#68-behavior-specification)
72. [ABI Specification](#69-abi-specification)
73. [SMP Kernel Support](#70-smp-symmetric-multi-processing-kernel-support)
74. [std/xml — XML](#71-missing-standard-library-modules)
75. [std/csv — CSV](#712-stdcsv--csv-parsing--emission)
76. [std/compress — Compression](#713-stdcompress--compression)
77. [std/encoding — Encoding Utilities](#714-stdencoding--encoding-utilities)
78. [std/unicode — Unicode Properties](#715-stdunicode--unicode-properties)
79. [std/sql — Database Interface](#716-stdsql--database-interface)
80. [std/template — Templating](#717-stdtemplate--text--html-templating)
81. [std/testing/mock — Mocking](#718-stdtestingmock--mocking)
82. [impl Block Syntax](#72-impl-block-syntax-grouping)
83. [Anonymous & Inline Structs](#73-anonymous--inline-structs)
84. [Associated Types in Interfaces](#74-associated-types-in-interfaces)
85. [where Clause on Structs](#75-where-clause-on-structs)
86. [Recursive Types](#76-recursive-types)
87. [Formal Memory Model](#77-formal-memory-model)
88. [Error Code Catalogue](#78-error-code-catalogue)
89. [std/sync — Synchronization Primitives](#79-stdsync--synchronization-primitives)
90. [std/net/tls — TLS / HTTPS](#80-stdnettls--tls--https)
91. [std/net — WebSocket](#81-stdnet--websocket)
92. [std/os/linux — Linux-specific](#82-stdoslinux--linux-specific)
93. [std/os/windows — Windows-specific](#83-stdoswindows--windows-specific)
94. [Profiling & Sanitizers](#84-profiling--sanitizers)
95. [WebAssembly Target](#85-webassembly-target)
96. [Debugger Integration](#86-debugger-integration)
97. [Package Registry](#87-package-registry)
98. [Incremental Compilation](#88-incremental-compilation)
99. [Compiler Plugin API](#89-compiler-plugin-api)

---

## 1. File Structure

Every `.gpl` file follows this layout:

```gpl
# file: main.gpl

module main                        # module declaration (required, first line)

import "std/io"                    # standard library
import "std/mem"
import "../utils/math" as gmath    # relative path with alias
import "std/io": {println, eprintln, stderr}  # selective import

# top-level constants
const VERSION: str = "1.0.0"

# top-level variables (file-scope)
var global_counter: i32 = 0

# entry point
fn main() -> i32:
    io.println("Hello, World!")
    return 0
```

**Rules:**
- `module` declaration must be the first non-comment line.
- Only one `module` declaration per file.
- Entry point is `fn main() -> i32` in `module main`.
- Files without `fn main` are library modules.

---

## 2. Header Files (.hdr)

Header files expose the public API: types, function signatures, constants.  
They are **never** compiled standalone — only used for declarations.

```hdr
# file: math.hdr

module math

pub const PI:  f64 = 3.14159265358979323846
pub const TAU: f64 = 6.28318530717958647692
pub const E:   f64 = 2.71828182845904523536

pub type Radians = f64
pub type Degrees = f64

pub struct Vector2:
    pub x: f64
    pub y: f64

pub fn sin(angle: Radians) -> f64
pub fn cos(angle: Radians) -> f64
pub fn sqrt(n: f64) -> f64
pub fn lerp(a: f64, b: f64, t: f64) -> f64

pub enum Axis:
    X
    Y
    Z
```

Include a header in source:

```gpl
import "math.hdr"        # relative header
import <std/string.hdr>  # system header (angle brackets = stdlib path)
```

**Rules:**
- `.hdr` files only contain declarations, no function bodies.
- `pub` is required on everything exported from a header.
- The `.hdr` file's `module` must match the `.gpl` implementation file's `module`.

---

## 3. Comments & Documentation

```gpl
# This is a single-line comment

#
# This is a
# multi-line comment block
#

## Single-line doc comment (attached to the next declaration)
## Supports **markdown** formatting.

##
## Multi-line doc comment.
##
## Parameters:
##   x - the input value
##   y - the multiplier
##
## Returns:
##   The product of x and y.
##
## Example:
##   var result := multiply(3, 4)  # => 12
##
fn multiply(x: i32, y: i32) -> i32:
    return x * y

# @see, @since, @deprecated tags inside doc comments:
##
## Parses a raw byte buffer into a Header struct.
##
## @since  0.2.0
## @see    parse_footer
## @deprecated Use parse_header_v2 instead.
##
fn parse_header(buf: []byte) -> Result[Header]
```

**Rules:**
- `#` — regular comment, ignored by tooling.
- `##` — doc comment, parsed by the doc generator (`gdoc`).
- Doc comments must immediately precede the declaration they document.

---

## 4. Data Types

### 4.1 Primitive Types

| Type     | Size      | Description                          |
|----------|-----------|--------------------------------------|
| `i8`     | 1 byte    | Signed 8-bit integer                 |
| `i16`    | 2 bytes   | Signed 16-bit integer                |
| `i32`    | 4 bytes   | Signed 32-bit integer                |
| `i64`    | 8 bytes   | Signed 64-bit integer                |
| `i128`   | 16 bytes  | Signed 128-bit integer               |
| `u8`     | 1 byte    | Unsigned 8-bit integer               |
| `u16`    | 2 bytes   | Unsigned 16-bit integer              |
| `u32`    | 4 bytes   | Unsigned 32-bit integer              |
| `u64`    | 8 bytes   | Unsigned 64-bit integer              |
| `u128`   | 16 bytes  | Unsigned 128-bit integer             |
| `f32`    | 4 bytes   | 32-bit IEEE 754 float                |
| `f64`    | 8 bytes   | 64-bit IEEE 754 float                |
| `bool`   | 1 byte    | Boolean: `true` / `false`            |
| `byte`   | 1 byte    | Alias for `u8`                       |
| `rune`   | 4 bytes   | Unicode code point (alias `u32`)     |
| `str`    | ptr+len   | UTF-8 string slice (immutable)       |
| `void`   | 0 bytes   | No value                             |
| `never`  | —         | Type of expressions that never return|
| `any`    | 2 words   | Type-erased value (type + pointer)   |
| `isize`  | arch      | Platform-sized signed integer        |
| `usize`  | arch      | Platform-sized unsigned integer      |

**`never` type** — used for functions that never return (infinite loops, panics, exits):

```gpl
fn crash(msg: str) -> never:
    io.eprintln("Fatal:", msg)
    os.exit(1)

fn infinite() -> never:
    loop:
        do_work()
```

### 4.2 Bit-field Integers

Bit-sized integers are valid only inside `@packed` structs:

```gpl
@packed
struct Flags:
    active:   u1    # 1-bit unsigned
    mode:     u3    # 3-bit unsigned (values 0–7)
    priority: u4    # 4-bit unsigned (values 0–15)
    reserved: u8    # 8-bit padding
```

Standalone bit integers outside structs are **not allowed**.

### 4.3 Compound Types

```gpl
# Fixed-size array
var arr: [5]i32 = [1, 2, 3, 4, 5]

# Slice (dynamic view into array)
var sl: []i32 = arr[1:3]

# Pointer
var p: *i32 = &arr[0]

# Pointer to const value
var cp: *const i32 = &arr[0]

# Nullable pointer
var np: ?*i32 = null

# Tuple
var t: (i32, str, bool) = (42, "hello", true)

# Map (hash map)
var m: map[str]i32 = {"alpha": 1, "beta": 2}

# Function type
var fn_ref: fn(i32, i32) -> i32 = add

# Set
var s: set[str] = {"apple", "banana", "cherry"}
```

### 4.4 Type Aliases & Newtypes

```gpl
# Type alias — interchangeable with the original type
type Seconds  = f64
type Buffer   = []byte
type Callback = fn(i32) -> void

# Newtype — NOT interchangeable; requires explicit cast
newtype UserId   = i32
newtype MeterId  = f64
newtype Password = str

var id: UserId = UserId(42)
var raw: i32   = id as i32      # must cast explicitly
var id2: UserId = raw           # ERROR: type mismatch
```

Newtypes prevent accidental mixing of semantically different values with the same underlying type.

---

## 5. Variables & Constants

### 5.1 Variable Declaration

```gpl
# Explicit type
var x: i32 = 10

# Type inference
var y := 3.14              # inferred f64

# Multiple declaration, same type
var a, b, c: i32 = 1, 2, 3

# Zero-initialized (no value required)
var count: i32             # count = 0

# Immutable binding
let name: str = "Alice"    # cannot be reassigned

# Ignore return value explicitly
_ := fn_with_side_effect()
```

### 5.2 Constants

```gpl
const BUFFER_SIZE: usize = 4096
const GRAVITY:     f64   = 9.80665
const APP_NAME:    str   = "MyApp"

# Constant expressions (evaluated at compile-time)
const KB: usize = 1024
const MB: usize = KB * 1024
const GB: usize = MB * 1024

# const fn result used as constant
const MAX_THREADS: i32 = cpu_count() * 2   # cpu_count() must be const fn
```

### 5.3 `const fn` — Compile-time Functions

```gpl
const fn fibonacci(n: i32) -> i32:
    if n <= 1: return n
    return fibonacci(n - 1) + fibonacci(n - 2)

const FIB_10: i32 = fibonacci(10)    # evaluated at compile-time
```

`const fn` restrictions:
- No heap allocation (`mem.alloc`)
- No I/O
- No mutable global state
- No unsafe blocks

### 5.4 Short Assignment (inside functions only)

```gpl
fn example() -> void:
    x   := 5         # inferred i32
    msg := "hello"   # inferred str
    ok  := true      # inferred bool
```

---

## 6. Operators

### 6.1 Arithmetic

```gpl
a + b      # addition
a - b      # subtraction
a * b      # multiplication
a / b      # division (integer truncation for int types)
a % b      # modulo
a ** b     # exponentiation (numeric types)
-a         # unary negation
```

### 6.2 Integer Overflow Variants

Default integer arithmetic **panics on overflow** in debug mode and **wraps** in release mode.  
Explicit overflow operators are always available:

```gpl
a +%  b    # wrapping add    (always wraps, no panic)
a -%  b    # wrapping sub
a *%  b    # wrapping mul
a +|  b    # saturating add  (clamps to min/max)
a -|  b    # saturating sub
a *|  b    # saturating mul
a +!  b    # checked add     (returns Option[T], None on overflow)
a -!  b    # checked sub
a *!  b    # checked mul
```

```gpl
var x: u8 = 250
var y: u8 = x +% 10    # y = 4  (wraps around)
var z: u8 = x +| 10    # z = 255 (saturates at max)
var r := (x +! 10)     # r = Option.None (overflowed)
```

### 6.3 Comparison

```gpl
a == b    # equal
a != b    # not equal
a <  b    # less than
a >  b    # greater than
a <= b    # less or equal
a >= b    # greater or equal
a is T    # type check: true if a is of type T
```

### 6.4 Logical

```gpl
a and b   # logical AND (short-circuit)
a or  b   # logical OR  (short-circuit)
not a     # logical NOT
```

### 6.5 Bitwise

```gpl
a &  b    # bitwise AND
a |  b    # bitwise OR
a ^  b    # bitwise XOR
~a        # bitwise NOT (complement)
a << n    # left shift
a >> n    # right shift (arithmetic for signed, logical for unsigned)
```

### 6.6 Assignment

```gpl
x  = 5
x += 1   x -= 1   x *= 2   x /= 2   x %= 3
x **= 2
x &= 0xFF   x |= 0x01   x ^= 0x10
x <<= 2     x >>= 1
x +%= 1     x -%= 1     x *%= 2    # wrapping assignment
x +|= 1     x -|= 1     x *|= 2   # saturating assignment
```

### 6.7 Memory & Reference

```gpl
&x          # address-of (returns *T)
*p          # dereference pointer
p->field    # pointer field access (sugar for (*p).field)
```

### 6.8 Range

```gpl
0..10       # exclusive range [0, 10)
0..=10      # inclusive range [0, 10]
```

### 6.9 Null-safety & Optional Chaining

```gpl
x ?? default_val    # null coalescing: x if not null, else default_val
x?.field            # optional chain: null if x is null
x?.method()         # optional method call
```

### 6.10 Pipe Operator

```gpl
x |> f              # equivalent to f(x)
x |> f |> g |> h   # equivalent to h(g(f(x)))

# example
var result := raw_data
    |> parse_bytes
    |> validate
    |> transform
    |> serialize
```

### 6.11 Spread Operator

```gpl
fn sum(nums: ...i32) -> i32: ...

var arr := [1, 2, 3]
sum(arr...)          # spread slice into variadic
```

---

## 7. Control Flow

### 7.1 If / Elif / Else

```gpl
if x > 0:
    io.println("positive")
elif x < 0:
    io.println("negative")
else:
    io.println("zero")

# single-line form
if x > 0: io.println("positive")
```

### 7.2 If as Expression

```gpl
var label := if x > 0: "pos" elif x < 0: "neg" else: "zero"

var abs_x := if x < 0: -x else: x
```

### 7.3 Type Narrowing in If

After an `is` check, the compiler narrows the type inside that branch automatically:

```gpl
fn describe(shape: Drawable) -> void:
    if shape is Circle:
        # shape is narrowed to Circle here
        io.println("circle radius:", shape.radius)
    elif shape is Rectangle:
        io.println("rect size:", shape.width, "x", shape.height)
    else:
        io.println("unknown shape")
```

### 7.4 While Loop

```gpl
while i < 10:
    i += 1

# labeled loop
@outer while i < 10:
    while j < 10:
        if j == 5: break @outer
        j += 1
```

### 7.5 For Loop

```gpl
# range-based
for i in 0..10:
    io.println(i)

# inclusive range
for i in 0..=10:
    io.println(i)

# iterate over array/slice
for item in my_array:
    process(item)

# with index
for i, item in my_array:
    io.println(i, item)

# iterate over map
for key, val in my_map:
    io.println(key, val)

# step (compile-time constant step only)
for i in 0..100 step 5:
    io.println(i)

# reverse
for i in (0..10).rev():
    io.println(i)
```

### 7.6 Loop (infinite)

```gpl
loop:
    if should_stop(): break
    do_work()
```

### 7.7 Match (pattern matching)

```gpl
# match on value
match status:
    200 => io.println("OK")
    404 => io.println("Not Found")
    500 => io.println("Server Error")
    _   => io.println("Unknown status:", status)

# match with guard
match value:
    n if n < 0  => io.println("negative:", n)
    0           => io.println("zero")
    n           => io.println("positive:", n)

# match on enum
match direction:
    Axis.X => move_x()
    Axis.Y => move_y()
    Axis.Z => move_z()

# match with destructuring (tuple)
match point:
    (0, 0)   => io.println("origin")
    (x, 0)   => io.println("on x-axis at", x)
    (0, y)   => io.println("on y-axis at", y)
    (x, y)   => io.println("at", x, y)

# match with destructuring (enum data)
match shape:
    Shape.Circle(r)        => io.println("circle r=", r)
    Shape.Rectangle(w, h)  => io.println("rect", w, "x", h)

# match as expression
var area := match shape:
    Shape.Circle(r)       => 3.14159 * r * r
    Shape.Rectangle(w, h) => w * h
    Shape.Triangle(b, h)  => 0.5 * b * h

# match with multiple patterns (OR)
match key:
    'a' | 'e' | 'i' | 'o' | 'u' => io.println("vowel")
    _                             => io.println("consonant")

# match with binding and guard
match event:
    Event.KeyPress(k) if k.ctrl and k.key == 'c' =>
        io.println("Ctrl+C pressed")
    Event.KeyPress(k) =>
        io.println("Key:", k.key)
    Event.MouseClick(x, y) =>
        io.println("Click at", x, y)
```

### 7.8 Defer

```gpl
fn read_file(path: str) -> Result[str]:
    var f := open(path)?
    defer f.close()          # runs at end of scope (LIFO order)
    return f.read_all()

# multiple defers — execute in reverse (LIFO)
fn example() -> void:
    defer io.println("third")   # runs last
    defer io.println("second")
    defer io.println("first")   # runs first
```

### 7.9 Break / Continue / Return

```gpl
break              # exit current loop
break @label       # exit labeled loop
continue           # next iteration of current loop
continue @label    # next iteration of labeled loop
return             # return void
return value       # return value
return x, y        # return multiple values
```

---

## 8. Functions

### 8.1 Basic Function

```gpl
fn add(a: i32, b: i32) -> i32:
    return a + b

# single-expression shorthand
fn square(x: i32) -> i32: x * x
```

### 8.2 Multiple Return Values

```gpl
fn divmod(a: i32, b: i32) -> (i32, i32):
    return a / b, a % b

var q, r := divmod(10, 3)
```

### 8.3 Named Return Values

```gpl
fn parse_header(data: []byte) -> (size: i32, offset: i32, err: ?Error):
    size   = read_i32(data, 0)
    offset = read_i32(data, 4)
    if size < 0:
        err = Error.new("invalid size: expected >= 0, got " + size)
    return    # bare return uses named values
```

### 8.4 Default Parameters

```gpl
fn connect(host: str, port: i32 = 8080, timeout: f64 = 30.0) -> Connection:
    ...

connect("localhost")                  # port=8080, timeout=30.0
connect("localhost", port: 9090)     # named argument
connect("localhost", timeout: 5.0)   # named, skip port
```

### 8.5 Variadic Functions

```gpl
fn sum(nums: ...i32) -> i32:
    var total: i32 = 0
    for n in nums:
        total += n
    return total

sum(1, 2, 3, 4, 5)

var arr := [1, 2, 3]
sum(arr...)           # spread
```

### 8.6 Function as First-Class Value

```gpl
var op: fn(i32, i32) -> i32 = add

fn apply(a: i32, b: i32, f: fn(i32, i32) -> i32) -> i32:
    return f(a, b)

apply(3, 4, add)
apply(3, 4, fn(a: i32, b: i32) -> i32: a * b)
```

### 8.7 Closures / Lambdas

```gpl
# single-expression lambda
var double := fn(x: i32) -> i32: x * 2

# multi-line lambda
var greet := fn(name: str) -> str:
    return "Hello, " + name

# closure captures outer scope
fn make_counter(start: i32) -> fn() -> i32:
    var count := start
    return fn() -> i32:
        count += 1
        return count

var next := make_counter(0)
next()    # 1
next()    # 2
next()    # 3
```

### 8.8 Method Syntax

```gpl
struct Circle:
    radius: f64

# immutable receiver (value copy)
fn Circle.area(self) -> f64:
    return 3.14159 * self.radius * self.radius

# mutable receiver (pointer)
fn Circle.scale(self: *Circle, factor: f64) -> void:
    self.radius *= factor

# static method (no receiver)
fn Circle.unit() -> Circle:
    return Circle{radius: 1.0}

# method chaining (returns Self)
fn Circle.with_radius(self: *Circle, r: f64) -> *Circle:
    self.radius = r
    return self
```

### 8.9 Access Modifiers

```gpl
pub fn public_fn() -> void: ...    # exported from module
fn private_fn() -> void: ...      # module-private (default)
```

### 8.10 `noreturn` Functions

```gpl
@noreturn
fn fatal(msg: str) -> never:
    io.eprintln("[FATAL]", msg)
    os.exit(1)
```

---

## 9. Pointers & References

```gpl
var x: i32 = 42

var p:  *i32       = &x          # mutable pointer
var cp: *const i32 = &x          # pointer to const (cannot write through cp)

*p = 100                          # dereference and write
io.println(*p)                    # dereference and read
io.println(p->x)                  # field access via pointer (structs only)

# Nullable pointer
var np: ?*i32 = null
if np != null:
    io.println(*np)

# Optional chain on pointer
np?.deref()                       # no-op if np is null

# Pointer arithmetic — only inside unsafe
unsafe:
    var next: *i32 = p + 1
    io.println(*next)
```

---

## 10. Arrays & Slices

### 10.1 Fixed Arrays

```gpl
var arr: [4]i32 = [10, 20, 30, 40]
arr[0] = 99
var len := arr.len    # compile-time: 4

# multi-dimensional
var mat: [3][3]f64
mat[0][0] = 1.0

# array of arrays
var grid: [8][8]bool
```

### 10.2 Slices

```gpl
var sl:  []i32 = arr[1:3]    # [20, 30]
var sl2 := arr[:]             # full slice
var sl3 := arr[2:]            # from index 2 to end
var sl4 := arr[:3]            # from start to index 2

sl.len     # element count
sl.cap     # capacity of underlying buffer
sl.ptr     # raw pointer to first element

sl = sl.append(50)
sl = sl.append_many([60, 70, 80])
```

### 10.3 Dynamic List (Heap-allocated)

```gpl
import "std/collections"

var list := collections.List[i32].new()
list.push(1)
list.push(2)
list.push(3)
var top := list.pop()           # Option[i32]
var got := list.get(0)          # Option[i32]
list.insert(1, 99)
list.remove(1)
list.sort()
list.sort_by(fn(a: i32, b: i32) -> bool: a > b)
```

### 10.4 Bounds Checking

Bounds checks are **on by default**. Disable per-block for hot paths:

```gpl
@bounds_check(false)
fn fast_sum(data: []f32) -> f32:
    var total: f32 = 0.0
    for i in 0..data.len:
        total += data[i]    # no bounds check
    return total
```

---

## 11. Strings

```gpl
# immutable UTF-8 string
var s: str = "Hello, World!"

# raw string (no escape sequences)
var path: str = r"C:\Users\name\file.txt"

# multiline string (strips common leading whitespace)
var doc: str = """
    Line one
    Line two
    Line three
"""

# string interpolation
var name := "Alice"
var age  := 30
var msg  := f"Name: {name}, Age: {age}"
var expr := f"2 + 2 = {2 + 2}"

# format specifiers
var pi   := 3.14159
var fmt1 := f"Pi is {pi:.2f}"        # "Pi is 3.14"
var fmt2 := f"Hex: {255:x}"          # "Hex: ff"
var fmt3 := f"Hex: {255:X}"          # "Hex: FF"
var fmt4 := f"Padded: {42:>8}"       # "Padded:       42"
var fmt5 := f"Zero-padded: {42:0>5}" # "Zero-padded: 00042"
var fmt6 := f"Binary: {10:b}"        # "Binary: 1010"
var fmt7 := f"Sci: {0.001:.3e}"      # "Sci: 1.000e-03"
```

Format spec syntax: `{value:[[fill]align][sign][#][0][width][.precision][type]}`

| type | meaning                  |
|------|--------------------------|
| `d`  | decimal integer (default)|
| `x`  | hex lowercase            |
| `X`  | hex uppercase            |
| `o`  | octal                    |
| `b`  | binary                   |
| `f`  | fixed-point float        |
| `e`  | scientific notation      |
| `g`  | shortest float           |
| `s`  | string (default)         |

```gpl
# String operations
s.len                      # byte length
s.rune_count()             # unicode character count
s.starts_with("Hello")
s.ends_with("!")
s.contains("World")
s.to_upper()
s.to_lower()
s.trim()
s.trim_start()
s.trim_end()
s.split(",")               # -> []str
s.split_once(",")          # -> ?(str, str)
s.replace("World", "G")
s.find("lo")               # -> Option[usize]
s.as_bytes()               # -> []byte
s.chars()                  # -> []rune
s[0]                       # -> byte (u8)
s[0:5]                     # -> str slice

# String builder
import "std/string"
var b := string.Builder.new()
b.write("Hello")
b.writeln(", World!")
b.write_fmt(f"Pi = {3.14:.2f}")
var result := b.build()    # -> str
```

---

## 12. Structs

### 12.1 Declaration

```gpl
struct Point:
    x: f64
    y: f64

struct Person:
    name:  str
    age:   i32
    email: str = ""    # default field value
```

### 12.2 Instantiation

```gpl
# Named fields (preferred)
var p := Point{x: 3.0, y: 4.0}

# Positional (must match field order exactly)
var p2 := Point(3.0, 4.0)

# Partial with defaults
var person := Person{name: "Alice", age: 30}

# Struct update syntax (copy + override fields)
var p3 := p{x: 10.0}         # copy p but set x = 10.0
var p4 := p{x: 5.0, y: 5.0}
```

### 12.3 Methods

```gpl
fn Point.distance(self, other: Point) -> f64:
    var dx := self.x - other.x
    var dy := self.y - other.y
    return math.sqrt(dx*dx + dy*dy)

fn Point.translate(self: *Point, dx: f64, dy: f64) -> void:
    self.x += dx
    self.y += dy

fn Point.to_string(self) -> str:
    return f"({self.x}, {self.y})"
```

### 12.4 Struct Embedding (Composition)

```gpl
struct Animal:
    name: str
    age:  i32

    fn speak(self) -> void:
        io.println(self.name, "makes a sound")

struct Dog:
    embed Animal    # inherits all fields and methods from Animal
    breed: str

fn Dog.speak(self) -> void:
    io.println(self.name, "says: Woof!")   # override Animal.speak

var d := Dog{name: "Rex", age: 3, breed: "Labrador"}
d.speak()          # calls Dog.speak (overridden)
d.Animal.speak()   # explicitly call Animal.speak
```

### 12.5 Memory Layout Attributes

```gpl
@packed
struct WireHeader:
    magic:   u32
    version: u16
    flags:   u8
    length:  u8

@align(64)
struct CacheAligned:
    data: [16]i32

@align(16)
@packed
struct SIMDVec4:
    x: f32
    y: f32
    z: f32
    w: f32
```

---

## 13. Enums

### 13.1 Basic Enum

```gpl
enum Color:
    Red
    Green
    Blue

var c := Color.Red

# namespace alias — avoid repeating enum name
use Color.*
var c2 := Red
```

### 13.2 Enum with Explicit Values

```gpl
enum StatusCode:
    OK          = 200
    NotFound    = 404
    ServerError = 500

var code := StatusCode.OK
var raw: i32 = code as i32     # 200
```

### 13.3 Enum with Data (Tagged Union / Sum Type)

```gpl
enum Shape:
    Circle(radius: f64)
    Rectangle(width: f64, height: f64)
    Triangle(base: f64, height: f64)

fn area(s: Shape) -> f64:
    match s:
        Shape.Circle(r)       => 3.14159 * r * r
        Shape.Rectangle(w, h) => w * h
        Shape.Triangle(b, h)  => 0.5 * b * h
```

### 13.4 Built-in `Option[T]`

```gpl
# Defined in std as:
# enum Option[T]:
#     Some(value: T)
#     None

fn find_index(arr: []i32, target: i32) -> Option[i32]:
    for i, v in arr:
        if v == target: return Option.Some(i)
    return Option.None

match find_index(arr, 42):
    Option.Some(i) => io.println("found at index", i)
    Option.None    => io.println("not found")

# convenience methods
var idx := find_index(arr, 42).unwrap()          # panic if None
var idx := find_index(arr, 42).unwrap_or(-1)     # default if None
var idx := find_index(arr, 42).unwrap_or_else(fn() -> i32: compute_default())
var has := find_index(arr, 42).is_some()
var has := find_index(arr, 42).is_none()
```

---

## 14. Union Types

### 14.1 Raw Union (unsafe)

A `union` overlays all fields in the same memory. Reading the wrong variant is **undefined behavior**.  
Must be accessed inside `unsafe`.

```gpl
union IntOrFloat:
    i: i32
    f: f32

unsafe:
    var u := IntOrFloat{i: 42}
    var bits: i32 = u.i        # read as integer
    u.f = 3.14                 # write as float
    var raw: i32 = u.i         # read raw float bits as int
```

### 14.2 Tagged Union via Enum

For safe variants, use enum with data instead of raw union (see §13.3).

---

## 15. Interfaces & Traits

### 15.1 Interface Declaration

```gpl
interface Drawable:
    fn draw(self) -> void
    fn bounds(self) -> (x: f64, y: f64, w: f64, h: f64)

interface Serializable:
    fn to_bytes(self) -> []byte
    fn from_bytes(data: []byte) -> Self    # Self = implementing type

interface Resizable:
    fn resize(self: *Self, factor: f64) -> void
```

### 15.2 Implementing an Interface

```gpl
struct Rect:
    x: f64
    y: f64
    width:  f64
    height: f64

impl Drawable for Rect:
    fn draw(self) -> void:
        io.println(f"Rect at ({self.x},{self.y}) size {self.width}x{self.height}")

    fn bounds(self) -> (f64, f64, f64, f64):
        return self.x, self.y, self.width, self.height
```

### 15.3 Interface Composition

```gpl
interface Widget: Drawable, Serializable:
    fn on_click(self: *Self, x: f64, y: f64) -> void
    fn on_hover(self: *Self) -> void
```

### 15.4 Using Interfaces (Dynamic Dispatch)

```gpl
fn render_all(shapes: []Drawable) -> void:
    for s in shapes:
        s.draw()

var shapes: []Drawable = [Rect{...}, Circle{...}]
render_all(shapes)
```

### 15.5 Built-in Operator Interfaces (for overloading)

| Interface    | Operator | Method signature                       |
|--------------|----------|----------------------------------------|
| `Add`        | `+`      | `fn add(self, other: Self) -> Self`    |
| `Sub`        | `-`      | `fn sub(self, other: Self) -> Self`    |
| `Mul`        | `*`      | `fn mul(self, other: Self) -> Self`    |
| `Div`        | `/`      | `fn div(self, other: Self) -> Self`    |
| `Rem`        | `%`      | `fn rem(self, other: Self) -> Self`    |
| `Neg`        | `-` (unary) | `fn neg(self) -> Self`             |
| `BitAnd`     | `&`      | `fn bitand(self, other: Self) -> Self` |
| `BitOr`      | `\|`     | `fn bitor(self, other: Self) -> Self`  |
| `BitXor`     | `^`      | `fn bitxor(self, other: Self) -> Self` |
| `Shl`        | `<<`     | `fn shl(self, n: u32) -> Self`         |
| `Shr`        | `>>`     | `fn shr(self, n: u32) -> Self`         |
| `Comparable` | `<><=>=` | `fn cmp(self, other: Self) -> Order`   |
| `Eq`         | `==` `!=`| `fn eq(self, other: Self) -> bool`     |
| `Index`      | `[]`     | `fn index(self, i: usize) -> T`        |
| `IndexMut`   | `[]=`    | `fn index_mut(self: *Self, i: usize) -> *T` |
| `Display`    | (str)    | `fn to_string(self) -> str`            |
| `Hash`       | (map key)| `fn hash(self) -> u64`                 |
| `Drop`       | (dtor)   | `fn drop(self: *Self) -> void`         |
| `Copy`       | (copy)   | `fn copy(self) -> Self`                |
| `Iter`       | (for-in) | `fn next(self: *Self) -> Option[T]`    |

---

## 16. Operator Overloading

Operator overloading is done by implementing the corresponding interface from §15.5.

```gpl
struct Vec3:
    x: f64
    y: f64
    z: f64

impl Add for Vec3:
    fn add(self, other: Vec3) -> Vec3:
        return Vec3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }

impl Sub for Vec3:
    fn sub(self, other: Vec3) -> Vec3:
        return Vec3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }

impl Mul for Vec3:
    fn mul(self, other: Vec3) -> Vec3:
        # dot product
        return Vec3{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }

impl Neg for Vec3:
    fn neg(self) -> Vec3:
        return Vec3{x: -self.x, y: -self.y, z: -self.z}

impl Eq for Vec3:
    fn eq(self, other: Vec3) -> bool:
        return self.x == other.x and self.y == other.y and self.z == other.z

impl Display for Vec3:
    fn to_string(self) -> str:
        return f"Vec3({self.x:.3f}, {self.y:.3f}, {self.z:.3f})"

# usage
var a := Vec3{x: 1.0, y: 2.0, z: 3.0}
var b := Vec3{x: 4.0, y: 5.0, z: 6.0}
var c := a + b
var d := a - b
var e := -a
io.println(c)    # calls to_string: "Vec3(5.000, 7.000, 9.000)"

# Index overload
struct Matrix:
    data: [4][4]f64

impl Index for Matrix:
    fn index(self, i: usize) -> f64:
        return self.data[i / 4][i % 4]

impl IndexMut for Matrix:
    fn index_mut(self: *Matrix, i: usize) -> *f64:
        return &self.data[i / 4][i % 4]

var m: Matrix
m[0] = 1.0     # calls index_mut
var v := m[0]  # calls index
```

---

## 17. Error Handling

### 17.1 `Result[T, E]` Type

```gpl
# Built-in:
# enum Result[T, E = Error]:
#     Ok(value: T)
#     Err(error: E)

fn parse_int(s: str) -> Result[i32]:
    ...

match parse_int("42"):
    Result.Ok(n)  => io.println("parsed:", n)
    Result.Err(e) => io.println("error:", e.message())
```

### 17.2 The `?` Propagation Operator

Unwraps `Ok(v)` to `v`, or returns early with `Err(e)`.

```gpl
fn read_config(path: str) -> Result[Config]:
    var data   := io.read_file(path)?
    var config := Config.parse(data)?
    return Result.Ok(config)

# on Option[T], ? returns early with None
fn first_even(arr: []i32) -> Option[i32]:
    var idx := find_even_index(arr)?   # returns None if not found
    return Option.Some(arr[idx])
```

### 17.3 Custom Error Types

```gpl
## Represents a parse error with source location.
struct ParseError:
    message: str
    line:    i32
    column:  i32

impl Error for ParseError:
    fn message(self) -> str:
        return f"Parse error at {self.line}:{self.column}: {self.message}"

    fn source(self) -> ?Error:
        return null    # no underlying cause
```

### 17.4 try / catch / finally

For recoverable runtime errors and panic recovery:

```gpl
try:
    var result := risky_operation()
    io.println("result:", result)
catch e: ParseError:
    io.eprintln("parse failed:", e.message())
catch e: IOError:
    io.eprintln("IO failed:", e.message())
catch e: Error:
    io.eprintln("unexpected error:", e.message())
finally:
    cleanup()    # always runs
```

### 17.5 panic / assert

```gpl
panic("unreachable state reached")

assert(x > 0, "x must be positive, got " + x)
assert_eq(got, expected)         # assert got == expected
assert_ne(got, bad_value)        # assert got != bad_value
assert_lt(a, b)                  # assert a < b
assert_gt(a, b)
```

`assert` is compiled out in `@release` builds unless `@cfg(keep_asserts)` is set.

---

## 18. Memory Management

### 18.1 Stack vs Heap

```gpl
var x: i32 = 10                       # stack

var p: *i32 = mem.alloc[i32]()        # heap, single value
defer mem.free(p)

var buf: *[]byte = mem.alloc_array[byte](1024)   # heap array
defer mem.free(buf)
```

### 18.2 Allocator System

G uses an **injectable allocator** pattern (inspired by Zig).  
Every allocation-capable function can accept an optional allocator.

```gpl
import "std/mem"

# Global allocator (default — wraps system malloc/free)
var p := mem.alloc[MyStruct]()

# Arena allocator — fast, free all at once
var arena := mem.Arena.new(4096)
defer arena.deinit()
var a := arena.alloc[MyStruct]()
var b := arena.alloc[i32]()
# no individual frees needed — arena.deinit() frees all

# Stack allocator — allocates from a fixed stack buffer
var stack_buf: [4096]byte
var stack := mem.StackAllocator.new(&stack_buf)
var c := stack.alloc[Node]()

# Pool allocator — fixed-size object pool
var pool := mem.Pool[Connection].new(capacity: 64)
defer pool.deinit()
var conn := pool.acquire()
defer pool.release(conn)

# Custom allocator interface
interface Allocator:
    fn alloc[T](self: *Self) -> *T
    fn alloc_array[T](self: *Self, count: usize) -> *T
    fn free[T](self: *Self, ptr: *T) -> void
    fn realloc[T](self: *Self, ptr: *T, new_count: usize) -> *T
```

Passing allocator explicitly:

```gpl
fn build_tree(data: []i32, allocator: *Allocator) -> *Node:
    var node := allocator.alloc[Node]()
    ...
    return node
```

### 18.3 Weak Pointers

For breaking reference cycles without leaks:

```gpl
import "std/mem"

var strong: *Node = mem.alloc[Node]()
var weak: mem.Weak[Node] = mem.Weak.from(strong)

# Upgrading a weak pointer (may fail if object was freed)
match weak.upgrade():
    Option.Some(ptr) => io.println("alive:", ptr->value)
    Option.None      => io.println("already freed")
```

### 18.4 `unsafe` Block

All direct pointer arithmetic and raw memory operations require `unsafe`:

```gpl
unsafe:
    var raw: *void   = mem.raw_alloc(128)
    var typed: *i32  = raw as *i32
    *typed = 999
    var next: *i32   = typed + 1
    *next  = 0
    mem.raw_free(raw)
```

---

## 19. RAII & Destructors

G uses **RAII** (Resource Acquisition Is Initialization) for deterministic resource cleanup.  
When a value goes out of scope, its `Drop.drop` method is called automatically (if implemented).

### 19.1 Implementing Drop

```gpl
struct FileHandle:
    fd: i32

impl Drop for FileHandle:
    fn drop(self: *FileHandle) -> void:
        if self.fd >= 0:
            os.close(self.fd)
            self.fd = -1
```

### 19.2 Drop Order

- Variables are dropped in **reverse declaration order** (LIFO).
- Struct fields are dropped in **reverse field order** after the struct's own `drop`.
- `defer` statements run **before** the local variable drops.

```gpl
fn example() -> void:
    var a := Resource.new("A")    # acquired first
    var b := Resource.new("B")    # acquired second
    # b dropped first, then a (LIFO)
```

### 19.3 Move Semantics

By default, assigning a struct **moves** it (the original is no longer valid).  
Implement `Copy` to allow implicit copies:

```gpl
struct Buffer:
    data: *[]byte
    len:  usize

impl Drop for Buffer:
    fn drop(self: *Buffer) -> void:
        mem.free(self.data)

# move
var buf1 := Buffer.new(1024)
var buf2 := buf1              # buf1 is MOVED to buf2
# buf1 is no longer valid — compiler error if used

# explicit clone
struct Buffer:
    ...
    fn clone(self) -> Buffer:
        var new_data := mem.alloc_array[byte](self.len)
        mem.copy(new_data, self.data, self.len)
        return Buffer{data: new_data, len: self.len}

var buf3 := buf2.clone()     # explicit deep copy — both valid
```

Primitives (`i32`, `f64`, `bool`, etc.) and types implementing `Copy` are always copied:

```gpl
struct Point:
    x: f64
    y: f64

impl Copy for Point:
    fn copy(self) -> Point:
        return Point{x: self.x, y: self.y}

var p1 := Point{x: 1.0, y: 2.0}
var p2 := p1    # copy, not move — p1 still valid
```

---

## 20. Modules & Imports

### 20.1 Module Declaration

```gpl
module myapp.utils.math    # dot-separated hierarchical path

pub fn clamp(v: f64, lo: f64, hi: f64) -> f64:
    if v < lo: return lo
    if v > hi: return hi
    return v
```

### 20.2 Import Syntax

```gpl
import "std/io"                              # standard library module
import "std/io" as io                        # explicit alias
import "../utils/math"                       # relative path
import "../utils/math" as gmath              # aliased
import "std/io": {println, eprintln}         # selective: named symbols only
import "std/io": {println, eprintln, stderr} # multiple selective
import <vendor/raylib>                       # vendor/system path
```

### 20.3 Namespace Alias (`use`)

```gpl
use Direction.*      # import all variants without prefix
use StatusCode.*

# now usable without enum name:
var d := North       # instead of Direction.North
var s := OK          # instead of StatusCode.OK
```

### 20.4 Multi-line Import

```gpl
import "std/collections": {
    List,
    Map,
    Set,
    Queue,
    BinaryHeap,
}
```

### 20.5 Visibility

```gpl
pub var exported_var: i32 = 0     # exported
var private_var: i32 = 0          # module-private (default)

pub fn exported_fn() -> void: ...
fn private_fn() -> void: ...

pub struct PublicStruct:
    pub name: str     # public field
    age: i32          # private field
```

---

## 21. Generics

### 21.1 Generic Functions

```gpl
fn max[T: Comparable](a: T, b: T) -> T:
    return if a > b: a else: b

max(3, 7)
max(3.14, 2.71)
max("apple", "banana")
```

### 21.2 Generic Structs

```gpl
struct Stack[T]:
    data: []T
    size: usize

fn Stack[T].new() -> Stack[T]:
    return Stack[T]{data: [], size: 0}

fn Stack[T].push(self: *Stack[T], item: T) -> void:
    self.data = self.data.append(item)
    self.size += 1

fn Stack[T].pop(self: *Stack[T]) -> Option[T]:
    if self.size == 0: return Option.None
    self.size -= 1
    return Option.Some(self.data[self.size])

fn Stack[T].peek(self) -> Option[*T]:
    if self.size == 0: return Option.None
    return Option.Some(&self.data[self.size - 1])
```

### 21.3 Constraints (Bounds)

```gpl
interface Numeric: Add, Sub, Mul, Div, Comparable, Eq

fn dot_product[T: Numeric](a: []T, b: []T) -> T:
    assert_eq(a.len, b.len)
    var result: T = 0
    for i in 0..a.len:
        result += a[i] * b[i]
    return result

# multiple constraints
fn sorted_unique[T: Comparable + Eq + Copy](arr: []T) -> []T:
    ...

# where clause for complex constraints
fn serialize[T, W](value: T, writer: W) -> Result[void]
    where T: Serializable,
          W: io.Writer:
    ...
```

### 21.4 Multiple Type Parameters

```gpl
struct Pair[A, B]:
    first:  A
    second: B

fn swap[A, B](p: Pair[A, B]) -> Pair[B, A]:
    return Pair{first: p.second, second: p.first}

fn zip[A, B](a: []A, b: []B) -> []Pair[A, B]:
    assert_eq(a.len, b.len)
    var result: []Pair[A, B] = []
    for i in 0..a.len:
        result = result.append(Pair{first: a[i], second: b[i]})
    return result
```

### 21.5 Covariance & Contravariance

Type parameters on interfaces are **invariant** by default.

```gpl
# Covariant (out T) — interface only produces T, never consumes
interface Producer[out T]:
    fn produce(self) -> T

# Contravariant (in T) — interface only consumes T, never produces
interface Consumer[in T]:
    fn consume(self: *Self, value: T) -> void

# Invariant (default)
interface Container[T]:
    fn get(self, i: usize) -> T
    fn set(self: *Self, i: usize, value: T) -> void
```

---

## 22. Type System Extensions

### 22.1 Type Narrowing

After `is` and `match` checks, the compiler narrows the type:

```gpl
fn process(v: any) -> void:
    if v is i32:
        io.println("integer:", v + 1)    # v is i32 here
    elif v is str:
        io.println("string:", v.to_upper()) # v is str here
    elif v is []byte:
        io.println("bytes, len:", v.len)

fn handle(shape: Drawable) -> void:
    match shape:
        Circle   => io.println("circle area:", shape.area())
        Rectangle => io.println("rect area:", shape.area())
```

### 22.2 Newtype Pattern

```gpl
newtype UserId    = i32
newtype MeterId   = f64
newtype EmailAddr = str

# Newtypes are distinct — you cannot accidentally mix them
fn get_user(id: UserId) -> User: ...

var raw_id: i32 = 42
get_user(raw_id)           # ERROR: expected UserId, got i32
get_user(UserId(raw_id))   # OK: explicit construction

# Newtype methods
fn UserId.is_valid(self) -> bool:
    return (self as i32) > 0

# Newtypes can implement interfaces
impl Display for UserId:
    fn to_string(self) -> str:
        return f"UserId({self as i32})"
```

### 22.3 `never` Type

`never` is the bottom type — a subtype of every type.

```gpl
fn unreachable_branch() -> never:
    panic("this should never be reached")

fn example(x: i32) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        unreachable_branch()    # type-checks because never <: str
```

### 22.4 `any` Type

Dynamic type erasure with runtime type info:

```gpl
var v: any = 42
var v2: any = "hello"
var v3: any = Point{x: 1.0, y: 2.0}

if v is i32:
    io.println("int:", v as i32)

# type assertion (panics if wrong type)
var n: i32 = v as! i32

# safe type assertion (returns Option)
var n: Option[i32] = v as? i32
```

---

## 23. Concurrency

### 23.1 Thread Spawn

```gpl
import "std/thread"

fn worker(id: i32) -> void:
    io.println("Worker", id, "running")

var t := thread.spawn(fn() -> void: worker(1))
t.join()

var t2 := thread.spawn_result[i32](fn() -> i32: compute())
var result := t2.join()    # blocks until done
```

### 23.2 Channels

```gpl
import "std/channel"

var ch    := channel.make[i32](10)     # buffered channel, capacity 10
var unbuf := channel.make[str](0)      # unbuffered channel

ch.send(42)
var val := ch.recv()

# non-blocking try
match ch.try_recv():
    Option.Some(v) => io.println("got:", v)
    Option.None    => io.println("channel empty")

# select — wait on multiple channels
select:
    val := ch1.recv()       => io.println("from ch1:", val)
    ch2.send(99)            => io.println("sent to ch2")
    val := ch3.try_recv()   => io.println("non-blocking:", val)
    after 1000ms            => io.println("timed out")
    default                 => io.println("no channel ready")
```

### 23.3 Mutex / RWMutex

```gpl
import "std/sync"

var mu := sync.Mutex.new()
mu.lock()
defer mu.unlock()
shared_data += 1

# Read-Write mutex
var rw := sync.RWMutex.new()

rw.read_lock()
defer rw.read_unlock()
var v := shared_data    # concurrent reads OK

rw.write_lock()
defer rw.write_unlock()
shared_data = new_value  # exclusive write
```

### 23.4 Atomic Operations

```gpl
import "std/atomic"

var counter := atomic.I32.new(0)

counter.fetch_add(1)                      # add and return old value
counter.fetch_sub(1)
counter.fetch_and(0xFF)
counter.fetch_or(0x01)
counter.store(100)
var v := counter.load()
var ok := counter.compare_exchange(expected: 5, new: 10)  # CAS
```

### 23.5 Thread Safety Annotations

```gpl
@thread_safe
fn safe_push(list: *SyncList, item: i32) -> void: ...

@not_thread_safe
fn fast_push(list: *List, item: i32) -> void: ...

@guarded_by(mu)
var shared_buffer: []byte

@requires_lock(mu)
fn modify_buffer() -> void: ...
```

---

## 24. Macros & Compile-time

### 24.1 `comptime` Expressions

```gpl
comptime var PLATFORM  := @os_name()
comptime var ARCH      := @arch()
comptime var IS_DEBUG  := @build_mode() == "debug"
comptime var IS_64BIT  := @size_of(usize) == 8
```

### 24.2 Conditional Compilation

```gpl
@if OS == "windows":
    fn clear_screen() -> void: os.exec("cls")
@elif OS == "linux" or OS == "macos":
    fn clear_screen() -> void: os.exec("clear")
@else:
    fn clear_screen() -> void: pass

# feature flags
@cfg(feature = "networking"):
    import "std/net"
    pub fn fetch(url: str) -> Result[str]: ...

@cfg(not(feature = "networking")):
    pub fn fetch(url: str) -> Result[str]:
        return Result.Err(Error.new("networking feature not enabled"))
```

### 24.3 Built-in Compile-time Macros

```gpl
@size_of(T)            # size of type T in bytes -> usize
@align_of(T)           # alignment of type T -> usize
@offset_of(T, field)   # byte offset of field in struct -> usize
@type_name(T)          # name of type T -> str  (compile-time)
@type_id(T)            # unique numeric ID for type T -> u64
@has_method(T, name)   # true if T has method `name` -> bool
@implements(T, I)      # true if T implements interface I -> bool
@file                  # current source filename -> str
@line                  # current line number -> i32
@col                   # current column number -> i32
@fn_name               # current function name -> str
@module                # current module name -> str
@os_name()             # target OS -> str
@arch()                # target arch -> str ("x86_64", "arm64", ...)
@build_mode()          # "debug" | "release" | "test"
@endian()              # "little" | "big"
@bit_cast[T](v)        # reinterpret bits of v as T (unsafe)
@unreachable()         # undefined behavior in release, panic in debug
@todo(msg)             # compiler warning; error in release
@unimplemented(msg)    # panic at runtime
```

### 24.4 `const fn` (Compile-time Evaluation)

```gpl
const fn pow(base: i64, exp: u32) -> i64:
    if exp == 0: return 1
    return base * pow(base, exp - 1)

const TABLE_SIZE: i64 = pow(2, 16)   # = 65536, computed at compile-time
```

### 24.5 Declarative Macros

```gpl
macro vec!(items):
    collections.List.from([items])

macro map!(pairs):
    collections.Map.from_pairs([pairs])

macro assert_approx!(a, b, eps):
    assert(abs(a - b) < eps, f"expected {a} ≈ {b} (eps={eps})")

# usage
var v := vec![1, 2, 3, 4, 5]
var m := map!["a": 1, "b": 2]
assert_approx!(result, 3.14159, 0.001)
```

---

## 25. Inline Assembly

Inline assembly requires an `unsafe` block. Uses a structured syntax for inputs/outputs/clobbers.

```gpl
unsafe:
    var result: u64
    var a: u64 = 10
    var b: u64 = 20
    asm:
        "mov rax, {0}"
        "add rax, {1}"
        "mov {2}, rax"
        : out(result)
        : in(a), in(b)
        : clobber("rax", "flags")

# CPUID example
unsafe:
    var eax, ebx, ecx, edx: u32
    asm:
        "cpuid"
        : out(eax), out(ebx), out(ecx), out(edx)
        : in(eax = 0)
        : clobber("memory")

# SIMD intrinsic
@target_feature("avx2")
unsafe:
    var a_vec: *f32 = &data[0]
    asm:
        "vmovups ymm0, [{0}]"
        "vmulps  ymm0, ymm0, ymm0"
        "vmovups [{0}], ymm0"
        : : in(a_vec) : clobber("ymm0", "memory")
```

---

## 26. Type Casting

```gpl
# Safe widening cast (always succeeds)
var x: i32 = 100
var y: i64 = x as i64        # i32 -> i64, always safe

# Narrowing cast (may truncate — explicit, visible)
var big: i64 = 100000
var small: i16 = big as i16  # truncates — must be explicit

# Float <-> int
var f: f64 = 3.99
var i: i32 = f as i32        # truncates toward zero: 3

# Numeric to string
var s: str = 42 as str       # ERROR: use formatting instead
var s: str = f"{42}"         # correct

# Bit reinterpret (unsafe)
var f32val: f32 = 3.14
var bits: u32
unsafe:
    bits = @bit_cast[u32](f32val)

# Pointer casts (unsafe)
unsafe:
    var vp: *void = p as *void
    var ip: *i32  = vp as *i32

# Type assertion (any -> T)
var v: any = 42
var n: i32   = v as! i32     # panics if wrong type
var n: ?i32  = v as? i32     # returns null if wrong type
```

---

## 27. Attributes & Annotations

```gpl
# Functions
@inline            fn fast() -> void: ...
@no_inline         fn always_call() -> void: ...
@cold              fn error_path() -> void: ...
@hot               fn critical_loop() -> void: ...
@noreturn          fn exit_now() -> never: ...
@must_use          fn result_fn() -> i32: ...    # warn if return value ignored
@pure              fn no_side_effects(x: i32) -> i32: ...
@deprecated("use new_fn() instead")  fn old_fn() -> void: ...
@since("0.2.0")    fn new_fn() -> void: ...
@see("other_fn")   fn related_fn() -> void: ...
@extern("C")       fn c_func(x: i32) -> i32     # link to C symbol
@export            fn for_c_consumers() -> void: ...
@link_name("_my_c_func")  fn wrapper() -> void: ...

# Structs
@packed            struct WireFormat: ...
@align(16)         struct SIMDData: ...
@align(64)         struct CacheLinePadded: ...
@repr("C")         struct CCompatible: ...       # C-compatible layout

# Thread safety
@thread_safe       fn safe_fn() -> void: ...
@not_thread_safe   fn unsafe_fn() -> void: ...
@guarded_by(lock)  var shared_value: i32 = 0
@requires_lock(mu) fn locked_fn() -> void: ...

# Optimization hints
@likely            if condition: ...    # branch prediction hint
@unlikely          if condition: ...
@bounds_check(false)  fn fast_loop() -> void: ...
@target_feature("avx2")  fn simd_fn() -> void: ...

# Testing
@test              fn test_addition() -> void: ...
@bench             fn bench_sort() -> void: ...
@test_only         fn helper_only_in_tests() -> void: ...
@skip("flaky on CI")  fn skipped_test() -> void: ...
@timeout(5000ms)   fn slow_test() -> void: ...

# Conditional compilation
@cfg(OS == "linux")        fn linux_only() -> void: ...
@cfg(feature = "ssl")      fn ssl_fn() -> void: ...
@cfg(debug)                fn debug_only() -> void: ...
@cfg(release)              fn release_only() -> void: ...
```

---

## 28. Testing & Benchmarks

### 28.1 Unit Tests

```gpl
@test
fn test_add() -> void:
    var result := add(2, 3)
    assert_eq(result, 5)

@test
fn test_string_interpolation() -> void:
    var name := "World"
    var s := f"Hello, {name}!"
    assert_eq(s, "Hello, World!")

@test
fn test_error_propagation() -> void:
    var result := parse_int("not_a_number")
    assert(result.is_err(), "expected Err, got Ok")
```

### 28.2 Table-driven Tests

```gpl
@test
fn test_clamp() -> void:
    struct Case:
        input: f64
        lo:    f64
        hi:    f64
        want:  f64

    var cases := [
        Case{input: -1.0, lo: 0.0, hi: 1.0, want: 0.0},
        Case{input:  0.5, lo: 0.0, hi: 1.0, want: 0.5},
        Case{input:  2.0, lo: 0.0, hi: 1.0, want: 1.0},
    ]

    for c in cases:
        var got := clamp(c.input, c.lo, c.hi)
        assert_eq(got, c.want)
```

### 28.3 Benchmarks

```gpl
@bench
fn bench_sort_1000() -> void:
    var data := generate_random_slice(1000)
    data.sort()    # benchmarked code

@bench
@timeout(10000ms)
fn bench_hash_map_insert() -> void:
    var m: map[str]i32 = {}
    for i in 0..10000:
        m[f"key_{i}"] = i
```

Run tests: `gpl test`  
Run benchmarks: `gpl bench`  
Run specific: `gpl test --filter test_add`

### 28.4 Built-in Assertions (test mode)

```gpl
assert_eq(a, b)            # a == b
assert_ne(a, b)            # a != b
assert_lt(a, b)            # a < b
assert_le(a, b)            # a <= b
assert_gt(a, b)            # a > b
assert_ge(a, b)            # a >= b
assert_approx(a, b, eps)   # |a - b| < eps
assert_contains(s, sub)    # string contains substring
assert_matches(s, pattern) # string matches regex pattern
assert_panics(fn() -> void: risky())  # block must panic
assert_no_panic(fn() -> void: safe()) # block must not panic
```

---

## 29. Standard Operators Summary

| Category          | Operators                                                                        |
|-------------------|----------------------------------------------------------------------------------|
| Arithmetic        | `+` `-` `*` `/` `%` `**` `-` (unary)                                           |
| Overflow-explicit | `+%` `-%` `*%` `+\|` `-\|` `*\|` `+!` `-!` `*!`                               |
| Comparison        | `==` `!=` `<` `>` `<=` `>=`                                                     |
| Logical           | `and` `or` `not`                                                                 |
| Bitwise           | `&` `\|` `^` `~` `<<` `>>`                                                      |
| Assignment        | `=` `+=` `-=` `*=` `/=` `%=` `**=` `&=` `\|=` `^=` `<<=` `>>=` `+%=` `+\|=`  |
| Memory            | `&` (addr-of) `*` (deref) `->` (ptr field)                                      |
| Range             | `..` (exclusive) `..=` (inclusive)                                               |
| Null-safety       | `??` (coalesce) `?.` (optional chain) `?` (propagate)                           |
| Type              | `as` (cast) `as!` (assert cast) `as?` (safe cast) `is` (type check)            |
| Spread            | `...` (variadic spread)                                                          |
| Pipe              | `\|>` (pipe: `x \|> f` ≡ `f(x)`)                                               |
| Ignore            | `_` (discard)                                                                    |

---

## Appendix A — Full `.gpl` Example

```gpl
module main

import "std/io"
import "std/mem"
import "std/collections"

# --- Types ---

enum Direction:
    North
    South
    East
    West

struct Player:
    name: str
    x:    i32
    y:    i32
    hp:   i32

impl Display for Player:
    fn to_string(self) -> str:
        return f"{self.name} at ({self.x},{self.y}) HP:{self.hp}"

impl Drop for Player:
    fn drop(self: *Player) -> void:
        io.println(f"[Drop] Player '{self.name}' removed from memory")

# --- Methods ---

fn Player.new(name: str) -> Player:
    return Player{name: name, x: 0, y: 0, hp: 100}

fn Player.move(self: *Player, dir: Direction) -> void:
    match dir:
        Direction.North => self.y -= 1
        Direction.South => self.y += 1
        Direction.East  => self.x += 1
        Direction.West  => self.x -= 1

fn Player.is_alive(self) -> bool:
    return self.hp > 0

fn Player.take_damage(self: *Player, amount: i32) -> void:
    self.hp = (self.hp - amount) +| 0    # saturating sub — never below 0

# --- Generic utility ---

fn repeat[T: Copy](value: T, times: i32) -> []T:
    var result: []T = []
    for _ in 0..times:
        result = result.append(value)
    return result

# --- Entry Point ---

fn main() -> i32:
    var p := Player.new("Hero")

    var moves := repeat(Direction.East, 3)

    for dir in moves:
        p.move(dir)
        io.println(p)

    p.take_damage(30)
    io.println(f"After damage: {p}")

    if p.is_alive():
        io.println("Player survived!")
    else:
        io.println("Player has fallen.")

    return 0
    # p is dropped here automatically (Drop.drop called)
```

---

## Appendix B — Full `.hdr` Example

```hdr
module player

pub const MAX_HP:    i32 = 100
pub const MAX_LEVEL: i32 = 99

pub enum Direction:
    North
    South
    East
    West

pub struct Player:
    pub name: str
    pub x:    i32
    pub y:    i32
    pub hp:   i32

pub fn Player.new(name: str) -> Player
pub fn Player.move(self: *Player, dir: Direction) -> void
pub fn Player.is_alive(self) -> bool
pub fn Player.take_damage(self: *Player, amount: i32) -> void
pub fn Player.to_string(self) -> str
```

---

## Appendix C — Design Decisions

### C.1 Integer Overflow Behavior

| Mode         | Default behavior     | Override         |
|--------------|----------------------|------------------|
| Debug build  | Panic on overflow    | Use `+%` / `+\|` |
| Release build| Wrapping (2's comp.) | Use `+%` / `+\|` |

Rationale: panic in debug catches bugs early; wrapping in release for performance. Explicit `+%`, `+|`, `+!` operators are always available for intentional behavior regardless of build mode.

### C.2 RAII vs Manual Memory

G uses **RAII** as the primary resource management strategy:
- Values with `impl Drop` are automatically cleaned up at end of scope.
- Raw heap allocations via `mem.alloc` are **manual** — use `defer mem.free(p)`.
- Allocators can be custom (arena, pool, stack) — see §18.2.
- No garbage collector. No reference counting in the core language (available as library).

### C.3 Allocator Strategy

G uses **injectable allocators** (Zig-inspired):
- Global allocator is the default (wraps `malloc`/`free`).
- Arena, pool, and stack allocators are in `std/mem`.
- Functions that allocate can accept an explicit `allocator` parameter.
- Allocator interface is defined in `std/mem` — fully customizable.

### C.4 Move vs Copy Semantics

- Structs are **moved** by default (no implicit copies).
- Primitives and types implementing `Copy` are always copied.
- `clone()` is the idiomatic explicit deep copy.
- Rationale: prevents accidental copies of large data; makes ownership explicit.

### C.5 Error Handling Philosophy

G has **two** error handling mechanisms by design:
1. **`Result[T]` + `?`** — for expected, recoverable errors (parsing, IO, network).
2. **`panic` + `try/catch`** — for unexpected programmer errors (invariant violations, unreachable code).

Do not use `panic` for recoverable errors. Do not use `Result` for programmer bugs.

---

---

## 30. Numeric Literals

### 30.1 Integer Literals

```gpl
# Decimal
var a: i32 = 1000000
var b: i32 = 1_000_000      # underscores as visual separators (ignored)
var c: i32 = 1_00_0_000     # any grouping allowed

# Hexadecimal (prefix 0x)
var h1: u32 = 0xFF
var h2: u32 = 0xDEAD_BEEF
var h3: u32 = 0xdead_beef   # lowercase also valid

# Binary (prefix 0b)
var b1: u8 = 0b1010_1010
var b2: u8 = 0b11111111

# Octal (prefix 0o)
var o1: u32 = 0o777
var o2: u32 = 0o644

# Type suffix
var s1 := 100u8             # explicit u8
var s2 := 100i64            # explicit i64
var s3 := 100usize          # explicit usize
var s4 := 0xFFu32           # hex with type suffix
```

### 30.2 Float Literals

```gpl
var f1: f64 = 3.14
var f2: f64 = 3.14_159_265
var f3: f64 = 1.5e10        # scientific: 1.5 × 10^10
var f4: f64 = 1.5e-3        # scientific: 0.0015
var f5: f64 = 1.5E+10       # capital E also valid
var f6: f64 = .5            # leading dot: 0.5
var f7: f64 = 5.            # trailing dot: 5.0

# Type suffix
var f8  := 3.14f32          # explicit f32
var f9  := 3.14f64          # explicit f64 (default for float literals)

# Special values (via std/math)
import "std/math"
var inf  := math.INF_F64
var nan  := math.NAN_F64
var ninf := math.NEG_INF_F64
```

### 30.3 Character Literals

```gpl
var c1: rune = 'A'
var c2: rune = '€'          # unicode character
var c3: rune = '\n'         # escape sequence
var c4: rune = '\u{1F600}'  # unicode code point (emoji 😀)
var c5: rune = '\x41'       # hex byte: 'A'

var b1: byte = b'A'         # byte literal (u8), ASCII only
```

### 30.4 String Escape Sequences

| Escape     | Meaning                          |
|------------|----------------------------------|
| `\n`       | Newline (LF, U+000A)             |
| `\r`       | Carriage Return (CR, U+000D)     |
| `\t`       | Horizontal Tab (U+0009)          |
| `\\`       | Backslash                        |
| `\"`       | Double quote                     |
| `\'`       | Single quote                     |
| `\0`       | Null character (U+0000)          |
| `\a`       | Bell (U+0007)                    |
| `\b`       | Backspace (U+0008)               |
| `\f`       | Form Feed (U+000C)               |
| `\v`       | Vertical Tab (U+000B)            |
| `\xHH`     | Byte value in hex (e.g. `\x41` = `A`) |
| `\u{HHHH}` | Unicode code point (1–6 hex digits) |
| `\{expr}`  | Interpolated expression (in `f""` strings only) |

Raw strings `r"..."` treat all characters literally — no escapes processed.

---

## 31. Operator Precedence

Operators are listed from **highest** (tightest binding) to **lowest** (loosest binding).  
Operators on the same row have equal precedence and follow the associativity shown.

| Level | Operators                                       | Associativity |
|-------|-------------------------------------------------|---------------|
| 14    | `()` `[]` `.` `->` `?.` function call           | Left          |
| 13    | `-` (unary) `~` `not` `&` (addr-of) `*` (deref) `as` `as!` `as?` | Right (unary) |
| 12    | `**`                                            | Right         |
| 11    | `*` `/` `%` `*%` `*\|` `*!`                    | Left          |
| 10    | `+` `-` `+%` `-%` `+\|` `-\|` `+!` `-!`        | Left          |
| 9     | `<<` `>>`                                       | Left          |
| 8     | `&` (bitwise AND)                               | Left          |
| 7     | `^` (bitwise XOR)                               | Left          |
| 6     | `\|` (bitwise OR)                               | Left          |
| 5     | `..` `..=`                                      | Non-assoc     |
| 4     | `==` `!=` `<` `>` `<=` `>=` `is`               | Non-assoc     |
| 3     | `and`                                           | Left          |
| 2     | `or`                                            | Left          |
| 1     | `??`                                            | Right         |
| 0     | `\|>` `=` `+=` `-=` `*=` `/=` `%=` etc.        | Right         |

**Notes:**
- `not` binds tighter than `and`/`or`: `not a and b` = `(not a) and b`
- `is` is non-associative: `a is T is U` is a compile error; write `(a is T) and (a is U)`
- `..` and `..=` are non-associative: `1..2..3` is a compile error
- When in doubt, use parentheses — the compiler will warn on ambiguous expressions

```gpl
# Examples of precedence in action:
2 + 3 * 4          # 14  (not 20)
not x and y        # (not x) and y
a | b & c          # a | (b & c)
x ?? y ?? z        # x ?? (y ?? z)   (right-assoc)
x |> f |> g        # (x |> f) |> g  (left-assoc, but same result)
a == b and c != d  # (a == b) and (c != d)
1..10              # range, not arithmetic
```

---

## 32. Iterator Protocol & Functional Methods

### 32.1 The `Iter` Interface

```gpl
interface Iter[T]:
    fn next(self: *Self) -> Option[T]

    # default methods (implemented in terms of next())
    fn map[U](self: *Self, f: fn(T) -> U) -> MapIter[Self, T, U]
    fn filter(self: *Self, pred: fn(T) -> bool) -> FilterIter[Self, T]
    fn reduce(self: *Self, f: fn(T, T) -> T) -> Option[T]
    fn fold[B](self: *Self, init: B, f: fn(B, T) -> B) -> B
    fn collect[C: FromIter[T]](self: *Self) -> C
    fn for_each(self: *Self, f: fn(T) -> void) -> void
    fn find(self: *Self, pred: fn(T) -> bool) -> Option[T]
    fn find_map[U](self: *Self, f: fn(T) -> Option[U]) -> Option[U]
    fn position(self: *Self, pred: fn(T) -> bool) -> Option[usize]
    fn any(self: *Self, pred: fn(T) -> bool) -> bool
    fn all(self: *Self, pred: fn(T) -> bool) -> bool
    fn count(self: *Self) -> usize
    fn sum(self: *Self) -> T           # T must implement Add + Zero
    fn product(self: *Self) -> T       # T must implement Mul + One
    fn min(self: *Self) -> Option[T]   # T must implement Comparable
    fn max(self: *Self) -> Option[T]
    fn enumerate(self: *Self) -> EnumerateIter[Self, T]
    fn zip[U](self: *Self, other: *impl Iter[U]) -> ZipIter[Self, T, U]
    fn chain(self: *Self, other: *impl Iter[T]) -> ChainIter[Self, T]
    fn take(self: *Self, n: usize) -> TakeIter[Self, T]
    fn skip(self: *Self, n: usize) -> SkipIter[Self, T]
    fn take_while(self: *Self, pred: fn(T) -> bool) -> TakeWhileIter[Self, T]
    fn skip_while(self: *Self, pred: fn(T) -> bool) -> SkipWhileIter[Self, T]
    fn flat_map[U](self: *Self, f: fn(T) -> impl Iter[U]) -> FlatMapIter[Self, T, U]
    fn flatten(self: *Self) -> FlattenIter[Self, T]  # T must be Iter
    fn peekable(self: *Self) -> PeekIter[Self, T]
    fn rev(self: *Self) -> RevIter[Self, T]          # Self must be DoubleEndedIter
    fn cloned(self: *Self) -> CopiedIter[Self, T]    # T must implement Copy
    fn nth(self: *Self, n: usize) -> Option[T]
    fn last(self: *Self) -> Option[T]
    fn step_by(self: *Self, step: usize) -> StepByIter[Self, T]
    fn windows(self: *Self, size: usize) -> WindowsIter[Self, T]
    fn chunks(self: *Self, size: usize) -> ChunksIter[Self, T]
```

### 32.2 `for-in` Uses `Iter`

Any type implementing `Iter[T]` can be used in a `for` loop:

```gpl
impl Iter[i32] for MyRange:
    fn next(self: *MyRange) -> Option[i32]:
        if self.current >= self.end:
            return Option.None
        var v := self.current
        self.current += 1
        return Option.Some(v)

for x in MyRange{current: 0, end: 5}:
    io.println(x)
```

### 32.3 Functional Pipeline

```gpl
var numbers := [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# map + filter + collect
var evens_squared: []i32 = numbers
    .iter()
    .filter(fn(x: i32) -> bool: x % 2 == 0)
    .map(fn(x: i32) -> i32: x * x)
    .collect()
# result: [4, 16, 36, 64, 100]

# sum of first 5 positive elements
var total := numbers
    .iter()
    .filter(fn(x: i32) -> bool: x > 0)
    .take(5)
    .sum()

# fold to build a string
var sentence := ["hello", "world", "from", "G"]
    .iter()
    .fold("", fn(acc: str, w: str) -> str: acc + " " + w)
    .trim()

# enumerate + for_each
numbers.iter()
    .enumerate()
    .for_each(fn(i: usize, v: i32) -> void:
        io.println(f"[{i}] = {v}"))

# zip two slices
var keys   := ["a", "b", "c"]
var values := [1,   2,   3  ]
var pairs: []Pair[str, i32] = keys
    .iter()
    .zip(values.iter())
    .collect()

# flatten
var nested := [[1, 2], [3, 4], [5, 6]]
var flat: []i32 = nested.iter().flatten().collect()

# chaining iterators
var a := [1, 2, 3]
var b := [4, 5, 6]
var combined: []i32 = a.iter().chain(b.iter()).collect()
```

### 32.4 `FromIter` Interface (for `collect()`)

```gpl
interface FromIter[T]:
    fn from_iter(iter: *impl Iter[T]) -> Self

# Built-in implementations:
# []T       implements FromIter[T]
# set[T]    implements FromIter[T]
# map[K]V   implements FromIter[Pair[K, V]]
# str       implements FromIter[rune]
```

### 32.5 Lazy Evaluation

All iterator adapters (`map`, `filter`, `take`, etc.) are **lazy** — they do not compute anything until consumed by a terminal operation (`collect`, `for_each`, `sum`, `count`, `fold`, etc.).

```gpl
# This creates NO intermediate arrays — fully lazy pipeline:
var result := (0..1_000_000)
    .iter()
    .filter(fn(x: i32) -> bool: x % 3 == 0)
    .map(fn(x: i32) -> i32: x * x)
    .take(10)
    .collect()    # only here does computation happen
```

---

## 33. Lifetime & Borrow Semantics

G uses a **simplified borrow model** — less strict than Rust, but with clear rules enforced by the compiler.

### 33.1 Ownership Rules

1. Every value has exactly **one owner** at a time.
2. When the owner goes out of scope, the value is **dropped**.
3. Ownership can be **transferred (moved)** or **borrowed**.

### 33.2 Borrows

```gpl
var x: i32 = 42

# Immutable borrow — read-only reference
var r: &i32 = &x          # borrow x
io.println(*r)             # read through borrow
# x is still valid — borrow ends when r goes out of scope

# Mutable borrow — read-write reference
var m: &mut i32 = &mut x  # mutable borrow
*m = 100                   # modify through borrow
# x == 100 after this
```

Borrow rules (checked by compiler):
- Any number of **immutable borrows** may coexist.
- Only **one mutable borrow** may exist at a time.
- You cannot have an immutable AND mutable borrow at the same time.

```gpl
var a: i32 = 5
var r1: &i32     = &a      # OK: immutable borrow
var r2: &i32     = &a      # OK: second immutable borrow
var m:  &mut i32 = &mut a  # ERROR: cannot borrow as mutable while
                            #        immutable borrows exist
```

### 33.3 Lifetime Annotations

Lifetimes are usually **inferred** by the compiler. Explicit annotation is only needed when the compiler cannot determine the relationship between input and output borrows.

```gpl
# Syntax: 'name  (tick + name)

# Function returning a borrow — must say whose lifetime it comes from
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str:
    if x.len >= y.len: return x
    return y

# Struct holding a borrow — must annotate
struct Parser<'a>:
    source: &'a str
    pos:    usize

fn Parser<'a>.new(src: &'a str) -> Parser<'a>:
    return Parser{source: src, pos: 0}
```

### 33.4 Lifetime Elision Rules

In most cases, lifetimes are elided (omitted) using these rules:

1. Each borrow parameter gets its own lifetime.
2. If there is exactly one borrow input parameter, its lifetime is assigned to all outputs.
3. If one of the parameters is `&self` or `&mut self`, its lifetime is assigned to outputs.

```gpl
# These are equivalent:
fn first(s: &str) -> &str: ...
fn first<'a>(s: &'a str) -> &'a str: ...

fn Parser.current(self: &Parser) -> &str: ...
fn Parser.current<'a>(self: &'a Parser) -> &'a str: ...
```

### 33.5 `'static` Lifetime

The `'static` lifetime means the value lives for the entire duration of the program.

```gpl
const MESSAGE: &'static str = "Hello, World!"

fn get_greeting() -> &'static str:
    return "Hi there!"    # string literals are 'static
```

---

## 34. FFI — C Interoperability

### 34.1 Calling C Functions

```gpl
# Declare external C function
@extern("C")
fn printf(fmt: *const byte, ...) -> i32

@extern("C")
fn malloc(size: usize) -> *void

@extern("C")
fn free(ptr: *void) -> void

@extern("C")
@link_name("strlen")
fn c_strlen(s: *const byte) -> usize

# Usage (inside unsafe)
unsafe:
    var n := printf("Hello from C\n")
    var p := malloc(64) as *byte
    free(p as *void)
```

### 34.2 C-Compatible Structs

```gpl
@repr("C")
struct CPoint:
    x: f64
    y: f64

@repr("C")
struct CString:
    ptr: *const byte
    len: usize

@repr("C")
@packed
struct CHeader:
    magic:   u32
    version: u16
    flags:   u8
    padding: u8
```

### 34.3 `str` ↔ C string (`*const byte`)

```gpl
import "std/ffi"

# G str -> null-terminated C string (*const byte)
var gs: str = "hello"
var cs: *const byte = ffi.str_to_cstr(gs)      # heap-allocated, must free
defer ffi.free_cstr(cs)

# C string -> G str (borrows, no copy)
var gs2: str = ffi.cstr_to_str(cs)             # valid while cs is alive

# C string -> G str (owns, copies data)
var gs3: str = ffi.cstr_to_str_owned(cs)
```

### 34.4 Exporting G Functions to C

```gpl
@export
@extern("C")
fn g_add(a: i32, b: i32) -> i32:
    return a + b

@export
@extern("C")
@link_name("g_create_point")
fn create_point(x: f64, y: f64) -> CPoint:
    return CPoint{x: x, y: y}
```

### 34.5 Linking External Libraries

Specify in `gpl.toml` (see §36), or inline:

```gpl
@link("m")       # link libm  (-lm)
@link("pthread") # link libpthread
@link("ssl")     # link libssl

fn sin_extern(x: f64) -> f64  # from libm
```

---

## 35. Async / Await

G supports asynchronous programming via `async`/`await` for non-blocking I/O, built on a cooperative task scheduler.

### 35.1 Async Functions

```gpl
import "std/async"

async fn fetch_data(url: str) -> Result[str]:
    var response := await http.get(url)?
    var body     := await response.read_body()?
    return Result.Ok(body)
```

An `async fn` returns a `Task[T]` (a future/promise). It does not execute until awaited or spawned.

### 35.2 `await`

`await` can only be used inside an `async fn`. It suspends the current task until the awaited `Task[T]` completes.

```gpl
async fn main_async() -> i32:
    var result := await fetch_data("https://example.com")?
    io.println(result)
    return 0
```

### 35.3 Running the Async Runtime

```gpl
fn main() -> i32:
    return async.run(main_async())   # blocks until main_async completes
```

### 35.4 Concurrent Tasks

```gpl
async fn parallel_fetch() -> void:
    # spawn two concurrent tasks
    var t1 := async.spawn(fetch_data("https://site1.com"))
    var t2 := async.spawn(fetch_data("https://site2.com"))

    # await both
    var r1 := await t1
    var r2 := await t2

    io.println(r1, r2)

# await all (concurrent, waits for all to finish)
var results := await async.all([
    fetch_data("https://a.com"),
    fetch_data("https://b.com"),
    fetch_data("https://c.com"),
])

# await any (returns first to complete)
var first := await async.any([task1, task2, task3])

# with timeout
var result := await async.timeout(fetch_data(url), 5000ms)
```

### 35.5 Async Iterators

```gpl
async fn read_lines(path: str) -> AsyncIter[str]:
    var f := await io.open_async(path)?
    defer await f.close()
    while await f.has_next():
        yield await f.read_line()

async fn process() -> void:
    async for line in read_lines("data.txt"):
        io.println(line)
```

### 35.6 Async/Sync Boundary

Async functions **cannot** be called from sync context without `async.run()` or `async.block_on()`.

```gpl
# sync context
fn sync_caller() -> void:
    var result := async.block_on(fetch_data(url))   # blocks current thread
    io.println(result)
```

---

## 36. Build System — `gpl.toml`

Every G project has a `gpl.toml` at the project root.

### 36.1 Minimal `gpl.toml`

```toml
[package]
name    = "myapp"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
license = "MIT"

[build]
entry = "src/main.gpl"
```

### 36.2 Full `gpl.toml`

```toml
[package]
name        = "myapp"
version     = "1.2.3"
authors     = ["Alice <alice@example.com>", "Bob <bob@example.com>"]
description = "A sample G application"
license     = "MIT OR Apache-2.0"
repository  = "https://github.com/user/myapp"
homepage    = "https://myapp.example.com"
keywords    = ["cli", "utility"]

[build]
entry       = "src/main.gpl"          # entry point file
output      = "bin/myapp"             # output binary name
target      = "x86_64-linux"          # default target (optional)
std         = "2024"                  # G std version to use

[build.options]
optimization = "speed"                # "none" | "size" | "speed" | "debug"
debug_info   = true
strip        = false
lto          = false                  # link-time optimization
pic          = false                  # position-independent code

[features]
networking = []                       # feature flag with no dependencies
ssl        = ["networking", "dep:openssl"]
full       = ["networking", "ssl"]
default    = ["networking"]           # enabled by default

[dependencies]
raylib   = { version = "5.0", source = "registry" }
openssl  = { version = "3.x", source = "system",   optional = true }
myutils  = { path = "../myutils" }    # local path dependency
jsonlib  = { git = "https://github.com/user/jsonlib", tag = "v1.2.0" }

[dev-dependencies]                    # only for tests and benchmarks
faker    = { version = "1.0" }

[target.x86_64-linux]
link     = ["m", "pthread"]

[target.x86_64-windows]
link     = ["ws2_32", "userenv"]
output   = "bin/myapp.exe"

[target.aarch64-macos]
link     = []

[profile.debug]
optimization = "none"
debug_info   = true
overflow     = "panic"                # integer overflow behavior
bounds_check = true

[profile.release]
optimization = "speed"
debug_info   = false
strip        = true
lto          = true
overflow     = "wrap"
bounds_check = false
```

### 36.3 Workspace `gpl.toml` (monorepo)

```toml
[workspace]
members = [
    "packages/core",
    "packages/net",
    "packages/cli",
    "tools/codegen",
]

[workspace.dependencies]
# shared dependency versions across all members
raylib = { version = "5.0" }
```

---

## 37. CLI Toolchain — `gpl`

The G compiler and toolchain is accessed via the `gpl` command.

### 37.1 Command Overview

```
gpl <command> [options] [files...]

Commands:
  run       Compile and run immediately (no output binary saved)
  build     Compile to binary
  check     Type-check without producing output
  test      Compile and run tests
  bench     Compile and run benchmarks
  fmt       Format source code
  doc       Generate HTML documentation
  clean     Remove build artifacts
  init      Create a new project
  new       Create a new project in a new directory
  add       Add a dependency
  remove    Remove a dependency
  update    Update dependencies
  install   Install a binary from a package
  version   Print gpl version
  help      Print help

Options (global):
  -v, --verbose         Verbose output
  -q, --quiet           Suppress non-error output
      --color <on|off|auto>  Color output
```

### 37.2 `gpl run` — Compile & Execute

```bash
# Run a single file (no gpl.toml needed)
gpl run main.gpl
gpl run main.gpl arg1 arg2          # pass arguments to the program

# Run the project (uses gpl.toml entry point)
gpl run
gpl run -- arg1 arg2                # arguments after --

# Options
gpl run main.gpl --release          # run with release optimizations
gpl run main.gpl --debug            # run with debug info (default)
gpl run main.gpl --no-check         # skip type-check, just run
gpl run main.gpl --emit-ir          # dump IR before running
gpl run main.gpl --time             # print compile + run time
```

### 37.3 `gpl build` — Compile to Binary

```bash
# Build current project (uses gpl.toml)
gpl build
gpl build --release                 # release profile
gpl build --debug                   # debug profile (default)

# Single file (GCC-style)
gpl build main.gpl                  # output: ./main (or main.exe)
gpl build main.gpl -o myapp         # specify output name
gpl build main.gpl -o bin/myapp     # specify output path

# Multiple files
gpl build src/main.gpl src/utils.gpl -o myapp

# Cross-compilation
gpl build --target x86_64-windows   # cross-compile to Windows
gpl build --target aarch64-linux    # cross-compile to ARM Linux
gpl build --target wasm32           # compile to WebAssembly

# Output type
gpl build --type exe                # executable (default)
gpl build --type lib                # static library (.a / .lib)
gpl build --type dylib              # dynamic library (.so / .dll)
gpl build --type obj                # object file only (.o)

# Optimization
gpl build -O0                       # no optimization
gpl build -O1                       # light optimization
gpl build -O2                       # standard optimization (default release)
gpl build -O3                       # aggressive optimization
gpl build -Os                       # optimize for size

# Debug info
gpl build -g                        # include debug info
gpl build -g0                       # no debug info

# Linking
gpl build -lm -lpthread             # link libraries (GCC-style)
gpl build -L/usr/local/lib -lraylib # custom library path

# Preprocessor-style defines
gpl build -D FEATURE_SSL            # define feature flag
gpl build -D MAX_CONNECTIONS=128    # define constant

# Include paths
gpl build -I ./include              # add include path for .hdr files
gpl build -I /usr/local/include

# Warnings
gpl build -W all                    # enable all warnings
gpl build -W error                  # treat warnings as errors
gpl build -W none                   # disable all warnings

# Features (from gpl.toml)
gpl build --features ssl,networking
gpl build --no-default-features
gpl build --all-features

# Verbose / diagnostics
gpl build --verbose                 # show each compile step
gpl build --emit asm                # output assembly (.s files)
gpl build --emit ir                 # output intermediate representation
gpl build --emit obj                # output object files
gpl build --emit all                # emit all intermediate outputs
gpl build --timings                 # print per-file compile times
```

### 37.4 `gpl check` — Type-check Only

```bash
gpl check                           # check project
gpl check main.gpl                  # check single file
gpl check --all                     # check all files including tests
```

### 37.5 `gpl test` — Run Tests

```bash
gpl test                            # run all tests in project
gpl test main.gpl                   # run tests in single file
gpl test --filter test_add          # run tests matching pattern
gpl test --filter "test_*"          # glob pattern
gpl test --release                  # run tests with release build
gpl test -v                         # verbose: show each test name
gpl test --no-capture               # show stdout from tests
gpl test --fail-fast                # stop on first failure
gpl test --threads 4                # run tests with 4 parallel threads
gpl test --timeout 30s              # per-test timeout
gpl test --list                     # list all tests without running
```

Output format:
```
running 5 tests
test test_add               ... ok   (0.1ms)
test test_string_ops        ... ok   (0.3ms)
test test_error_handling    ... ok   (0.2ms)
test test_generics          ... ok   (0.1ms)
test test_flaky             ... FAILED (1.2ms)

failures:
  test_flaky: assertion failed at src/main.gpl:42
    expected: 10
    got:      9

test result: FAILED. 4 passed, 1 failed, 0 skipped (1.9ms total)
```

### 37.6 `gpl bench` — Run Benchmarks

```bash
gpl bench                           # run all benchmarks
gpl bench --filter bench_sort       # run specific benchmark
gpl bench --baseline ./old_results  # compare to baseline
gpl bench --output results.json     # save results to file
gpl bench --warmup 3                # warmup iterations
gpl bench --runs 100                # measurement iterations
```

Output format:
```
bench bench_sort_1000     … 142.3 ns/iter  (±3.2 ns)  [130–160 ns]
bench bench_hash_insert   … 89.7 ns/iter   (±1.1 ns)  [88–92 ns]
bench bench_parse_json    … 2.14 µs/iter   (±0.08 µs) [2.0–2.3 µs]
```

### 37.7 `gpl fmt` — Formatter

```bash
gpl fmt                             # format all .gpl files in project
gpl fmt main.gpl                    # format single file (in-place)
gpl fmt main.gpl --check            # check without modifying (exit 1 if dirty)
gpl fmt main.gpl --diff             # show diff instead of rewriting
gpl fmt --line-width 100            # max line width (default: 80)
gpl fmt --indent 4                  # indent size (default: 4)
```

### 37.8 `gpl doc` — Documentation Generator

```bash
gpl doc                             # generate docs for project -> docs/
gpl doc --open                      # generate and open in browser
gpl doc --output ./site             # custom output directory
gpl doc --private                   # include private symbols
gpl doc --format html               # output format: html (default) | json | md
```

### 37.9 `gpl init` / `gpl new` — Project Scaffolding

```bash
# Initialize in current directory
gpl init
gpl init --name myapp
gpl init --type lib                 # library project (no main)
gpl init --type exe                 # executable project (default)
gpl init --template cli             # use a starter template

# Create new directory + project
gpl new myapp
gpl new myapp --type lib
gpl new myapp --vcs git             # initialize git repo (default)
gpl new myapp --vcs none            # no VCS

# Generated structure (exe):
# myapp/
# ├── gpl.toml
# ├── .gitignore
# ├── README.md
# └── src/
#     └── main.gpl

# Generated structure (lib):
# mylib/
# ├── gpl.toml
# ├── .gitignore
# ├── README.md
# └── src/
#     ├── lib.gpl     ← library root
#     └── lib.hdr     ← public API header
```

### 37.10 `gpl add` / `gpl remove` — Dependency Management

```bash
gpl add raylib                      # add latest version
gpl add raylib@5.0                  # specific version
gpl add raylib@">=4.0, <6.0"        # version range
gpl add --dev faker                 # dev-only dependency
gpl add --optional openssl          # optional dependency
gpl add --path ../myutils           # local path dependency
gpl add --git https://github.com/user/lib --tag v1.0

gpl remove raylib                   # remove dependency
gpl update                          # update all to latest compatible
gpl update raylib                   # update specific package
```

### 37.11 GCC-style One-liner Compilation

For users who prefer GCC-style without `gpl.toml`:

```bash
# Equivalent to: gcc main.c -o main
gpl build main.gpl -o main

# Equivalent to: gcc main.c utils.c -o app -lm -O2
gpl build main.gpl utils.gpl -o app -lm -O2

# Equivalent to: gcc -shared -fPIC mylib.c -o mylib.so
gpl build mylib.gpl --type dylib -o mylib.so

# Equivalent to: gcc -c main.c -o main.o
gpl build main.gpl --type obj -o main.o

# Run directly (like scripting)
gpl run script.gpl

# Shebang support in .gpl files:
#!/usr/bin/env gpl run
module main
import "std/io"
fn main() -> i32:
    io.println("script mode!")
    return 0
```

### 37.12 Environment Variables

| Variable              | Effect                                          |
|-----------------------|-------------------------------------------------|
| `GPL_PATH`            | Additional search path for imports              |
| `GPL_CACHE`           | Build cache directory (default: `~/.gpl/cache`) |
| `GPL_REGISTRY`        | Package registry URL                            |
| `GPL_TOOLCHAIN`       | Override active toolchain version               |
| `GPL_LOG`             | Log level: `error` `warn` `info` `debug` `trace`|
| `GPL_COLOR`           | `on` / `off` / `auto` (default: `auto`)         |
| `GPL_JOBS`            | Parallel compile jobs (default: CPU count)      |
| `GPL_TARGET`          | Default cross-compile target                    |

### 37.13 Compiler Diagnostics Format

G compiler produces structured, human-friendly error messages:

```
error[E0042]: type mismatch
  --> src/main.gpl:15:12
   |
14 |     var id: UserId = 42
   |             ------   ^^ expected UserId, found i32
   |             |
   |             declared as UserId here
   |
   = hint: wrap with UserId(42) to construct a UserId
   = see: G Language Spec §22.2 Newtype Pattern

error[E0017]: cannot borrow as mutable while immutable borrow is active
  --> src/main.gpl:28:5
   |
25 |     var r1 = &a         # immutable borrow starts here
   |              --
26 |     var r2 = &a
27 |     var m  = &mut a     # ERROR: mutable borrow attempted
   |              ^^^^^^ mutable borrow here
28 |     io.println(*r1)
   |                --- immutable borrow used here
   |
   = hint: immutable borrows (r1, r2) must end before mutable borrow (m) begins

warning[W0011]: unused variable 'count'
  --> src/main.gpl:8:9
   |
 8 |     var count: i32 = 0
   |         ^^^^^ consider prefixing with _ to suppress: _count
```

---

## 38. SIMD Intrinsics

### 38.1 SIMD Vector Types

```gpl
import "std/simd"

# Fixed-width SIMD vectors
type f32x4  = simd.Vec[f32, 4]     # 128-bit: 4 × f32
type f32x8  = simd.Vec[f32, 8]     # 256-bit: 8 × f32 (AVX)
type f32x16 = simd.Vec[f32, 16]    # 512-bit: 16 × f32 (AVX-512)
type i32x4  = simd.Vec[i32, 4]
type i32x8  = simd.Vec[i32, 8]
type i64x4  = simd.Vec[i64, 4]
type f64x2  = simd.Vec[f64, 2]
type f64x4  = simd.Vec[f64, 4]
```

### 38.2 SIMD Operations

```gpl
@target_feature("avx2")
fn dot_product_simd(a: []f32, b: []f32) -> f32:
    assert_eq(a.len, b.len)
    assert(a.len % 8 == 0, "length must be multiple of 8")

    var sum := simd.splat[f32x8](0.0)

    for i in (0..a.len).step_by(8):
        var va := simd.load[f32x8](&a[i])
        var vb := simd.load[f32x8](&b[i])
        sum = sum + va * vb              # operator overloading works on SIMD

    return simd.reduce_add(sum)          # horizontal add

# Portable SIMD (no explicit target_feature — compiler chooses best)
fn add_arrays(a: []f32, b: []f32, out: []f32) -> void:
    simd.foreach_vectorized[f32x8](a, b, out,
        fn(va: f32x8, vb: f32x8) -> f32x8: va + vb)
```

### 38.3 SIMD Intrinsic Operations

```gpl
simd.load[V](ptr)             # aligned load
simd.loadu[V](ptr)            # unaligned load
simd.store(ptr, v)            # aligned store
simd.storeu(ptr, v)           # unaligned store
simd.splat[V](scalar)         # broadcast scalar to all lanes
simd.extract(v, lane)         # extract lane value
simd.insert(v, lane, val)     # insert value into lane
simd.shuffle(v, mask)         # lane permutation
simd.blend(a, b, mask)        # select lanes from a or b
simd.reduce_add(v)            # horizontal sum
simd.reduce_mul(v)            # horizontal product
simd.reduce_min(v)            # horizontal min
simd.reduce_max(v)            # horizontal max
simd.min(a, b)                # lane-wise min
simd.max(a, b)                # lane-wise max
simd.abs(v)                   # lane-wise abs
simd.sqrt(v)                  # lane-wise sqrt
simd.fma(a, b, c)             # fused multiply-add: a*b + c
simd.cmpeq(a, b)              # lane-wise ==, returns mask
simd.cmplt(a, b)              # lane-wise <
simd.and_mask(v, mask)        # apply boolean mask
```

---

## 39. Reflection & Runtime Type Info

### 39.1 Compile-time Type Queries

```gpl
@size_of(T)            # -> usize
@align_of(T)           # -> usize
@offset_of(T, field)   # -> usize
@type_name(T)          # -> str  (compile-time string)
@type_id(T)            # -> u64  (unique per type, stable per build)
@has_method(T, "name") # -> bool
@implements(T, I)      # -> bool: does T implement interface I?
@is_pointer(T)         # -> bool
@is_slice(T)           # -> bool
@is_struct(T)          # -> bool
@is_enum(T)            # -> bool
@is_interface(T)       # -> bool
@is_numeric(T)         # -> bool
@is_float(T)           # -> bool
@is_integer(T)         # -> bool
@is_signed(T)          # -> bool
@field_count(T)        # -> usize (number of fields in a struct)
@field_names(T)        # -> []str (struct field names, compile-time)
@field_types(T)        # -> []TypeInfo (compile-time)
```

### 39.2 Runtime Type Info (`TypeInfo`)

```gpl
import "std/reflect"

var ti: reflect.TypeInfo = reflect.type_of(my_value)
io.println(ti.name)          # "MyStruct"
io.println(ti.size)          # 24
io.println(ti.kind)          # TypeKind.Struct
io.println(ti.fields.len)    # 3

for field in ti.fields:
    io.println(field.name, field.offset, field.type.name)

# Dynamic dispatch via reflection
var method := reflect.find_method(ti, "to_string")
if method != null:
    var result := method.call(my_value) as str
    io.println(result)
```

### 39.3 `TypeKind` Enum

```gpl
enum TypeKind:
    Bool
    Integer
    Float
    Pointer
    Slice
    Array
    Str
    Struct
    Enum
    Union
    Interface
    Function
    Tuple
    Never
    Void
    Any
```

---

---

---

## 40. Standard Library Overview

The G standard library (`std`) is organized into focused modules.  
All modules are opt-in via `import`. Nothing is imported by default except built-in types.

```
std/
├── io          # I/O: stdin, stdout, stderr, print
├── fs          # Filesystem: files, directories, paths
├── net         # Networking: TCP, UDP, HTTP, DNS
├── os          # OS interface: args, env, process, signals
├── time        # Date, time, duration, timers
├── mem         # Memory: allocators, copy, set, compare
├── string      # String builder, parsing, encoding
├── fmt         # Formatting: sprintf-style, number formatting
├── math        # Math functions, constants
├── rand        # Random number generation
├── sync        # Mutex, RWMutex, WaitGroup, Once
├── channel     # Channels, select
├── thread      # Thread spawn, join
├── atomic      # Atomic types with memory ordering
├── async       # Async runtime, Task, spawn
├── collections # List, Map, Set, Queue, Heap, BTreeMap
├── json        # JSON encode/decode
├── xml         # XML parse/emit
├── csv         # CSV parse/emit
├── bytes       # Byte slice utilities
├── unicode     # Unicode properties, normalization
├── regex       # Regular expressions
├── hash        # Hash functions: fnv, siphash, sha256, etc.
├── crypto      # Encryption, signing, TLS
├── compress    # gzip, zstd, lz4
├── path        # Cross-platform path manipulation
├── env         # Environment variables (alias: os.env)
├── log         # Structured logging
├── reflect     # Runtime type info
├── ffi         # C interoperability helpers
├── testing     # Test helpers (available in @test blocks)
└── simd        # SIMD vector types and operations
```

---

## 41. `std/io` — Input & Output

```gpl
import "std/io"

# --- stdout ---
io.print("hello")                    # no newline
io.println("hello")                  # with newline
io.println("x =", x, "y =", y)      # multiple args, space-separated
io.printf("%s is %d years old\n", name, age)   # C-style format
io.printfln("%s is %d years old", name, age)   # printf + newline

# --- stderr ---
io.eprint("error: something went wrong")
io.eprintln("error:", msg)
io.eprintf("[ERROR] %s\n", msg)

# --- stdin ---
var line := io.read_line()           # -> Result[str]
var line := io.read_line()?          # propagate error
var all  := io.read_all_stdin()?     # read entire stdin

# --- formatted output to string (see std/fmt) ---
var s := io.sprint("x =", x)        # -> str
var s := io.sprintf("%.2f", 3.14)   # -> str

# --- Writer interface ---
interface io.Writer:
    fn write(self: *Self, data: []byte) -> Result[usize]
    fn write_str(self: *Self, s: str) -> Result[usize]
    fn flush(self: *Self) -> Result[void]

# --- Reader interface ---
interface io.Reader:
    fn read(self: *Self, buf: []byte) -> Result[usize]  # returns bytes read
    fn read_all(self: *Self) -> Result[[]byte]
    fn read_line(self: *Self) -> Result[str]
    fn read_exact(self: *Self, buf: []byte) -> Result[void]

# --- Buffered I/O ---
var bw := io.BufWriter.new(writer, capacity: 4096)
bw.write_str("hello")?
bw.flush()?

var br := io.BufReader.new(reader, capacity: 4096)
var line := br.read_line()?

# --- Pipe ---
var reader, writer := io.pipe()
```

---

## 42. `std/fs` — Filesystem

```gpl
import "std/fs"

# --- Read file ---
var data: []byte = fs.read(path)?           # read entire file as bytes
var text: str    = fs.read_str(path)?       # read entire file as UTF-8 string
var lines: []str = fs.read_lines(path)?     # read lines (strips \n)

# --- Write file ---
fs.write(path, data)?                       # write bytes (overwrites)
fs.write_str(path, text)?                   # write string (overwrites)
fs.append(path, data)?                      # append bytes
fs.append_str(path, text)?                  # append string

# --- File object ---
var f := fs.open(path)?                     # open read-only
var f := fs.open_opts(path, fs.OpenOptions{
    read:   true,
    write:  true,
    create: true,
    append: false,
    truncate: true,
    mode: 0o644,                            # Unix permission bits
})?

f.read(buf)?
f.read_all()?
f.write(data)?
f.write_str(text)?
f.seek(offset, fs.SeekFrom.Start)?
f.seek(offset, fs.SeekFrom.Current)?
f.seek(offset, fs.SeekFrom.End)?
f.tell()?                                   # -> Result[u64] current position
f.flush()?
f.close()?
f.sync()?                                   # fsync — flush OS buffers to disk
defer f.close()                             # RAII pattern

# file metadata
var meta := f.metadata()?
meta.size                                   # u64
meta.modified                               # time.Time
meta.created                                # time.Time
meta.is_file                                # bool
meta.is_dir                                 # bool
meta.is_symlink                             # bool
meta.permissions                            # fs.Permissions

# --- Directory ---
fs.create_dir(path)?
fs.create_dir_all(path)?                    # mkdir -p
fs.remove_file(path)?
fs.remove_dir(path)?
fs.remove_dir_all(path)?                    # rm -rf

var entries := fs.read_dir(path)?           # -> []fs.DirEntry
for entry in entries:
    io.println(entry.name, entry.kind)      # DirEntryKind: File | Dir | Symlink

# recursive walk
fs.walk(path, fn(entry: fs.WalkEntry) -> fs.WalkControl:
    io.println(entry.path)
    if entry.name == "skip_me":
        return fs.WalkControl.SkipDir
    return fs.WalkControl.Continue
)?

# --- Path operations ---
fs.exists(path)                             # -> bool
fs.is_file(path)                            # -> bool
fs.is_dir(path)                             # -> bool
fs.copy(src, dst)?
fs.rename(src, dst)?                        # also works as move
fs.symlink(target, link_path)?
fs.canonicalize(path)?                      # -> Result[str] absolute path
fs.metadata(path)?                          # -> Result[fs.Metadata]
fs.permissions(path)?                       # -> Result[fs.Permissions]
fs.set_permissions(path, perms)?

# temp files
var tmp := fs.temp_file()?                  # -> Result[fs.File]
var tmp_dir := fs.temp_dir()?               # -> Result[str]
```

---

## 43. `std/path` — Path Manipulation

```gpl
import "std/path"

# All path functions are pure (no filesystem access)
path.join("usr", "local", "bin")            # "usr/local/bin"
path.join("/usr", "local", "bin")           # "/usr/local/bin"
path.base("/usr/local/bin/gpl")             # "gpl"
path.dir("/usr/local/bin/gpl")              # "/usr/local/bin"
path.ext("main.gpl")                        # ".gpl"
path.stem("main.gpl")                       # "main"
path.is_absolute("/usr/local")              # true
path.is_relative("src/main.gpl")            # true
path.clean("a/b/../c")                      # "a/c"
path.split("/usr/local/bin")                # ("/usr/local", "bin")
path.split_ext("main.gpl")                  # ("main", ".gpl")
path.with_ext("main.gpl", ".hdr")           # "main.hdr"
path.with_base("/usr/local/bin", "gpl2")    # "/usr/local/bin/gpl2"
path.components("/usr/local/bin")           # ["/", "usr", "local", "bin"]
path.SEP                                    # "/" on Unix, "\" on Windows
path.LIST_SEP                               # ":" on Unix, ";" on Windows
```

---

## 44. `std/os` — Operating System Interface

```gpl
import "std/os"

# --- Command-line arguments ---
var args: []str = os.args()                 # includes program name at [0]
var args_only := os.args()[1:]              # skip program name

# --- Environment variables ---
var home := os.env("HOME")?                 # -> Result[str]
var home := os.env_or("HOME", "/root")      # -> str (with default)
var all  := os.env_all()                    # -> map[str]str
os.set_env("KEY", "value")?
os.unset_env("KEY")?

# --- Process ---
os.exit(0)                                  # exit with code
os.exit(1)
var pid := os.getpid()                      # -> i32
var ppid := os.getppid()                    # -> i32

# spawn subprocess
var proc := os.Command.new("ls")
    .arg("-la")
    .arg("/tmp")
    .env("HOME", "/root")
    .stdout(os.Stdio.Pipe)
    .stderr(os.Stdio.Inherit)
    .spawn()?

var output := proc.wait_with_output()?
io.println(output.stdout)
io.println("exit code:", output.status.code)

# shorthand
var out := os.run(["ls", "-la", "/tmp"])?   # -> os.Output
var out := os.run_str("ls -la /tmp")?       # shell-style (unsafe: injection risk)

# --- Signals ---
os.signal(os.Signal.SIGINT, fn(sig: os.Signal) -> void:
    io.println("interrupted")
    os.exit(0)
)
os.signal(os.Signal.SIGTERM, os.SIG_DFL)    # restore default
os.signal(os.Signal.SIGUSR1, os.SIG_IGN)   # ignore

# --- System info ---
os.hostname()?                              # -> Result[str]
os.username()?                              # -> Result[str]
os.homedir()?                               # -> Result[str]
os.tempdir()                                # -> str
os.num_cpus()                               # -> usize
os.page_size()                              # -> usize
os.OS                                       # comptime str: "linux"|"macos"|"windows"
os.ARCH                                     # comptime str: "x86_64"|"aarch64"|...

# --- Dynamic libraries ---
var lib := os.dynlib_open("libraylib.so")?
var fn_ptr: fn(i32) -> void = lib.symbol("InitWindow")?
dynlib_close(lib)
```

---

## 45. `std/time` — Date & Time

```gpl
import "std/time"

# --- Duration ---
var d1 := 5 * time.Second
var d2 := 100 * time.Millisecond
var d3 := 2 * time.Hour + 30 * time.Minute
var d4 := time.Duration.from_secs(3600)
var d5 := time.Duration.from_millis(500)
var d6 := time.Duration.from_nanos(1_000_000)

d.as_secs()                                 # -> f64
d.as_millis()                               # -> i64
d.as_nanos()                                # -> i64
d.as_mins()                                 # -> f64

# Duration constants
time.Nanosecond   # 1ns
time.Microsecond  # 1000ns
time.Millisecond  # 1_000_000ns
time.Second       # 1_000_000_000ns
time.Minute       # 60 * Second
time.Hour         # 60 * Minute

# --- Instant (monotonic clock, for measuring elapsed time) ---
var start := time.Instant.now()
do_work()
var elapsed := start.elapsed()              # -> Duration
io.println(f"took {elapsed.as_millis()}ms")

# --- Time (wall clock) ---
var now := time.now()                       # -> time.Time (UTC)
var now_local := time.now_local()           # -> time.Time (local timezone)

now.year()                                  # i32
now.month()                                 # time.Month (Jan=1..Dec=12)
now.day()                                   # i32 (1-31)
now.hour()                                  # i32 (0-23)
now.minute()                                # i32 (0-59)
now.second()                                # i32 (0-59)
now.nanosecond()                            # i32
now.weekday()                               # time.Weekday (Sun=0..Sat=6)
now.unix()                                  # i64 (seconds since epoch)
now.unix_millis()                           # i64
now.unix_nanos()                            # i64

# formatting
now.format("2006-01-02 15:04:05")           # -> str (Go-style reference time)
now.format_rfc3339()                        # "2024-03-15T10:30:00Z"
now.format_rfc2822()                        # "Fri, 15 Mar 2024 10:30:00 +0000"
now.format_http()                           # "Fri, 15 Mar 2024 10:30:00 GMT"

# parsing
var t := time.parse("2024-03-15", "2006-01-02")?

# arithmetic
var tomorrow := now + 24 * time.Hour
var yesterday := now - 24 * time.Hour
var diff := t2 - t1                         # -> Duration

# comparison
t1 < t2
t1 == t2
t1.before(t2)
t1.after(t2)

# sleep
time.sleep(500 * time.Millisecond)
time.sleep_until(deadline)

# timer
var timer := time.Timer.new(1 * time.Second)
await timer.wait()                          # async
timer.reset(2 * time.Second)
timer.cancel()

# ticker (repeating)
var ticker := time.Ticker.new(100 * time.Millisecond)
defer ticker.stop()
loop:
    await ticker.tick()
    do_periodic_work()
```

---

## 46. `std/fmt` — Formatting

```gpl
import "std/fmt"

# sprintf-style (type-safe, not variadic void*)
var s := fmt.sprintf("Hello, %s! You are %d years old.", name, age)
var s := fmt.sprintf("%.2f", 3.14159)       # "3.14"
var s := fmt.sprintf("%08X", 0xDEAD)        # "0000DEAD"
var s := fmt.sprintf("%+d", 42)             # "+42"
var s := fmt.sprintf("%-10s|", "left")      # "left      |"
var s := fmt.sprintf("%10s|", "right")      # "     right|"
var s := fmt.sprintf("%e", 0.001)           # "1.000000e-03"
var s := fmt.sprintf("%v", my_struct)       # auto-format using Display

# format to writer
fmt.fprintf(writer, "x = %d\n", x)?

# format verbs
# %v   — default format (calls to_string() / Display)
# %T   — type name
# %d   — decimal integer
# %b   — binary
# %o   — octal
# %x   — hex lowercase
# %X   — hex uppercase
# %f   — decimal float
# %e   — scientific notation lowercase
# %E   — scientific notation uppercase
# %g   — shortest float representation
# %s   — string
# %q   — quoted string (with escape sequences)
# %p   — pointer address
# %c   — character (rune)
# %%   — literal percent sign

# number formatting
fmt.format_int(n, base: 16)                 # -> str
fmt.format_float(f, precision: 4)           # -> str
fmt.format_bytes(1_048_576)                 # "1.00 MB"
fmt.format_duration(duration)               # "2h30m5s"
fmt.format_number(1_234_567.89)             # "1,234,567.89" (locale-aware)
```

---

## 47. `std/net` — Networking

```gpl
import "std/net"

# --- TCP ---
# Server
var listener := net.TcpListener.bind("0.0.0.0:8080")?
defer listener.close()
io.println("Listening on :8080")

loop:
    var conn, addr := listener.accept()?
    io.println("connection from:", addr)
    thread.spawn(fn() -> void: handle_conn(conn))

fn handle_conn(conn: net.TcpStream) -> void:
    defer conn.close()
    var buf: [4096]byte
    loop:
        var n := conn.read(buf[:])?
        if n == 0: break          # connection closed
        conn.write(buf[:n])?      # echo back

# Client
var conn := net.TcpStream.connect("example.com:80")?
defer conn.close()
conn.write_str("GET / HTTP/1.0\r\nHost: example.com\r\n\r\n")?
var response := conn.read_all()?
io.println(response)

# TCP options
var conn := net.TcpStream.connect_opts("example.com:443", net.TcpOptions{
    timeout:      30 * time.Second,
    nodelay:      true,
    keepalive:    true,
    recv_buf:     65536,
    send_buf:     65536,
})?

# --- UDP ---
var sock := net.UdpSocket.bind("0.0.0.0:9000")?
defer sock.close()

var buf: [1500]byte
var n, addr := sock.recv_from(buf[:])?
sock.send_to(buf[:n], addr)?

# UDP client
var sock := net.UdpSocket.bind("0.0.0.0:0")?
sock.send_to("hello".as_bytes(), "1.2.3.4:9000")?

# --- DNS ---
var addrs := net.lookup_host("example.com")?     # -> []net.IpAddr
var name  := net.lookup_addr("93.184.216.34")?   # -> str

# --- IP Addresses ---
var ip4 := net.IpAddr.v4(127, 0, 0, 1)
var ip6 := net.IpAddr.parse("::1")?
var ip  := net.IpAddr.parse("192.168.1.1")?
ip.is_loopback()
ip.is_private()
ip.is_v4()
ip.is_v6()

# --- HTTP (std/http) ---
import "std/http"

# HTTP client
var resp := http.get("https://example.com")?
io.println(resp.status)                     # 200
io.println(resp.headers["content-type"])
var body := resp.text()?                    # -> str
var body := resp.bytes()?                   # -> []byte
var obj  := resp.json[MyStruct]()?          # decode JSON

# HTTP client with options
var client := http.Client.new(http.ClientOptions{
    timeout:        30 * time.Second,
    follow_redirects: true,
    max_redirects:  5,
    user_agent:     "MyApp/1.0",
})

var resp := client.post("https://api.example.com/data",
    http.Body.json(my_struct))?

var resp := client.request(http.Request{
    method:  "PATCH",
    url:     "https://api.example.com/item/1",
    headers: {"Authorization": "Bearer " + token},
    body:    http.Body.json(update),
})?

# HTTP server
var srv := http.Server.new(http.ServerOptions{
    addr:         "0.0.0.0:8080",
    read_timeout: 30 * time.Second,
})

srv.route("GET", "/",           handle_index)
srv.route("GET", "/users/{id}", handle_user)
srv.route("POST", "/users",     handle_create_user)
srv.use(middleware_logger)
srv.use(middleware_auth)

srv.listen_and_serve()?

fn handle_index(req: *http.Request, res: *http.Response) -> void:
    res.status(200)
    res.header("Content-Type", "text/plain")
    res.body("Hello, World!")

fn handle_user(req: *http.Request, res: *http.Response) -> void:
    var id := req.param("id")
    var user := db.find_user(id) ?? return res.status(404).body("Not Found")
    res.status(200).json(user)
```

---

## 48. `std/json` — JSON Encoding & Decoding

```gpl
import "std/json"

# --- Decode (Unmarshal) ---
struct User:
    name:  str
    age:   i32
    email: str

var user := json.decode[User]("""{"name":"Alice","age":30,"email":"a@b.com"}""")?

# decode into map
var obj := json.decode[map[str]any](text)?

# streaming decode (large files)
var decoder := json.Decoder.new(reader)
while decoder.has_next():
    var item := decoder.decode[LogEntry]()?
    process(item)

# --- Encode (Marshal) ---
var text := json.encode(user)?              # -> Result[str]
var text := json.encode_pretty(user)?       # indented output

# encode to writer
json.encode_to(user, writer)?

# --- Field customization via attributes ---
struct Config:
    @json("server_host")
    host: str                               # JSON key: "server_host"

    @json("port")
    port: i32

    @json("debug", omit_empty: true)
    debug: bool                             # omitted from JSON if false

    @json("-")
    internal_state: str                     # never serialized

    @json("timeout_ms")
    timeout: time.Duration                  # custom encode/decode via impl

# --- Custom encode/decode ---
impl json.Encode for time.Duration:
    fn json_encode(self) -> json.Value:
        return json.Value.Int(self.as_millis())

impl json.Decode for time.Duration:
    fn json_decode(v: json.Value) -> Result[time.Duration]:
        var ms := v.as_int()?
        return Result.Ok(time.Duration.from_millis(ms))

# --- json.Value (dynamic) ---
var v := json.parse(text)?                  # -> json.Value
match v:
    json.Value.Object(obj)  => obj["name"].as_str()
    json.Value.Array(arr)   => arr[0]
    json.Value.String(s)    => s
    json.Value.Int(n)       => n
    json.Value.Float(f)     => f
    json.Value.Bool(b)      => b
    json.Value.Null         => null
```

---

## 49. `std/log` — Structured Logging

```gpl
import "std/log"

# default global logger (writes to stderr)
log.debug("starting up")
log.info("server listening on :8080")
log.warn("connection pool nearly full:", pool.size)
log.error("failed to connect:", err)
log.fatal("unrecoverable error:", err)      # logs then calls os.exit(1)

# structured fields
log.info("request handled",
    log.field("method",  req.method),
    log.field("path",    req.path),
    log.field("status",  resp.status),
    log.field("latency", elapsed.as_millis()),
)

# custom logger
var logger := log.Logger.new(log.LoggerOptions{
    level:  log.Level.Info,
    format: log.Format.Json,            # Json | Text | Pretty
    output: writer,
    fields: {"service": "api", "version": "1.0"},
})

logger.info("server started", log.field("addr", addr))

# child logger with extra context
var req_log := logger.with(
    log.field("request_id", req_id),
    log.field("user_id",    user_id),
)
req_log.info("processing request")
req_log.error("failed", log.field("error", err.message()))

# log levels
log.Level.Trace
log.Level.Debug
log.Level.Info
log.Level.Warn
log.Level.Error
log.Level.Fatal
```

---

## 50. `std/regex` — Regular Expressions

```gpl
import "std/regex"

# compile pattern (fails at runtime if invalid)
var re := regex.compile(r"(\d{4})-(\d{2})-(\d{2})")?

# test
re.is_match("2024-03-15")                   # true

# find first match
var m := re.find("date: 2024-03-15")?       # -> Option[regex.Match]
m.full()                                    # "2024-03-15"
m.group(1)                                  # "2024"
m.group(2)                                  # "03"
m.group(3)                                  # "15"
m.start()                                   # usize byte offset
m.end()                                     # usize byte offset

# find all matches
for m in re.find_all("2024-01-01 and 2024-12-31"):
    io.println(m.full())

# named groups
var re2 := regex.compile(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})")?
var m := re2.find("2024-03-15")?
m.named("year")                             # "2024"
m.named("month")                            # "03"

# replace
var result := re.replace("2024-03-15", "$3/$2/$1")      # "15/03/2024"
var result := re.replace_all(text, replacement)

# split
var parts := re.split("one,two,,three")     # ["one", "two", "", "three"]

# compile-time regex (zero-cost, validated at compile)
@regex(r"\d+")
const INT_RE: regex.Regex

# shorthand macros
regex.is_match(r"\d+", "abc123")            # true (compiles on each call — avoid in loops)
```

---

## 51. `std/hash` & `std/crypto`

```gpl
import "std/hash"
import "std/crypto"

# --- Non-cryptographic hashes (fast, for hash maps) ---
var h := hash.fnv64("hello world")          # -> u64
var h := hash.siphash("hello", key: [16]byte{...})

# custom hashable type
impl Hash for Point:
    fn hash(self) -> u64:
        return hash.combine(
            hash.fnv64_f64(self.x),
            hash.fnv64_f64(self.y),
        )

# --- Cryptographic hashes ---
var digest := crypto.sha256("hello world")          # -> [32]byte
var digest := crypto.sha512(data)                   # -> [64]byte
var digest := crypto.blake3(data)                   # -> [32]byte
var hex    := crypto.to_hex(digest)                 # -> str

# streaming hash
var h := crypto.Sha256.new()
h.update(chunk1)
h.update(chunk2)
var digest := h.finalize()                          # -> [32]byte

# --- HMAC ---
var mac := crypto.hmac_sha256(key, message)         # -> [32]byte
crypto.hmac_verify(key, message, mac)?              # -> Result[void]

# --- Random (cryptographically secure) ---
import "std/crypto/rand"
var bytes: [32]byte
rand.fill(bytes[:])?                                # fills with CSPRNG bytes
var n := rand.u64()?                                # random u64

# --- Symmetric encryption (AES-GCM) ---
var key:   [32]byte                                 # AES-256 key
var nonce: [12]byte
rand.fill(key[:])?
rand.fill(nonce[:])?

var ciphertext := crypto.aes_gcm_encrypt(key, nonce, plaintext, aad: []byte{})?
var plaintext  := crypto.aes_gcm_decrypt(key, nonce, ciphertext, aad: []byte{})?

# --- TLS (via std/net/tls) ---
import "std/net/tls"
var tls_conn := tls.connect("example.com:443",
    tls.Config{verify_certs: true})?
```

---

## 52. `std/collections` — Data Structures

```gpl
import "std/collections"

# --- List[T] (dynamic array) ---
var list := collections.List[i32].new()
var list := collections.List[i32].with_capacity(64)
var list := collections.List.from([1, 2, 3, 4, 5])

list.push(6)
list.push_front(0)
list.pop()                                  # -> Option[i32]
list.pop_front()                            # -> Option[i32]
list.insert(idx, value)
list.remove(idx)                            # -> i32
list.get(idx)                               # -> Option[i32]
list.get_unchecked(idx)                     # unsafe, no bounds check
list[idx]                                   # operator[] — panics if OOB
list.len
list.cap
list.is_empty()
list.clear()
list.contains(value)
list.index_of(value)                        # -> Option[usize]
list.sort()
list.sort_by(comparator)
list.sort_stable_by(comparator)
list.reverse()
list.dedup()                                # remove consecutive duplicates
list.retain(fn(x: i32) -> bool: x > 0)    # keep only matching
list.extend(other_slice)
list.as_slice()                             # -> []i32
list.clone()

# --- Map[K, V] (hash map) ---
var m := collections.Map[str, i32].new()
var m := collections.Map.from_pairs([("a", 1), ("b", 2)])

m.insert("key", 42)
m.get("key")                                # -> Option[i32]
m.get_or_default("key", 0)                  # -> i32
m.remove("key")                             # -> Option[i32]
m.contains_key("key")                       # -> bool
m.len
m.is_empty()
m.clear()
m.keys()                                    # -> []K  (snapshot)
m.values()                                  # -> []V
m.entries()                                 # -> []Pair[K, V]
m.get_or_insert("key", default_val)         # insert if absent, return &V
m.update("key", fn(v: i32) -> i32: v + 1)

for key, val in m:
    io.println(key, val)

# --- Set[T] ---
var s := collections.Set[str].new()
var s := collections.Set.from(["a", "b", "c"])

s.insert("d")
s.remove("a")
s.contains("b")                             # -> bool
s.len
s.union(other)                              # -> Set[T]
s.intersection(other)
s.difference(other)
s.is_subset(other)
s.is_superset(other)

# --- Queue[T] (FIFO, double-ended deque) ---
var q := collections.Queue[i32].new()
q.push_back(1)
q.push_front(0)
q.pop_front()                               # -> Option[i32]
q.pop_back()                                # -> Option[i32]
q.peek_front()                              # -> Option[&i32]

# --- BinaryHeap[T] (priority queue) ---
var heap := collections.BinaryHeap[i32].new()   # max-heap by default
var heap := collections.BinaryHeap[i32].min()   # min-heap
heap.push(5)
heap.push(3)
heap.push(8)
heap.pop()                                  # -> Option[i32]  (returns 8 for max)
heap.peek()                                 # -> Option[&i32]

# --- BTreeMap[K, V] (sorted map) ---
var bt := collections.BTreeMap[str, i32].new()
bt.insert("banana", 2)
bt.insert("apple",  1)
bt.insert("cherry", 3)
for k, v in bt:                             # iterates in sorted key order
    io.println(k, v)
bt.range("apple".."cherry")                # -> iterator over range
bt.first()                                  # -> Option[Pair[K,V]]
bt.last()                                   # -> Option[Pair[K,V]]
```

---

## 53. `std/rand` — Random Numbers

```gpl
import "std/rand"

# default global RNG (seeded from OS entropy at startup)
var n   := rand.i32()                       # random i32
var n   := rand.i64()
var n   := rand.u64()
var f   := rand.f64()                       # [0.0, 1.0)
var f   := rand.f64_range(1.0, 10.0)       # [lo, hi)
var n   := rand.i32_range(1, 100)           # [lo, hi)
var b   := rand.bool()
var idx := rand.usize_range(0, arr.len)

rand.shuffle(arr[:])                        # Fisher-Yates shuffle

# seeded RNG (reproducible, for tests/simulations)
var rng := rand.Rng.seeded(12345)
var n   := rng.i32()
var n   := rng.i32_range(1, 6)             # dice roll

# choose random element
var item := rand.choose(arr[:])?            # -> Option[T]
var item := rand.choose_weighted(items, weights)?
```

---

## 54. Freestanding Mode (`no_std`)

For kernel, embedded, and bare-metal development where no OS or standard library is available.

### 54.1 Declaring Freestanding Mode

```gpl
#![no_std]          # must be first line (before module declaration)
#![no_runtime]      # disable G runtime (no panic handler, no allocator)
module kernel

# no std/io, std/mem, std/collections available
# only built-in types, operators, unsafe, inline asm
```

### 54.2 What is Available in `no_std`

Available without `std`:
- All primitive types (`i8`..`u128`, `f32`, `f64`, `bool`, `str` literals, `usize`)
- Pointers, arrays, slices (but no heap operations)
- `unsafe` blocks
- Inline assembly (`asm:`)
- `const fn`, `comptime`, all compile-time macros
- `match`, `if`, `for`, `while`, `loop`
- All operators including overflow variants
- `@size_of`, `@align_of`, `@offset_of`, `@type_id`
- Structs, enums, unions, interfaces
- `impl`, operator overloading
- `@packed`, `@align`, `@repr("C")`
- `@section`, `@link_name`, `@extern`, `@export`
- `@naked`, `@callconv`, `@interrupt`
- SIMD types and intrinsics
- `#![no_std]` compatible modules from `core/` (subset of `std`)

Available via `core/` (zero-dependency subset):
```gpl
import "core/mem"       # mem.copy, mem.set, mem.compare (no alloc)
import "core/fmt"       # fmt.sprintf (stack-only, fixed buffer)
import "core/atomic"    # all atomic types + memory ordering
import "core/math"      # pure math functions
import "core/ptr"       # pointer utilities
import "core/cmp"       # Comparable, Eq
import "core/convert"   # type conversion traits
```

### 54.3 Providing Runtime Hooks

In `no_std` + `no_runtime`, you must provide these hooks yourself:

```gpl
#![no_std]
#![no_runtime]
module kernel

# Required: panic handler
@panic_handler
fn kernel_panic(info: *core.PanicInfo) -> never:
    # info.message() -> str
    # info.file()    -> str
    # info.line()    -> u32
    serial_print("KERNEL PANIC: ")
    serial_println(info.message())
    loop: {}    # halt

# Required if any allocation is used: global allocator
@global_allocator
var ALLOCATOR: KernelAllocator = KernelAllocator.new()

# Optional: stack overflow handler
@stack_overflow_handler
fn handle_stack_overflow() -> never:
    serial_println("STACK OVERFLOW")
    loop: {}
```

---

## 55. Volatile Memory Access

Volatile accesses prevent the compiler from optimizing away or reordering reads/writes.  
Essential for memory-mapped I/O (MMIO) and hardware registers.

```gpl
import "core/volatile"

# Volatile read — compiler cannot cache or eliminate this read
var val := volatile.read[u32](0xFEE00300 as *const u32)

# Volatile write — compiler cannot eliminate or reorder this write
volatile.write[u32](0xFEE00300 as *mut u32, 0x000FF000)

# Volatile read/write on a typed register struct
@repr("C")
@packed
struct UartRegisters:
    data:    u8      # offset 0x00: data register
    ier:     u8      # offset 0x01: interrupt enable
    iir_fcr: u8      # offset 0x02: interrupt id / FIFO control
    lcr:     u8      # offset 0x03: line control
    mcr:     u8      # offset 0x04: modem control
    lsr:     u8      # offset 0x05: line status
    msr:     u8      # offset 0x06: modem status
    scr:     u8      # offset 0x07: scratch

const UART0_BASE: usize = 0x10000000

fn uart_write_byte(b: u8) -> void:
    var regs: *mut UartRegisters = UART0_BASE as *mut UartRegisters
    # wait until transmit buffer empty (bit 5 of LSR)
    while volatile.read_field[u8](regs, UartRegisters.lsr) & 0x20 == 0: {}
    volatile.write_field[u8](regs, UartRegisters.data, b)

# VolatileCell — wrapper type for MMIO regions
struct VolatileCell[T]:
    _ptr: *mut T

fn VolatileCell[T].read(self) -> T:
    return volatile.read[T](self._ptr)

fn VolatileCell[T].write(self, val: T) -> void:
    volatile.write[T](self._ptr, val)

fn VolatileCell[T].modify(self, f: fn(T) -> T) -> void:
    self.write(f(self.read()))

# MMIO region definition pattern
@repr("C")
struct GicDistributor:
    ctlr:     VolatileCell[u32]     # 0x000 Distributor Control
    typer:    VolatileCell[u32]     # 0x004 Interrupt Controller Type
    iidr:     VolatileCell[u32]     # 0x008 Distributor Implementer ID
    _res0:    [5]u32                # 0x00C reserved
    igroupr:  [32]VolatileCell[u32] # 0x080 Interrupt Group Registers

fn GicDistributor.enable(self: *GicDistributor) -> void:
    self.ctlr.write(1)
```

---

## 56. Memory-Mapped I/O Patterns

```gpl
#![no_std]
#![no_runtime]
module kernel.drivers.uart

import "core/volatile"

# Pattern 1: Address constant + cast
const UART_BASE: usize = 0x09000000     # QEMU virt UART

fn uart_putchar(c: u8) -> void:
    unsafe:
        var dr: *mut u8 = UART_BASE as *mut u8
        volatile.write[u8](dr, c)

# Pattern 2: Extern symbol from linker script
@extern("C")
var UART_REGS: UartRegisters           # defined at link-time

# Pattern 3: MMIO region as struct (recommended for complex peripherals)
struct MmioRegion[T]:
    base: usize

fn MmioRegion[T].get(self) -> *mut T:
    return self.base as *mut T

fn MmioRegion[T].read_reg(self, offset: usize) -> u32:
    unsafe:
        var p: *const u32 = (self.base + offset) as *const u32
        return volatile.read[u32](p)

fn MmioRegion[T].write_reg(self, offset: usize, val: u32) -> void:
    unsafe:
        var p: *mut u32 = (self.base + offset) as *mut u32
        volatile.write[u32](p, val)

const UART0: MmioRegion[UartRegisters] = MmioRegion{base: 0x09000000}
```

---

## 57. Calling Conventions

```gpl
# Default: platform ABI (sysv64 on Linux x86_64, win64 on Windows)
fn normal_fn(x: i32) -> i32: x + 1

# Explicit calling convention
@callconv("sysv64")
fn sysv_fn(a: i32, b: i32, c: i32) -> i32: a + b + c

@callconv("win64")
fn win_fn(a: i32, b: i32) -> i32: a + b

@callconv("fastcall")
fn fast_fn(a: i32, b: i32) -> i32: a + b

@callconv("cdecl")
fn cdecl_fn(a: i32, b: i32) -> i32: a + b

@callconv("stdcall")
fn stdcall_fn(a: i32, b: i32) -> i32: a + b

@callconv("aapcs")                          # ARM Procedure Call Standard
fn arm_fn(a: i32) -> i32: a

@callconv("aapcs64")                        # AArch64
fn aarch64_fn(a: i32) -> i32: a

# Naked function — NO prologue/epilogue generated by compiler
# You are 100% responsible for stack management and return
@naked
fn raw_fn() -> void:
    asm:
        "nop"
        "ret"
        : : : clobber("memory")

# Naked + explicit calling convention (common for ISRs)
@naked
@callconv("sysv64")
fn syscall_entry() -> void:
    asm:
        "swapgs"
        "mov qword ptr [gs:0x8], rsp"
        "mov rsp, qword ptr [gs:0x0]"
        "push rax"
        "call syscall_handler"
        "pop rax"
        "swapgs"
        "sysretq"
        : : : clobber("memory")
```

---

## 58. Interrupt Handlers

```gpl
#![no_std]
#![no_runtime]
module kernel.interrupts

import "core/volatile"

# x86_64 interrupt frame pushed by CPU
@repr("C")
struct InterruptFrame:
    ip:     u64     # instruction pointer
    cs:     u64     # code segment
    flags:  u64     # RFLAGS
    sp:     u64     # stack pointer
    ss:     u64     # stack segment

# x86_64 exception with error code
@repr("C")
struct ExceptionFrame:
    error_code: u64
    ip:         u64
    cs:         u64
    flags:      u64
    sp:         u64
    ss:         u64

# Interrupt Service Routine — @interrupt generates correct entry/exit code
@interrupt
fn handler_divide_by_zero(frame: *InterruptFrame) -> void:
    serial_println("Exception: Divide by Zero")
    serial_println(f"  IP: 0x{frame.ip:X}")
    loop: {}

@interrupt
fn handler_page_fault(frame: *ExceptionFrame) -> void:
    var cr2: u64
    unsafe:
        asm: "mov {0}, cr2" : out(cr2) : : clobber()
    serial_println(f"Page Fault at 0x{cr2:X}, error=0x{frame.error_code:X}")
    loop: {}

@interrupt
fn handler_timer(frame: *InterruptFrame) -> void:
    TICK_COUNT.fetch_add(1, atomic.Ordering.Relaxed)
    lapic_eoi()     # signal end-of-interrupt to APIC

# Software interrupt (syscall via int 0x80 style)
@interrupt
@callconv("sysv64")
fn handler_syscall(frame: *InterruptFrame) -> void:
    # rax = syscall number, rdi/rsi/rdx/r10/r8/r9 = args
    var syscall_num: u64
    unsafe:
        asm: "mov {0}, rax" : out(syscall_num) : :

    dispatch_syscall(syscall_num, frame)

# IDT (Interrupt Descriptor Table) entry
@repr("C")
@packed
struct IdtEntry:
    offset_low:  u16
    selector:    u16
    ist:         u8
    type_attr:   u8
    offset_mid:  u16
    offset_high: u32
    _reserved:   u32

fn IdtEntry.new(handler: fn(*InterruptFrame) -> void, selector: u16, dpl: u8) -> IdtEntry:
    var addr := handler as usize
    return IdtEntry{
        offset_low:  (addr & 0xFFFF) as u16,
        selector:    selector,
        ist:         0,
        type_attr:   0x8E | ((dpl & 3) << 5),
        offset_mid:  ((addr >> 16) & 0xFFFF) as u16,
        offset_high: ((addr >> 32) & 0xFFFFFFFF) as u32,
        _reserved:   0,
    }

@align(16)
var IDT: [256]IdtEntry

fn idt_init() -> void:
    IDT[0]  = IdtEntry.new(handler_divide_by_zero, 0x08, 0)
    IDT[14] = IdtEntry.new(handler_page_fault,     0x08, 0)
    IDT[32] = IdtEntry.new(handler_timer,          0x08, 0)
    idt_load(&IDT)

@naked
fn idt_load(idt_ptr: *IdtDescriptor) -> void:
    asm:
        "lidt [{0}]"
        "ret"
        : : in(idt_ptr) :
```

---

## 59. Linker Script Integration

### 59.1 `@section` Attribute

```gpl
# Place symbol in specific linker section
@section(".text.boot")
@naked
fn _start() -> never:
    asm:
        "mov rsp, 0x7C00"   # set up stack
        "call kmain"
        "hlt"
        : : :

@section(".rodata")
const KERNEL_VERSION: str = "0.1.0"

@section(".bss")
var KERNEL_STACK: [16384]u8             # 16KB stack in BSS

@section(".data")
var BOOT_INFO: BootInfo

# Force symbol to be kept (not stripped by linker)
@used
@section(".multiboot")
const MULTIBOOT_HEADER: MultibootHeader = MultibootHeader{
    magic:    0x1BADB002,
    flags:    0x00000003,
    checksum: -(0x1BADB002 + 0x00000003),
}
```

### 59.2 Linker Symbols

```gpl
# Access symbols defined in linker script
@extern("C") var _kernel_start: u8
@extern("C") var _kernel_end:   u8
@extern("C") var _bss_start:    u8
@extern("C") var _bss_end:      u8
@extern("C") var _stack_top:    u8

fn bss_clear() -> void:
    unsafe:
        var start := &_bss_start as usize
        var end   := &_bss_end   as usize
        core.mem.set(start as *mut u8, 0, end - start)

fn kernel_size() -> usize:
    unsafe:
        return &_kernel_end as usize - &_kernel_start as usize
```

### 59.3 `gpl.toml` for Kernel Build

```toml
[package]
name    = "mykernel"
version = "0.1.0"

[build]
entry  = "src/main.gpl"
no_std = true

[build.options]
optimization = "speed"
debug_info   = true
lto          = true
pic          = false                    # kernels must NOT be PIC (usually)
stack_size   = 0                        # no default stack setup

[build.kernel]
linker_script = "kernel.ld"
target        = "x86_64-unknown-none"   # bare metal target
entry_symbol  = "_start"

[target.x86_64-unknown-none]
linker        = "ld"
linker_flags  = ["-n", "--gc-sections", "-z", "max-page-size=0x1000"]
features      = ["-mmx", "-sse", "-sse2", "+soft-float"]  # disable FPU in early boot
```

---

## 60. CPU Control Instructions

```gpl
import "core/cpu"

# Halt — stop CPU until next interrupt
cpu.halt()

# Halt loop — common kernel idle pattern
fn cpu_idle() -> never:
    loop:
        cpu.enable_interrupts()
        cpu.halt()              # waits for interrupt, then loops

# Interrupt control
cpu.enable_interrupts()         # sti
cpu.disable_interrupts()        # cli
var flags := cpu.save_flags()   # pushfq; pop rax
cpu.restore_flags(flags)        # push rax; popfq

# Interrupt-safe critical section
fn with_interrupts_disabled(f: fn() -> void) -> void:
    var flags := cpu.save_flags()
    cpu.disable_interrupts()
    f()
    cpu.restore_flags(flags)

# Memory barriers / fences
cpu.memory_fence()              # mfence
cpu.load_fence()                # lfence
cpu.store_fence()               # sfence

# Cache control
cpu.clflush(addr)               # flush cache line
cpu.prefetch(addr)              # prefetch cache line
cpu.wbinvd()                    # write-back + invalidate all caches (privileged)

# Read CPU timestamp counter
var tsc := cpu.rdtsc()          # -> u64

# Read/write MSRs (privileged)
unsafe:
    var val := cpu.rdmsr(0xC0000080)        # read MSR (e.g. EFER)
    cpu.wrmsr(0xC0000080, val | 0x100)      # write MSR

# CPUID
var info := cpu.cpuid(leaf: 0, subleaf: 0)
info.eax
info.ebx
info.ecx
info.edx

# I/O ports (x86)
unsafe:
    cpu.outb(port: 0x3F8, value: b'A')      # write byte to I/O port
    cpu.outw(port: 0x3F8, value: 0x1234)    # write word
    cpu.outd(port: 0x3F8, value: 0xDEAD)    # write dword
    var b := cpu.inb(port: 0x3F8)           # read byte
    var w := cpu.inw(port: 0x3F8)           # read word
    var d := cpu.ind(port: 0x3F8)           # read dword

# Control registers (x86_64, privileged)
unsafe:
    var cr0 := cpu.read_cr0()
    var cr2 := cpu.read_cr2()           # page fault address
    var cr3 := cpu.read_cr3()           # page table base
    var cr4 := cpu.read_cr4()
    cpu.write_cr0(cr0 | (1 << 16))      # set WP bit
    cpu.write_cr3(new_page_table)
    cpu.write_cr4(cr4 | (1 << 7))       # set PGE bit

# TLB
cpu.invlpg(virt_addr)           # invalidate single TLB entry
cpu.tlb_flush_all()             # reload CR3 (flushes all non-global)
```

---

## 61. Atomic Operations with Memory Ordering

Full memory ordering model — essential for lock-free kernel code.

### 61.1 Memory Ordering

```gpl
import "core/atomic"

enum atomic.Ordering:
    Relaxed     # no ordering guarantee — just atomicity
    Acquire     # no reads/writes can move before this load
    Release     # no reads/writes can move after this store
    AcqRel      # Acquire + Release (for RMW operations)
    SeqCst      # total sequential consistency (strongest, slowest)
```

**Ordering guidelines:**

| Operation         | Typical ordering                  |
|-------------------|-----------------------------------|
| Counter increment | `Relaxed`                         |
| Lock acquire      | `Acquire`                         |
| Lock release      | `Release`                         |
| Lock compare-exchange | `AcqRel` success / `Relaxed` fail |
| Publish pointer   | `Release` store, `Acquire` load   |
| Sequentially consistent flag | `SeqCst`            |

### 61.2 Atomic Types

```gpl
# Available types: I8, I16, I32, I64, U8, U16, U32, U64, Usize, Bool, Ptr[T]
var counter := atomic.U64.new(0)

# Load / Store
var v := counter.load(atomic.Ordering.Acquire)
counter.store(42, atomic.Ordering.Release)

# Read-Modify-Write
var old := counter.fetch_add(1,   atomic.Ordering.AcqRel)
var old := counter.fetch_sub(1,   atomic.Ordering.AcqRel)
var old := counter.fetch_and(mask, atomic.Ordering.AcqRel)
var old := counter.fetch_or(mask,  atomic.Ordering.AcqRel)
var old := counter.fetch_xor(mask, atomic.Ordering.AcqRel)
var old := counter.fetch_max(val,  atomic.Ordering.AcqRel)
var old := counter.fetch_min(val,  atomic.Ordering.AcqRel)
var old := counter.swap(new_val,   atomic.Ordering.AcqRel)

# Compare-and-Swap (CAS)
var result := counter.compare_exchange(
    expected: 0,
    new:      1,
    success:  atomic.Ordering.AcqRel,
    failure:  atomic.Ordering.Relaxed,
)
match result:
    Result.Ok(old)  => io.println("swapped, old was:", old)
    Result.Err(cur) => io.println("failed, current is:", cur)

# Weak CAS (may spuriously fail, but faster on some architectures)
var ok := counter.compare_exchange_weak(
    expected: 0,
    new:      1,
    success:  atomic.Ordering.AcqRel,
    failure:  atomic.Ordering.Relaxed,
)

# Atomic pointer
var ptr := atomic.Ptr[Node].new(null)
var old := ptr.swap(new_node, atomic.Ordering.AcqRel)
var cur := ptr.load(atomic.Ordering.Acquire)
```

### 61.3 Atomic Fence

```gpl
# Standalone fence — emit a barrier without an atomic operation
atomic.fence(atomic.Ordering.SeqCst)
atomic.fence(atomic.Ordering.Acquire)
atomic.fence(atomic.Ordering.Release)

# Compiler-only fence (no CPU instruction — prevents compiler reordering only)
atomic.compiler_fence(atomic.Ordering.AcqRel)
```

### 61.4 Lock-free Patterns

```gpl
# Spinlock using atomic
struct Spinlock:
    locked: atomic.U8

fn Spinlock.new() -> Spinlock:
    return Spinlock{locked: atomic.U8.new(0)}

fn Spinlock.lock(self: *Spinlock) -> void:
    while self.locked.compare_exchange_weak(
        expected: 0,
        new:      1,
        success:  atomic.Ordering.Acquire,
        failure:  atomic.Ordering.Relaxed,
    ).is_err():
        # spin — hint CPU we're in a spin loop
        cpu.pause()     # x86: rep nop / pause instruction

fn Spinlock.unlock(self: *Spinlock) -> void:
    self.locked.store(0, atomic.Ordering.Release)

fn Spinlock.try_lock(self: *Spinlock) -> bool:
    return self.locked.compare_exchange(
        expected: 0, new: 1,
        success: atomic.Ordering.Acquire,
        failure: atomic.Ordering.Relaxed,
    ).is_ok()

# Lock guard (RAII)
struct SpinlockGuard:
    lock: *Spinlock

impl Drop for SpinlockGuard:
    fn drop(self: *SpinlockGuard) -> void:
        self.lock.unlock()

fn Spinlock.acquire(self: *Spinlock) -> SpinlockGuard:
    self.lock()
    return SpinlockGuard{lock: self}

# Usage
var lock := Spinlock.new()
{
    var _guard := lock.acquire()    # locked
    # critical section
}                                   # _guard dropped here -> unlocked

# Lock-free stack (Treiber stack)
struct LockFreeStack[T]:
    head: atomic.Ptr[Node[T]]

struct Node[T]:
    value: T
    next:  *Node[T]

fn LockFreeStack[T].push(self: *LockFreeStack[T], val: T) -> void:
    var node := mem.alloc[Node[T]]()
    node->value = val
    loop:
        var head := self.head.load(atomic.Ordering.Relaxed)
        node->next = head
        if self.head.compare_exchange_weak(
            expected: head, new: node,
            success:  atomic.Ordering.Release,
            failure:  atomic.Ordering.Relaxed,
        ).is_ok():
            break

fn LockFreeStack[T].pop(self: *LockFreeStack[T]) -> Option[T]:
    loop:
        var head := self.head.load(atomic.Ordering.Acquire)
        if head == null: return Option.None
        var next := head->next
        if self.head.compare_exchange_weak(
            expected: head, new: next,
            success:  atomic.Ordering.Relaxed,
            failure:  atomic.Ordering.Relaxed,
        ).is_ok():
            var val := head->value
            unsafe: mem.free(head)
            return Option.Some(val)
```

---

## 62. Virtual Memory & Paging

```gpl
#![no_std]
#![no_runtime]
module kernel.mm

import "core/volatile"
import "core/atomic"

# Physical and virtual address newtypes (prevent mixing)
newtype PhysAddr = usize
newtype VirtAddr = usize

fn PhysAddr.page_aligned(self) -> bool:
    return (self as usize) % 4096 == 0

fn PhysAddr.page_frame(self) -> usize:
    return (self as usize) >> 12

fn VirtAddr.page_offset(self) -> usize:
    return (self as usize) & 0xFFF

fn VirtAddr.p1_index(self) -> usize: ((self as usize) >> 12) & 0x1FF
fn VirtAddr.p2_index(self) -> usize: ((self as usize) >> 21) & 0x1FF
fn VirtAddr.p3_index(self) -> usize: ((self as usize) >> 30) & 0x1FF
fn VirtAddr.p4_index(self) -> usize: ((self as usize) >> 39) & 0x1FF

# Page table entry flags
const PTE_PRESENT:    u64 = 1 << 0
const PTE_WRITABLE:   u64 = 1 << 1
const PTE_USER:       u64 = 1 << 2
const PTE_WRITE_THRU: u64 = 1 << 3
const PTE_NO_CACHE:   u64 = 1 << 4
const PTE_ACCESSED:   u64 = 1 << 5
const PTE_DIRTY:      u64 = 1 << 6
const PTE_HUGE:       u64 = 1 << 7
const PTE_GLOBAL:     u64 = 1 << 8
const PTE_NO_EXEC:    u64 = 1 << 63

@repr("C")
struct PageTableEntry:
    value: u64

fn PageTableEntry.new(phys: PhysAddr, flags: u64) -> PageTableEntry:
    return PageTableEntry{value: (phys as u64) | flags}

fn PageTableEntry.phys_addr(self) -> PhysAddr:
    return PhysAddr((self.value & 0x000FFFFF_FFFFF000) as usize)

fn PageTableEntry.flags(self) -> u64:
    return self.value & 0xFFF

fn PageTableEntry.is_present(self) -> bool:
    return self.value & PTE_PRESENT != 0

fn PageTableEntry.is_huge(self) -> bool:
    return self.value & PTE_HUGE != 0

@align(4096)
@repr("C")
struct PageTable:
    entries: [512]PageTableEntry

fn PageTable.zero(self: *PageTable) -> void:
    for i in 0..512:
        self.entries[i] = PageTableEntry{value: 0}

fn PageTable.map_page(
    self:  *PageTable,
    virt:  VirtAddr,
    phys:  PhysAddr,
    flags: u64,
    alloc: fn() -> PhysAddr,
) -> void:
    var p4 := self
    var p3 := table_at_or_create(&p4.entries[virt.p4_index()], alloc)
    var p2 := table_at_or_create(&p3.entries[virt.p3_index()], alloc)
    var p1 := table_at_or_create(&p2.entries[virt.p2_index()], alloc)
    p1.entries[virt.p1_index()] = PageTableEntry.new(phys, flags | PTE_PRESENT)
    cpu.invlpg(virt as usize)
```

---

## 63. Physical Memory Manager

```gpl
#![no_std]
#![no_runtime]
module kernel.mm.pmm

# Bitmap physical frame allocator
struct BitmapAllocator:
    bitmap:     *mut u8
    bitmap_len: usize       # in bytes
    total_frames: usize
    free_frames:  atomic.Usize

fn BitmapAllocator.init(
    self:         *BitmapAllocator,
    bitmap_base:  *mut u8,
    memory_limit: usize,
) -> void:
    self.bitmap     = bitmap_base
    self.total_frames = memory_limit / 4096
    self.bitmap_len = (self.total_frames + 7) / 8
    self.free_frames = atomic.Usize.new(0)
    # mark all as used initially; firmware/bootloader marks free regions
    core.mem.set(bitmap_base, 0xFF, self.bitmap_len)

fn BitmapAllocator.mark_free(self: *BitmapAllocator, phys: PhysAddr) -> void:
    var frame := (phys as usize) / 4096
    var byte  := frame / 8
    var bit   := frame % 8
    unsafe:
        volatile.write[u8](self.bitmap + byte,
            volatile.read[u8](self.bitmap + byte) & ~(1u8 << bit))
    self.free_frames.fetch_add(1, atomic.Ordering.Relaxed)

fn BitmapAllocator.alloc_frame(self: *BitmapAllocator) -> Option[PhysAddr]:
    for byte_idx in 0..self.bitmap_len:
        unsafe:
            var byte_val := volatile.read[u8](self.bitmap + byte_idx)
            if byte_val == 0xFF: continue    # all bits set = all used
            for bit in 0..8:
                if byte_val & (1u8 << bit) == 0:
                    # mark as used
                    volatile.write[u8](
                        self.bitmap + byte_idx,
                        byte_val | (1u8 << bit),
                    )
                    self.free_frames.fetch_sub(1, atomic.Ordering.Relaxed)
                    var frame := byte_idx * 8 + bit
                    return Option.Some(PhysAddr(frame * 4096))
    return Option.None

fn BitmapAllocator.free_frame(self: *BitmapAllocator, phys: PhysAddr) -> void:
    self.mark_free(phys)
```

---

## 64. Stack & Context Switching

```gpl
#![no_std]
#![no_runtime]
module kernel.sched

# Saved register state for context switch (x86_64)
@repr("C")
struct CpuContext:
    r15: u64
    r14: u64
    r13: u64
    r12: u64
    rbx: u64
    rbp: u64
    rip: u64    # return address / instruction pointer

# Context switch: save current registers, load new ones
# Called as: context_switch(&old_ctx, &new_ctx)
@naked
@callconv("sysv64")
fn context_switch(old: *mut CpuContext, new: *CpuContext) -> void:
    asm:
        # save callee-saved registers to old context
        "mov [rdi + 0x00], r15"
        "mov [rdi + 0x08], r14"
        "mov [rdi + 0x10], r13"
        "mov [rdi + 0x18], r12"
        "mov [rdi + 0x20], rbx"
        "mov [rdi + 0x28], rbp"
        # save return address (rsp points to it after call)
        "mov rax, [rsp]"
        "mov [rdi + 0x30], rax"
        # load new context
        "mov r15, [rsi + 0x00]"
        "mov r14, [rsi + 0x08]"
        "mov r13, [rsi + 0x10]"
        "mov r12, [rsi + 0x18]"
        "mov rbx, [rsi + 0x20]"
        "mov rbp, [rsi + 0x28]"
        "mov rax, [rsi + 0x30]"
        # jump to new task's instruction pointer
        "mov [rsp], rax"
        "ret"
        : : : clobber("rax", "memory")

# Task stack setup
fn setup_task_stack(stack_top: *mut u8, entry: fn() -> void) -> *mut CpuContext:
    unsafe:
        var sp := stack_top as usize
        # align to 16 bytes
        sp &= ~0xF
        # push entry point as fake return address
        sp -= 8
        *(sp as *mut u64) = entry as u64
        # reserve space for CpuContext
        sp -= @size_of(CpuContext)
        var ctx := sp as *mut CpuContext
        core.mem.set(ctx as *mut u8, 0, @size_of(CpuContext))
        ctx->rip = entry as u64
        return ctx
```

---

## 65. Kernel Address Space Types

```gpl
# Type-safe physical/virtual address wrappers with kernel helpers

newtype PhysAddr = usize
newtype VirtAddr = usize
newtype PhysFrame = usize   # frame number (PhysAddr / PAGE_SIZE)
newtype Page      = usize   # page number  (VirtAddr / PAGE_SIZE)

const PAGE_SIZE:  usize = 4096
const PAGE_SHIFT: usize = 12

fn PhysAddr.from_frame(frame: PhysFrame) -> PhysAddr:
    return PhysAddr((frame as usize) << PAGE_SHIFT)

fn PhysAddr.to_frame(self) -> PhysFrame:
    return PhysFrame((self as usize) >> PAGE_SHIFT)

fn PhysAddr.kernel_virt(self) -> VirtAddr:
    # assumes higher-half kernel mapping
    const KERNEL_OFFSET: usize = 0xFFFFFFFF80000000
    return VirtAddr((self as usize) + KERNEL_OFFSET)

fn VirtAddr.to_ptr[T](self) -> *mut T:
    return (self as usize) as *mut T

fn VirtAddr.is_canonical(self) -> bool:
    # x86_64: bits 48–63 must be sign extension of bit 47
    var v := self as usize
    var high := v >> 47
    return high == 0 or high == 0x1FFFF

fn VirtAddr.page_table_indices(self) -> (u9, u9, u9, u9, u12):
    var v := self as usize
    return (
        ((v >> 39) & 0x1FF) as u9,  # PML4 index
        ((v >> 30) & 0x1FF) as u9,  # PDPT index
        ((v >> 21) & 0x1FF) as u9,  # PD index
        ((v >> 12) & 0x1FF) as u9,  # PT index
        (v & 0xFFF) as u12,         # page offset
    )
```

---

## 66. Kernel `core/mem` — No-alloc Memory Operations

```gpl
import "core/mem"

# copy — like memcpy (regions must not overlap)
core.mem.copy(dst: *mut u8, src: *const u8, count: usize) -> void

# move — like memmove (handles overlapping regions)
core.mem.move(dst: *mut u8, src: *const u8, count: usize) -> void

# set — like memset
core.mem.set(dst: *mut u8, value: u8, count: usize) -> void

# compare — like memcmp
core.mem.compare(a: *const u8, b: *const u8, count: usize) -> i32

# zero out a typed value
core.mem.zero[T](ptr: *mut T) -> void:
    core.mem.set(ptr as *mut u8, 0, @size_of(T))

# swap two values in memory
core.mem.swap[T](a: *mut T, b: *mut T) -> void

# read unaligned (safe on x86, UB on strict-align archs without unsafe)
unsafe:
    var v: u32 = core.mem.read_unaligned[u32](ptr)
    core.mem.write_unaligned[u32](ptr, value)

# fence wrappers
core.mem.fence()            # sequential consistency fence
core.mem.acquire_fence()
core.mem.release_fence()

# size/align helpers (same as @size_of / @align_of but usable at runtime)
core.mem.size_of[T]()       # -> usize
core.mem.align_of[T]()      # -> usize
core.mem.size_of_val(val)   # -> usize (for dynamically-sized types)
```

---

---

## 67. Formal Grammar (EBNF)

This is the authoritative grammar for the G language. Implementations must conform exactly.

### Notation

```
rule       = definition
|          = alternation
[ ... ]    = optional (zero or one)
{ ... }    = repetition (zero or more)
( ... )    = grouping
"..."      = terminal string
'...'      = terminal character
UPPER      = lexer token (defined in §67.1)
lower      = parser rule (defined below)
```

---

### 67.1 Lexer Tokens

```ebnf
(* Whitespace — ignored between tokens *)
WHITESPACE  = ' ' | '\t' | '\r' | '\n' ;
NEWLINE     = '\n' | '\r\n' ;

(* Comments *)
LINE_COMMENT    = '#' { ANY_CHAR } NEWLINE ;
DOC_COMMENT     = '##' { ANY_CHAR } NEWLINE ;

(* Identifiers *)
IDENT       = ( ALPHA | '_' ) { ALPHA | DIGIT | '_' } ;
ALPHA       = 'a'..'z' | 'A'..'Z' ;
DIGIT       = '0'..'9' ;
HEX_DIGIT   = DIGIT | 'a'..'f' | 'A'..'F' ;
OCT_DIGIT   = '0'..'7' ;
BIN_DIGIT   = '0' | '1' ;

(* Keywords — reserved, cannot be used as IDENT *)
KEYWORD = 'module' | 'import' | 'as' | 'use'
        | 'var' | 'let' | 'const' | 'comptime'
        | 'fn' | 'return' | 'async' | 'await' | 'yield'
        | 'struct' | 'enum' | 'union' | 'interface' | 'impl'
        | 'embed' | 'pub' | 'extern'
        | 'if' | 'elif' | 'else'
        | 'for' | 'while' | 'loop' | 'in' | 'step'
        | 'match' | 'break' | 'continue' | 'defer'
        | 'and' | 'or' | 'not' | 'is'
        | 'true' | 'false' | 'null'
        | 'unsafe' | 'asm'
        | 'type' | 'newtype'
        | 'try' | 'catch' | 'finally'
        | 'panic' | 'assert'
        | 'select' | 'after' | 'default'
        | 'never' | 'void' | 'any'
        | 'self' | 'Self'
        | '_' ;

(* Primitive type keywords *)
TYPE_KW = 'i8' | 'i16' | 'i32' | 'i64' | 'i128'
        | 'u8' | 'u16' | 'u32' | 'u64' | 'u128'
        | 'f32' | 'f64'
        | 'bool' | 'byte' | 'rune' | 'str'
        | 'isize' | 'usize' ;

(* Integer literals *)
INT_LIT     = DEC_LIT | HEX_LIT | BIN_LIT | OCT_LIT ;
DEC_LIT     = DIGIT { DIGIT | '_' } [ INT_SUFFIX ] ;
HEX_LIT     = '0x' HEX_DIGIT { HEX_DIGIT | '_' } [ INT_SUFFIX ] ;
BIN_LIT     = '0b' BIN_DIGIT { BIN_DIGIT | '_' } [ INT_SUFFIX ] ;
OCT_LIT     = '0o' OCT_DIGIT { OCT_DIGIT | '_' } [ INT_SUFFIX ] ;
INT_SUFFIX  = 'i8' | 'i16' | 'i32' | 'i64' | 'i128'
            | 'u8' | 'u16' | 'u32' | 'u64' | 'u128'
            | 'usize' | 'isize' ;

(* Float literals *)
FLOAT_LIT   = ( DEC_LIT '.' [ DEC_FRAC ] [ FLOAT_EXP ]
              | '.' DEC_FRAC [ FLOAT_EXP ]
              | DEC_LIT FLOAT_EXP ) [ FLOAT_SUFFIX ] ;
DEC_FRAC    = DIGIT { DIGIT | '_' } ;
FLOAT_EXP   = ( 'e' | 'E' ) [ '+' | '-' ] DIGIT { DIGIT | '_' } ;
FLOAT_SUFFIX = 'f32' | 'f64' ;

(* String literals *)
STRING_LIT  = '"' { STR_CHAR } '"' ;
RAW_STR     = 'r"' { ANY_CHAR_EXCEPT_DQUOTE } '"' ;
FSTRING     = 'f"' { STR_CHAR | '{' expr '}' } '"' ;
MULTILINE   = '"""' NEWLINE { ANY_CHAR } '"""' ;
RAW_MULTI   = 'r"""' NEWLINE { ANY_CHAR } '"""' ;

STR_CHAR    = ESCAPE_SEQ | ANY_CHAR_EXCEPT_DQUOTE_BACKSLASH ;
ESCAPE_SEQ  = '\\' ( 'n' | 'r' | 't' | '\\' | '"' | "'" | '0'
                   | 'a' | 'b' | 'f' | 'v'
                   | 'x' HEX_DIGIT HEX_DIGIT
                   | 'u' '{' HEX_DIGIT { HEX_DIGIT } '}' ) ;

(* Character literal *)
CHAR_LIT    = "'" ( STR_CHAR ) "'" ;
BYTE_LIT    = "b'" ASCII_CHAR "'" ;

(* Lifetime *)
LIFETIME    = "'" IDENT ;

(* Compiler directive *)
DIRECTIVE   = '#' '!' '[' IDENT [ '(' IDENT ')' ] ']' ;
```

---

### 67.2 Top-level Structure

```ebnf
file        = { DIRECTIVE } module_decl { import_decl } { top_level_item } EOF ;

module_decl = 'module' module_path NEWLINE ;
module_path = IDENT { '.' IDENT } ;

import_decl = 'import' import_path [ 'as' IDENT ]
            | 'import' import_path ':' '{' import_list '}'
            | 'use' type_path '.' '*'
            ;
import_path = '"' PATH_STR '"' | '<' PATH_STR '>' ;
import_list = IDENT { ',' IDENT } [ ',' ] ;

top_level_item
    = fn_decl
    | struct_decl
    | enum_decl
    | union_decl
    | interface_decl
    | impl_decl
    | type_alias
    | newtype_decl
    | const_decl
    | var_decl
    | extern_decl
    ;
```

---

### 67.3 Declarations

```ebnf
(* Functions *)
fn_decl     = { attribute } [ 'pub' ] [ 'async' ] [ 'const' ] 'fn'
              fn_name '(' [ param_list ] ')' [ '->' type_expr ] fn_body ;
fn_name     = IDENT | type_path '.' IDENT ;
param_list  = param { ',' param } [ ',' ] ;
param       = [ IDENT ':' ] param_type [ '=' expr ]
            | '...' IDENT ':' '...' type_expr
            ;
param_type  = type_expr ;
fn_body     = ':' NEWLINE INDENT { stmt } DEDENT
            | ':' expr NEWLINE          (* single-expression shorthand *)
            ;

(* Structs *)
struct_decl = { attribute } [ 'pub' ] 'struct' IDENT [ generic_params ]
              ':' NEWLINE INDENT { struct_field } DEDENT ;
struct_field = { attribute } [ 'pub' ] [ 'embed' ] IDENT ':' type_expr
               [ '=' expr ] NEWLINE
             | fn_decl ;

(* Enums *)
enum_decl   = { attribute } [ 'pub' ] 'enum' IDENT [ generic_params ]
              ':' NEWLINE INDENT { enum_variant } DEDENT ;
enum_variant = { attribute } IDENT [ '(' named_field_list ')' ]
               [ '=' expr ] NEWLINE ;
named_field_list = named_field { ',' named_field } [ ',' ] ;
named_field = [ IDENT ':' ] type_expr ;

(* Unions *)
union_decl  = { attribute } [ 'pub' ] 'union' IDENT
              ':' NEWLINE INDENT { union_field } DEDENT ;
union_field = IDENT ':' type_expr NEWLINE ;

(* Interfaces *)
interface_decl = { attribute } [ 'pub' ] 'interface' IDENT [ generic_params ]
                 [ ':' interface_supers ]
                 ':' NEWLINE INDENT { interface_item } DEDENT ;
interface_supers = interface_bound { ',' interface_bound } ;
interface_bound  = type_path [ generic_args ] ;
interface_item   = fn_sig NEWLINE | fn_decl | type_assoc ;
fn_sig           = [ 'async' ] 'fn' IDENT '(' [ param_list ] ')' [ '->' type_expr ] ;
type_assoc       = 'type' IDENT [ '=' type_expr ] NEWLINE ;

(* Impl *)
impl_decl   = { attribute } 'impl' [ generic_params ] interface_bound
              'for' type_path [ generic_args ]
              ':' NEWLINE INDENT { fn_decl } DEDENT ;

(* Type aliases and newtypes *)
type_alias  = [ 'pub' ] 'type' IDENT [ generic_params ] '=' type_expr NEWLINE ;
newtype_decl = [ 'pub' ] 'newtype' IDENT '=' type_expr NEWLINE ;

(* Constants and variables *)
const_decl  = [ 'pub' ] 'const' IDENT ':' type_expr '=' expr NEWLINE ;
var_decl    = [ 'pub' ] 'var' var_names ':' type_expr [ '=' expr_list ] NEWLINE
            | [ 'pub' ] 'var' var_names ':=' expr NEWLINE
            | [ 'pub' ] 'let' IDENT ':' type_expr '=' expr NEWLINE ;
var_names   = IDENT { ',' IDENT } ;

(* Extern declarations *)
extern_decl = { attribute } 'fn' IDENT '(' [ extern_params ] ')' '->' type_expr NEWLINE
            | { attribute } 'var' IDENT ':' type_expr NEWLINE ;
extern_params = extern_param { ',' extern_param } [ '...' ] ;
extern_param  = type_expr ;
```

---

### 67.4 Type Expressions

```ebnf
type_expr   = primitive_type
            | '*' [ 'const' | 'mut' ] type_expr        (* pointer *)
            | '?' type_expr                             (* optional *)
            | '[' ']' type_expr                         (* slice *)
            | '[' expr ']' type_expr                    (* array *)
            | '(' type_list ')'                         (* tuple *)
            | 'fn' '(' [ type_list ] ')' '->' type_expr (* function type *)
            | 'map' '[' type_expr ']' type_expr         (* map *)
            | 'set' '[' type_expr ']'                   (* set *)
            | '&' [ LIFETIME ] [ 'mut' ] type_expr      (* borrow *)
            | '&' "'" 'static' type_expr                (* static borrow *)
            | type_path [ generic_args ]                 (* named type *)
            | 'impl' interface_bound                    (* impl trait *)
            | '!' | 'never'                             (* never type *)
            | 'void'                                    (* void *)
            | 'any'                                     (* any type *)
            ;

primitive_type = TYPE_KW ;
type_path   = IDENT { '.' IDENT } ;
type_list   = type_expr { ',' type_expr } [ ',' ] ;

generic_params = '[' generic_param { ',' generic_param } [ ',' ] ']' ;
generic_param  = IDENT [ ':' generic_bound ] [ '=' type_expr ]
               | LIFETIME ;
generic_bound  = interface_bound { '+' interface_bound }
               | LIFETIME ;
generic_args   = '[' ( type_expr | LIFETIME ) { ',' ( type_expr | LIFETIME ) } ']' ;
where_clause   = 'where' where_item { ',' where_item } ;
where_item     = type_expr ':' generic_bound
               | LIFETIME ':' LIFETIME ;
```

---

### 67.5 Statements

```ebnf
stmt        = var_stmt
            | assign_stmt
            | expr_stmt
            | return_stmt
            | break_stmt
            | continue_stmt
            | defer_stmt
            | if_stmt
            | while_stmt
            | for_stmt
            | loop_stmt
            | match_stmt
            | try_stmt
            | unsafe_stmt
            | asm_stmt
            | comptime_stmt
            | label_stmt
            | NEWLINE
            ;

var_stmt    = 'var' var_names ':' type_expr [ '=' expr_list ] NEWLINE
            | 'var' var_names ':=' expr NEWLINE
            | 'let' IDENT ':' type_expr '=' expr NEWLINE
            | '_' ':=' expr NEWLINE ;

assign_stmt = assign_target assign_op expr NEWLINE ;
assign_target = expr ;
assign_op   = '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '**='
            | '&=' | '|=' | '^=' | '<<=' | '>>='
            | '+%=' | '-%=' | '*%=' | '+|=' | '-|=' | '*|=' ;

expr_stmt   = expr NEWLINE ;
return_stmt = 'return' [ expr_list ] NEWLINE ;
break_stmt  = 'break' [ '@' IDENT ] NEWLINE ;
continue_stmt = 'continue' [ '@' IDENT ] NEWLINE ;
defer_stmt  = 'defer' ( expr NEWLINE | block ) ;

if_stmt     = 'if' expr ':' block
              { 'elif' expr ':' block }
              [ 'else' ':' block ] ;

while_stmt  = [ '@' IDENT ] 'while' expr ':' block ;

for_stmt    = [ '@' IDENT ] 'for' for_pattern 'in' expr
              [ 'step' expr ] ':' block ;
for_pattern = IDENT | IDENT ',' IDENT | '_' ;

loop_stmt   = [ '@' IDENT ] 'loop' ':' block ;

match_stmt  = 'match' expr ':' NEWLINE INDENT { match_arm } DEDENT ;
match_arm   = match_pattern { '|' match_pattern } [ 'if' expr ]
              '=>' ( expr NEWLINE | block ) ;
match_pattern
            = '_'
            | 'null'
            | 'true' | 'false'
            | INT_LIT | FLOAT_LIT | STRING_LIT | CHAR_LIT
            | type_path [ '(' pattern_fields ')' ]
            | '(' pattern_list ')'
            | IDENT                  (* binding *)
            | INT_LIT '..' INT_LIT   (* range pattern *)
            | INT_LIT '..=' INT_LIT
            ;
pattern_fields = pattern_field { ',' pattern_field } [ ',' ] ;
pattern_field  = [ IDENT ':' ] match_pattern ;
pattern_list   = match_pattern { ',' match_pattern } ;

try_stmt    = 'try' ':' block
              { 'catch' IDENT ':' type_expr ':' block }
              [ 'finally' ':' block ] ;

unsafe_stmt = 'unsafe' ':' block ;
asm_stmt    = 'asm' ':' NEWLINE INDENT asm_body DEDENT ;
asm_body    = { STRING_LIT NEWLINE }
              [ ':' asm_outputs ]
              [ ':' asm_inputs ]
              [ ':' asm_clobbers ] ;
asm_outputs  = asm_operand { ',' asm_operand } ;
asm_inputs   = asm_operand { ',' asm_operand } ;
asm_clobbers = STRING_LIT { ',' STRING_LIT } ;
asm_operand  = ( 'out' | 'in' | 'inout' ) '(' expr ')' ;

comptime_stmt = 'comptime' var_stmt ;

label_stmt  = '@' IDENT NEWLINE ;

block       = NEWLINE INDENT { stmt } DEDENT ;
```

---

### 67.6 Expressions

```ebnf
expr        = pipe_expr ;

pipe_expr   = or_expr { '|>' or_expr } ;

or_expr     = and_expr { 'or' and_expr } ;
and_expr    = not_expr { 'and' not_expr } ;
not_expr    = 'not' not_expr | coalesce_expr ;

coalesce_expr = compare_expr { '??' compare_expr } ;

compare_expr  = bitor_expr [ cmp_op bitor_expr ] ;
cmp_op        = '==' | '!=' | '<' | '>' | '<=' | '>=' | 'is' ;

bitor_expr  = bitxor_expr { '|' bitxor_expr } ;
bitxor_expr = bitand_expr { '^' bitand_expr } ;
bitand_expr = shift_expr  { '&' shift_expr  } ;
shift_expr  = add_expr    { ( '<<' | '>>' ) add_expr } ;

add_expr    = mul_expr { add_op mul_expr } ;
add_op      = '+' | '-' | '+%' | '-%' | '+|' | '-|' | '+!' | '-!' ;

mul_expr    = range_expr { mul_op range_expr } ;
mul_op      = '*' | '/' | '%' | '*%' | '*|' | '*!' ;

range_expr  = pow_expr [ ( '..' | '..=' ) pow_expr ] ;

pow_expr    = cast_expr [ '**' pow_expr ] ;    (* right-associative *)

cast_expr   = unary_expr { ( 'as' | 'as!' | 'as?' ) type_expr } ;

unary_expr  = unary_op unary_expr | postfix_expr ;
unary_op    = '-' | '~' | '&' | '&' 'mut' | '*' ;

postfix_expr = primary_expr { postfix_op } ;
postfix_op  = '.' IDENT                        (* field access *)
            | '->' IDENT                       (* pointer field access *)
            | '?.' IDENT                       (* optional chain field *)
            | '?.' IDENT '(' [ arg_list ] ')'  (* optional chain call *)
            | '[' expr ']'                     (* index *)
            | '[' expr ':' expr ']'            (* slice *)
            | '[' expr ':' ']'                 (* slice to end *)
            | '[' ':' expr ']'                 (* slice from start *)
            | '[' ':' ']'                      (* full slice *)
            | '(' [ arg_list ] ')'             (* function call *)
            | '?'                              (* error propagation *)
            | '!'                              (* unwrap *)
            ;

primary_expr = INT_LIT | FLOAT_LIT | STRING_LIT | FSTRING | CHAR_LIT
             | BYTE_LIT | 'true' | 'false' | 'null'
             | IDENT
             | type_path '{' [ field_init_list ] '}'  (* struct literal *)
             | type_path '(' [ expr_list ] ')'         (* tuple/positional struct *)
             | '[' [ expr_list ] ']'                   (* array/slice literal *)
             | '{' [ map_entry_list ] '}'              (* map literal *)
             | '(' expr_list ')'                       (* tuple or grouped expr *)
             | lambda_expr
             | if_expr
             | match_expr
             | block_expr
             | builtin_call
             ;

lambda_expr  = 'fn' '(' [ param_list ] ')' '->' type_expr ':'
               ( expr | NEWLINE block ) ;

if_expr      = 'if' expr ':' expr 'elif' expr ':' expr 'else' ':' expr ;
match_expr   = 'match' expr ':' NEWLINE INDENT { match_arm } DEDENT ;
block_expr   = NEWLINE INDENT { stmt } expr NEWLINE DEDENT ;

field_init_list = field_init { ',' field_init } [ ',' ]
                | field_init_list ',' '..' expr  (* struct update syntax *)
                ;
field_init   = IDENT ':' expr | IDENT ;     (* IDENT alone = shorthand for IDENT: IDENT *)

map_entry_list = map_entry { ',' map_entry } [ ',' ] ;
map_entry    = expr ':' expr ;

arg_list     = arg { ',' arg } [ ',' ] ;
arg          = expr | IDENT ':' expr | expr '...' ;  (* named arg | spread *)

expr_list    = expr { ',' expr } ;

builtin_call = '@' IDENT [ '[' type_expr ']' ] '(' [ arg_list ] ')' ;
```

---

### 67.7 Attributes

```ebnf
attribute   = '@' IDENT [ '(' attr_args ')' ] NEWLINE? ;
attr_args   = attr_arg { ',' attr_arg } ;
attr_arg    = IDENT | STRING_LIT | INT_LIT | IDENT '=' attr_val
            | IDENT '(' attr_args ')' ;
attr_val    = STRING_LIT | INT_LIT | IDENT | 'true' | 'false' ;
```

---

### 67.8 Indentation Rules

G uses significant indentation (like Python). The lexer produces virtual `INDENT` and `DEDENT` tokens.

**Rules:**
1. Indentation unit is **4 spaces**. Tabs are not allowed. Mixing is a lexer error.
2. After a `:` that ends a line, the next non-empty line must be indented exactly 4 spaces more.
3. A line indented less than the current block closes zero or more blocks (`DEDENT` tokens).
4. A line at the same indentation continues the current block.
5. Blank lines and comment-only lines do not affect indentation tracking.
6. The file starts at indentation level 0.
7. `DEDENT` tokens are emitted to match all open `INDENT` tokens at EOF.

**Example of indent/dedent token stream:**
```
fn foo() -> void:          # NEWLINE INDENT
    if x:                  # NEWLINE INDENT
        return             # NEWLINE DEDENT
    io.println("hi")       # NEWLINE DEDENT
```

---

### 67.9 Operator Precedence (Formal)

This is the normative precedence table. Higher number = tighter binding.

| Prec | Operator(s)                                              | Assoc  |
|------|----------------------------------------------------------|--------|
| 14   | `()` `[]` `.` `->` `?.` (postfix)                       | Left   |
| 13   | `-` `~` `&` `&mut` `*` (unary prefix)                  | Right  |
| 12   | `as` `as!` `as?`                                        | Left   |
| 11   | `**`                                                     | Right  |
| 10   | `*` `/` `%` `*%` `*\|` `*!`                            | Left   |
| 9    | `+` `-` `+%` `-%` `+\|` `-\|` `+!` `-!`               | Left   |
| 8    | `<<` `>>`                                               | Left   |
| 7    | `&` (binary bitwise AND)                                | Left   |
| 6    | `^`                                                     | Left   |
| 5    | `\|` (binary bitwise OR)                                | Left   |
| 4    | `..` `..=`                                              | None   |
| 3    | `==` `!=` `<` `>` `<=` `>=` `is`                       | None   |
| 2    | `not`                                                   | Right  |
| 1    | `and`                                                   | Left   |
| 0    | `or` `??` `\|>`                                        | Left   |

---

## 68. Behavior Specification

This section defines the **normative behavior** of G programs — resolving all ambiguities.  
Implementations must reproduce this behavior exactly.

---

### 68.1 Evaluation Order

**Function arguments** are evaluated **left-to-right**, strictly:
```gpl
f(a(), b(), c())    # a() first, then b(), then c()
```

**Binary operator operands** are evaluated **left-to-right**:
```gpl
x + y    # x evaluated first, then y
```

**Struct literal fields** are evaluated in **declaration order** (not source order):
```gpl
Point{y: f(), x: g()}    # g() runs first (x declared first in struct)
```

**Short-circuit operators:**
- `a and b`: if `a` is `false`, `b` is **not evaluated**
- `a or b`: if `a` is `true`, `b` is **not evaluated**
- `a ?? b`: if `a` is not `null`, `b` is **not evaluated**

**`defer` statements** are evaluated in **LIFO order** when the enclosing scope exits,  
regardless of how the scope exits (normal return, error propagation, panic).

---

### 68.2 Move Semantics

**Definition:** Moving a value transfers ownership. The source is no longer valid.

**When a move occurs:**
- Assigning a non-`Copy` value to a new binding: `var b := a`
- Passing a non-`Copy` value to a function by value: `f(a)`
- Returning a non-`Copy` value from a function: `return a`
- Storing a non-`Copy` value in a struct or collection

**After a move**, using the original binding is a **compile error**:
```gpl
var a := Buffer.new(64)
var b := a              # a is moved to b
f(a)                    # ERROR: use of moved value 'a'
```

**Copy types** (implementing `Copy`) are implicitly copied instead of moved.  
Built-in `Copy` types: all primitives (`i8`..`u128`, `f32`, `f64`, `bool`, `byte`, `rune`, `usize`, `isize`),  
raw pointers (`*T`, `*const T`), and any struct/enum whose fields are all `Copy`.

**Return value optimization (RVO):** The compiler may construct a return value  
directly in the caller's stack frame, eliding a move. Behavior is identical to a move.

---

### 68.3 Closure Capture Semantics

Closures capture variables from their enclosing scope. Capture mode is **inferred** per variable:

| Usage in closure body         | Capture mode       |
|-------------------------------|--------------------|
| Only read, type is `Copy`     | Copy (value copy)  |
| Only read, type is non-`Copy` | Borrow (immutable) |
| Written to                    | Borrow (mutable)   |
| Outlives enclosing scope      | Move               |

**Move closure** — force all captures to be moved:
```gpl
var data := Buffer.new(64)
var f := move fn() -> void:
    use_buffer(data)    # data moved into closure
# data is no longer valid here
```

**Borrow closure** — default, borrows enclosing variables:
```gpl
var count := 0
var inc := fn() -> void:
    count += 1    # borrows count mutably
inc()
io.println(count)    # 1
```

**Capture of loop variable:** Each loop iteration creates a **new binding** for the loop variable:
```gpl
for i in 0..5:
    var f := fn() -> i32: i    # each f captures its own copy of i
```

---

### 68.4 Function Parameter Passing

| Parameter type      | Behavior                                      |
|---------------------|-----------------------------------------------|
| `T` (value)         | Move if non-Copy, copy if Copy                |
| `*T` (pointer)      | Pass pointer value (Copy)                     |
| `*const T`          | Pass const pointer (Copy)                     |
| `&T` (borrow)       | Immutable borrow — caller retains ownership   |
| `&mut T`            | Mutable borrow — caller retains ownership     |
| `...T` (variadic)   | Caller constructs a slice; callee sees `[]T`  |

**`self` receiver rules:**
- `fn Foo.method(self)` — immutable value receiver (copy if Copy, error if non-Copy without `&`)
- `fn Foo.method(self: *Foo)` — mutable pointer receiver
- `fn Foo.method(self: *const Foo)` — const pointer receiver
- `fn Foo.method(self: &Foo)` — immutable borrow receiver
- `fn Foo.method(self: &mut Foo)` — mutable borrow receiver

---

### 68.5 Drop Order

Values are dropped in the following order:

1. Local variables are dropped in **reverse declaration order** when their scope exits.
2. Struct fields are dropped in **reverse field declaration order** after the struct's `Drop.drop` body runs.
3. Enum variants with data: the data is dropped when the enum value is dropped.
4. Function arguments are dropped in **reverse parameter order** after the function returns.
5. Temporaries in an expression are dropped at the **end of the statement** containing them.
6. `defer` closures run **before** local variable drops.

**Drop during panic:** All in-scope values are dropped normally during a panic unwind,  
unless `@no_unwind` is set, in which case drops are skipped and the process aborts.

**Double-drop prevention:** After `Drop.drop` is called, the compiler ensures the value  
is never dropped again. Moving a value out of a `Drop` type is a compile error unless  
the field is replaced before `drop` returns.

---

### 68.6 Integer Overflow Behavior

| Build mode | Default behavior       | Explicit operators       |
|------------|------------------------|--------------------------|
| `debug`    | Panic with message     | `+%` wraps, `+\|` saturates, `+!` returns `Option` |
| `release`  | Two's complement wrap  | Same                     |
| `test`     | Panic with message     | Same                     |

The default can be overridden globally in `gpl.toml`:
```toml
[profile.release]
overflow = "panic"    # "wrap" | "panic" | "undefined"
```

`"undefined"` enables UB-based optimizations (not recommended except for expert use).

**Shift overflow:** Shifting by ≥ bit width of the type is always a panic in debug,  
and implementation-defined (but not UB) in release.

**Float overflow:** IEEE 754 rules always apply. No panic. `+inf`, `-inf`, `nan` are valid results.

---

### 68.7 String Behavior

- `str` is a **fat pointer**: `(ptr: *const u8, len: usize)`. It is always valid UTF-8.
- Indexing `s[i]` returns a `byte` (`u8`), not a `rune`. This is O(1).
- Slicing `s[i:j]` is valid only if `i` and `j` are on UTF-8 character boundaries. Panics otherwise.
- `s.len` is the **byte length**, not the character count. O(1).
- `s.rune_count()` is the **character count**. O(n).
- String literals are stored in the `.rodata` section. They have `'static` lifetime.
- Two string literals with the same content **may or may not** share memory (implementation-defined).
- `str` is **immutable**. To build or modify strings, use `string.Builder`.
- `f"..."` interpolation is syntactic sugar: the compiler desugars it into `string.Builder` calls.

---

### 68.8 Null and Optional Semantics

- `null` is a value of type `?T` for any `T`. It is **not** a valid `*T`.
- Dereferencing a `null` pointer (`*null_ptr`) is **undefined behavior** inside `unsafe`,  
  and a **compile error** outside `unsafe`.
- `?T` has the same size as `*T` for pointer types (null pointer optimization applies).
- For non-pointer `?T`, the size is `@size_of(T) + 1` byte (with padding for alignment).
- `Option.None` and `null` are interchangeable: `Option.None` desugars to `null` for `?T`.

---

### 68.9 Panic Behavior

**What causes a panic:**
- Explicit `panic(msg)` call
- Failed `assert(cond, msg)` in debug/test builds
- Integer overflow in debug/test builds (unless `+%` / `+|` used)
- Out-of-bounds slice/array access (unless `@bounds_check(false)`)
- Calling `.unwrap()` on `Option.None` or `Result.Err`
- Stack overflow (platform-dependent, best-effort)
- Division by zero (integer only; float division by zero yields `inf`)
- Null pointer dereference in safe code (compile error) or unsafe code (UB)

**Panic unwind:**
1. The panic message is formatted (file, line, message).
2. Stack unwind begins: each stack frame's `defer`s run, then locals are dropped.
3. If a `try/catch` block with matching `PanicError` exists, it catches the panic.
4. If uncaught, the `@panic_handler` is called (user-defined or default).
5. Default panic handler: prints to stderr and calls `os.exit(101)`.

**In `no_std` mode:** The `@panic_handler` attribute is **required**. No default handler.

---

### 68.10 Borrow Checker Rules (Normative)

The borrow checker enforces these rules at compile time:

1. **Exclusivity:** At any point in the program, for a given value `v`,  
   either (a) any number of immutable borrows `&v` exist,  
   or (b) exactly one mutable borrow `&mut v` exists.  
   Never both simultaneously.

2. **Lifetime containment:** A borrow's lifetime must not exceed the lifetime of the borrowed value.

3. **No borrow through move:** You cannot borrow a value that has been moved.

4. **Pointer invalidation:** Creating a mutable borrow of a collection invalidates all existing  
   element pointers into that collection.

5. **Self-borrow in methods:** Calling a `&mut self` method while a borrow of `self` (or any  
   field of `self`) is active is a compile error.

6. **Return borrows:** A function returning `&T` must return a borrow derived from one of its parameters  
   or a `'static` reference. It cannot return a borrow of a local variable.

---

### 68.11 Const fn Restrictions (Normative)

A function marked `const fn` must satisfy:
- No heap allocation (`mem.alloc`, `mem.alloc_array`)
- No I/O operations (`io.println`, `fs.read`, etc.)
- No mutation of global/static variables
- No `unsafe` blocks
- No `async`/`await`
- No runtime trait dispatch (only static dispatch allowed)
- No `loop` without a guaranteed `break` (must terminate)
- All called functions must also be `const fn`
- No floating-point operations in `const fn` (platform-specific behavior)

Violation is a **compile error**, not a runtime error.

---

### 68.12 Undefined Behavior (UB)

The following are **undefined behavior** in G. The compiler may assume they never occur:

| # | Undefined behavior                                              |
|---|----------------------------------------------------------------|
| 1 | Dereferencing a null or dangling pointer (in `unsafe`)        |
| 2 | Data race: concurrent unsynchronized access to shared mutable data |
| 3 | Use of uninitialized memory                                   |
| 4 | Reading union field that was not the last written             |
| 5 | Out-of-bounds pointer arithmetic (beyond one-past-end)        |
| 6 | Integer overflow when `overflow = "undefined"` in profile     |
| 7 | Shift by ≥ bit width in release mode                          |
| 8 | Calling a function through a pointer with wrong signature     |
| 9 | Violating aliasing rules in `unsafe` (two `*mut T` to same location) |
| 10| Returning from a `@naked` function without restoring stack    |
| 11| Writing to a `*const T` (casting away const and writing)      |
| 12| Creating a `str` that is not valid UTF-8                      |

All UB in the above list is only reachable through `unsafe` blocks,  
with the exception of items 2 (data race) and 6 (explicit profile setting).

---

### 68.13 Implementation-Defined Behavior

The following behavior is **implementation-defined** (must be documented by the compiler):

- Struct field padding and alignment (beyond `@packed` and `@align` guarantees)
- Enum discriminant values (when not explicitly assigned)
- `any` internal representation
- Size of `bool` (guaranteed ≥ 1 byte, exact size implementation-defined)
- Order of iteration over `map[K]V` (explicitly unordered)
- Whether string literals are deduplicated in `.rodata`
- Signal handler stack size
- Maximum recursion depth before stack overflow

---

## 69. ABI Specification

The G ABI defines struct layout, calling conventions, name mangling, and FFI compatibility.  
This section is normative for all G implementations targeting the same platform.

---

### 69.1 Primitive Type Sizes & Alignment

| Type     | Size (bytes) | Alignment (bytes) |
|----------|-------------|-------------------|
| `i8`     | 1           | 1                 |
| `i16`    | 2           | 2                 |
| `i32`    | 4           | 4                 |
| `i64`    | 8           | 8                 |
| `i128`   | 16          | 16                |
| `u8`     | 1           | 1                 |
| `u16`    | 2           | 2                 |
| `u32`    | 4           | 4                 |
| `u64`    | 8           | 8                 |
| `u128`   | 16          | 16                |
| `f32`    | 4           | 4                 |
| `f64`    | 8           | 8                 |
| `bool`   | 1           | 1                 |
| `byte`   | 1           | 1                 |
| `rune`   | 4           | 4                 |
| `usize`  | arch        | arch              |
| `isize`  | arch        | arch              |
| `*T`     | arch        | arch              |
| `str`    | 2×arch      | arch              |
| `[]T`    | 2×arch      | arch              |

`arch` = 4 on 32-bit targets, 8 on 64-bit targets.

---

### 69.2 Struct Layout

Without attributes, struct layout follows these rules:

1. Fields are laid out in **declaration order**.
2. Each field is aligned to its natural alignment (see §69.1).
3. Padding bytes are inserted between fields as needed.
4. The struct's total size is rounded up to a multiple of its largest field alignment.
5. The struct's alignment is the maximum alignment of any field.

```gpl
struct Example:
    a: u8     # offset 0, size 1
              # 3 bytes padding
    b: u32    # offset 4, size 4
    c: u8     # offset 8, size 1
              # 7 bytes padding
    d: u64    # offset 16, size 8
# total size: 24, alignment: 8
```

**`@packed` struct:** No padding. Fields are adjacent at byte boundaries.  
Access may be unaligned — compiler emits unaligned load/store instructions or byte-by-byte copies.

**`@align(N)` struct:** Minimum alignment raised to N. N must be a power of 2.

**`@repr("C")` struct:** Layout is identical to the equivalent C struct with the same fields  
under the platform's C ABI. This is required for FFI interoperability.

---

### 69.3 Enum Layout

**Basic enum (no data):**
- Discriminant type: smallest integer type that fits all variant values.
- If all variants have explicit values, the discriminant is the assigned integer.
- Default: variants numbered 0, 1, 2, ... in declaration order.

**Tagged union enum (with data):**
- Layout: `(discriminant, union_of_variant_payloads)`.
- Discriminant: `u8` if ≤ 256 variants, `u16` if ≤ 65536, `u32` otherwise.
- Size: discriminant size + max payload size + alignment padding.
- Null pointer optimization: if enum has exactly one non-data variant and one pointer variant,  
  the null pointer value is used for the non-data variant (no discriminant field needed).

---

### 69.4 Calling Convention (x86_64 Linux/macOS — SysV AMD64 ABI)

**Integer/pointer arguments:** RDI, RSI, RDX, RCX, R8, R9 (in order), then stack.  
**Float arguments:** XMM0–XMM7 (in order), then stack.  
**Return values:**
- ≤ 8 bytes: RAX
- 9–16 bytes: RAX + RDX
- > 16 bytes: caller allocates buffer, passes pointer in RDI (hidden first argument), returned in RAX

**Caller-saved:** RAX, RCX, RDX, RSI, RDI, R8–R11, XMM0–XMM15  
**Callee-saved:** RBX, RSP, RBP, R12–R15

**Stack alignment:** RSP must be 16-byte aligned before a `call` instruction  
(i.e., 8-byte aligned at the point of `call`, since `call` pushes 8 bytes).

**G functions follow SysV ABI by default on Linux/macOS.**

---

### 69.5 Calling Convention (x86_64 Windows — Microsoft x64 ABI)

**Integer/pointer arguments:** RCX, RDX, R8, R9 (in order), then stack (32-byte shadow space).  
**Float arguments:** XMM0–XMM3 (in order, same slots as integer args), then stack.  
**Return values:**
- ≤ 8 bytes: RAX
- 9–16 bytes: via hidden pointer in RCX
- > 8 bytes: always via hidden pointer

**Caller-saved:** RAX, RCX, RDX, R8–R11, XMM0–XMM5  
**Callee-saved:** RBX, RBP, RDI, RSI, RSP, R12–R15, XMM6–XMM15

**G functions follow Microsoft x64 ABI by default on Windows.**

---

### 69.6 Name Mangling

G uses a deterministic name mangling scheme for exported symbols.

**Format:** `_G_<module>_<name>_<type_hash>`

Where:
- `<module>` = module path with `.` replaced by `__`
- `<name>` = function or variable name
- `<type_hash>` = 8-character hex hash of the function signature

**Examples:**
```
module main
fn add(a: i32, b: i32) -> i32

# mangled: _G_main_add_3f8a21bc

module std.io
fn println(s: str)

# mangled: _G_std__io_println_a1b2c3d4
```

**`@extern("C")` and `@export`:** No mangling. Symbol name is used as-is (or as specified by `@link_name`).

**`pub` functions in a library:** Mangled by default. Use `@export` to export with C name.

---

### 69.7 Object File Format

| Platform      | Object format | Executable format |
|---------------|---------------|-------------------|
| Linux         | ELF64         | ELF64             |
| macOS         | Mach-O 64     | Mach-O 64         |
| Windows       | COFF/PE64     | PE32+             |
| WebAssembly   | Wasm module   | Wasm module       |
| Bare metal    | ELF64         | Raw binary / ELF  |

---

## 70. SMP (Symmetric Multi-Processing) Kernel Support

### 70.1 `per_cpu` Variables

Per-CPU variables have a **separate instance per logical CPU core**,  
accessed via the CPU's segment register (GS on x86_64) or platform equivalent.

```gpl
#![no_std]
#![no_runtime]
module kernel.smp

import "core/atomic"
import "core/cpu"

# Declare a per-CPU variable
@per_cpu
var current_task: *Task = null

@per_cpu
var cpu_id: u32 = 0

@per_cpu
var idle_stack: [8192]u8

# Access per-CPU variable (reads from current CPU's instance)
fn get_current_task() -> *Task:
    return @per_cpu_read(current_task)

fn set_current_task(t: *Task) -> void:
    @per_cpu_write(current_task, t)

# Access another CPU's instance
fn get_task_on_cpu(cpu: u32) -> *Task:
    return @per_cpu_read_on(current_task, cpu)
```

### 70.2 AP (Application Processor) Startup Trampoline

The BSP (Bootstrap Processor) starts the system. APs are started via INIT-SIPI-SIPI sequence.  
The trampoline must run in real mode (16-bit) and transition to long mode (64-bit).

```gpl
#![no_std]
#![no_runtime]
module kernel.smp.trampoline

# Trampoline placed at a low physical address (e.g. 0x8000)
# Must be position-independent and fit in one 4KB page

@section(".trampoline")
@align(4096)
var TRAMPOLINE_START: u8

# Real mode → protected mode → long mode transition (x86_64)
@section(".trampoline.text")
@naked
@callconv("cdecl")
fn ap_trampoline_16() -> never:
    asm:
        ".code16"
        "cli"
        "cld"
        # load GDT for protected mode
        "lgdt [gdt32_ptr - ap_trampoline_16 + 0x8000]"
        "mov eax, cr0"
        "or eax, 1"
        "mov cr0, eax"             # enter protected mode
        "jmp 0x08:ap_trampoline_32 - ap_trampoline_16 + 0x8000"
        : : :

@section(".trampoline.text")
@naked
@callconv("cdecl")
fn ap_trampoline_32() -> never:
    asm:
        ".code32"
        # set up segments
        "mov ax, 0x10"
        "mov ds, ax"
        "mov es, ax"
        "mov ss, ax"
        # enable PAE
        "mov eax, cr4"
        "or eax, 0x20"
        "mov cr4, eax"
        # load PML4 from BSP
        "mov eax, [pml4_phys_addr]"
        "mov cr3, eax"
        # enable long mode via EFER
        "mov ecx, 0xC0000080"
        "rdmsr"
        "or eax, 0x100"
        "wrmsr"
        # enable paging → activates long mode
        "mov eax, cr0"
        "or eax, 0x80000001"
        "mov cr0, eax"
        "jmp 0x08:ap_trampoline_64"
        : : :

@section(".trampoline.text")
@naked
fn ap_trampoline_64() -> never:
    asm:
        ".code64"
        # set up 64-bit GDT and stack
        "lgdt [gdt64_ptr]"
        "mov rsp, [ap_stack_top]"
        "xor rbp, rbp"
        "call ap_main"
        "hlt"
        : : :

# Called by each AP after entering 64-bit mode
fn ap_main() -> never:
    var id := lapic_id()
    @per_cpu_write(cpu_id, id)
    cpu.enable_interrupts()
    kernel_ap_init(id)
    cpu_idle()
```

### 70.3 Inter-Processor Interrupts (IPI)

```gpl
import "core/volatile"

const LAPIC_BASE:    usize = 0xFEE00000
const LAPIC_ICR_LO:  usize = 0x300
const LAPIC_ICR_HI:  usize = 0x310
const LAPIC_EOI:     usize = 0x0B0

fn lapic_write(offset: usize, val: u32) -> void:
    unsafe:
        volatile.write[u32]((LAPIC_BASE + offset) as *mut u32, val)

fn lapic_read(offset: usize) -> u32:
    unsafe:
        return volatile.read[u32]((LAPIC_BASE + offset) as *const u32)

fn lapic_eoi() -> void:
    lapic_write(LAPIC_EOI, 0)

# Send IPI to specific APIC ID
fn send_ipi(apic_id: u8, vector: u8, mode: u8) -> void:
    # write destination first
    lapic_write(LAPIC_ICR_HI, (apic_id as u32) << 24)
    # write command (triggers send)
    lapic_write(LAPIC_ICR_LO, (vector as u32) | ((mode as u32) << 8))
    # wait for delivery
    while lapic_read(LAPIC_ICR_LO) & (1 << 12) != 0: {}

# Broadcast IPI to all APs (excluding self)
fn broadcast_ipi(vector: u8) -> void:
    lapic_write(LAPIC_ICR_HI, 0)
    lapic_write(LAPIC_ICR_LO, (vector as u32) | (0b11 << 18) | (1 << 19))
    while lapic_read(LAPIC_ICR_LO) & (1 << 12) != 0: {}

# TLB shootdown via IPI
var tlb_shootdown_addr: atomic.Usize = atomic.Usize.new(0)

fn tlb_shootdown(virt: VirtAddr) -> void:
    tlb_shootdown_addr.store(virt as usize, atomic.Ordering.Release)
    broadcast_ipi(IPI_VECTOR_TLB_SHOOTDOWN)
    # wait for all CPUs to acknowledge
    while tlb_ack_count.load(atomic.Ordering.Acquire) < cpu_count() - 1: {}
    tlb_ack_count.store(0, atomic.Ordering.Release)

@interrupt
fn ipi_handler_tlb_shootdown(frame: *InterruptFrame) -> void:
    var addr := tlb_shootdown_addr.load(atomic.Ordering.Acquire)
    cpu.invlpg(addr)
    tlb_ack_count.fetch_add(1, atomic.Ordering.Release)
    lapic_eoi()
```

### 70.4 Slab Allocator (Kernel Heap)

```gpl
#![no_std]
#![no_runtime]
module kernel.mm.slab

import "core/atomic"
import "core/mem"

# Slab allocator: fixed-size object pool backed by pages
struct SlabCache:
    obj_size:  usize
    obj_align: usize
    free_list: atomic.Ptr[FreeObject]
    page_list: *SlabPage
    total:     atomic.Usize
    in_use:    atomic.Usize

struct FreeObject:
    next: *FreeObject

struct SlabPage:
    next: *SlabPage
    base: PhysAddr
    virt: VirtAddr
    used: u64           # bitmap of 64 objects per page

fn SlabCache.new(obj_size: usize, obj_align: usize) -> SlabCache:
    return SlabCache{
        obj_size:  if obj_size < @size_of(*void): @size_of(*void) else: obj_size,
        obj_align: obj_align,
        free_list: atomic.Ptr[FreeObject].new(null),
        page_list: null,
        total:     atomic.Usize.new(0),
        in_use:    atomic.Usize.new(0),
    }

fn SlabCache.alloc(self: *SlabCache) -> ?*void:
    # try free list first (lock-free)
    loop:
        var head := self.free_list.load(atomic.Ordering.Acquire)
        if head == null: break
        var next := head->next
        if self.free_list.compare_exchange_weak(
            expected: head, new: next,
            success: atomic.Ordering.AcqRel,
            failure: atomic.Ordering.Relaxed,
        ).is_ok():
            self.in_use.fetch_add(1, atomic.Ordering.Relaxed)
            return head as *void
    # no free objects — allocate new slab page
    return self.alloc_from_new_page()

fn SlabCache.free(self: *SlabCache, ptr: *void) -> void:
    var obj := ptr as *FreeObject
    loop:
        var head := self.free_list.load(atomic.Ordering.Relaxed)
        obj->next = head
        if self.free_list.compare_exchange_weak(
            expected: head, new: obj,
            success: atomic.Ordering.Release,
            failure: atomic.Ordering.Relaxed,
        ).is_ok():
            self.in_use.fetch_sub(1, atomic.Ordering.Relaxed)
            return

# Global kernel slab caches (common sizes)
@per_cpu var SLAB_16:   SlabCache
@per_cpu var SLAB_32:   SlabCache
@per_cpu var SLAB_64:   SlabCache
@per_cpu var SLAB_128:  SlabCache
@per_cpu var SLAB_256:  SlabCache
@per_cpu var SLAB_512:  SlabCache
@per_cpu var SLAB_1024: SlabCache
@per_cpu var SLAB_2048: SlabCache

fn kmalloc(size: usize) -> ?*void:
    if size <= 16:   return (@per_cpu_ref(SLAB_16)).alloc()
    if size <= 32:   return (@per_cpu_ref(SLAB_32)).alloc()
    if size <= 64:   return (@per_cpu_ref(SLAB_64)).alloc()
    if size <= 128:  return (@per_cpu_ref(SLAB_128)).alloc()
    if size <= 256:  return (@per_cpu_ref(SLAB_256)).alloc()
    if size <= 512:  return (@per_cpu_ref(SLAB_512)).alloc()
    if size <= 1024: return (@per_cpu_ref(SLAB_1024)).alloc()
    if size <= 2048: return (@per_cpu_ref(SLAB_2048)).alloc()
    return page_alloc_large(size)   # fallback to page allocator
```

### 70.5 `#![no_fp]` — Disable Floating Point

Kernels must save/restore FPU state on context switch, which is expensive.  
Many kernels disable FPU in kernel context entirely.

```gpl
#![no_std]
#![no_runtime]
#![no_fp]           # disables all f32/f64 operations in this module
module kernel.core

# With #![no_fp], using f32 or f64 is a compile error:
var x: f32 = 3.14   # ERROR: floating-point not allowed in #![no_fp] module

# Exception: @allow_fp overrides locally
@allow_fp
fn compute_float() -> f32:
    return 3.14     # OK: explicitly opted in
```

When `#![no_fp]` is active:
- The compiler emits no FPU/SSE/AVX instructions for this module.
- All SIMD types and operations are also disabled.
- `f32` and `f64` types cannot be used.
- Applies to all functions in the module, including inlined functions from other modules.

---

## 71. Missing Standard Library Modules

### 71.1 `std/xml` — XML Parsing & Emission

```gpl
import "std/xml"

# --- Parse ---
var doc := xml.parse("""
    <?xml version="1.0" encoding="UTF-8"?>
    <root>
        <item id="1">Hello</item>
        <item id="2">World</item>
    </root>
""")?

var root := doc.root()
root.name                               # "root"
root.children()                         # -> []xml.Node
root.find("item")                       # -> Option[xml.Node]
root.find_all("item")                   # -> []xml.Node
root.attr("id")                         # -> Option[str]
root.text()                             # -> str (inner text)

# Navigate
for item in root.find_all("item"):
    var id  := item.attr("id").unwrap_or("")
    var txt := item.text()
    io.println(id, txt)

# --- Build ---
var doc := xml.Document.new("1.0", "UTF-8")
var root := doc.element("root")
root.child("item")
    .attr("id", "1")
    .text("Hello")
root.child("item")
    .attr("id", "2")
    .text("World")

var output := doc.to_string()
doc.write_to(writer)?
```

### 71.2 `std/csv` — CSV Parsing & Emission

```gpl
import "std/csv"

# --- Parse ---
var reader := csv.Reader.new(text, csv.Options{
    delimiter:  ',',
    quote:      '"',
    has_header: true,
    trim:       true,
})

# read as records ([]str per row)
for row in reader.records():
    io.println(row[0], row[1])

# read as typed structs
struct Employee:
    @csv(0) name:   str
    @csv(1) age:    i32
    @csv(2) salary: f64

for emp in reader.deserialize[Employee]():
    io.println(emp.name, emp.salary)

# --- Write ---
var writer := csv.Writer.new(output_writer, csv.Options{delimiter: ','})
writer.write_header(["name", "age", "salary"])?
writer.write_record(["Alice", "30", "75000.00"])?
writer.write_struct(emp)?
writer.flush()?
```

### 71.3 `std/compress` — Compression

```gpl
import "std/compress"
import "std/compress/gzip"
import "std/compress/zstd"
import "std/compress/lz4"

# Compress bytes
var compressed := gzip.compress(data)?
var compressed := zstd.compress(data, level: 3)?
var compressed := lz4.compress(data)?

# Decompress bytes
var original := gzip.decompress(compressed)?
var original := zstd.decompress(compressed)?

# Streaming compress/decompress
var encoder := gzip.Encoder.new(writer, level: gzip.BEST_SPEED)
encoder.write(chunk1)?
encoder.write(chunk2)?
encoder.finish()?

var decoder := gzip.Decoder.new(reader)
var data    := decoder.read_all()?

# Zstd with dictionary
var dict       := zstd.Dictionary.train(samples)?
var compressed := zstd.compress_with_dict(data, dict, level: 9)?
var original   := zstd.decompress_with_dict(compressed, dict)?
```

### 71.4 `std/encoding` — Encoding Utilities

```gpl
import "std/encoding/base64"
import "std/encoding/hex"
import "std/encoding/utf8"
import "std/encoding/utf16"

# Base64
var encoded := base64.encode(data)          # -> str
var decoded := base64.decode(encoded)?      # -> []byte
var url_enc := base64.encode_url(data)      # URL-safe variant
var decoded := base64.decode_url(encoded)?

# Hex
var hex_str := hex.encode(data)             # -> str  "deadbeef"
var decoded := hex.decode("deadbeef")?      # -> []byte
hex.encode_upper(data)                      # "DEADBEEF"

# UTF-8 utilities
utf8.is_valid(bytes)                        # -> bool
utf8.decode(bytes)?                         # -> str
utf8.encode(s)                              # -> []byte (same as s.as_bytes())
utf8.rune_count(bytes)                      # -> usize
utf8.iter_runes(s)                          # -> Iter[rune]

# UTF-16 (Windows interop)
utf16.encode(s)                             # -> []u16
utf16.decode(words)?                        # -> str
utf16.encode_le(s)                          # little-endian
utf16.encode_be(s)                          # big-endian
```

### 71.5 `std/unicode` — Unicode Properties

```gpl
import "std/unicode"

# Character properties
unicode.is_alphabetic(r)            # -> bool
unicode.is_numeric(r)               # -> bool
unicode.is_alphanumeric(r)
unicode.is_whitespace(r)
unicode.is_uppercase(r)
unicode.is_lowercase(r)
unicode.is_control(r)
unicode.is_ascii(r)                 # r < 128
unicode.to_uppercase(r)             # -> rune
unicode.to_lowercase(r)             # -> rune
unicode.general_category(r)         # -> unicode.Category
unicode.script(r)                   # -> unicode.Script ("Latin", "Han", ...)
unicode.block(r)                    # -> unicode.Block ("Basic Latin", ...)

# String normalization
unicode.normalize_nfc(s)            # -> str  Canonical Decomposition + Composition
unicode.normalize_nfd(s)            # -> str  Canonical Decomposition
unicode.normalize_nfkc(s)           # -> str  Compatibility Decomposition + Composition
unicode.normalize_nfkd(s)           # -> str  Compatibility Decomposition

# Grapheme cluster iteration (visible characters, not code points)
for cluster in unicode.graphemes(s):
    io.println(cluster)             # each cluster is a str

# Case folding (for case-insensitive comparison)
unicode.case_fold(s)                # -> str
unicode.eq_fold(a, b)               # -> bool (case-insensitive equal)
```

### 71.6 `std/sql` — Database Interface

```gpl
import "std/sql"

# sql defines interfaces; actual drivers are separate packages
# Register a driver (e.g. sqlite, postgres, mysql)
import "drivers/sqlite3"    # registers "sqlite3" driver

# Open connection
var db := sql.open("sqlite3", "myapp.db")?
defer db.close()

# Query
var rows := db.query("SELECT id, name, age FROM users WHERE age > ?", 18)?
defer rows.close()

while rows.next():
    var id:   i64
    var name: str
    var age:  i32
    rows.scan(&id, &name, &age)?
    io.println(id, name, age)

# Query single row
var name: str
db.query_row("SELECT name FROM users WHERE id = ?", 1)
    .scan(&name)?

# Typed query using struct
struct User:
    @sql("id")   id:   i64
    @sql("name") name: str
    @sql("age")  age:  i32

var users := db.query_as[User]("SELECT id, name, age FROM users")?
for u in users:
    io.println(u.name)

# Execute (INSERT, UPDATE, DELETE)
var result := db.exec("INSERT INTO users(name, age) VALUES(?, ?)", "Alice", 30)?
var id     := result.last_insert_id()
var rows_n := result.rows_affected()

# Transactions
var tx := db.begin()?
defer tx.rollback()           # rollback if not committed

tx.exec("UPDATE accounts SET balance = balance - ? WHERE id = ?", 100, from_id)?
tx.exec("UPDATE accounts SET balance = balance + ? WHERE id = ?", 100, to_id)?
tx.commit()?

# Prepared statements
var stmt := db.prepare("SELECT * FROM users WHERE name = ?")?
defer stmt.close()
var rows := stmt.query("Alice")?

# Connection pool options
var db := sql.open_pool("postgres", dsn, sql.PoolOptions{
    max_open:     25,
    max_idle:     5,
    max_lifetime: 30 * time.Minute,
})?
```

### 71.7 `std/template` — Text & HTML Templating

```gpl
import "std/template"
import "std/template/html"   # HTML-safe (auto-escapes)

# Text templates
var tmpl := template.parse("""
Hello, {{.name}}!
You have {{.count}} messages.
{{if .count > 0}}
  Latest: {{.latest}}
{{end}}
""")?

var output := tmpl.render({
    "name":   "Alice",
    "count":  3,
    "latest": "Meeting at 3pm",
})?

# HTML templates (auto-escapes all values)
var tmpl := html.parse("""
<h1>Hello, {{.name}}!</h1>
<ul>
{{range .items}}
  <li>{{.}}</li>
{{end}}
</ul>
""")?

var result := tmpl.render({
    "name":  "<script>alert('xss')</script>",  # auto-escaped
    "items": ["Apple", "Banana", "Cherry"],
})?

# Template with helpers
var tmpl := html.parse("""<p>{{.date | format "Jan 02, 2006"}}</p>""")?
```

### 71.8 `std/testing/mock` — Mocking

```gpl
import "std/testing/mock"

# Define interface to mock
interface Database:
    fn find_user(id: i32) -> Option[User]
    fn save_user(u: User) -> Result[void]

# Generate mock (compile-time macro)
@mock
struct MockDatabase: Database

@test
fn test_handler_with_mock() -> void:
    var db := MockDatabase.new()

    # set up expectations
    db.expect("find_user")
        .with(42)
        .returns(Option.Some(User{id: 42, name: "Alice"}))
        .times(1)

    db.expect("save_user")
        .with(mock.any())
        .returns(Result.Ok(void))
        .times(1)

    # run the code under test
    var handler := UserHandler.new(&db as *Database)
    handler.update_name(42, "Bob")

    # verify all expectations were met
    db.verify()
```

---

## 72. `impl` Block Syntax (Grouping)

As an alternative to writing `fn Struct.method()` at the top level,  
methods can be grouped in an `impl` block for readability:

```gpl
struct Rectangle:
    x:      f64
    y:      f64
    width:  f64
    height: f64

# Grouped impl block — equivalent to individual fn Rectangle.xxx() declarations
impl Rectangle:
    fn new(x: f64, y: f64, w: f64, h: f64) -> Rectangle:
        return Rectangle{x: x, y: y, width: w, height: h}

    fn area(self) -> f64:
        return self.width * self.height

    fn perimeter(self) -> f64:
        return 2.0 * (self.width + self.height)

    fn contains(self, px: f64, py: f64) -> bool:
        return px >= self.x and px <= self.x + self.width
           and py >= self.y and py <= self.y + self.height

    fn scale(self: *Rectangle, factor: f64) -> void:
        self.width  *= factor
        self.height *= factor

# Interface impl block
impl Display for Rectangle:
    fn to_string(self) -> str:
        return f"Rect({self.x},{self.y} {self.width}×{self.height})"

impl Eq for Rectangle:
    fn eq(self, other: Rectangle) -> bool:
        return self.x == other.x and self.y == other.y
           and self.width == other.width and self.height == other.height
```

Both styles (`impl Block` and `fn Type.method()`) are valid and can be mixed in the same file.  
`impl Block` is preferred for types with many methods. Standalone `fn Type.method()` is preferred  
for methods defined in a different file from the type declaration.

---

## 73. Anonymous & Inline Structs

```gpl
# Inline struct type (anonymous)
var point: struct{ x: f64, y: f64 } = {x: 1.0, y: 2.0}

# As function parameter
fn print_size(size: struct{ width: i32, height: i32 }) -> void:
    io.println(size.width, "x", size.height)

print_size({width: 1920, height: 1080})

# As return type
fn get_bounds() -> struct{ x: f64, y: f64, w: f64, h: f64 }:
    return {x: 0.0, y: 0.0, w: 100.0, h: 50.0}

# Destructure
var {x, y} := get_bounds()         # ignores w and h

# Named destructure from struct
var p := Point{x: 3.0, y: 4.0}
var {x: px, y: py} := p            # px = 3.0, py = 4.0
```

---

## 74. Associated Types in Interfaces

```gpl
interface Container:
    type Item                   # associated type

    fn get(self, index: usize) -> Option[Self.Item]
    fn len(self) -> usize
    fn is_empty(self) -> bool: self.len() == 0   # default method body

interface Iter:
    type Item

    fn next(self: *Self) -> Option[Self.Item]

# Implementing with associated type
struct NumberList:
    data: []i32

impl Container for NumberList:
    type Item = i32

    fn get(self, index: usize) -> Option[i32]:
        if index >= self.data.len: return Option.None
        return Option.Some(self.data[index])

    fn len(self) -> usize:
        return self.data.len

# Using associated types in generics
fn print_all[C: Container](c: C) -> void
    where C.Item: Display:
    for i in 0..c.len():
        io.println(c.get(i).unwrap())

# Constraining the associated type
fn sum_container[C: Container<Item = i32>](c: C) -> i32:
    var total: i32 = 0
    for i in 0..c.len():
        total += c.get(i).unwrap_or(0)
    return total
```

---

## 75. `where` Clause on Structs

```gpl
# Struct with generic constraints
struct SortedVec[T] where T: Comparable + Copy:
    data: []T

fn SortedVec[T].insert(self: *SortedVec[T], val: T) -> void
    where T: Comparable + Copy:
    # binary search insert
    var pos := self.data
        .iter()
        .position(fn(x: T) -> bool: x >= val)
        .unwrap_or(self.data.len)
    self.data.insert(pos, val)

# Generic function with where clause (multi-line constraints)
fn merge_maps[K, V](
    a: map[K]V,
    b: map[K]V,
    resolve: fn(V, V) -> V,
) -> map[K]V
    where K: Eq + Hash + Copy,
          V: Copy:
    var result := a.clone()
    for k, v in b:
        if result.contains_key(k):
            result.insert(k, resolve(result.get(k).unwrap(), v))
        else:
            result.insert(k, v)
    return result
```

---

## 76. Recursive Types

Recursive types (a struct containing itself) must use indirection (pointer or slice)  
so the compiler can determine the type's size:

```gpl
# ERROR: infinite size — struct contains itself directly
struct Node:
    value: i32
    next:  Node    # ERROR: recursive type has infinite size

# OK: indirection via pointer (size is known: pointer-sized)
struct Node:
    value: i32
    next:  ?*Node  # nullable pointer to next node

# OK: indirection via heap allocation
struct Tree:
    value:    i32
    children: []Tree   # slice = pointer + length, known size

# OK: Box[T] (heap-allocated single value)
import "std/mem"
struct Tree:
    value: i32
    left:  ?mem.Box[Tree]
    right: ?mem.Box[Tree]
```

`mem.Box[T]` is a heap-allocated unique pointer with automatic deallocation (implements `Drop`):
```gpl
var node := mem.Box.new(Tree{value: 42, left: null, right: null})
var val  := node.value      # auto-deref
node.value = 100            # auto-deref
# freed automatically when node goes out of scope
```

---

---

## 77. Formal Memory Model

The G memory model defines how concurrent memory accesses relate to each other.
It is based on the C++20 memory model with modifications for G's ownership system.

### 77.1 Fundamental Definitions

**Memory location:** A scalar object (primitive type) or a field of a struct.
Two objects share a memory location if and only if they occupy the same bytes.

**Memory access:** Any read or write to a memory location.
- **Read:** load of a value
- **Write:** store of a value
- **Read-Modify-Write (RMW):** atomic fetch_add, compare_exchange, etc.

**Sequenced-before:** Within a single thread, operations are sequenced in program order.
`A` sequenced-before `B` means `A` completes before `B` begins, in the same thread.

**Synchronizes-with:** An atomic store with `Release` semantics *synchronizes-with*
an atomic load of the same location with `Acquire` semantics that reads that stored value.

**Happens-before:** The transitive closure of sequenced-before and synchronizes-with.
`A` happens-before `B` means the effect of `A` is visible to `B`.

**Data race:** Two memory accesses in different threads where:
1. At least one is a write, AND
2. Neither happens-before the other, AND
3. Neither access is atomic

**Data races are undefined behavior in G.**

---

### 77.2 Happens-Before Rules

```
Rule 1 (Sequencing):
  If A is sequenced-before B in the same thread, then A happens-before B.

Rule 2 (Synchronization):
  If a Release store A synchronizes-with an Acquire load B,
  then A happens-before B.
  Everything sequenced-before A also happens-before B and everything after B.

Rule 3 (Transitivity):
  If A happens-before B, and B happens-before C,
  then A happens-before C.

Rule 4 (SeqCst total order):
  All SeqCst operations form a single total modification order
  consistent with happens-before.

Rule 5 (Thread spawn):
  All operations in the spawning thread before thread.spawn()
  happen-before all operations in the spawned thread.

Rule 6 (Thread join):
  All operations in a thread happen-before thread.join() returns
  in the joining thread.

Rule 7 (Channel send/recv):
  A channel send happens-before the corresponding channel receive.

Rule 8 (Mutex):
  mutex.unlock() happens-before the next mutex.lock() on the same mutex.

Rule 9 (Defer):
  All statements before a defer call happen-before the deferred function runs.
  All deferred functions happen-before the function return is observable.
```

---

### 77.3 Atomic Ordering Semantics (Formal)

```
Relaxed:
  - Only guarantees atomicity of the single operation.
  - No ordering guarantee relative to other operations.
  - May be reordered freely by compiler and CPU.
  - Use: independent counters, statistics.

Acquire (loads only):
  - This load is a fence: no read or write in this thread
    that is sequenced after this load may be reordered before it.
  - Pairs with Release stores.
  - Use: acquiring a lock, reading a published pointer.

Release (stores only):
  - This store is a fence: no read or write in this thread
    that is sequenced before this store may be reordered after it.
  - Pairs with Acquire loads.
  - Use: releasing a lock, publishing a pointer.

AcqRel (RMW only):
  - Combines Acquire and Release semantics on one RMW operation.
  - Use: compare_exchange on a lock, fetch_add on a shared counter
    where ordering matters.

SeqCst:
  - All SeqCst operations appear in a single global total order
    consistent across all threads.
  - Strongest guarantee, highest cost.
  - Use: when you need a global ordering of events across multiple
    atomic variables simultaneously.
```

---

### 77.4 Data Race Examples

```gpl
# DATA RACE — undefined behavior
var x: i32 = 0
var t1 := thread.spawn(fn() -> void: x = 1)   # write
var t2 := thread.spawn(fn() -> void: x = 2)   # write — races with t1
t1.join()
t2.join()

# SAFE — atomic
var x := atomic.I32.new(0)
var t1 := thread.spawn(fn() -> void: x.store(1, atomic.Ordering.Relaxed))
var t2 := thread.spawn(fn() -> void: x.store(2, atomic.Ordering.Relaxed))
t1.join()
t2.join()
# x is either 1 or 2, no UB

# SAFE — mutex
var x: i32 = 0
var mu := sync.Mutex.new()
var t1 := thread.spawn(fn() -> void:
    mu.lock(); defer mu.unlock(); x = 1)
var t2 := thread.spawn(fn() -> void:
    mu.lock(); defer mu.unlock(); x = 2)
t1.join()
t2.join()
# x is either 1 or 2, no UB

# SAFE — channel ownership transfer
var ch := channel.make[Buffer](1)
var t1 := thread.spawn(fn() -> void:
    var buf := Buffer.new(64)
    buf.write("hello")
    ch.send(buf)           # ownership transferred, t1 no longer owns buf
)
var buf := ch.recv()       # main thread now owns buf
io.println(buf.as_str())
```

---

### 77.5 Compiler & Hardware Reordering

The G compiler and the underlying hardware may reorder memory operations,
subject to the happens-before guarantees above.

**Allowed reorderings (no atomic/sync):**
- Compiler may reorder non-atomic loads and stores within a thread.
- CPU may execute out of order (store buffers, load speculation).

**Forbidden reorderings:**
- Across `atomic.fence()` calls
- Across Acquire/Release atomic operations
- Across `mutex.lock()` / `mutex.unlock()`
- Across `channel.send()` / `channel.recv()`
- Across `thread.spawn()` / `thread.join()`

**`volatile` memory:** Volatile reads/writes (`core/volatile`) are not reordered
relative to each other, but are NOT atomic and do NOT synchronize threads.
Use `volatile` for MMIO; use atomics for inter-thread communication.

---

## 78. Error Code Catalogue

Every compiler diagnostic has a unique code in the format `E` (error) or `W` (warning)
followed by a 4-digit number. Codes are stable across compiler versions.

### 78.1 Syntax Errors (E0001–E0099)

| Code  | Name                        | Description                                              |
|-------|-----------------------------|----------------------------------------------------------|
| E0001 | `unexpected_token`          | Token not valid in this position                         |
| E0002 | `unexpected_eof`            | File ended unexpectedly                                  |
| E0003 | `invalid_indent`            | Indentation is not a multiple of 4 spaces                |
| E0004 | `mixed_indent`              | Tabs and spaces mixed in indentation                     |
| E0005 | `missing_colon`             | Expected `:` to open block                               |
| E0006 | `invalid_escape`            | Unknown escape sequence in string                        |
| E0007 | `unterminated_string`       | String literal not closed                                |
| E0008 | `invalid_unicode_escape`    | `\u{...}` value out of range or invalid hex              |
| E0009 | `invalid_number_literal`    | Malformed numeric literal                                |
| E0010 | `invalid_attribute`         | Attribute syntax error                                   |
| E0011 | `duplicate_module_decl`     | More than one `module` declaration in file               |
| E0012 | `missing_module_decl`       | File has no `module` declaration                         |
| E0013 | `invalid_raw_string`        | Raw string delimiter mismatch                            |
| E0014 | `trailing_comma_not_allowed`| Trailing comma not allowed here                          |

### 78.2 Type Errors (E0100–E0199)

| Code  | Name                        | Description                                              |
|-------|-----------------------------|----------------------------------------------------------|
| E0100 | `type_mismatch`             | Expression type does not match expected type             |
| E0101 | `cannot_infer_type`         | Type cannot be inferred; add explicit annotation         |
| E0102 | `undefined_type`            | Type name not found in scope                             |
| E0103 | `wrong_type_arg_count`      | Wrong number of generic type arguments                   |
| E0104 | `constraint_not_satisfied`  | Type does not implement required interface               |
| E0105 | `recursive_type_no_indirection` | Recursive type must use pointer or slice             |
| E0106 | `newtype_mismatch`          | Newtype used where underlying type expected (or vice versa) |
| E0107 | `invalid_cast`              | Cannot cast between these types                          |
| E0108 | `void_used_as_value`        | `void` expression used where value is required           |
| E0109 | `never_used_as_value`       | `never` expression used incorrectly                      |
| E0110 | `array_size_not_const`      | Array size must be a compile-time constant               |
| E0111 | `invalid_bit_field`         | Bit-field type only valid inside `@packed` struct        |
| E0112 | `tuple_index_out_of_range`  | Tuple index exceeds tuple length                         |
| E0113 | `ambiguous_method`          | Multiple methods match; use explicit type path           |
| E0114 | `missing_interface_method`  | `impl` block missing required method                     |
| E0115 | `wrong_return_type`         | Return type does not match declared type                 |
| E0116 | `missing_return`            | Not all code paths return a value                        |
| E0117 | `unreachable_code`          | Code after `never`-typed expression                      |
| E0118 | `invalid_operator`          | Operator not defined for these types                     |
| E0119 | `associated_type_missing`   | `impl` block must specify associated type                |

### 78.3 Ownership & Borrow Errors (E0200–E0299)

| Code  | Name                        | Description                                              |
|-------|-----------------------------|----------------------------------------------------------|
| E0200 | `use_after_move`            | Value used after being moved                             |
| E0201 | `move_out_of_borrow`        | Cannot move out of borrowed content                      |
| E0202 | `borrow_while_moved`        | Cannot borrow a moved value                              |
| E0203 | `mutable_borrow_conflict`   | Cannot borrow as mutable while other borrows active      |
| E0204 | `borrow_outlives_value`     | Borrow lifetime exceeds the borrowed value's lifetime    |
| E0205 | `return_local_borrow`       | Cannot return borrow of local variable                   |
| E0206 | `double_free`               | Value may be dropped more than once                      |
| E0207 | `move_non_copy_in_loop`     | Moving non-Copy value inside loop is likely a bug        |
| E0208 | `partial_move`              | Cannot use struct after partial field move               |
| E0209 | `borrow_of_moved_out_field` | Cannot borrow field; parent struct was partially moved   |
| E0210 | `mutable_ref_of_immutable`  | Cannot take mutable borrow of immutable binding          |
| E0211 | `lifetime_mismatch`         | Lifetime parameters are incompatible                     |
| E0212 | `lifetime_missing`          | Lifetime annotation required but missing                 |
| E0213 | `static_lifetime_required`  | This reference must have `'static` lifetime              |

### 78.4 Name & Scope Errors (E0300–E0399)

| Code  | Name                        | Description                                              |
|-------|-----------------------------|----------------------------------------------------------|
| E0300 | `undefined_variable`        | Variable not found in scope                              |
| E0301 | `undefined_function`        | Function not found in scope                              |
| E0302 | `undefined_module`          | Imported module not found                                |
| E0303 | `undefined_field`           | Field not found on this type                             |
| E0304 | `undefined_method`          | Method not found on this type                            |
| E0305 | `undefined_variant`         | Enum variant not found                                   |
| E0306 | `duplicate_binding`         | Variable declared more than once in same scope           |
| E0307 | `duplicate_field`           | Struct field declared more than once                     |
| E0308 | `duplicate_variant`         | Enum variant declared more than once                     |
| E0309 | `duplicate_method`          | Method defined more than once for this type              |
| E0310 | `private_access`            | Symbol is private to its module                          |
| E0311 | `module_not_pub`            | Accessing non-`pub` symbol from another module           |
| E0312 | `keyword_as_ident`          | Keyword cannot be used as identifier                     |
| E0313 | `shadow_warning_error`      | Variable shadows outer binding (when `-W error`)         |
| E0314 | `import_cycle`              | Circular import dependency detected                      |
| E0315 | `glob_import_conflict`      | `use X.*` causes name conflict                           |

### 78.5 Safety Errors (E0400–E0499)

| Code  | Name                        | Description                                              |
|-------|-----------------------------|----------------------------------------------------------|
| E0400 | `unsafe_without_block`      | Unsafe operation outside `unsafe` block                  |
| E0401 | `asm_without_unsafe`        | Inline assembly requires `unsafe` block                  |
| E0402 | `raw_ptr_deref_outside_unsafe` | Pointer dereference requires `unsafe` block           |
| E0403 | `pointer_arith_outside_unsafe` | Pointer arithmetic requires `unsafe` block            |
| E0404 | `union_access_outside_unsafe`  | Union field access requires `unsafe` block            |
| E0405 | `bit_cast_outside_unsafe`   | `@bit_cast` requires `unsafe` block                      |
| E0406 | `extern_call_without_unsafe` | Calling `@extern` function requires `unsafe` block      |
| E0407 | `mutable_static_outside_unsafe` | Mutable global variable access requires `unsafe`     |
| E0408 | `no_fp_violation`           | Float operation in `#![no_fp]` module                    |
| E0409 | `naked_fn_non_asm_body`     | `@naked` function body must consist only of `asm:` block |

### 78.6 Const & Compile-time Errors (E0500–E0599)

| Code  | Name                        | Description                                              |
|-------|-----------------------------|----------------------------------------------------------|
| E0500 | `const_fn_violation`        | Operation not allowed in `const fn`                      |
| E0501 | `const_eval_overflow`       | Compile-time evaluation overflowed                       |
| E0502 | `const_eval_div_zero`       | Compile-time division by zero                            |
| E0503 | `const_eval_infinite_loop`  | Compile-time evaluation did not terminate                |
| E0504 | `non_const_in_const`        | Non-const expression used where compile-time value needed|
| E0505 | `comptime_type_mismatch`    | `comptime` expression type mismatch                      |
| E0506 | `cfg_unknown_key`           | Unknown key in `@cfg(...)` attribute                     |
| E0507 | `regex_invalid`             | Invalid regex pattern at compile-time (`@regex`)         |

### 78.7 Runtime / Panic Codes (P0001–P0099)

These are not compiler errors but runtime panic identifiers, printed in panic messages.

| Code  | Name                        | Trigger                                                  |
|-------|-----------------------------|----------------------------------------------------------|
| P0001 | `index_out_of_bounds`       | `arr[i]` where `i >= arr.len`                            |
| P0002 | `slice_out_of_bounds`       | `arr[i:j]` where `i > j` or `j > arr.len`               |
| P0003 | `integer_overflow`          | Arithmetic overflow in debug mode                        |
| P0004 | `divide_by_zero`            | Integer division by zero                                 |
| P0005 | `unwrap_none`               | `.unwrap()` on `Option.None`                             |
| P0006 | `unwrap_err`                | `.unwrap()` on `Result.Err`                              |
| P0007 | `explicit_panic`            | `panic(msg)` called                                      |
| P0008 | `assert_failed`             | `assert(false, ...)` called                              |
| P0009 | `stack_overflow`            | Stack exhausted (best-effort detection)                  |
| P0010 | `shift_overflow`            | Shift amount >= bit width                                |
| P0011 | `invalid_utf8`              | Attempt to construct `str` from invalid UTF-8            |
| P0012 | `null_deref`                | Safe null dereference detected (if possible)             |
| P0013 | `cast_overflow`             | `as!` type assertion failed                              |
| P0014 | `slice_not_on_boundary`     | Slicing `str` not on UTF-8 character boundary            |

### 78.8 Warnings (W0001–W0099)

| Code  | Name                        | Description                                              |
|-------|-----------------------------|----------------------------------------------------------|
| W0001 | `unused_variable`           | Variable declared but never used                         |
| W0002 | `unused_import`             | Imported module never used                               |
| W0003 | `unused_function`           | Function never called (in final binary)                  |
| W0004 | `unused_result`             | Return value of `@must_use` function ignored             |
| W0005 | `dead_code`                 | Code after unconditional `return`/`break`/`continue`     |
| W0006 | `variable_shadows`          | Variable shadows binding in outer scope                  |
| W0007 | `deprecated`                | Using a `@deprecated` symbol                             |
| W0008 | `ambiguous_precedence`      | Expression precedence may be confusing; add parentheses  |
| W0009 | `integer_truncation`        | `as` cast may truncate value                             |
| W0010 | `float_comparison`          | Floating-point equality comparison (`==`) may be unreliable |
| W0011 | `missing_doc`               | Public symbol missing doc comment (`##`)                 |
| W0012 | `todo_in_release`           | `@todo(...)` present in release build                    |
| W0013 | `large_stack_allocation`    | Stack allocation > 1MB may cause overflow                |
| W0014 | `unreachable_pattern`       | Match arm can never be reached                           |
| W0015 | `non_exhaustive_match`      | Match may not cover all cases (when `_` is missing)      |

---

## 79. `std/sync` — Synchronization Primitives

```gpl
import "std/sync"

# --- Mutex ---
var mu := sync.Mutex.new()
mu.lock()
defer mu.unlock()

# try without blocking
if mu.try_lock():
    defer mu.unlock()
    critical_section()

# RAII guard
var guard := mu.acquire()    # -> sync.MutexGuard
# guard auto-unlocks when dropped

# --- RWMutex ---
var rw := sync.RWMutex.new()

# multiple concurrent readers
var rg := rw.read()          # -> sync.ReadGuard (auto-unlock on drop)
var val := shared_data

# exclusive writer
var wg := rw.write()         # -> sync.WriteGuard (auto-unlock on drop)
shared_data = new_value

# --- WaitGroup ---
var wg := sync.WaitGroup.new()

for i in 0..10:
    wg.add(1)
    thread.spawn(fn() -> void:
        defer wg.done()
        do_work(i)
    )

wg.wait()     # blocks until counter reaches 0

# --- Once ---
var once := sync.Once.new()
var instance: ?*Service = null

fn get_service() -> *Service:
    once.do(fn() -> void:
        instance = Service.new()
    )
    return instance!

# --- Semaphore ---
var sem := sync.Semaphore.new(max: 5)   # allow 5 concurrent

fn limited_operation() -> void:
    sem.acquire()
    defer sem.release()
    do_work()

# try without blocking
if sem.try_acquire():
    defer sem.release()
    do_work()

# --- CondVar (Condition Variable) ---
var mu   := sync.Mutex.new()
var cond := sync.CondVar.new()
var ready := false

# waiter thread
fn waiter() -> void:
    var guard := mu.acquire()
    while not ready:
        cond.wait(&guard)    # atomically unlocks mu and sleeps
    do_work_when_ready()
    # guard re-acquired before cond.wait returns

# notifier thread
fn notifier() -> void:
    var guard := mu.acquire()
    ready = true
    cond.notify_one()    # wake one waiter
    # or: cond.notify_all()

# wait with timeout
var guard := mu.acquire()
if not cond.wait_timeout(&guard, 5 * time.Second):
    io.println("timed out waiting")

# --- Barrier ---
var barrier := sync.Barrier.new(parties: 4)

fn worker(id: i32) -> void:
    phase_one(id)
    barrier.wait()         # all 4 threads must arrive before any continue
    phase_two(id)

# --- Lazy (thread-safe lazy initialization) ---
var config := sync.Lazy[Config].new(fn() -> Config:
    return Config.load_from_file("config.toml").unwrap()
)

fn get_config() -> *Config:
    return config.get()    # initializes on first call, safe across threads

# --- RwLock[T] (data + lock together) ---
var data := sync.RwLock[map[str]i32].new({"a": 1})

var read  := data.read()     # -> sync.RwReadGuard[map[str]i32]
io.println(read.get()["a"])

var write := data.write()    # -> sync.RwWriteGuard[map[str]i32]
write.get_mut().insert("b", 2)
```

---

## 80. `std/net/tls` — TLS / HTTPS

```gpl
import "std/net/tls"
import "std/net"

# --- TLS Client ---
var config := tls.ClientConfig{
    verify_certs:       true,
    root_certs:         tls.SystemRoots,     # use OS cert store
    min_version:        tls.Version.TLS12,
    max_version:        tls.Version.TLS13,
    alpn_protocols:     ["h2", "http/1.1"],
    server_name:        "example.com",       # for SNI; optional if in dial
    session_cache:      tls.SessionCache.new(capacity: 64),
}

# dial TLS directly
var conn := tls.connect("example.com:443", config)?
defer conn.close()
conn.write_str("GET / HTTP/1.0\r\n\r\n")?
var resp := conn.read_all()?

# wrap existing TCP connection
var tcp  := net.TcpStream.connect("example.com:443")?
var conn := tls.Client.new(tcp, config)?
conn.handshake()?

# peer certificate info
var cert := conn.peer_cert()?
io.println(cert.subject)
io.println(cert.issuer)
io.println(cert.not_after)
io.println(cert.fingerprint_sha256())

# --- TLS Server ---
var server_config := tls.ServerConfig{
    cert_file:      "server.crt",
    key_file:       "server.key",
    min_version:    tls.Version.TLS12,
    client_auth:    tls.ClientAuth.None,   # None | Request | Require
    alpn_protocols: ["http/1.1"],
}

var listener := net.TcpListener.bind("0.0.0.0:443")?
loop:
    var tcp  := listener.accept()?
    var conn := tls.Server.new(tcp, server_config)?
    conn.handshake()?
    thread.spawn(fn() -> void: handle_tls(conn))

# --- Mutual TLS (mTLS) ---
var mtls_config := tls.ClientConfig{
    verify_certs:  true,
    root_certs:    tls.CertFile("ca.crt"),
    client_cert:   tls.CertKeyPair("client.crt", "client.key"),
    min_version:   tls.Version.TLS13,
}

# --- TLS versions ---
tls.Version.TLS10    # deprecated, avoid
tls.Version.TLS11    # deprecated, avoid
tls.Version.TLS12
tls.Version.TLS13    # preferred

# --- Certificate management ---
var cert  := tls.Certificate.load("cert.crt", "key.pem")?
var chain := tls.CertChain.load("fullchain.pem")?
var pool  := tls.CertPool.new()
pool.add_from_file("ca.crt")?
pool.add_from_dir("/etc/ssl/certs")?

# Self-signed cert generation (for testing)
var cert := tls.generate_self_signed("localhost", valid_days: 365)?
cert.save("test.crt", "test.key")?
```

---

## 81. `std/net` — WebSocket

```gpl
import "std/net/ws"

# --- WebSocket Client ---
var conn := ws.connect("wss://echo.websocket.org")?
defer conn.close()

# send
conn.send_text("Hello, WebSocket!")?
conn.send_binary(data)?
conn.ping("ping")?

# receive
loop:
    var msg := conn.recv()?
    match msg.kind:
        ws.MessageKind.Text   => io.println("text:", msg.text())
        ws.MessageKind.Binary => process(msg.bytes())
        ws.MessageKind.Ping   => conn.pong(msg.bytes())?
        ws.MessageKind.Pong   => {}
        ws.MessageKind.Close  => break

# with options
var conn := ws.connect_opts("wss://api.example.com/ws", ws.Options{
    headers:         {"Authorization": "Bearer " + token},
    ping_interval:   30 * time.Second,
    read_timeout:    60 * time.Second,
    max_message_size: 1024 * 1024,    # 1MB
})?

# --- WebSocket Server ---
import "std/http"

var srv := http.Server.new(http.ServerOptions{addr: "0.0.0.0:8080"})

srv.route("GET", "/ws", fn(req: *http.Request, res: *http.Response) -> void:
    # upgrade to WebSocket
    var conn := ws.upgrade(req, res, ws.UpgradeOptions{
        protocols: ["chat", "v1"],
    })?
    defer conn.close()

    io.println("WebSocket connected:", req.remote_addr)
    loop:
        var msg := conn.recv() ?? break
        match msg.kind:
            ws.MessageKind.Text =>
                conn.send_text(f"Echo: {msg.text()}")?
            ws.MessageKind.Close => break
            _ => {}
)

srv.listen_and_serve()?
```

---

## 82. `std/os/linux` — Linux-specific

```gpl
import "std/os/linux"

# --- Syscalls (direct) ---
var pid := linux.syscall(linux.SYS_getpid) as i32
var fd  := linux.syscall(linux.SYS_open,
    path.as_ptr(), linux.O_RDONLY, 0) as i32

# --- epoll (I/O event notification) ---
var epfd := linux.epoll_create1(0)?
defer linux.close(epfd)

var event := linux.EpollEvent{
    events: linux.EPOLLIN | linux.EPOLLET,
    data:   linux.EpollData{fd: server_fd},
}
linux.epoll_ctl(epfd, linux.EPOLL_CTL_ADD, server_fd, &event)?

var events: [64]linux.EpollEvent
loop:
    var n := linux.epoll_wait(epfd, events[:], timeout_ms: -1)?
    for i in 0..n:
        handle_event(events[i])

# --- io_uring (high-performance async I/O) ---
var ring := linux.IoUring.new(queue_depth: 256)?
defer ring.close()

# submit read
var buf: [4096]byte
var sqe := ring.get_sqe()?
sqe.prep_read(fd, buf[:], offset: 0)
sqe.user_data = 1
ring.submit()?

# wait for completion
var cqe := ring.wait_cqe()?
if cqe.res < 0:
    io.println("read error:", cqe.res)
else:
    io.println("read", cqe.res, "bytes")
ring.cqe_seen(cqe)

# --- signalfd ---
var mask := linux.SigSet.new()
mask.add(linux.SIGINT)
mask.add(linux.SIGTERM)
linux.sigprocmask(linux.SIG_BLOCK, &mask)?

var sfd := linux.signalfd(-1, &mask, 0)?
defer linux.close(sfd)

var info: linux.SignalfdSiginfo
linux.read(sfd, &info as *mut u8, @size_of(linux.SignalfdSiginfo))?
io.println("got signal:", info.ssi_signo)

# --- memfd (anonymous memory-backed file) ---
var fd := linux.memfd_create("shared_buf", 0)?
linux.ftruncate(fd, 4096)?
var ptr := linux.mmap(null, 4096,
    linux.PROT_READ | linux.PROT_WRITE,
    linux.MAP_SHARED, fd, 0)?
defer linux.munmap(ptr, 4096)

# --- namespaces / cgroups (containers) ---
linux.unshare(linux.CLONE_NEWNET | linux.CLONE_NEWPID)?
linux.setns(fd, linux.CLONE_NEWNET)?

# --- seccomp (syscall filtering) ---
var filter := linux.SeccompFilter.new(linux.SCMP_ACT_KILL)
filter.allow(linux.SYS_read)
filter.allow(linux.SYS_write)
filter.allow(linux.SYS_exit)
filter.allow(linux.SYS_exit_group)
filter.load()?
```

---

## 83. `std/os/windows` — Windows-specific

```gpl
import "std/os/windows"

# --- Win32 API ---
var hwnd := windows.CreateWindowExW(
    dwExStyle:    0,
    lpClassName:  "MyWindow",
    lpWindowName: "Hello, G!",
    dwStyle:      windows.WS_OVERLAPPEDWINDOW,
    x: 100, y: 100, nWidth: 800, nHeight: 600,
    hWndParent: null, hMenu: null,
    hInstance: windows.GetModuleHandleW(null),
    lpParam: null,
)?

windows.ShowWindow(hwnd, windows.SW_SHOW)
windows.UpdateWindow(hwnd)

# message loop
var msg: windows.MSG
while windows.GetMessageW(&msg, null, 0, 0):
    windows.TranslateMessage(&msg)
    windows.DispatchMessageW(&msg)

# --- Registry ---
var key := windows.RegOpenKeyExW(
    windows.HKEY_LOCAL_MACHINE,
    "SOFTWARE\\MyApp",
    windows.KEY_READ,
)?
defer key.close()

var val := key.query_string("InstallPath")?
io.println("installed at:", val)

# --- Named pipes ---
var pipe := windows.CreateNamedPipeW(
    r"\\.\pipe\MyPipe",
    windows.PIPE_ACCESS_DUPLEX,
    windows.PIPE_TYPE_MESSAGE | windows.PIPE_READMODE_MESSAGE,
    instances: 1,
    out_buf:   4096,
    in_buf:    4096,
    timeout:   0,
    security:  null,
)?

windows.ConnectNamedPipe(pipe, null)?

# --- COM (Component Object Model) ---
windows.CoInitializeEx(null, windows.COINIT_MULTITHREADED)?
defer windows.CoUninitialize()

var hr := windows.CoCreateInstance(
    &CLSID_WbemLocator,
    null,
    windows.CLSCTX_INPROC_SERVER,
    &IID_IWbemLocator,
    &locator,
)?

# --- IOCP (I/O Completion Ports) ---
var iocp := windows.CreateIoCompletionPort(
    windows.INVALID_HANDLE_VALUE, null, 0,
    threads: os.num_cpus() as u32,
)?

var bytes: u32
var key:   usize
var overlapped: *windows.OVERLAPPED
windows.GetQueuedCompletionStatus(iocp, &bytes, &key, &overlapped, timeout: windows.INFINITE)?
```

---

## 84. Profiling & Sanitizers

### 84.1 Profiling

```toml
# gpl.toml
[profile.profiling]
optimization = "speed"
debug_info   = true
strip        = false
profile      = true      # enables profiling instrumentation
```

```bash
# Build with profiling
gpl build --profile profiling -o myapp

# Run and collect profile
gpl run --profile profiling -- args
# generates myapp.gpprof

# Analyze
gpl prof myapp.gpprof
gpl prof myapp.gpprof --flame    # generate flame graph (SVG)
gpl prof myapp.gpprof --top 20   # top 20 hottest functions
gpl prof myapp.gpprof --diff other.gpprof  # compare two profiles
```

Per-function annotation:

```gpl
@profile          fn hot_fn() -> void: ...    # always profile this function
@no_profile       fn skip_fn() -> void: ...   # exclude from profiling
```

### 84.2 AddressSanitizer (ASan)

Detects: heap-use-after-free, heap-buffer-overflow, stack-buffer-overflow,
use-after-return, use-after-scope, double-free.

```bash
gpl build --sanitize address -o myapp_asan
./myapp_asan

# Example output on error:
# ==12345==ERROR: AddressSanitizer: heap-buffer-overflow
# READ of size 4 at 0x602000000014 thread T0
#   #0 0x4012ab in main src/main.gpl:42
#   #1 0x7f... in __libc_start_main
# ...
```

### 84.3 ThreadSanitizer (TSan)

Detects: data races, lock-order violations, use of uninitialized mutexes.

```bash
gpl build --sanitize thread -o myapp_tsan
./myapp_tsan
```

### 84.4 UndefinedBehaviorSanitizer (UBSan)

Detects: integer overflow (when UB mode), null dereference, invalid cast,
shift overflow, invalid enum value, misaligned access.

```bash
gpl build --sanitize undefined -o myapp_ubsan
./myapp_ubsan
```

### 84.5 MemorySanitizer (MSan)

Detects: use of uninitialized memory (reads before writes).

```bash
gpl build --sanitize memory -o myapp_msan
./myapp_msan
```

### 84.6 Combined

```bash
gpl build --sanitize address,undefined -o myapp_san
```

---

## 85. WebAssembly Target

### 85.1 Building for Wasm

```toml
# gpl.toml
[target.wasm32]
features = ["-bulk-memory", "+simd128"]

[target.wasm32.wasi]
# WASI (WebAssembly System Interface) — access to OS-like APIs
wasi_version = "preview2"
```

```bash
# Build WASI module (command-line / server-side wasm)
gpl build --target wasm32-wasi -o myapp.wasm

# Build browser module (no WASI, manual JS imports)
gpl build --target wasm32-unknown-unknown -o myapp.wasm

# Run WASI module with wasmtime
wasmtime myapp.wasm

# Optimize wasm binary size
gpl build --target wasm32-wasi -Os -o myapp.wasm
wasm-opt -Oz myapp.wasm -o myapp.opt.wasm
```

### 85.2 Exporting to JavaScript

```gpl
module mylib

# Export functions callable from JS
@export
@extern("C")
fn add(a: i32, b: i32) -> i32:
    return a + b

@export
@extern("C")
fn alloc_buffer(size: usize) -> *mut u8:
    return mem.raw_alloc(size) as *mut u8

@export
@extern("C")
fn free_buffer(ptr: *mut u8, size: usize) -> void:
    mem.raw_free(ptr as *void)
```

```javascript
// JavaScript side
const wasm = await WebAssembly.instantiateStreaming(fetch('mylib.wasm'));
const { add, alloc_buffer, free_buffer, memory } = wasm.instance.exports;

console.log(add(2, 3));  // 5

// Pass string to G
const str = "Hello from JS";
const enc = new TextEncoder().encode(str);
const ptr = alloc_buffer(enc.length);
new Uint8Array(memory.buffer, ptr, enc.length).set(enc);
// ... call G function with ptr and enc.length
free_buffer(ptr, enc.length);
```

### 85.3 Importing from JavaScript

```gpl
module myapp

# Import JS functions into G
@extern("env")
fn js_log(ptr: *const u8, len: usize) -> void

@extern("env")
fn js_now() -> f64

@extern("env")
fn js_random() -> f64

fn log(msg: str) -> void:
    unsafe:
        js_log(msg.as_bytes().ptr, msg.len)

fn now() -> f64:
    unsafe: return js_now()
```

### 85.4 WASI Standard Library

With `--target wasm32-wasi`, the following `std` modules are available:
- `std/io` — stdin/stdout/stderr via WASI
- `std/fs` — file access via WASI preopens
- `std/os` — args, env, clock, random
- `std/net` — sockets (WASI preview2 only)

Not available in WASI:
- `std/thread` — single-threaded by default (SharedArrayBuffer threads: experimental)
- `std/os/linux`, `std/os/windows` — platform-specific

---

## 86. Debugger Integration

### 86.1 DWARF Debug Info

G emits DWARF 5 debug information when compiled with `-g` or `--debug-info`.

DWARF mappings:
- G source lines → `DW_TAG_subprogram`, `DW_AT_decl_line`
- Variables → `DW_TAG_variable` with `DW_AT_location`
- Types → `DW_TAG_structure_type`, `DW_TAG_enumeration_type`
- Generics → monomorphized instances, each with their own DWARF entry
- Closures → `DW_TAG_subprogram` with `__closure_env` variable

### 86.2 GDB/LLDB Usage

```bash
# Compile with debug info
gpl build -g -o myapp

# GDB
gdb myapp
(gdb) break main
(gdb) run
(gdb) next
(gdb) print x
(gdb) info locals
(gdb) backtrace

# LLDB
lldb myapp
(lldb) breakpoint set --name main
(lldb) run
(lldb) frame variable
(lldb) thread backtrace
```

### 86.3 Pretty Printers

G provides GDB and LLDB pretty printers for standard types.
Install via `gpl toolchain install pretty-printers`.

```
# GDB output with pretty printers:
(gdb) print my_list
$1 = List[i32] { len: 3, data: [1, 2, 3] }

(gdb) print my_option
$2 = Option::Some(42)

(gdb) print my_result
$3 = Result::Err(ParseError { message: "invalid digit", line: 5, col: 3 })

(gdb) print my_map
$4 = Map[str, i32] { "alpha": 1, "beta": 2 }
```

### 86.4 VS Code / Editor Integration

Install the `gpl-lsp` language server:

```bash
gpl toolchain install lsp
```

`gpl-lsp` provides:
- Syntax highlighting
- Error diagnostics (inline, on-type)
- Auto-complete (types, methods, fields, imports)
- Go-to-definition, find all references
- Hover documentation (from `##` doc comments)
- Rename symbol (cross-file)
- Code actions (auto-import, add missing impl methods)
- Inlay hints (inferred types, parameter names)
- Format on save (via `gpl fmt`)
- Semantic tokens

---

## 87. Package Registry

### 87.1 Registry Spec

The official G package registry is at `https://registry.gpl-lang.org`.

**Package index format** (JSON, served over HTTPS):

```json
{
  "name": "raylib",
  "versions": ["5.0.0", "4.5.2", "4.0.0"],
  "latest": "5.0.0",
  "description": "A simple and easy-to-use library to enjoy games programming",
  "license": "zlib",
  "repository": "https://github.com/user/raylib-g",
  "keywords": ["graphics", "gamedev", "opengl"]
}
```

**Package manifest** (inside `.tar.zst` archive):

```toml
[package]
name        = "raylib"
version     = "5.0.0"
authors     = ["user <user@example.com>"]
license     = "zlib"
description = "..."
repository  = "..."
checksum    = "sha256:abc123..."   # SHA-256 of the archive
```

### 87.2 Versioning

G packages follow **Semantic Versioning 2.0.0** (semver.org):
- `MAJOR.MINOR.PATCH`
- Breaking API change → bump MAJOR
- New backwards-compatible feature → bump MINOR
- Bug fix → bump PATCH

**Version requirements in `gpl.toml`:**

```toml
[dependencies]
raylib   = "5.0"           # shorthand: >=5.0.0, <6.0.0
raylib   = "5.0.2"         # exact minor: >=5.0.2, <5.1.0
raylib   = "=5.0.0"        # exact version
raylib   = ">=4.0, <6.0"   # explicit range
raylib   = "*"             # any version (not recommended)
```

### 87.3 Checksum & Security

Every package download is verified against a SHA-256 checksum stored in `gpl.lock`:

```toml
# gpl.lock (auto-generated, commit to VCS)
[[package]]
name     = "raylib"
version  = "5.0.0"
source   = "registry+https://registry.gpl-lang.org"
checksum = "sha256:3a4b5c6d7e8f..."

[[package]]
name     = "openssl"
version  = "3.2.1"
source   = "registry+https://registry.gpl-lang.org"
checksum = "sha256:1a2b3c4d5e6f..."
```

### 87.4 Publishing

```bash
# Login to registry
gpl registry login

# Verify package before publishing
gpl publish --dry-run

# Publish
gpl publish

# Yank a version (marks as broken, existing users warned)
gpl yank raylib@5.0.0

# Un-yank
gpl yank --undo raylib@5.0.0

# Owner management
gpl owner add raylib alice@example.com
gpl owner remove raylib bob@example.com
gpl owner list raylib
```

---

## 88. Incremental Compilation

### 88.1 Dependency Graph

The G compiler tracks a dependency graph of all compilation units.
A compilation unit is one `.gpl` file (after import resolution).

**Invalidation rules:**
- If file `A` is modified → recompile `A` and all files that (directly or transitively) import `A`.
- If a `.hdr` file changes → recompile all files that import that header.
- If a `comptime` constant changes → recompile all files that use that constant.
- If `gpl.toml` changes → full rebuild.
- If a dependency package version changes → full rebuild of that package and dependents.

### 88.2 Build Cache

The build cache lives at `~/.gpl/cache/` (or `$GPL_CACHE`).

Cache key per compilation unit:
```
SHA256(
  file_content +
  compiler_version +
  target_triple +
  profile_flags +
  feature_flags +
  dependency_checksums
)
```

```bash
# Show cache stats
gpl cache stats

# Clear cache
gpl cache clear
gpl cache clear --package raylib    # clear specific package

# Disable cache for one build
gpl build --no-cache
```

---

## 89. Compiler Plugin API

### 89.1 Plugin Entry Point

Compiler plugins are G libraries loaded at compile time. They can:
- Add custom attributes
- Generate code (proc-macro style)
- Add custom lints
- Transform the AST

```gpl
module my_plugin

import "std/compiler_plugin" as plugin

@plugin_entry
fn init(ctx: *plugin.Context) -> void:
    ctx.register_attribute("my_attr", handle_my_attr)
    ctx.register_derive("MyTrait", derive_my_trait)
    ctx.register_lint("my_lint", lint_my_lint)
```

### 89.2 Attribute Plugin

```gpl
fn handle_my_attr(
    attr: plugin.Attribute,
    item: plugin.Item,
    ctx:  *plugin.Context,
) -> plugin.Result[plugin.Item]:
    # inspect the item
    match item:
        plugin.Item.Fn(f) =>
            # add a wrapper function
            var wrapper := ctx.parse_item(f"""
                fn {f.name}_logged() -> {f.return_type}:
                    io.println("calling {f.name}")
                    return {f.name}()
            """)?
            return plugin.Result.Ok(plugin.Item.Many([item, wrapper]))
        _ =>
            return plugin.Result.Err("@my_attr only applies to functions")
```

### 89.3 Derive Plugin

```gpl
fn derive_my_trait(
    item: plugin.StructOrEnum,
    ctx:  *plugin.Context,
) -> plugin.Result[plugin.Item]:
    match item:
        plugin.StructOrEnum.Struct(s) =>
            var impl_code := ctx.parse_item(f"""
                impl MyTrait for {s.name}:
                    fn my_method(self) -> str:
                        return "{s.name}"
            """)?
            return plugin.Result.Ok(impl_code)
        _ =>
            return plugin.Result.Err("MyTrait can only be derived for structs")
```

### 89.4 Using a Plugin

```toml
# gpl.toml
[build]
plugins = ["my_plugin"]

[dev-dependencies]
my_plugin = { path = "../my_plugin" }
```

```gpl
import "my_plugin"

@my_attr
fn greet(name: str) -> str:
    return "Hello, " + name

@derive(MyTrait)
struct User:
    name: str
    age:  i32
```

---

*G Language Specification v1.0.0-draft*  
*All syntax subject to change before stable release.*  
*Feedback and contributions welcome on GitHub.*

---

## Revision History

| Version | Date       | Summary                                                     |
|---------|------------|-------------------------------------------------------------|
| 0.1.0   | —          | Initial draft: core syntax, types, functions, modules       |
| 0.2.0   | —          | Added: operator overloading, RAII, generics variance, iterators |
| 0.3.0   | —          | Added: numeric literals, operator precedence, lifetimes, FFI, async, CLI, SIMD |
| 0.4.0   | —          | Added: full stdlib (io/fs/net/json/log/regex/collections), kernel primitives |
| 0.5.0   | —          | Added: grammar EBNF, behavior spec, ABI, SMP, xml/csv/sql/compress/encoding |
| 1.0.0   | —          | Added: memory model, error catalogue, sync/tls/ws, platform libs, wasm, debugger, registry |
