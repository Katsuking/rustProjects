// main.rs

mod module; use std::{vec, collections::HashMap};

// モジュールをインポート
use module::greet; // モジュールから関数をインポート


// ###########################################################
// 軽くHeapとStackについて
// ###########################################################

// 構造体
struct MyStruct {
    name: String,
    age: i32,
}

fn main() {
    // ###########################################################
    // 軽くHeapとStackについて
    // ###########################################################

    // module.rs import
    greet("world!");

    // スタックにMyStructインスタンスを作成
    let my_struct = MyStruct {
        name: String::from("Tay Keith"),
        age: 27,
    };

    // ヒープにベクターを用意
    // VectorはかならずHeap使うからね
    let mut my_vec = Vec::new();

    // ベクターにMyStructのインスタンスを追加
    my_vec.push(MyStruct{
        name: String::from("biggie"),
        age: 30,
    });

    // スタックにもう一つのMyStructのインスタンス作成
    let another_struct = MyStruct {
        name: String::from("tupuc"),
        age: 33,
    };

    // ベクターにさらにanother_structを追加
    my_vec.push(another_struct);

    // ベクターの各要素をループして、名前を出力
    for st in my_vec {
        println!("{}", st.name);
    }

    // #######################################################
    // キーと値のペアを格納することができるデータ構造の一種
    // #######################################################

    // 新しいHashmapを作成
    let mut map = HashMap::new();

    // HashMapにベクター要素を追加
    map.insert(String::from("Apple"), 3);
    map.insert(String::from("Banana"), 15);

    // キーを使用して、値を取得
    let qty_of_apple = map.get("Apple");

    // Option<&V>を返すため、出力する際に{:?}フォーマット指定子を使用しています。
    println!("The quantity of apples is {:?}", qty_of_apple );

    // Optionは、値が存在する場合と存在しない場合の両方を表現できる列挙型
    match qty_of_apple {
        Some(qty) => println!("The quantity of apples is {}", qty),
        None => println!("Apple not found"),
    }

    // 一つのパターンしかチェックできないけど、if let でもいける。
    if let Some(qty) = qty_of_apple {
        println!("Apple: {}", qty);
    }

}