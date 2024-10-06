// about_path モジュールの中にこれこれ使いますよって話ね

pub mod accessible; // モジュールの外でも使用する
mod inaccessible; // モジュールの外では利用しない

pub fn learn_about_module() {
    // call func defined in the module.
    accessible::call_private_func_in_accessible();
    inaccessible::public_func_in_inaccessible();
}
