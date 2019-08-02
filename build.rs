extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/wirehair/wirehair.cpp")
        .file("src/wirehair/gf256.cpp")
        .file("src/wirehair/WirehairCodec.cpp")
        .file("src/wirehair/WirehairTools.cpp")
        .include("src/wirehair")
        .flag("-msse4.1")
        .out_dir("wirehair")
        .shared_flag(true)
        .compile("wirehair");
}
