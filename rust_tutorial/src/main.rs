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
    let one_message = Message::Move { x: 3, y: 4 };
    // `one_message`が`Message::Move`の場合に、`x`と`y`に値が束縛されます。
    // ここで`x`と`y`を使用して処理を実行できます。
    if let Message::Move { x, y } = one_message  {
        println!("Move {}:{} ", x, y)
    } else {
        // `one_message`が`Message::Move`でない場合の処理
        println!("Not Move")
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
