/*
Goal:
- コンパイルして動作を確認してください
- [1]をコメントアウトから戻し、コンパイルしてください
- この途中でエラーが出た場合は、それを修正してください
- [2]をコメントアウトから戻し、各要素を2乗した値を出力するようにプログラムを変更してください
- [3]をコメントアウトから戻し、奇数だけを出力するようにプログラムを変更してください
- [4]をコメントアウトから戻し、偶数を2乗した値のみを出力するようにプログラムを修正してください
*/

fn main() {
    let range = 1..20;

    for value in range {
        println!("{}{} number is {}", value, ordinal(value), value);
    }

/* [1]
    hr();

    for (index, value) in range.enumerate() {
        println!("{}{} number is {}", index, ordinal(index), value);
    }
*/
/* [2]
    hr();

    for value in range {
        println!("{}{} number is {}", value, ordinal(value), value);
    }
*/
/* [3]
    hr();

    for value in range {
        println!("{}{} number is {}", value, ordinal(value), value);
    }
*/
/* [4]
    hr();

    for value in range {
        println!("{}{} number is {}", value, ordinal(value), value);
    }
*/
}

fn ordinal<'a>(number: usize) -> &'a str {
    if number == 11 || number == 12 {
        "th"
    } else {
        match number % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    }
}

fn hr(){
    println!("------------------------------------");
}