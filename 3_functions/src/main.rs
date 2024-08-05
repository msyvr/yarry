fn main() {
    // Syntax as for main(); return type omitted
    // is the same as -> (), which returns the 
    // unit type (similar to `void` in `C`)

    for n in 1..=10 {
        let x = fibonacci(n);
        println!("Fibonacci({n}) = {x}");
    }
}

fn fibonacci(n: u32) -> u32 {
  if n < 2 {
    return n
  }

  fibonacci(n-1) + fibonacci(n-2)
}