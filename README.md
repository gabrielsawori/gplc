<div align="center">

```
   ____   _
  / ___| | |     __ _  _ __    __ _  _   _   __ _   __ _   ___
 | |  _  | |    / _` || '_ \  / _` || | | | / _` | / _` | / _ \
 | |_| | | |___| (_| || | | || (_| || |_| || (_| || (_| ||  __/
  \____| |_____|\__,_||_| |_| \__, | \__,_| \__,_| \__, | \___|
                               |___/                 |___/
```

**A systems programming language with C-level control and Python-level readability.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/spec-v1.0.0--draft-orange.svg)](docs/spec/G_Language_Spec_v1.0.0.md)
[![Status](https://img.shields.io/badge/status-pre--alpha-red.svg)]()
[![Spec Pages](https://img.shields.io/badge/spec-89%20sections%20%7C%20240KB-green.svg)](docs/spec/G_Language_Spec_v1.0.0.md)

[Overview](#overview) · [Features](#features) · [Syntax](#syntax-at-a-glance) · [Examples](#examples) · [Roadmap](#roadmap) · [Spec](#language-specification) · [Contributing](#contributing)

</div>

---

## Overview

**G** (`.gpl`) is a compiled, statically typed systems programming language designed to fill the gap between low-level languages like C and readable languages like Python.

G gives you:
- **Zero-cost abstractions** — generics, iterators, closures with no runtime overhead
- **Manual memory control** — allocators, RAII, move semantics, no GC
- **Kernel-ready** — `no_std`, volatile MMIO, interrupt handlers, inline assembly
- **Readable syntax** — indentation-based, no semicolons, no `{}` noise
- **Safety by default** — borrow checker, null safety, overflow detection in debug

G is designed to be used for:
- Operating systems and kernels
- Embedded and bare-metal systems
- High-performance applications
- Game engines and graphics
- CLI tools and system utilities
- WebAssembly modules

---

## Features

### Language
- ✅ Statically typed with full type inference
- ✅ Move semantics + RAII (no garbage collector)
- ✅ Borrow checker with lifetime annotations
- ✅ Generics with interface constraints
- ✅ Pattern matching (`match`) with exhaustiveness check
- ✅ Sum types (tagged unions via `enum`)
- ✅ Operator overloading via interfaces
- ✅ Closures and first-class functions
- ✅ Async / await with cooperative scheduler
- ✅ Inline assembly with structured syntax
- ✅ Compile-time evaluation (`const fn`, `comptime`)
- ✅ Metaprogramming via compiler plugins
- ✅ Iterator protocol with lazy functional methods

### Memory
- ✅ Stack allocation by default
- ✅ Injectable allocators (arena, pool, slab, stack)
- ✅ `mem.Box[T]` for heap-owned values
- ✅ Weak pointers for cycle prevention
- ✅ `unsafe` block for raw operations
- ✅ Volatile memory access for MMIO

### Systems / Kernel
- ✅ `#![no_std]` freestanding mode
- ✅ `#![no_runtime]` bare metal mode
- ✅ `#![no_fp]` disable floating point
- ✅ Interrupt handlers (`@interrupt`)
- ✅ Calling conventions (`@callconv`, `@naked`)
- ✅ Linker section control (`@section`)
- ✅ Per-CPU variables (`@per_cpu`)
- ✅ SMP / AP trampoline support
- ✅ SIMD intrinsics (SSE, AVX, AVX-512, NEON)

### Toolchain
- ✅ `gpl run` — compile and run immediately
- ✅ `gpl build` with GCC-style flags (`-O2`, `-lm`, `-o`)
- ✅ `gpl test` — built-in test runner
- ✅ `gpl bench` — benchmarking
- ✅ `gpl fmt` — opinionated formatter
- ✅ `gpl doc` — documentation generator
- ✅ `gpl-lsp` — language server (LSP)
- ✅ Cross-compilation built-in
- ✅ WebAssembly target (WASI + browser)

