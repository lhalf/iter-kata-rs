# Iterators

This Kata is all about using iterators in Rust and the functional style of programming they enable.

In main.rs you will see a series of functions currently with the todo!() macro, I wonder what that might suggest??

Under the tests module you will find a valid test for each of these functions, for instance for the following test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of() {
        assert_eq!(32, sum_of(&[5, 10, 5, 12]))
    }
    //...
}
```

You would need to complete the following function:

```rust
fn sum_of(_values: &[i32]) -> i32 {
    todo!()
}
```

The names of the function might hold a hint as to the iterator method you might consider using...

## Running the tests

```bash
cargo test
```