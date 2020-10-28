#!/bin/bash

echo "=== bot_ecosystem:build"
binaries=bin
projects=(sudoku artist employer)

echo "=== repository update"
git checkout -- .
git pull

echo "=== preparation:"
rm -rf "./$binaries/*"

echo "=== building atelier:"
for app in "${projects[@]}"; do

    echo "=== $app:start"
    cd "$app"

    echo "=== release cleanup:"
    rm -rf ./target/*
    
    echo "=== building $app:"
    cargo build --release >> "../logs/build.txt"
    executable="./target/release/$app"
    
    echo "=== now $app executable is in $executable"
    
    echo "=== strip:"
    strip "$executable"

    echo "=== copy to ../$binaries"
    cp "$executable" "../$binaries"
    
    echo "=== $app:done"
    cd ..
done

echo "=== build done."
