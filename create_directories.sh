#!/bin/bash

for i in {1..25}
do
    DAY_STRING="${i}"
    if [ $i -lt 10 ]
    then
        DAY_STRING="0${i}"
    fi
    DIR="src/day$DAY_STRING"
    INPUT_FILE_PATH="src/day$DAY_STRING/input.txt"
    README_FILE_PATH="src/day$DAY_STRING/README.md"
    CURRENT_YEAR=`date +'%Y'`
    echo "Creating ${DIR}"
    mkdir -p $DIR
    echo "Creating ${INPUT_FILE_PATH}"
    touch $INPUT_FILE_PATH
    touch $README_FILE_PATH
    echo "# Day ${i} 

See the [problem here](https://adventofcode.com/${CURRENT_YEAR}/day/${i}) and copy the input into [input.txt](./input.txt)" > $README_FILE_PATH
done