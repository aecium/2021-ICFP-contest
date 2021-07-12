#!/bin/bash

#You will need to creat a .env in the tools dir with the var APIKEY=your teams api key

source .env
problem_number=1
problems_path=../problems

while true ; do 
    echo "grabing problem: ${problem_number}"
    problem=$(curl https://poses.live/api/problems/${problem_number} -H "Accept: application/json" -H "Authorization: Bearer ${APIKEY}")
    echo ${problem}
    if $(echo ${problem} | grep -q "hole"); then
        echo ${problem} > ${problems_path}/${problem_number}.json
    else
        echo "fail or hit the end of the problems" 
        exit 
    fi
    ((problem_number++))
    sleep 1
done
echo Done