#![allow(unused_mut)]
/*
ゴール：
- コンパイルエラーを修正してください
- 1でコメントアウトされている部分を戻すと、何が起こるか確認してください
- コンパイルエラーが発生した場合は、それを修正してください
*/
fn main() {
    let source = "hello, world".to_string();
    {
        let text = &source;
        p(text, "text")
    }
    p(text, text);

    let mut text;
    {
        /* [1]
        let source = "new source".to_string();
        */
        p(&source, "&source");
        text = &source;
        p(text, "text");
    }
    p(text, "text");
}

fn p(text: &str, source: &str) {
    println!("{:?} from {}", text, source);
}