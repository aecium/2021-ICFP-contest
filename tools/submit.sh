#!/bin/bash

#You will need to creat a .env in the tools dir with the var APIKEY=your teams api key

source .env

solutions_path=../solutions

solutions=$(ls $solutions_path)

for solution in $solutions; do
    echo "submitting: ${solution}"
    curl https://poses.live/api/problems/$(basename --suffix=.json $solution)/solutions \
    --request POST -g \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer ${APIKEY}" \
    --data "$(cat ${solutions_path}/${solution})"
done
echo Done