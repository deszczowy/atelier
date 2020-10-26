#!/bin/bash

echo "=== bot_ecosystem:run"
binaries=bin
projects=(sudoku employer)

echo "=== run:preparation:"
cd "$binaries"
rm ./*.list

for app in "${projects[@]}"; do
	echo "=== running $app"
    eval "./$app >> ../logs/$app.log &"
    echo "$!" >> pid.list
done

cd ..
echo "=== run:done."