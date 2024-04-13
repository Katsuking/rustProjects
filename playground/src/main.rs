mod learn_traits;
use learn_traits::{t::do_something_with_struct_using_traits, Cat, Dog, Sound};
use learn_traits::{Action, Animal};

fn main() {
    // traits - learn_traits
    let dog = Dog::new(String::from("uncle tom"));
    let cat = Cat::new(String::from("hello kitty"));
    dog.make_some_noise();
    cat.make_some_noise(); // from Sound trait
    cat.do_some_action(); // from Action trait

    // traits
    do_something_with_struct_using_traits(&dog);

    let new_cat = Animal::new(cat, String::from("master kitty"));
    new_cat.make_loud_noise();
    new_cat.dash_loudly();

    // t2.ts Dog は Cat のように Action traitを実装してないのでエラーになる
    let new_dog = Animal::new(dog, String::from("doggy dog"));
    new_dog.make_loud_noise();
    // new_dog.dash_loudly(); // error: its trait bounds were not satisfied
}
