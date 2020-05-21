#!/bin/bash

echo "bot_ecosystem:build"
binaries=bin
projects=(sudoku postmaster)

echo "preparation"
rm "$binaries/*"

echo "build"
for app in "${projects[@]}"; do
    cd "$app"
    
    cargo build --release >> "../logs/build.txt"
    executable="./target/release/$app"
    
    echo "$executable"
    
    strip "$executable"
    cp "$executable" "../$binaries"
    
    cd ..
done

echo "done."
