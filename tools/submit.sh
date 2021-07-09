#!/bin/bash

#You will need to creat a .env in the tools dir with the var APIKEY=your teams api key

source .env

solutions_path=../solutions

solutions=$(ls $solutions_path/*.json)

for solution in $solutions; do
    solution_id=$(basename --suffix=.json $solution)

    if [ -f  ${solutions_path}/${solution_id}.lastSent ]; then
        last_sent=$(($(date +%s) - $(date +%s -r ${solutions_path}/${solution_id}.lastSent)))
    else
        last_sent=999
    fi

    if [ ${last_sent} -ge 305 ]; then
        echo -e "submitting: ${solution}"
        curl https://poses.live/api/problems/${solution_id}/solutions \
        --request POST -g \
        -H "Accept: application/json" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer ${APIKEY}" \
        --data "$(cat ${solutions_path}/${solution})" && \
        touch ${solutions_path}/${solution_id}.lastSent
    else
        echo -e "\nHold your hourses there buddy!"
        echo "Too soon to submit: ${solution}"
        echo -e "Waite another $(( 305 - ${last_sent} )) seconds\n"
    fi
done
echo Done