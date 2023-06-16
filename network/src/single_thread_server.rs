// 標準ライブラリのみ
// あくまでお勉強用
// single thread

use std::{net::{TcpListener, TcpStream}, error::Error, io::{Read, Write}, fs::File, path::PathBuf};

pub fn check_connection(addr: String) -> Result<TcpListener, Box<dyn Error>> {
    // 引数で受けとったアドレスをもとに
    // TCPストリームを受信するためアドレスをリッスンする
    // ブラウザに127.0.0.1:8080 のように引数で受け取ったアドレスを入力して、
    // 出力を確認
    let listner = TcpListener::bind(addr)?;
    println!("Listner: {:?}", listner);
    for stream in listner.incoming() {
        let stream = stream?;
        handle_connection(stream);
    }
    Ok(listner)
}

pub fn handle_connection(mut stream: TcpStream) {
    // TcpStreamインスタンスが内部で返すデータを追いかけるので、可変で引数を用意
    // ブラウザからのリクエストを受け取って、
    // 実際にストリームから読み取る手順は、2つ
    let mut buffer = [0; 1024]; // バッファーサイズ 1024
    stream.read(&mut buffer).unwrap(); // TcpStreamからバイトを読み取ってバッファーに置く

    // 無効なUTF-8シーケンスを目の当たりにした際のこの関数の振る舞いを示唆している。
    // 無効なシーケンス? U+FFFD REPLACEMENT CHARACTERで置き換える
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // &[u8]を受け取り、Stringを生成

    let get = b"GET / HTTP/1.1\r\n"; //リクエストに対応するデータをget変数へハードコード b""バイト文字列

    if buffer.starts_with(get) {
        // HTMLを返す
        let contents = match open_html(PathBuf::from("./hello.html")) {
            Ok(contents) => contents,
            Err(err) => panic!("HTMLを開けませんでした...: {}", err)
        };

        let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents);
        stream.write(response.as_bytes()).unwrap(); // &[u8]をとって、接続に直接そのバイトを送信
        stream.flush().unwrap(); // 待機->バイトがすべて接続に書き込まれるまでプログラム維持
    } else {
        println!("Not Get");
    }
}

pub fn open_html(path: PathBuf) -> Result<String, Box<dyn Error>> {
    // htmlファイルを読み込んで、中身を返す,なければエラー
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

