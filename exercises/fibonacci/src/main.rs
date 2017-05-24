/*
ゴール：std::vec::Vecオブジェクトであるmemoを使って、次のプログラムを高速化してください
*/

fn main() {
    let n = 40;
    let memo = vec![-1; n + 1];

    for i in 0..n + 1 {
        let result = fib(i, &memo);
        println!("fib({}) = {}", i, result);
    }
}

fn fib(n: usize, memo: &Vec<i32>) -> i32 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fib(n - 1, memo) + fib(n - 2, memo),
    }
}

