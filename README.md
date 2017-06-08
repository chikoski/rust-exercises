# Rust exercises

Rustの基本的な文法と、考え方を学ぶためのエクササイズ集です。

|番号|内容|フォルダ名|
|--|---|--------|
|1|[Hello world](exercises/hello_world/)|hello_world|
|2|[所有権に関するもの](exercises/ownership/)|ownership|
|3|[参照に関するもの](exercises/reference/)|reference|
|4|[変更可能な参照に関するもの](exercises/mutable_reference/)|mutable_reference|
|5|[nullを返す可能性のある関数](exercises/option/)|option|
|6|[エラーを起こす可能性のある関数](/exercises/result/)|result|
|7|[ライフタイムとスコープについて](/exercises/scope-and-lifetime)|scope-and-lifetime|
|8|[動的計画法に関するもの](exercises/fibonacci/)|fibonacci|
|9|[ユーザ定義型に関するもの](exercises/cart/)|cart|

## 楽しみ方

1. exercisesフォルダを開く
2. エクササイズを選ぶ
3. そのエクササイズのフォルダをターミナルで開く
4. src/main.rsをエディタ/IDEで開く
5. Enjoy!

### Cargoを使ったコンパイル

コンパイルはcargoコマンドを使って行います。
使い方は次をご覧ください。

~~~sh
$ cargo build # デバッグビルドの作成
$ cargo build --release # リリースビルドの作成
$ cargo run # ビルド後、コマンドライン引数なしで起動
$ cargo run -- -c -o foo.txt bar.txt 
# -c -o foo.txt bar.txt を引数に指定して、ビルドしたプログラムを起動
~~~

## エクササイズの準備

1. rustupを使ってRustの環境を整えてください。[詳細はこちら](http://qiita.com/chikoski/items/b6461367e8c3875bb235)をご覧ください
2. 手に馴染んだエディタ / IDEを用意してください
   - 私は[Visual Studio Code](https://code.visualstudio.com/)に[Rust for Visual Studio Code](https://github.com/editor-rs/vscode-rust)を入れて使っています
   - 詳しい設定方法は[こちらの記事](http://qiita.com/skitoy4321/items/0bf6826f948720bed821)をご覧ください
3. このレポジトリを手元に持って来てください

## Acknowledgement 

エクササイズ1, 2, 3, 4は[intorust](https://intorust.com/)にあるものを参考にさせていただきました。

## LISENCE

[MIT](LICENSE)