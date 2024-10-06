
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

