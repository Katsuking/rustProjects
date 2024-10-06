
### Stringについて理解を深める

```
2  |     let server = Server::new("127.0.0.1:4005");
   |                  ----------- ^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                  |           |
   |                  |           expected `String`, found `&str`
   |                  arguments to this function are incorrect
```
このようなエラーにはよく遭遇する。
`&str`は string sliceと呼ばれている。

`String`:
  1. mutable
  2. ownershipを持つ
  3. ヒープに持つ

`&str`:
  1. immutable
  2. 参照なので、所有権は持たない
  3. スタックに保存されるが、実際にはデータの所有者(e.g. String)がヒープにデータ保持


### rust のmethodで使うselfの話

```rust
fn main() {
    let server = Server::new("127.0.0.1:4005".to_string());
    server.run()
}

struct Server {
    addr: String,
}

impl Server {
    // associated func
    fn new(addr: String) -> Self {
        Server { addr }
    }

    // method
    fn run(self) {}
}
```

`fn run(self) {}`のように書いた場合、
rust のownershipの仕組み上、
この関数がexitした際に、deallocateされてしまう。
なので、deallocateされてほしくない場合は、`fn run(&self) {}`のように書く必要がある


### インスタンス作成

Self と Serverは同じなのでどちらで書いても大丈夫

```rust
// associated func
    fn new(addr: String) -> Self {
        Server { addr }
    }
```

```rust
// associated func
    fn new(addr: String) -> Server {
        Server { addr }
    }
```

### カスタムエラーを作成するときに怒られる

このようにカスタムエラーを作成していると、
下の方にあるようなエラーがあるよってコンパイラーが教えてくれる

```rust
use std::error::Error;

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {}
```

```
`ParseError` doesn't implement `std::fmt::Display`
the trait `std::fmt::Display` is not implemented for `ParseError`
in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) insteadrustcClick for full compiler diagnostic
error.rs(32, 26): required by a bound in `std::error::Error`

`ParseError` doesn't implement `Debug`
the trait `Debug` is not implemented for `ParseError`
add `#[derive(Debug)]` to `ParseError` or manually `impl Debug for ParseError`rustcClick for full compiler diagnostic

```

詳しく見ていくと、以下のような `Debug + Display` というsyntaxが登場する
これは、Debug + Display のtraitを実装してないとダメだよってことなので、別途実装する。

[std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html)

```
 | pub trait Error: Debug + Display {
   |                          ^^^^^^^ required by this bound in `Error`
```

```rust
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
```

### 文字列からEnumへの型変換

[method.rs](../scratch/src/http/method.rs)

`use std::str::FromStr`の`impl FromStr` はめっちゃよく使われる文字列からいろんな型へ変換を行うtrait

### lifetime

```rust
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
```
もともとはこのように書いていたけど、
変更する予定がない、かつ、HeapにRequestをHeapに保存する必要がないので、
以下のように書き換える

```rust
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}
```

このように書くことでこの構造体自体は、文字列を所有しない。
データを持たないので超軽量

ただlifetimeの問題がでてくる...

今回の場合は、`let mut buffer = [0; 1024];`のようなbufferのlifetimeがそのまま
Request構造体のlifetimeになるので、`pub struct Request<'buf> {` のようにわかりやすく`'buf`
を使っていく。

↓エラーがでる原因
```rust
fn try_from<'a>(buf: &'a [u8]) -> Result<Request<'buf>, Self::Error> {
```
bufとRequestは同じlifetimeを持つので、同じにならないといけない。

### dynamic dispatch vs static dispatch

```rust
impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    // tcp stream にデータを書き込む
    // このままだと、TcpStreamにしか書き込めないけど、テストを想定するとここはGeneric であったほうがいい
    pub fn send(&self, stream: &mut TcpStream) -> ioResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
```
1. Dynamic dispatch

- dyn -> dynamic dispatch
コンパイル時には、コンパイラは Write トレイトに必要なメソッド
（この場合は write メソッド）が存在することを確認しますが、
具体的な型（例えば TcpStream や File、Vec<u8> など）はコンパイル時には決まっていません。

実際の実装のマッピングはランタイム時に行われます。
Rustでは、トレイトオブジェクトの背後に「バーチャルテーブル（vtable）」が存在し、
このテーブルが実際の型のメソッドへのポインタを保持します。
これにより、実行時に適切なメソッドが呼び出されます。

```rust
pub fn send(&self, stream: &mut dyn Write) -> ioResult<()> {
    let body = match &self.body {
        Some(b) => b,
        None => "",
    };
```

2. Static dispatch

vtableを使わなくて良くなるのでパフォーマンスが上がるから基本的にはこっちが使われるが、
全くデメリットがないわけでもない。

embedded systemではfuncの実装が増えるのでコンパイルサイズが大きくなる

```rust

```

### traiが&mut selfを引数に持つ理由

このようなコード

```rust
pub trait Handler {
    fn handle_request(&mut self);
}
```

- 状態の変更:
&mut selfを使用することで、
Handlerトレイトを実装する型(e.g. struct )の内部状態を変更できるようになります。
これは、リクエストを処理する際に、そのハンドラーの状態を更新する必要がある場合に便利です。

- 所有権と借用の安全性:
Rustの所有権システムにより、&mut selfを使うことで、
一度に一つの可変参照しか持たないというルールが適用されます。
これにより、データ競合や不正なメモリアクセスを防ぎ、安全な並行処理が可能になります。

- 明示的な意図:
&mut selfを使うことで、メソッドがオブジェクトの状態を変更することを明示的に示すことができます。
これにより、コードの可読性が向上し、他の開発者がメソッドの意図を理解しやすくなります。
