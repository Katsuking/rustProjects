// main.rs

use dotenv::dotenv;

mod module; use std::{vec, collections::HashMap, env};

// モジュールをインポート
use module::greet; // モジュールから関数をインポート

// ###########################################################
// struct と method
// ###########################################################

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// ###########################################################
// 軽くHeapとStackについて
// ###########################################################

// 構造体
struct MyStruct {
    name: String,
    age: i32,
}

    // #####################################################
    //  enum CやC++の列挙型よりも強力で柔軟性
    // #####################################################
    enum MyEnum {
        Variant1, // 引数を持たないenumの一つのバリアント
        Variant2(i32), // 整数値を受け取るバリアントであり、その整数値はバリアントのインスタンスに関連する値
        Variant3{name: String, age:u8}, // 名前と年齢の2つのフィールドを持つ構造体を受け取るバリアント
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

    // #####################################################
    //  環境変数を読み込むパターン
    // #####################################################

    // 環境変数の読み込み .envファイルはプロジェクトのルートディレクトリ
    dotenv().ok();

    // 環境変数から値の取得
    let dlurl = env::var("DOWNLOAD_PATH").expect("DownloadのPathの設定がされていない。");
    println!("DownloadのPath:{}", dlurl);
    
    // #####################################################
    //  改めて、所有権を変更せずに、一時的に値を借りて、他の場所で使用することができるようにする概念 borrow
    // 複数のスレッドや関数間での値の共有が難しいっていう問題への対処
    // #####################################################

    // fn change_vector, print_vector
    let mut v = vec![1,2,3,4,5];

    change_vector(&mut v);

    print_vector(&v);

    let rect1 = Rectangle {
        width: 30,
        height: 25,
    };

    println!("{}", rect1.area());


    // #####################################################
    //  スライス
    // #####################################################

    let mut a
     = [1, 2, 3, 4, 5];
    let b = &a[1..3];
    let c = &mut a[3..5];

    for i in &a {
        println!("{}", i);
    }

    // #####################################################
    //  enum
    // #####################################################
    
    let my_enum = MyEnum::Variant2(32); // 補完機能ででてくれんの熱すぎ

    let result = handle_my_enum(my_enum);
    match result {
        Ok(()) => println!("There is no error"),
        Err(error) => println!("Error Occured! {}", error),
    }

    // ####################################################
    // Option<T>
    // ####################################################
    let answer1 = divide(10, 5);
    match answer1 {
        Some(y) => println!("これは割れるぞ: {}だな...", y),
        None => println!("Cannot divide by 0")
    }

    let result2 = divide(10, 0);
    match result2 {
        Some(x) => println!("これは割れるぞ: {}", x),
        None => println!("これはさすがにNone"),
    }

    
}

fn change_vector(v: &mut Vec<i32>) {
    v.push(6);
}

fn print_vector(v: &Vec<i32>) {
    for item in v {
        // println!("{}", item);
    }
}

// #####################################################
//  enum 定義したmy_enumに対しての関数を用意　エラーハンドリングもしっかり
// #####################################################
// 成功した場合は値がなく(())、エラーが発生した場合はエラーの詳細を示す文字列(String)が返される

fn handle_my_enum(my_enum: MyEnum) -> Result<(), String> {
    match my_enum {
        MyEnum::Variant1 => {
            println!("Variant1");
            Ok(())
        },
        MyEnum::Variant2(value) => {
            if value > 100 {
                Err("Value is too large".to_string())         
            } else {
                println!("value is {}", value);
                Ok(())
            }
        },
        MyEnum::Variant3 { name, age } => {
            if name.is_empty() {
                Err("name is empty".to_string())
            } else if age < 18 {
                Err("too young!".to_string())
            } else {
                println!("name:{}, age:{}", name, age);
                Ok(())
            }
        } 
    }
}

// Option<T>型を使用することで、T型の値が存在しない場合にクラッシュを回避する
fn divide(a: u32, b:u32) -> Option<u32> {
    if b == 0 {
        None
    } else {
        Some(a/b)
    }
}