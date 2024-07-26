fn main() {

    // Rust is statically typed. 
    // What happens if type isn't assigned explicitly?
    let mut x = 10;
    println!("x is: {x}");

    x = 11;
    println!("x is now: {x}");

    // OUTPUT:
    // ```
    // x is: 10
    // x is now: 11
    // ```

    // Rust is statically typed, but it does employ
    // type inference where the type isn't explicitly
    // assigned.

    // Types annotated:
    let name_a: &str = "Captain Blackbeard";
    let age_a: u32 = 35;

    // Types not annotated:
    let name_b = "Captain Cook";
    let age_b = 40;

    println!("Pirate ages: {} is {}, {} is {}", name_a, age_a, name_b, age_b);

    // OUTPUT:
    // ```
    // Pirate ages: Captain Blackbeard is 35, Captain Cook is 40
    // ```


    // Primitive types
    // u8, u16, u32, u64, u128, usize
    // - usize is a pointer-sized unsigned integer
    //   - used to index into collections
    //   - on 64-bit systems, this is often a 64-bit integer
    // i8, i16, i32, i64, i128, isize
    // - isize is a pointer-sized signed integer, handy for representing
    //   the difference between two indices
    // f32, f64
    // - decimal representation: 1.2 (1.2f32)
    // char
    // - 4 byte unicode character
    // - use single quotes: 'a'
    // bool
    // - true or false

    // Compound types (multiple elements)
    // tuples: `()`, elements can have mixed types
    // arrays: `[]`, elements must have same types

    // Strings
    // `str` is the primitive type for a string
    // - `str` is a UTF-8 encoded string slice
    // - it's usually seen in the reference form: `&str`
    // - since String is UTF-8 encoded, characters have 
    //   variable-width encoding; this complicates accessing
    //   characters at specific indices
    // - String handling is complicated in Rust

    // Unit
    // `()` is the "unit type"
    // - basically, means 'nothing'
    // - it's an empty, 0-element tuple
    // - equivalent to `void` in other languages

    // Casting between types
    // `as`: var_name as type
    // - eg: y as i64;

    let x: i64 = -13;
    let y: u8 = 10;
    let z = x + (y as i64);
    println!("y cast from u8 to i64: {z}");
    let z: u8 = x as u8;
    println!("x {} cast from i64 to u8: {}\nint overflow!!!", x, z);

    // OUTPUT:
    // ```
    // y cast from u8 to i64: -3
    // x -13 cast from i64 to u8: 243
    // int overflow!!!
    // ```    

}
