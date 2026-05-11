Development
===========

This command-line arguments are supported:

- `s` -- to compile CV for *startups*
- `e` -- to compile CV for *enterprise*
- `letter` -- to compile a (motivation) letter (from src/letter.tex)

By default `s e` will be provided as arguments.

## In Docker

```shell
docker build --tag cv .
docker run --rm -v .:/out cv
```

## Local on macos

### Deps

```shell
brew install libpng graphite2 freetype icu4c
```

### Setting up build env

```shell
export PATH="$(brew --prefix icu4c)/bin:$PATH"
export PKG_CONFIG_PATH="$(brew --prefix icu4c)/lib/pkgconfig:$PKG_CONFIG_PATH"
export LDFLAGS="-L$(brew --prefix icu4c)/lib"
export CPPFLAGS="-I$(brew --prefix icu4c)/include"
```

### Creating pdf

```shell
cargo run
```

## Debugging the latex compilation:

```shell
cd src
pdflatex debug.tex
```
