# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Roadmap]

- upgrade to clap >= 4.0

### Added

- ls file.odt | machexplode -d final_dir // all content files on odt (images, fonts) in a directory
- ls file.epub | machexplode -d final_dir // all content files (images, fonts) in a directory
- ls *.pdf | machexplode --type "text" -f final.doc // extract only text from pdf

- ls *.csv | machmap *.html
- ls *.csv | machmap *.pdf
- ls *.csv | machmap *.xls
- ls *.csv | machmap *.ods

- ls *.yaml |machmap *.toml
- ls *.json |machmap *.toml
- ls *.toml |machmap *.yaml
- ls *.toml |machmap *.json

- ls *.yaml |machmap *.pdf // colorized and insert on pdf file
- ls *.json |machmap *.pdf // colorized and insert on pdf file
- ls *.toml |machmap *.pdf // colorized and insert on pdf file
- ls *.yaml |machmap *.html // colorized and insert on html file
- ls *.json |machmap *.html // colorized and insert on html file
- ls *.toml |machmap *.html // colorized and insert on html file
- ls *.yaml |machmap *.md // insert code on markdown file
- ls *.json |machmap *.md // insert code on markdown file
- ls *.toml |machmap *.md // insert code on markdown file

- ls *.svg | machconvert --sanitize // using svg-hush

- ls *.jpg | machconvert --resize 800x600
- ls *.jpg | machconvert --resize-x 800
- ls *.jpg | machconvert --resize-y 800

- ls *.jpg | machreduce -o result.odt
- ls *.jpg | machreduce -o result.odp


- base64 <-> image
- minifize/deminifieze json, html, svg

## [Unreleased]

### Added

- ls *.yaml |machmap *.json
- ls *.json |machmap *.yaml

### Fixed

- preserve order on converting yaml to json 

## [0.5.3]  - 2022-08-19

### Added

- Convert webp files to png or jpg files

### Removed

- Converting an image to avif file is too slow.
Waiting resolution of https://github.com/image-rs/image/issues/1760

### Fixed

- Converting an svg file to a png or a jpg file lost the svg filename
- Reducing multiple png files to a zip archive only used the first image
- Converting an image file to a pdf creates a blank page on certain sizes
