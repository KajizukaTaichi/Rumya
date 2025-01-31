# Rumya
Rumya programming language's binding for Rust.

You can utilize Rumya program embedded in your project.
```rust
let rumya = Rumya::new().set_rumya(PATH);
let result = rumya.eval::<i32>("let x = 0. for i in 1 ~ 10 do x += i. x");
assert_eq!(result, Some(45));
```
