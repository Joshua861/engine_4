#!/run/current-system/sw/bin/sh

text=$(tail -n +1 ./examples/*.rs ./src/*.rs ./src/*/*.rs ./src/*/*/*.rs ./assets/shaders/*/* Cargo.toml)
echo "$text" | wl-copy || echo "$text" >output.txt
