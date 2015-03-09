### What is this?
"Sample" Rust code for interposition, in the spirit of [15-440 Spring 2015 Project 1](http://www.cs.cmu.edu/~15-440/PROJECTS/15440-p1-handout.pdf).
("Sample" because ok, quite seriously, I wouldn't recommend anyone consult this. See Caveats below.)

### Why???
> Not because it is good, but because we can.

### Caveats
* Let's be serious, atm practically every single line is wrapped in `unsafe` anyway, so why would you do this?
* I'm not entirely sure where to put the `unsafe` actually. See comments in `src/lib.rs`!
* `write()` can be interposed on, but you can't print using the Rust stdlib from within it. Awkward.
* `struct stat` remains to be done.
* This uses a Makefile instead of Cargo for reasons.
  * It doesn't seem that Rust has support for defining variadic functions, even if they're clearly marked `unsafe` and `extern "C"` and all that stuff. So the `open()` interposition is done by using a C shim that then calls out to 2 different Rust functions depending on how many arguments it got.
  * As a result, the build also needs to build a C object. This would seem manageable using Cargo. Except...
  * I couldn't figure out how to link the C and Rust stuff together into a single `.so` without building static libraries from each of them individually first. This would still seem manageable using Cargo. Except...
  * You need to build them into _position-independent_ static libraries! For this, you can use `rustc -C --relocation-model=pic`. But Cargo doesn't seem to support this...

In summary, urgh.

