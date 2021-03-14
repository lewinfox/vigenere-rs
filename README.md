# vigenere-rs

A Vigenere cypher in Rust.

Clone the repo and run `cargo run -- -h` for help.

## Demo

```
# Create some plaintext
echo "Lorem ipsum dolor sit amet" > plain.txt

# Encrypt it
vigenere-rs -i plain.txt -k secretkey > cypher.txt

# Decrypt it
vigenere-rs -d -i cypher.txt -k secretkey
## LOREMIPSUMDOLORSITAMET
```
