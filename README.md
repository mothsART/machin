# Machin

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/machin.svg)](https://crates.io/crates/machin)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.60.0+-lightgray.svg)](#rust-version-requirements)

## Intro

**Machin** is a cli program that simplifies file conversions and batch processing.
It's inspired from filter/map/reduce

## Install on your system

clone the projet and install it with :

```zsh
cargo install --path .
```

## Some **machmap** examples

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
ls README.md | machmap -o result.png
```

## Some **machconvert** examples

Apply a 180 degree rotation of photo.jpg to prefix_photo.jpg

```zsh
ls photo.jpg | machconvert -r 180 -p prefix_
```

## Some **machreduce** examples

Concatenate images on same file :

```zsh
ls *.png | machreduce -o result.png
```

## Dev

Makefile inspire by https://git.sr.ht/~julienxx/castor/tree/master/item/Makefile

## Publish

```zsh
make cargo-publish
```
