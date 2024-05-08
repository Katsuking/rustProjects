### Path vs PathBuf

- Path: immutable 主に、ファイルシステム上のエンティティ（ファイル、ディレクトリなど）への参照を取得するために使用されます。
- PathBuf: mutable 主に、ファイルシステム上のエンティティを操作するために使用されます。たとえば、ファイルの作成、削除、名前変更などを行うことができます。

```rust
use std::path::Path;
let path = Path::new("/home/username/Downloads/wizard_of_oz.txtaa");
// path の正規化
let cpath = path.canonicalize().expect("path does not exists");
println!("{:?}", path);
println!("{:?}", cpath);
```

### ダイアログからファイルを取得

python では、下記のようにコンソールアプリでは使っているので、
これくらい簡単に GUI の explorer でファイルを取得を行いたい。

```python
from tkinter import filedialog
filedialog.askopenfilename()
```

rust では、`rfd`というクレートで同じことが実現できて、
ちゃんとマルチプラットフォーム。(windows, mac, linux)
使い方も超シンプルでありがたい

```rust
use rfd::FileDialog;

let path = FileDialog::new()
    .add_filter("text", &["txt", "rs"])
    .set_directory("/")
    .pick_file()
    .expect("Failed to read file...");
```
