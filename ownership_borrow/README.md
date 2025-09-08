# Ownership and Borrow in Rust

This repo is for learning basic Rust concept:
- ownership
- borrowing
- mutable and immutable reference
- scope and shadowing

Each example is inside `src/main.rs` with comments for explanation.  
It is like small documentation with code.

## Run
```bash
cargo run

Notes

Only one mutable borrow is allowed at same time.

Multiple immutable borrow is allowed.

Ownership moves when assign variable.

Scope is important for borrow checker.
