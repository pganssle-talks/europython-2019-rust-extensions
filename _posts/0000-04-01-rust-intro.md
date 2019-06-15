<img src="external-images/logos/rust-logo-with-text.svg"
     alt="The logo for the Rust language"
     style="height: 300px; max-height: 300px"
     />

- Memory safety
- Fearless concurrency
- High performance

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

--

# Python: Reference counting

```python
def take_ownership(v):
    # Reference count increases by 1
    ...      # Do stuff with v
    # Decrease reference count of v by 1


def main():
    v = [1, 2, 3]   # Reference count of v is 1

    take_ownership(v)  # Reference count goes up during function execution

    print(v)        # v still alive to be printed
```

<br/><br/>

# Rust: Borrowing

```rust
fn borrow_reference(&v : Vec<i32>) {
    // Do stuff with v
}  // v still owned by whoever called borrow_reference

fn main() {
    v = vec![1, 2, 3];      // v owns vector

    borrow_reference(v);    // borrow_reference gets read-only access to vector

    println!("{}", v[0]);   // v still alive
}
```

<br/>
Zero-cost abstraction: Memory safety is guaranteed at *compile time*.
