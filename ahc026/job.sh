#!/bin/bash
rm -f scores.txt
for i in {0..99}; do
  i_padded=$(printf "%04d" $i)
  input_file="in/${i_padded}.txt"
  output_file="out/${i_padded}.txt"
  # cargo run --bin a < "$input_file" > "$output_file" 2>> "scores.txt"
  cargo run --bin a < "$input_file" >> "scores.txt" 2> /dev/null
done
input_file="scores.txt" # 入力ファイルの名前
sum=0

while IFS= read -r line; do
  # 各行の数字を和に追加
  sum=$((sum + line))
done < "$input_file"

echo "sum: $sum"

