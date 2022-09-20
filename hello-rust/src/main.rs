use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let mess = String::from("First Rust App!");
    let width = mess.chars().count();
    
    let mut writer = BufWriter::new(stdout.lock());
    say(mess.as_bytes(), width, &mut writer).unwrap();
}
