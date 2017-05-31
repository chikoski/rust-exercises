/*
ゴール
1. コードをコンパイルして動作させてください
2. [1]のコメントを外すと、コンパイルエラーが起きます。原因を特定して、修正してください
3. 2を修正せずに、[2]のコメントを外すと、コンパイルエラーが起きます。修正してください
4. どの商品を何個買ったかをまとめたHashMapを作成するメソッドhistogramを実装してください

ヒント：ユーザ定義型にHash, PartialEq, Eqのtraitを実装すると、その値をHashMapのキーに使えます
ヒント：deriveを使うことで、traitの標準実装をユーザ定義型に追加できます
*/

#![allow(dead_code)]
use std::collections::HashMap;

fn main() {
    let mut catalog = HashMap::new();
    catalog.insert("apple", Item::new("りんご", 480));
    catalog.insert("orange", Item::new("オレンジ", 130));
    catalog.insert("strawberry", Item::new("いちご", 320));
    catalog.insert("pineapple", Item::new("パイナップル", 270));
    catalog.insert("banana", Item::new("バナナ", 220));

    let mut cart = Cart::new();
    cart.take("banana", &catalog);
    cart.take("banana", &catalog);
    cart.take("banana", &catalog);
    cart.take("orange", &catalog);
    cart.take("apple", &catalog);

    for item in cart.list {
        println!("{} : {}円", item.name, item.price);
    }

    // [1]
    /*
    for item in cart.list {
        println!("{}", item);
    }
    */

    // [2]
    /*
    println!("total amount = {}", cart.total());
    */

    // [3]
    /*
    for (item, amount) in cart.histogram() {
        println!("{} : {}", item.name, amount);
    }
    */
}

struct Cart {
    list: Vec<Item>,
}

impl Cart {
    fn new() -> Cart {
        Cart { list: vec![] }
    }
    fn take(&mut self, key: &str, catalog: &HashMap<&str, Item>) {
        match catalog.get(key) {
            Some(item) => self.list.push(item.clone()),
            None => (),
        }
    }
    fn total(&self) -> u32 {
        self.list
            .iter()
            .map(|item| item.price)
            .fold(0, |accm, item| accm + item)
    }
}

#[derive(Clone)]
struct Item {
    name: String,
    price: u32,
}

impl Item {
    fn new(name: &str, price: u32) -> Item {
        Item {
            name: name.to_string(),
            price: price,
        }
    }
}