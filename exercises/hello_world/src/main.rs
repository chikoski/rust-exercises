/*
ゴール：
- コンパイルし、実行してください
- worldを自分の名前に置き換えてみてください
- 変数 name を導入してみましょう
- println!("{}", name)を試しください
- println!("{}", name)とprintln!("{:?}", name)の違いを確認してください
- greetingsを呼ぶように変更してください
*/
#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    //greetings(name);
}

fn greetings(name: String) {
    println!("Hello, {}!", name);
}

