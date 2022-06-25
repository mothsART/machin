# Machin

## Intro

**Machin** is a cli program that simplifies file conversions and batch processing.
It's inspired from filter/map/reduce

## Install on your system

clone the projet and install it with :

```zsh
cargo install --path .
```

# Some examples

Give the list of supported conversion for an entry format :

```zsh
machmap -s svg
```

Convert svg file to png :

```zsh
/bin/ls content/clipPath.svg | machmap -o result.png
```

Convert png file to jpg :

```zsh
/bin/ls content/essai.png | machmap -o result.jpg
```

Convert jpg file to png :

```zsh
/bin/ls content/essai.jpg | machmap -o result.png
```

Convert markdown to 

```zsh
/bin/ls README.md | machmap -o result.png
```
