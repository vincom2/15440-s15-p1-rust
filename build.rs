extern crate gcc;

fn main() {
    gcc::compile_library("libopen.a", &["src/open_varargs.c"]);
}

