Example 1:

```rust
fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    s = String::from("goodbye");
    println!("{}", ref3.to_uppercase());
}
```

Won't compile. `ref1` borrows `s`, so `s` shouldn'f be modified. (3 immutable reference and 1 mutable reference).

Example 2:

```rust
fn drip_drop() -> &String {
    let s = String::from("hello world!");
    return &s;
}
```

Won't compile. Dangling reference.

Example 3:

```rust
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0];
    println!("{}", s2);
}
```

Won't compile. `let s2 = v[0];` will borrow `v[0]`, which is not allowed. Consider borrow or clone here.
