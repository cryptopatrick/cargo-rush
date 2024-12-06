# cargo-rush

When you're in a rush and don't feel like creating a rust file to test out 
small snippets of simple Rust code - simply copy the Rust code to the operating system's clipboard, next, open a terminal and run `cargo rush`.

# Example

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
