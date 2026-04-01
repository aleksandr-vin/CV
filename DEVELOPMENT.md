Development
===========

## Creating pdf

```shell
cargo run
```

## Debugging the latex compilation:

```shell
cd src
pdflatex debug.tex
```

## Known problems

```
cargo clean
export PKG_CONFIG_PATH="$(brew --prefix harfbuzz)/lib/pkgconfig:$(brew --prefix freetype)/lib/pkgconfig:$(brew --prefix graphite2)/lib/pkgconfig:$(brew --prefix icu4c@78)/lib/pkgconfig"
export CXXFLAGS="-I$(brew --prefix harfbuzz)/include -std=c++17"
export CC_SHELL_ESCAPED_FLAGS=1
cargo build -vv
```
