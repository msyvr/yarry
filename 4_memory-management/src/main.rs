fn main() {
    // A defining characteristic of Rust:
    // full control over memory.
    // - no runtime garbage collector
    // - no default reference counting
    //   - can explicitly request it to, though
    // - you control allocation and deallocation
    //   - modulo safety rules: can't deallocate 
    //     and then reuse

    // IOW: to understand Rust, must understand
    // memory management.

    // Stack and Heap
    // stack: push on, pop off (first in, last out)
    // - but! also referenced from the middle of the stack
    // heap: open field of memory (not a traditional heap
    // data structure)

    // Stack
    // Controlled by program's execution:
    // call frames (pushed on for function calls) + the
    // local variables of those functions
    // Once the function ends, its call frame is popped off
    // the stack and its local variables can no longer be
    // referenced.

    // Heap
    // Like an open field; essentially unbounded. Variables 
    // can live as long as they're not deallocated.
    // Memory that's allocated on the heap MUST be deliberately 
    // deallocated later.
    // A bit less efficient than memory allocated on the stack:
    // the TL;DR is that variables on the stack are likely in
    // the CPU cach when related code is executed; not true for
    // heap memory, where a slow fetch is required since the CPU's
    // ability to predict what you'll need in that scenario is
    // limited.

    // References
    // - reference to variable `x`: `&x`
    // - to access the underlying value, dereference `x`: `*x`
    // - Rust will automatically dereference references so, usually, can omit `*`

    // Here, the output for each call to `println!` will be the same:

    let x = 10;
    let ref_x: &u32 = &x;
    println!("x = {}", *ref_x);
    println!("x = {}", ref_x);

}
