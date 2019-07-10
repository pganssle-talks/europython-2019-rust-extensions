<img src="external-images/logos/rust-logo-with-text.svg"
     alt="The logo for the Rust language"
     style="height: 300px; max-height: 300px"
     />

- Memory safety
- Fearless concurrency
- High performance
- Broad community and open source ecosystem


Notes:

This is where Rust comes in, because Rust is a modern systems programming
language that attempts to build solutions to these and other problems deep
into the structure of the language. It is a memory safe language that seeks
to enable fearless concurrency without sacrificing performance. And one thing
that I think is particularly appealing to those of us from the Python community
is that it has a high-quality package manager and a huge ecosystem of open
source packages.

--

# Rust: Ownership

- Variable bindings have ownership over the resource they're bound to:
<br/>

```rust
fn some_func() {
    let v = vec![1, 2, 3];
} // v goes out of scope here
```
<br/>

- Assignment of resources *moves* the ownership to the new variable

```rust
fn take_ownership(v : Vec<i32>) {
    // Do stuff with v
} // Resources associated with v can be cleaned up


fn main() {
    let v = vec![1, 2, 3];  // Vector is owned by "v"

    take_ownership(v);      // Vector moves into take_ownership

    println!("{}", v[0]);   // Error: use of moved value `v`
}
```

<br/>
<br/>

```
error[E0382]: borrow of moved value: `v`
  --> src/main.rs:11:20
   |
7  |     let v = vec![1, 2, 3];  // Vector is owned by "v"
   |         - move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
8  |
9  |     take_ownership(v);      // Vector moves into take_ownership
   |                    - value moved here
10 |
11 |     println!("{}", v[0]);   // Error: use of moved value `v`
   |                    ^ value borrowed here after move
```
<fragment/>

