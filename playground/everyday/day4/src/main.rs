mod about_path;

use about_path as p;

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
    // println!("{}", sum);

    // matchを使ってパターンをすべて処理する
    let result = plus_one(y);
    let n_result = plus_one(None);
    println!("{:?}, {:?}", n_result, result);

    p::learn_about_module();
    let relative_path = std::path::Path::new("README.md");
    let full_path = p::accessible::path_absolute_form(&relative_path).unwrap();
    println!("{:?}", full_path);
}

fn plus_one(value: Option<i8>) -> Option<i8> {
    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}
