# Boss Monster Card Reader

## Description

This project aims to extract information (title & description) from Boss Monster board game card's scans.

> Be aware that this algorithm is not bulletproof and a human verification/correction is necessary.

## Build

### Dependencies

Cargo takes care of all rusty dependencies.
However, `leptess` the rusty bindings for the lib *Tesseract* needs :
 - development *tesseract* package
 - development *leptonica* package

See [leptess GitHub](https://github.com/houqp/leptess) for more details.

### Manual

```
cargo build --release
```

## Usage

### CLI

Get an overview of the supported arguments by running :

```
boss_monster_card_reader-cli --help
```

- `-i <path>` add an input image path (multiple files can be added)
- `-o <path>` set **the** output file

### GUI

To do

### Diagnostics

To get diagnostics features, crate features need to be enabled, see `boss_monster_card_reader-core/Cargo.toml` to get an exhaustive list.

## Input scan

From an acquisition point of view, card have to be placed on a white background with edge aligned with the scan borders.

All image format supported by the `image` crate can be used.

The algorithm virtually supports all languages as far as you have associated tesseract trained data, however, the language and path to data are still hard-coded in the `boss_monster_card_reader-core::read_card` function.

## Output format

- JSON

Every format supported by the `serde` crate but, for the time, only JSON export have been added.

