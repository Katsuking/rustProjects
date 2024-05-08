mod path;
use rfd::FileDialog;
use std::{
    fs::{metadata, FileType, Metadata},
    path::PathBuf,
};

fn main() {
    let file = select_path();
    println!("{:?}", file);
}

// 絶対ファイルを選択マン
fn select_path() -> PathBuf {
    let path = FileDialog::new()
        .add_filter("text", &["txt", "rs"])
        .set_directory("/")
        .pick_file()
        .expect("Failed to read file...");

    // そもそもファイル以外選択できないようになってる...
    let metadata = metadata(&path).expect("Could not read metadata");
    if metadata.is_dir() {
        println!("Please select a file");
        select_path();
    }
    path
}
