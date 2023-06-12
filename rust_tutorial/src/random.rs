use rand::{Rng, distributions::{Uniform, Alphanumeric}, prelude::Distribution, thread_rng};


pub fn random_numbers() {
    let mut rng = rand::thread_rng();

    // 明示的に型を渡す
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random n1:{}", n1);
    println!("Random n2:{}", n2);

}

pub fn random_numbers_within_range() {
    let mut rng = rand::thread_rng();

    // 範囲を与える
    println!("Integer: {}", rng.gen_range(1..=100));
    println!("Integer: {}", rng.gen_range(1.0..=100.0));
}

pub fn roll_dice() {
    let mut rng = rand::thread_rng();
    let dice = Uniform::from(1..=7); // 1~6のサイコロを振り続ける

    loop {
        let throw = dice.sample(&mut rng);
        println!("Roll the dice: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

pub fn generate_password() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric) // 乱数生成器から、Alphanumericという乱数のサンプルを繰り返し生成するイテレータを取得
        .take(30) // .take(30): イテレータから最初の30個の要素を取得します。つまり、30文字のランダムな文字列を生成
        .map(char::from) // .map(char::from): 各要素を文字に変換します。char::fromは文字列から文字への変換
        .collect(); // .collect(): 変換された文字列をStringに収集します。
    println!("Generated password: {}", rand_string);
}

pub fn generate_password_from_user_defined_chars() {
    // バイトスライスは、&[T] 形式で表されるスライスの特殊なケース
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;

    let mut rng = rand::thread_rng();

    // let idx = rng.gen_range(0..CHARSET.len());
    // println!("random index: {}", idx);

    let password : String= (0..PASSWORD_LEN)
        .map(|_|{
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("Generated password: {}", password);
}

