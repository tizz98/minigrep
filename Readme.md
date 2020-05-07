# minigrep

A grep-like cli program to search files.

## Usage

```bash
cargo run to poem.txt  # search for "to" in poem.txt (case-sensitive)
cargo run to poem.txt --insensitive
env CASE_INSENSITIVE=1 cargo run to poem.txt
```
