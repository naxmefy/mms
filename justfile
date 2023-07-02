alias l := lint
alias t := test
alias b := build
alias bb := book-build
alias bs := book-serve

default:
    @just --list --list-prefix=....

lint:
    cargo check

test:
    cargo test

build:
    cargo build --release
book-build:
    mdbook build book/
book-serve:
    mdbook serve --open book/