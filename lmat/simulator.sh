#!/bin/bash

sum=0
rounds=100000
bet=1.0
deposit=$(echo "100 * $rounds * $bet" | bc -l)

curl --silent -X 'POST' \
  'http://localhost:8080/deposit' \
  -H 'accept: text/plain' \
  -H 'Content-Type: application/json' \
  -d "{\"amount\": $deposit}" > /dev/null

for i in `seq 1 $rounds`; do
  res=$(curl --silent --fail -X 'POST' \
    'http://localhost:8080/spin' \
    -H 'accept: text/plain' \
    -H 'Content-Type: application/json' \
    -d "{\"bet\": $bet}" || echo 0)
  sum=$(echo $sum + $res | bc -l)
done

echo RTP $(echo $sum / $rounds / $bet | bc -l)