### Standard Library
- ✅ `std/io`, `std/fs`, `std/path`
- ✅ `std/net` (TCP, UDP, HTTP, WebSocket, TLS)
- ✅ `std/os` (args, env, process, signals)
- ✅ `std/time`, `std/rand`, `std/fmt`
- ✅ `std/sync` (Mutex, RWMutex, WaitGroup, CondVar, Semaphore)
- ✅ `std/collections` (List, Map, Set, BTreeMap, Heap)
- ✅ `std/json`, `std/xml`, `std/csv`
- ✅ `std/compress` (gzip, zstd, lz4)
- ✅ `std/crypto` (AES-GCM, SHA-256, BLAKE3, TLS)
- ✅ `std/regex`, `std/unicode`
- ✅ `std/sql` (database interface)
- ✅ `std/os/linux` (epoll, io_uring, seccomp)
- ✅ `std/os/windows` (Win32, IOCP, COM)

---

## Syntax at a Glance

```gpl
module main

import "std/io"
import "std/collections"

# Enums with data (tagged unions)
enum Shape:
    Circle(radius: f64)
    Rectangle(width: f64, height: f64)
    Triangle(base: f64, height: f64)

# Generic function with constraint
fn area[T: Display](shape: Shape) -> f64:
    return match shape:
        Shape.Circle(r)       => 3.14159 * r * r
        Shape.Rectangle(w, h) => w * h
        Shape.Triangle(b, h)  => 0.5 * b * h

# Struct with methods
struct Vec2:
    x: f64
    y: f64

impl Vec2:
    fn new(x: f64, y: f64) -> Vec2:
        return Vec2{x: x, y: y}

    fn length(self) -> f64:
        return (self.x * self.x + self.y * self.y) ** 0.5

    fn normalize(self: *Vec2) -> void:
        var len := self.length()
        self.x /= len
        self.y /= len

impl Display for Vec2:
    fn to_string(self) -> str:
        return f"({self.x:.2f}, {self.y:.2f})"

fn main() -> i32:
    # Type inference, move semantics
    var shapes := [
        Shape.Circle(radius: 5.0),
        Shape.Rectangle(width: 3.0, height: 4.0),
        Shape.Triangle(base: 6.0, height: 8.0),
    ]

    # Functional iterator pipeline (lazy, zero allocation)
    var total := shapes
        .iter()
        .map(fn(s: Shape) -> f64: area(s))
        .sum()

    io.println(f"Total area: {total:.2f}")

    # Pattern matching with guard
    for shape in shapes:
        match shape:
            Shape.Circle(r) if r > 3.0 =>
                io.println(f"Large circle, r={r}")
            Shape.Circle(r) =>
                io.println(f"Small circle, r={r}")
            _ =>
                io.println("other shape")

    return 0
```

---

## Examples

### Hello World
```gpl
module main
import "std/io"

fn main() -> i32:
    io.println("Hello, World!")
    return 0
```

```bash
gpl run hello.gpl
# Hello, World!
```

---

### HTTP Server
```gpl
module main

import "std/http"
import "std/io"

fn main() -> i32:
    var srv := http.Server.new(http.ServerOptions{addr: "0.0.0.0:8080"})

    srv.route("GET", "/", fn(req: *http.Request, res: *http.Response) -> void:
        res.status(200).body("Hello from G!")
    )

    srv.route("GET", "/users/{id}", fn(req: *http.Request, res: *http.Response) -> void:
        var id := req.param("id")
        res.status(200).json({"id": id, "name": "Alice"})
    )

    io.println("Listening on :8080")
    srv.listen_and_serve()!
    return 0
```

```bash
gpl run server.gpl
# Listening on :8080
```

---

### Kernel "Hello World" (x86_64 bare metal)
```gpl
#![no_std]
#![no_runtime]
module kernel

import "core/volatile"

const VGA_BUFFER: usize = 0xB8000
const WHITE_ON_BLACK: u8 = 0x0F

fn vga_print(msg: str) -> void:
    unsafe:
        var buf := VGA_BUFFER as *mut u8
        for i, ch in msg:
            volatile.write[u8](buf + i * 2,     ch as u8)
            volatile.write[u8](buf + i * 2 + 1, WHITE_ON_BLACK)

@panic_handler
fn panic(info: *core.PanicInfo) -> never:
    vga_print("KERNEL PANIC")
    loop: {}

@section(".text.boot")
@naked
pub fn _start() -> never:
    asm:
        "mov rsp, 0x7C00"
        "call kmain"
        "hlt"
        : : :

pub fn kmain() -> void:
    vga_print("Hello from G kernel!")
    loop: {}
```

---

