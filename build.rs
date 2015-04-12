extern crate gcc;

use std::default::Default;

fn main() {
    gcc::compile_library("libsdk.a",
                         &["sdk.cpp"]);
}
