# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Roadmap]

### Added

- ls file.odt | machexplode -d final_dir // all content files on odt (images, fonts) in a directory
- ls file.epub | machexplode -d final_dir // all content files (images, fonts) in a directory
- ls *.pdf | machexplode --type "text" -f final.doc // extract only text from pdf

- ls *.csv | machmap *.html
- ls *.csv | machmap *.pdf
- ls *.csv | machmap *.xls
- ls *.csv | machmap *.ods

- ls *.jpg | machconvert --resize 800x600
- ls *.jpg | machconvert --resize-x 800
- ls *.jpg | machconvert --resize-y 800

- ls *.jpg | machreduce -o result.odt
- ls *.jpg | machreduce -o result.odp

## [Unreleased]

### Added

- Convert webp files to png or jpg files

### Fixed

- Converting an svg file to a png or a jpg file lost the svg filename
- Reducing multiple png files to a zip archive only used the first image
- Converting an image file to a pdf creates a blank page on certain sizes