### Concurrent Worker Pool
```gpl
module main

import "std/io"
import "std/thread"
import "std/channel"
import "std/sync"

fn main() -> i32:
    var jobs    := channel.make[i32](100)
    var results := channel.make[i32](100)
    var wg      := sync.WaitGroup.new()

    # spawn 4 workers
    for id in 0..4:
        wg.add(1)
        thread.spawn(fn() -> void:
            defer wg.done()
            for job in jobs:
                results.send(job * job)    # square the number
        )

    # send 20 jobs
    for i in 0..20:
        jobs.send(i)
    jobs.close()

    # collect results in background
    thread.spawn(fn() -> void:
        wg.wait()
        results.close()
    )

    var total: i32 = 0
    for r in results:
        total += r

    io.println(f"Sum of squares 0..19 = {total}")
    return 0
```

---

### Zero-copy Parser (systems-level)
```gpl
module main

import "std/io"
import "std/fs"

struct HttpRequest:
    method:  str    # borrows from input buffer
    path:    str    # borrows from input buffer
    version: str    # borrows from input buffer

fn parse_request<'a>(buf: &'a str) -> Result[HttpRequest]:
    var lines := buf.split("\r\n")
    if lines.len == 0:
        return Result.Err(Error.new("empty request"))

    var parts := lines[0].split(" ")
    if parts.len != 3:
        return Result.Err(Error.new("invalid request line"))

    return Result.Ok(HttpRequest{
        method:  parts[0],    # zero-copy: points into buf
        path:    parts[1],
        version: parts[2],
    })

fn main() -> i32:
    var raw := "GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\n"
    var req := parse_request(raw)!

    io.println(f"Method:  {req.method}")
    io.println(f"Path:    {req.path}")
    io.println(f"Version: {req.version}")
    return 0
```

---

## Project Structure

```
my-g-project/
├── gpl.toml          # project manifest
├── gpl.lock          # dependency lockfile (commit this)
├── README.md
├── src/
│   ├── main.gpl      # entry point
│   ├── lib.gpl       # library root (for lib projects)
│   └── lib.hdr       # public API header
├── tests/
│   └── integration.gpl
├── benches/
│   └── perf.gpl
└── docs/
    └── guide.md
```

`gpl.toml`:
```toml
[package]
name    = "my-project"
version = "0.1.0"
authors = ["You <you@example.com>"]

[build]
entry = "src/main.gpl"

[dependencies]
raylib = { version = "5.0" }

[dev-dependencies]
faker  = { version = "1.0" }
```

---

## Quick Start

