fn main() {
    // Modules enable more maintainable code and
    // also let you hide implementation details.

    // Public modules: anyone consuming the crate
    // can use the module and its pub members.
    // Private modules: accessible only to themselves
    // and their descendants.

    // To use anything declared in an external module
    // (eg math.rs), the `use` keyword is needed; eg: 
    // `use std::collections::HashMap`;

    // A reference to `super::thing` gets `thing` from
    // the parent module.
    // A reference to `crate::thing` gets `thing` from
    // the root of the crate you're in.
    println!("add 5 and 6 using the math module to get {}", math::add(5,6));

}

// if breaking the module into a different file, store as
// math.rs declare in main.rs as `pub mod math;`
// nb: math.rs preferable to math/mod.rs as multiple modules
// will end up in `mod.rs` files (confusing, though valid)
pub mod math {
    pub fn add(x: u32, y:u32) -> u32 {
        x + y
    }
}

// in a math.rs module file, just include:
// pub fn add(x: u32, y:u32) -> u32 {
//     x + y
// }
