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

In summary, urgh.

