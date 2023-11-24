#!/bin/bash

# Loop through file numbers from 0 to 999
for i in {0..4999}; do
    # Construct the input filename
    input_file="$(printf "./in2/%04d.txt" "$i")"
    
    # Extract the last number from the first line of the input file
    last_number=$(awk 'NR==1{print $3}' "$input_file")
    
    # Calculate the expected output line count
    expected_line_count="$((last_number + 1))"

    # Run ./a.out with the input file and capture its output
    output="$(cargo run --bin test2 < "$input_file")"

    # Actual output line count
    actual_line_count=$(echo "$output" | wc -l)

    # Compare expected and actual line counts
    if [ "$actual_line_count" -eq "$expected_line_count" ]; then
        echo "Test $i: PASSED"
    else
        echo "Test $i: FAILED (Expected $expected_line_count lines, got $actual_line_count lines)"
    fi
done
