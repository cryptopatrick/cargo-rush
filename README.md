# cargo-rush

When you're in a rush and don't feel like creating a rust file to test out
small snippets of simple Rust code - simply copy the Rust code to the operating system's clipboard, next, open a terminal and run `cargo rush`.

# Example 1: main

### Step 1: Copy some Rust code to memory

Let's say that we copy the lines below to the clipboard (operating system memory).

```rust
fn main() {
    let a = 1;
    let b = 2;
    println!("{} + {} = {}", a, b, a + b);
}
```

### Step 2:

In the terminal:

```shell
cargo install cargo-rush
cargo rush
# The command will call `rustc` on whatever is in the clipboard:
1 + 2 = 3
```

---

## Example 2: Unit Tests

`cargo rush` can also run unit tests by using the `-t` flag.
Let's say we have the following code on the OS clipboard:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn main() {
    println!("Hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

### Step 1: Copy some Rust code to memory

### Step 2: cargo rush -t
