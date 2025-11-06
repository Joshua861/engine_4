#!/run/current-system/sw/bin/sh

text=$(tail -n +1 ./src/* ./assets/shaders/*/* Cargo.toml)
echo "$text" | wl-copy || echo "$text" >output.txt
