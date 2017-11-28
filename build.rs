extern crate cc;

fn main() {
    cc::Build::new().file("src/lodepng.c").compile("lodepng");
}
