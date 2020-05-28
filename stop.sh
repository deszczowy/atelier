#!/bin/bash

echo "=== bot_ecosystem:stop"
binaries=bin

echo "=== stop:getting pids:"
readarray -t pids < "$binaries/pid.list"

echo "=== stopping:"
for pid in "${pids[@]}"; do
	echo "=== closing pid $pid"
    kill -9 "$pid"
done

echo "=== stop:done."