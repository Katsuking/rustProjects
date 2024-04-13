// t1.rs で定義したものをmainで使う
pub mod t;
pub use t::{Cat, Dog, Sound}; // 必須ではないけど、mainで使うときにきれい

// t2.rs で定義したものをmainで使う
pub mod t2;
pub use t2::{Action, Animal}; // 必須ではないけど、mainで使うときにきれい
