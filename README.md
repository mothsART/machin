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

Convert svg files to png, jpg :

```zsh
ls *.svg | machmap -e png
ls *.svg | machmap -e jpg
```

Convert png files to jpg, avif, pdf :

```zsh
ls *.png | machmap -e jpg
ls *.png | machmap -e avif
ls *.png | machmap -e pdf
```

Convert jpg file to png, avif, pdf :

```zsh
ls *.jpg | machmap -e png
ls *.jpg | machmap -e avif
ls *.jpg | machmap -e pdf
```

Convert markdown to 

```zsh
ls *.md | machmap -e png
```

## Some **machconvert** examples

Apply a grayscale, a vertical flip and at last a 90 degree rotation of photo.jpg to prefix_photo.jpg

Order is important :
1. On first, color option
2. flip option (vertical or horizontal)
3. rotation

```zsh
ls *.png | machconvert -c grayscale -f vertical -r 90 -p prefix_
ls *.jpg | machconvert -c grayscale -f vertical -r 90 -p prefix_
```

## Some **machreduce** examples

Concatenate images on same file :

```zsh
ls *.png | machreduce -o result.png
```

Create pdf with images:

```zsh
ls *.jpg | machreduce -o result.pdf
```

Create archiv (zip) with files:

```zsh
ls *.png | machreduce -o archive.zip
```

Create one pdf with several pdfr

```zsh
ls *.pdf | machreduce -o merge.pdf
```

## Dev

Makefile inspire by https://git.sr.ht/~julienxx/castor/tree/master/item/Makefile

## Publish

```zsh
make cargo-publish
```
