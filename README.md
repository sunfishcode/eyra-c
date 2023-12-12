# eyra-c

Support for compiling C programs with Eyra.

Specifically, this repo compiles [Eyra] into a [staticlib] to make a libc.a:

```sh
$ cargo build
   Compiling eyra-c v0.0.0 (/home/eyra/ecosystem/eyra-c)
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
$ ls target/debug/libc.a 
target/debug/libc.a
$
```

This libc.a can be used by C compilers, with -nostdlib to disable the
system libraries:

```sh
$ cat c-tests/src/hello.c 
#include <stdio.h>

int main(void) {
    printf("Hello, world!\n");
    return 0;
}
$ cc c-tests/src/hello.c -nostdlib target/debug/libc.a 
$ ./a.out 
Hello, world!
$
```

💃

Amusingly, even though this libc.a is built entirely from Rust, it cannot be
used by Rust programs, because a staticlib library is meant to be linked into
a C program, so it includes the Rust standard library. If linked into a Rust
program, it would conflict with the Rust standard library.

To use Eyra in a Rust program, [use Eyra as a dependency].

[Eyra]: https://github.com/sunfishcode/eyra
[staticlib]: https://doc.rust-lang.org/reference/linkage.html#linkage
[use Eyra as a dependency]: https://github.com/sunfishcode/eyra#quick-start
