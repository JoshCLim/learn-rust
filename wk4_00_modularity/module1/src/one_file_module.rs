#![allow(dead_code)]

// same as private
pub(self) fn public_within_this_module() {
    println!("I'm public within this module!");
}

pub(super) fn public_within_parent_module() {
    println!("I'm public within the parent module!");
}

pub(crate) fn public_within_crate() {
    println!("I'm public within the crate!");
}

pub(in crate::one_file_module) fn public_within_crate_module() {
    println!("I'm public within the module!");
}

pub fn public() {
    println!("I'm public!");
}
