#![allow(unused_variables, dead_code)]

/* 
ゴール：
1. cargo run -- Cargo.toml と実行して、このプログラムの動作を確認してください
2. 存在しないファイル名を指定して実行すると、panicがおきます。修正してください
3. コンビネータを使って、read_file2を実装してください
*/

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut args = env::args();
    if let Some(file) = args.nth(1) {
        println!("{}", read_file(&file).unwrap());        
        /*
        println!("{}", read_file2(&file));
        */
    }
}

fn read_file(filename: &String) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file2(filename: &String) -> String {
    String::new()
}

