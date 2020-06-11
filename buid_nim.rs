use std::io::{self, Write};
use std::process::Command;

fn main() {
    let j = std::thread::spawn(move || {
        let output_ = Command::new("nim")
        .arg("c")
        .arg("--noMain")
        .arg("--noLinking")
        .arg("--nimcache:nimcache")
        .arg("src/nim/main.nim")
        .output();
        match output_{
            None => {},
            Some(output) => {
            if output.status.success(){
                cc::Build::new()
                    .include("/usr/lib/nim")
                    .warnings(false)
                    .file("src/nim/nimcache/fib.nim.c")
                    .file("src/nim/nimcache/stdlib_system.nim.c")
                    .compile("base_nim");
            }
            }
        }
    }).join();
    match j{
        Some(_) => {},
        None => {},
    }
}