alias l := lint
alias t := test
alias b := build

default:
    @just --list --list-prefix=....

lint:
    cargo check

test:
    cargo test

build:
    cargo build --release
