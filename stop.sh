#!/bin/bash

echo "bot_ecosystem:stop"
binaries=bin

readarray -t pids < "$binaries/pid.txt"

for pid in "${pids[@]}"; do
	echo "closing pid $pid"
    eval "kill -9 $pid"
done

