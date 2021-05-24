# Machin

## Intro

**Machin** is a cli program that simplifies file conversions and batch processing.
It's inspired from filter/map/reduce

# Some examples

Give the list of supported conversion for an entry format :

```zsh
cargo build && ./target/debug/mmap -s svg
```

Convert svg file to png :

```zsh
cargo build && /bin/ls content/clipPath.svg | ./target/debug/mmap -o result.png
```

Convert png file to jpg :

```zsh
cargo build && /bin/ls content/essai.png | ./target/debug/mmap -o result.jpg
```

Convert jpg file to png :

```zsh
cargo build && /bin/ls content/essai.jpg | ./target/debug/mmap -o result.png
```
