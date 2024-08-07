#![allow(unused)]
enum LanguageResource {
    // This variant has a field by position
    Website(String),
    // This variant has fields by name
    Book { title: String, pages: u64 },
    // This variant is of the unit type, no data contained
    SelfTeaching,
}

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal integers"))
    }
}

fn main() {
    // In comparison to other languages, enums in Rust
    // are relatively powerful: they're important tools
    // in structuring data and programs idiomatically.

    // In Rust, enums capture more than just a constant:
    // each variant of the enum can also have data.
    // Rust enums are like tagged unions in C.    
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("error: {msg}"),
    }
}

// fn main() {
        // This example from YARR needs debugging.

//     // Create instances of each variant:
//     let site = LanguageResource::Website("https://yet-another-rust-resource.pages.dev/".to_owned());
//     let book = LanguageResource::Book { title: "The Rust Programming Language".to_owned(), pages: 300 };
//     let independent = LanguageResource::SelfTeaching;

//     println!("the site is at {}", site.0);
//     println!("the book {} is {} pages long", book.title, book.pages);

// }
