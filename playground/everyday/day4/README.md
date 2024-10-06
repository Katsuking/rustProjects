### なぜ rust には null が存在しないのか?

[null の概念は大事だが、実装に問題がある](<https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html#option-enum%E3%81%A8null%E5%80%A4%E3%81%AB%E5%8B%9D%E3%82%8B%E5%88%A9%E7%82%B9:~:text=null%E3%81%AE%E9%96%8B%E7%99%BA%E8%80%85%E3%81%A7%E3%81%82%E3%82%8B%E3%83%88%E3%83%8B%E3%83%BC%E3%83%BB%E3%83%9B%E3%83%BC%E3%82%A2(Tony%20Hoare)%E3%81%AE2009%E5%B9%B4%E3%81%AE%E3%83%97%E3%83%AC%E3%82%BC%E3%83%B3%E3%83%86%E3%83%BC%E3%82%B7%E3%83%A7%E3%83%B3>)

第一に option は enum の型です。
rust には、null は存在しない代わりに概念をコード化した enum が存在する
これは超便利ですが、ただの enum です。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### なぜ null より Option<T>が優れているか

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    println!("{}", sum)
}
```

これを実行すると、下記のようなエラーを吐きます。

```
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            <i8 as Add>
            <i8 as Add<&i8>>
            <&'a i8 as Add<i8>>
            <&i8 as Add<&i8>>
```

つまり、Rust において、i8 のような型の値がある場合、
コンパイラが常に有効な値であることを確認してくれます。

逆にプログラマが Option<T> -> T に変換する作業が必要になる
null になる可能性のある値を保持するには、その値の型を Option<T>にすることで明示的に同意しなければなりません。
それからその値を使用する際には、値が null である場合を明示的に処理する必要があります。

### rust における module の考え方
