use std::vec;

mod struct_and_enum;
use struct_and_enum::{Dimensions, Color, ShippingBox};
mod random;

fn main() {

    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::Write(String::from("writing a code...")),
        Message::ChangeColor(255, 255, 0)
    ];

    for msg in msgs {
        show_message(msg);
    }

    /////////////////////////////////////////////////
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9'];
    for ab in alphabets {
        // matches!マクロと組み合わせて、Arrayの要素をassertできる。
        // 'A'..='Z'の書き方に注目
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }
    println!("Successs");

    ////////////////////////////////////////////////
    // enum で遊びたい -> enumのvector,
    let v = vec![ Message::Quit, Message::Write(String::from("jump at a show")), Message::ChangeColor(2, 3, 4) ]; // VectorにEnum仕込じゃうのね...
    for e in v {
        if matches!(e, Message::Quit) { // matcheds!マクロはまじで便利
            println!("Gotta quit!!");
        }
    }

    ////////////////////////////////////////////////
    // match はてーへんなので
    let o = Some(7); // Some() と ジェネリックは後ほど
    // // match でやるパターン　let 変数名 = match {} を使えば, パターンマッチの結果を束縛できる
    let o = match o {
        Some(a) => a,
        _ => 0
        // None => panic!("値が存在しない")
    };
    print!("中身取り出す {}\n", o);

    /////////////////////////////////////////////////
    // 他にも Enum のマッチパターン
    let one_message = Message::Move { x: 3, y: 10 };
    // `one_message`が`Message::Move`の場合に、`x`と`y`に値が束縛されます。
    // ここで`x`と`y`を使用して処理を実行できます。
    if let Message::Move { x, y } = one_message  {
        println!("Move {}:{} ", x, y)
    } else {
        // `one_message`が`Message::Move`でない場合の処理
        println!("Not Move")
    }

    /////////////////////////////////////////////////
    match one_message {
        Message::Move { x: 0..=5, y: y @ (10 | 20 | 30)} => println!("On the y axis at {}", y),
        _ => println!("Not Message::Move")
    }
    ////////////////////////////////////////////////
    // match guard: アームにマッチしたときだけ、評価される条件式
    let num_op = Some(4);
    let split = 5;
    match num_op {
        Some(x) if x < split => assert!(x < split, "x < splitじゃないってまじ?"),
        Some(x) => assert!(x >= split),
        None => (),
    }
    println!("match guardの確認");

    /////////////////////////////////////////////////
    // ignoring remaining parts of the value
    let numbers_tuple = (String::from("Talk"), 2 , 3, 4, 14, 2134, 22);
    match numbers_tuple {
        // これで最初と最期の値のみ
        (first, .. , last) => {
            assert_eq!(first, "Talk");
            assert_eq!(last, 22);
        }
    }
    /////////////////////////////////////////////////
    // &mut V
    let mut v = String::from("hello, ");
    let r = &mut v;
    // r をmatchしたいときはどうするのか?
    match r {
        value => {
            value.push_str("world!");
            println!("{}", value)
        },
        _ => ()
    }
    /////////////////////////////////////////////////
    // メソッドを触ってみる
    let hot = Temperature {degree_f: 99.9};
    hot.show_temp();

    let cold = Temperature::freezing(); // メソッドを使って新しい構造体のインスタンスを作成
    cold.show_temp();

    ////////////////////////////////////////////////
    // map() や filter() は便利
    let numbers_for_map = vec![1, 2, 3, 4, 5];

    // iter()を使うことで、loopを回せる
    let nums_iter = numbers_for_map.iter(); // 所有権は移動しない
    for num in nums_iter {
        println!("into_iter() {}", num)
    }
    // filter() を使って、偶数の要素を出力
    let filtered_iter = numbers_for_map
        .iter()
        .filter(|&num| num % 2 == 0); // ベクタや配列の要素は所有権を持っており、イテレータが要素に直接アクセスすると所有権が移動してしまうため 参照を使う
    for num in filtered_iter {
        println!("filtered iter {}", num);
    }

    // vectorの合計値を計算する
    let sum_nums: i32 = numbers_for_map
        .iter()
        .sum();
    println!("vectorの合計値 {}", sum_nums);

    // filter() と map()一緒に
    let filtered_squared_numbers: Vec<i32> = numbers_for_map
        .iter()
        .filter(|&num| num % 2 == 0)
        .map(|&num| num * num)
        .collect();
    println!("filtered squared numbers {:?}", filtered_squared_numbers);

    // クロージャーは、引数を使わない
    // ランダムな数字のVectorになるだけ
    let squared_numbers :Vec<i32> = numbers_for_map
        .iter()
        .map(|_| {
            let random_numbers = rand::random::<i32>(); // ランダムな値を生成
            random_numbers
        })
        .collect();
    println!("クロージャー |_|として引数を受け取らない場合:{:?}", squared_numbers );

    // into_iter()は、所有権も移動
    let doubled_numbers : Vec<i32>= numbers_for_map
        .into_iter() // 所有権を持つコレクション（ベクタやハッシュマップなど）をイテレータに変換 所有権を移動
        .map(|n| n * 2)
        .collect();
    println!("map()  doubled numbers: {:?}", doubled_numbers);

    /////////////////////////////////////////////////
    // 別ファイルから ファイル名:random.rs randでランダムな数字を生成してみる
    println!("別ファイルから");
    random::random_numbers();
    random::random_numbers_within_range();
    random::roll_dice();
    random::generate_password();
    random::generate_password_from_user_defined_chars();

    /////////////////////////////////////////////////
    // enum や structのおもろい世界
    // struct_and_enum.rs を確認
    let small_dimensions = struct_and_enum::Dimensions{
        width: 1.0,
        height: 2.0,
        depth: 3.0
    };
    // Dimensionのインスタンスを使って、 ShippingBoxのインスタンスを作成
    let small_box = ShippingBox::new(5.0, Color::Black, small_dimensions);
    small_box.print();

    /////////////////////////////////////////////////
    // String &str
    // String: 可変の所有権を持つ文字列型です。ヒープ上にメモリを確保し、可変長の文字列を格納
    // &str: 不変の参照を持つ文字列型で、文字列スライス メモリ上のテキストデータへのポインタと長さの情報を持つ
    // 基本的に関数には &str　を渡す
    print_it("a string slice");
    let owned_string = "owned_string".to_owned(); //  &str から String への変換
    let another_way_to_owned = String::from("another");
    print_it(&owned_string);
    print_it(&another_way_to_owned);

    #[derive(Debug)]
    struct Employee {
        name: String, // &str の場合は、所有権の破棄ができないのでコンパイルエラー
    }
    let emp_name = "Tay Keith".to_owned();
    let emp = Employee { name: emp_name};
    println!("struct では &strは使えない{:?}", emp);

    ///////////////////////////////////////////////////
    // 一瞬触れたい struct match
    struct Ticket {
        event: String,
        price: i32
    }
    let musical_concert = Ticket{ // インスタンス化
        event: "Tay Keith".to_owned(), // ちゃんと所有権もたせてて偉い
        price:20
    };
    match musical_concert {
        // structのmatchのやり方
        Ticket {price:50, ..} => println!("price is 50!!!"),
        Ticket {price, ..} => {
            if price > 100 {
                println!("too expensive");
            } else {
                println!("price is not over 100: {}", price)
            }
        }
    }
}

enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

fn show_message(msg: Message) {
    // 情報があっているか assertマクロを使って確認!!
    match msg {
        Message::Move { x:a, y:b } => {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(r,g ,b ) => {
            assert_eq!(r, 255);
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        },
        _ => println!("No data!")
    }
}
///////////////////////////////////////
// impl
// さらに、struct_and_enum.rsをみるとおもろいよ
struct Temperature {
    degree_f: f64,
}

impl Temperature {
    // Selfは、Temperatureの構造体そのものを指している -> 新しい構造体のインスタンスを作成できる
    // Selfのように大文字であることに注意
    fn freezing() -> Self {
        Self {
            degree_f: 32.0
        }
    }

    fn show_temp(&self) {
        println!("{} degrees F", self.degree_f)
    }
}

/////////////////////////////////////////
// String と &strについて
fn print_it(data: &str) {
    println!("{:?}", data);
}


