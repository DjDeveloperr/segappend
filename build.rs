fn main() {
    cc::Build::new()
        .cpp(true)
        .include("include")
        .file("src/segappend.cc")
        .flag("-std=c++17")
        .compile("segappend");
}
