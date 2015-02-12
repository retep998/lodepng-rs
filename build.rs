extern crate gcc;

fn main() {
    gcc::compile_library("liblodepng.a", &["src/lodepng.c"]);
}
