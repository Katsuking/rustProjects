use super::{Cat, Sound}; // 親モジュールからimport

pub trait Action {
    fn do_some_action(&self);
}

pub struct Animal<T> {
    pub animal: T,
    pub name: String,
}
impl<T> Animal<T> {
    pub fn new(animal: T, name: String) -> Self {
        Self { animal, name }
    }
}

impl<T: Sound> Animal<T> {
    pub fn make_loud_noise(&self) {
        println!("Screeeeeeem");
    }
}

// Sound Action 両方を満たす場合のみ
impl<T: Sound + Action> Animal<T> {
    pub fn dash_loudly(&self) {
        println!("Animal is running around loudly");
    }
}

// すでに Sound traitを実装しているCat structに
// Action traitも実装する
// 一方, Dog には実装しない
impl Action for Cat {
    fn do_some_action(&self) {
        println!("cat is moving so fast")
    }
}
