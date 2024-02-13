## Project goals
- Get better and faster at writing Rust
- Learn more of the ecosystem
- Learn from past mistakes
    - Use anyhow from the beginning
    - Use sqlite from the beginning
    - Don't unwrap willy nilly 


## Getting Started

First, initialize the database. This only needs to be done once.
```bash
createdb railroad_inc_ai
yarn reset-db
```

Install sqlx
```bash
cargo install sqlx-cli --no-default-features --features sqlite
```
