#!/bin/bash

counter=0
start_time=$(date +%s)
runtime=10

while true; do
    curl -s "$1" > /dev/null
    counter=$((counter + 1))

    current_time=$(date +%s)
    elapsed=$((current_time - start_time))

    if [[ $elapsed -ge $runtime ]]; then
        break
    fi
done

echo "GET $1 $counter times $runtime seconds"
