#!/bin/bash

echo "bot_ecosystem:run"
binaries=bin
projects=(sudoku postmaster)

cd "$binaries"
rm ./*.txt

for app in "${projects[@]}"; do
	echo "$app"
    eval "./$app >> ../logs/$app.txt &"
    echo "$!" >> pid.txt
done

cd ..
