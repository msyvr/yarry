fn main() {
    // Unit tests
    // To have these compile only with a test
    // flag enabled, structure as: 
    // create child module `mod test {}`; import from
    // the parent; test things by running `cargo test`

    // Unit tests vs integration tests: pretty standard -
    // - unit tests test individual functions
    // - integration tests can only consume any public API 
    // provided by your code

    // !! Any file in the `tests/` directory will be 
    // treated as an integration test.
    // Example: `plus` function in a crate. Create
    // `tests/my_test.rs` containing:
    // use my_library::plus;
    // #[test]
    // fn test_addition() {
    //     assert_eq!(plus(10, 20), 30)};
    // }

    // Doc tests: write tests in your documentation!
    // - put a docstring on a function, module, etc. with `///`
    // - if markdown style code blocks are used, the test 
    // will be compiled and run on `cargo test`
    // ! side effect: doc code examples automatically break
    // the build if they're out of date

    // Example doc test for the `plus` function
    /// Adds together two numbers; doesn't handle rollover.
    /// 
    /// ```
    /// use my_library::plus;
    /// assert_eq!(30, plus(10, 20));
    /// ```

}

pub fn plus(x: i32, y:i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    // although unusual otherwise, it's idiomatic 
    // to import all for tests
    use super::*;
    // to mark the subsequent function as a test:
    #[test]
    fn adds_small_numbers() {
        let x = 10;
        let y = 20;
        let expected = 30;
        assert_eq!(plus(x, y), expected, "err msg: x and y don't add up");
    }

    #[test]
    fn another_test_for_plus() {
        // the maximum value i32 can hold is 2_147_483_647
        // so this test should fail
        let x = 2_000_000_000;
        let y = x;
        assert!(plus(x, y) > 0, "err msg: this sum should be positive");
    }
}
