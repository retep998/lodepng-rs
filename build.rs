extern crate gcc;

fn main() {
    use std::default::Default;
    gcc::compile_library("liblodepng.a", &Default::default(), &["src/lodepng.c"]);
}