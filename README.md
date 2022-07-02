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
ls file.svg | machmap -o result.png
```

Convert png file to jpg :

```zsh
ls file.png | machmap -o result.jpg
```

Convert png file to avif :

```zsh
ls file.png | machmap -o result.avif
```

Convert jpg file to png :

```zsh
ls content/essai.jpg | machmap -o result.png
```

Convert jpg file to avif :

```zsh
ls file.jpg | machmap -o result.avif
```

Convert markdown to 

```zsh
/bin/ls README.md | machmap -o result.png
```

Apply a 180 degree rotation of photo.jpg to prefix_photo.jpg

```zsh
ls photo.jpg | machconvert -r 180 -p prefix_
```

## Dev

Makefile inspire by https://git.sr.ht/~julienxx/castor/tree/master/item/Makefile

## Publish

```zsh
make cargo-publish
```
