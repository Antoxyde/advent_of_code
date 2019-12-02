#!/bin/bash

for day in $(seq 0 25); do

	if ! [ -d "day_${day}" ]; then
		mkdir "day_${day}"
		sed "s/REPLACE/$day/g" skeleton.rs > "day_${day}/day_${day}_part1.rs"
	else
		echo "$day already done"
	fi

done