> ⚠️ **The G compiler is not yet implemented.** This repository contains the language specification only. See [Contributing](#contributing) if you want to help build it.

Once the compiler is available:

```bash
# Install
curl -sSf https://gpl-lang.org/install.sh | sh

# Create new project
gpl new hello-world
cd hello-world

# Run
gpl run

# Build release binary
gpl build --release -o hello

# Run tests
gpl test

# Format code
gpl fmt
```

---

## Roadmap

### Phase 1 — Specification (current)
- [x] Core language spec (types, control flow, functions)
- [x] Memory model (ownership, borrows, RAII)
- [x] Formal grammar (EBNF)
- [x] ABI specification
- [x] Standard library API spec
- [x] Kernel / bare-metal spec
- [x] Build system & CLI spec
- [x] Error code catalogue

### Phase 2 — Bootstrap Compiler (next)
- [ ] Lexer & parser (produces AST)
- [ ] Type checker & borrow checker
- [ ] IR (Intermediate Representation)
- [ ] LLVM backend (initial target: x86_64 Linux)
- [ ] Basic standard library (`std/io`, `std/mem`, `std/fs`)
- [ ] `gpl build` and `gpl run`

### Phase 3 — Self-hosting
- [ ] Rewrite the compiler in G itself
- [ ] Full standard library implementation
- [ ] `gpl test`, `gpl fmt`, `gpl doc`
- [ ] Package registry

### Phase 4 — Ecosystem
- [ ] `gpl-lsp` language server
- [ ] Additional targets (ARM64, RISC-V, WASM)
- [ ] Debugger integration (GDB/LLDB)
- [ ] VS Code extension
- [ ] Kernel template project

---

## Language Specification

The full language specification is in [`docs/spec/G_Language_Spec_v1.0.0.md`](docs/spec/G_Language_Spec_v1.0.0.md).

**89 sections covering:**

| Category              | Sections                                              |
|-----------------------|-------------------------------------------------------|
| Core language         | Types, variables, operators, control flow, functions  |
| Type system           | Generics, traits, newtypes, associated types          |
| Memory                | Ownership, borrows, lifetimes, RAII, allocators       |
| Concurrency           | Threads, channels, async/await, atomics, memory model |
| Systems / kernel      | no_std, MMIO, interrupts, SMP, paging, context switch |
| Standard library      | 30+ modules from io/fs/net to sql/crypto/compress     |
| Toolchain             | CLI, build system, package registry, debugger         |
| Formal spec           | EBNF grammar, behavior spec, ABI, error catalogue     |

---

## Design Philosophy

**1. Explicit over implicit.**
Memory allocation, type conversions, and unsafe operations are always visible in the source.

**2. One way to do it.**
Unlike languages with multiple equally-valid idioms, G has a canonical style enforced by `gpl fmt`.

**3. Errors are values.**
G uses `Result[T]` and `Option[T]` everywhere. Exceptions exist only for unrecoverable panics.

**4. The compiler is your ally.**
Error messages include the error code, the problematic line, a hint, and a link to the spec.

**5. Spec first.**
The language spec is written before the compiler. This ensures the language is designed, not evolved by accident.

---

## Comparison

| Feature               | G       | C       | C++     | Rust    | Zig     | Go      |
|-----------------------|---------|---------|---------|---------|---------|---------|
| Memory safety         | ✅ opt   | ❌       | ❌       | ✅       | ⚠️ manual| ✅ GC    |
| No GC                 | ✅       | ✅       | ✅       | ✅       | ✅       | ❌       |
| `no_std` / kernel     | ✅       | ✅       | ⚠️       | ✅       | ✅       | ❌       |
| Readable syntax       | ✅       | ⚠️       | ❌       | ⚠️       | ⚠️       | ✅       |
| Pattern matching      | ✅       | ❌       | ⚠️       | ✅       | ⚠️       | ⚠️       |
| Generics              | ✅       | ❌       | ✅       | ✅       | ✅       | ✅ (1.18)|
| Async / await         | ✅       | ❌       | ⚠️       | ✅       | ⚠️       | ✅ (goroutine)|
| Compile speed         | 🎯 fast  | ✅       | ❌       | ❌       | ✅       | ✅       |
| Error as values       | ✅       | ❌       | ❌       | ✅       | ✅       | ✅       |
| Inline assembly       | ✅       | ✅       | ✅       | ✅       | ✅       | ❌       |
| Built-in formatter    | ✅       | ❌       | ❌       | ✅       | ✅       | ✅       |
| Built-in test runner  | ✅       | ❌       | ❌       | ✅       | ✅       | ✅       |

---

## Contributing

G is in the **specification phase**. The most impactful contribution right now is:

1. **Review the spec** — read [`G_Language_Spec_v1.0.0.md`](docs/spec/G_Language_Spec_v1.0.0.md) and open issues for anything unclear, ambiguous, or missing.
2. **Write example programs** — add `.gpl` files to `examples/` showing real-world usage.
3. **Start the compiler** — see [`COMPILER.md`](docs/COMPILER.md) for the implementation guide.
4. **Improve error messages** — suggest better wording for the error catalogue in §78.

### Development Setup

```bash
git clone https://github.com/your-org/g-lang
cd g-lang
```

Repository structure:
```
g-lang/
├── docs/
│   └── spec/
│       └── G_Language_Spec_v1.0.0.md   # the spec
├── examples/                            # example .gpl programs
│   ├── hello.gpl
│   ├── http_server.gpl
│   └── kernel_hello/
├── compiler/                            # compiler implementation (future)
├── std/                                 # standard library (future)
└── README.md
```

### Opening Issues

- **Spec bug:** something is ambiguous, contradictory, or wrong in the spec
- **Feature request:** something useful that's not in the spec
- **Example:** you want an example program added
- **Compiler:** implementation discussion

---

## License

G language specification and tooling is licensed under the **MIT License**.
See [LICENSE](LICENSE) for details.

---

<div align="center">

*Designed with ❤️ for systems programmers who are tired of choosing between control and readability.*

**[Read the Full Spec →](docs/spec/G_Language_Spec_v1.0.0.md)**

</div>
