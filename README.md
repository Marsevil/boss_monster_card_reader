# Boss Monster Card Reader

## Description

This project aims to extract information (title & description) from Boss Monster board game card's scans.

## Build

### Dependencies

Cargo takes care of all rusty dependencies.
However, `leptess` the rusty bindings for the lib *Tesseract* needs :
 - debug *tesseract* package
 - debug *letptonica* package

See [leptess GitHub](https://github.com/houqp/leptess) for more details.

### Manual

- `cargo build --release`

## Usage

### CLI

`boss_monster_card_reader-cli --help` provides a list of all available options.

- `-i <path>` can be used to add an input image path (multiple file can be added).
- `-o <path>` can be used to set the output file

### GUI

To do

## Input scan

From an acquisition point of view, card have to be placed on a white background with edge aligned with the scan borders.

## Output format

To do
