// main.rs 

mod module; // モジュールをインポート
use module::greet; // モジュールから関数をインポート


fn main() {
    greet("world!");
}