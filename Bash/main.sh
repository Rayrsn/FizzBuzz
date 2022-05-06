#!/bin/bash
read -p "How many numbers to display: " numcount
re='^[0-9]+$'

if ! [[ $numcount =~ $re ]] ; then
   echo "error: Not a number" >&2; exit 1
fi

for (( i=1; i<=$numcount; i++ )); do
    if [ $(($i%3)) -eq 0 ] && [ $(($i%5)) -eq 0 ] ; then
        echo "FizzBuzz"
    elif [ $(($i%3)) -eq 0 ] ; then
        echo "Fizz"
    elif [ $(($i%5)) -eq 0 ] ; then
        echo "Buzz"
    else
        echo $i
    fi
done