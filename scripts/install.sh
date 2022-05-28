#!/usr/bin/env sh

# rustup override set nightly
apt-get install gcc-avr
apt-get install arduino-core-avr
sudo apt install avr-libc gcc-avr pkg-config avrdude
cargo add panic-halt
"
[build]
target = "avr-atmega328p.json"

[unstable]
build-std = ["core"]
" >> ~/.cargo/config.toml
# rustup override set nightly-2021-01-07
rustup override set nightly-2020-08-26
