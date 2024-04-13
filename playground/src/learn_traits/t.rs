// traits について理解する

pub struct Dog {
    pub name: String,
}

impl Dog {
    pub fn new(name: String) -> Self {
        Dog { name }
    }
}

pub struct Cat {
    pub name: String,
}

impl Cat {
    pub fn new(name: String) -> Self {
        Cat { name }
    }
}

// traits
// Dog も Cat も make_some_noiseを持つ

pub trait Sound {
    fn make_some_noise(&self);
}

impl Sound for Dog {
    fn make_some_noise(&self) {
        println!("woof woof")
    }
}

impl Sound for Cat {
    fn make_some_noise(&self) {
        println!("meow")
    }
}

// struct を渡すわけだか、traitを実装してなければいけない
pub fn do_something_with_struct_using_traits(animal: &impl Sound) {
    animal.make_some_noise()
}
