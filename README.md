# vector

## What is it?
`vector` is a rust library for creation and manipulation of mathematical vectors.

## What does it do?
`vector` allows for creation of three dimensional vectors as well as performing arithmetic operaitons on them, some of which are: addition, subtraction, multiplication, division of the magnitude, cross and dot prodcuts and normalizaton.

## How do I get started?
Make sure you have a project set up using `cargo` then:

### If using `cargo-edit`: 
`cd` into the said project directory and execute
```
cargo add vector
```

### If not using `cargo-edit`:
Add this under your dependencies in `Cargo.toml`
```
vector = "1.1.0"
```

## Examples:
```rust
let a = Vector3::from_i32(1, 2, 3);
let b = Vector3::from_i32(1,2,3);

assert_eq!(a.dot(&b), 14.0);
```

```rust
let a = Vector3::from_i32(1, 2, 3);
let b = Vector3::from_i32(3, 2, 1);

assert_eq!(a.cross(&b), Vector3::from_i32(-4, 8, -4));
```

```rust
let a = Vector3::from_i32(1, 0, 0);
let b = Vector3::from_i32(0, 0, 1);

assert_eq!(a.angle(b) * (180.0 / PI), 90.0);
```