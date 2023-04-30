// module.rs

// moduleなので、pubキーワードをつけます。
// 他のモジュールからアクセス可能にする
pub fn greet(name: &str) {
    println!("Hello {}!", name);
}