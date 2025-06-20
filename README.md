# Machin

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/mothsART/machin/actions/workflows/ci.yml/badge.svg)](https://github.com/mothsART/machin/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/mothsART/machin/badge.svg?branch=master)](https://coveralls.io/github/mothsART/machin?branch=master)
[![Crates.io Version](https://img.shields.io/crates/v/machin.svg)](https://crates.io/crates/machin)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.62.0+-lightgray.svg)](#rust-version-requirements)

## Intro

**Machin** is a cli program that simplifies file conversions and batch processing.
It's inspired from filter/map/reduce

## Last stable version

[![Packaging status](https://repology.org/badge/vertical-allrepos/machin.svg)](https://repology.org/project/machin/versions)

## Install with crates.io

```zsh
cargo install machin
```

## Install on your system

clone the project and install it with :

```zsh
cargo install --path .
```

## Some **machmap** examples

Give the list of supported conversion for an entry format :

```zsh
machmap -s svg
```

Convert svg files to png, jpg, avif or pdf :

```zsh
ls *.svg | machmap -e png
ls *.svg | machmap -e jpg
ls *.svg | machmap -e avif
ls *.svg | machmap -e pdf
```

Convert webp files to png, jpg and avif :

```zsh
ls *.webp | machmap -e png
ls *.webp | machmap -e avif
ls *.webp | machmap -e jpg
```

Convert png files to jpg, avif and pdf :

```zsh
ls *.png | machmap -e jpg
ls *.png | machmap -e avif
ls *.png | machmap -e pdf
ls *.png | machmap -e xcf
```

Convert jpg files to png, avif, pdf, odt :

```zsh
ls *.jpg | machmap -e png
ls *.jpg | machmap -e avif
ls *.jpg | machmap -e pdf
ls *.jpg | machmap -e odt
ls *.jpg | machmap -e xcf
```

Convert markdown files to html :

```zsh
ls *.md | machmap -e html
```

Convert json files to yaml :

```zsh
ls *.json | machmap -e yaml
```

Convert yaml files to json :

```zsh
ls *.yaml | machmap -e json
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

Create pdf with images :

```zsh
ls *.jpg | machreduce -o result.pdf
```

Create archive (zip) with files :

```zsh
ls *.png | machreduce -o archive.zip
```

Create one pdf with multiple pdf :

```zsh
ls *.pdf | machreduce -o merge.pdf
```

## Autocomplete

On zsh :

Add this on your ~/.zshrc :

```zsh
fpath=("dir_of/_pouf" "${fpath[@]}")
```

before :
```zsh
autoload -Uz compinit && compinit
```

## Dev

Makefile inspired by https://git.sr.ht/~julienxx/castor/tree/master/item/Makefile

## Publish

```zsh
make cargo-publish
```
