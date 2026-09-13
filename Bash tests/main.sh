#!/usr/bin/env bash

if [ $# -lt 1 ]; then
  echo "Need to be like that: 'alias=\"command with its arguments\"'"
  exit 1
fi

value="$1"

if [ "$value" =  ]; then
  echo "value 1 test string output"


elif [ "$value" = 2 ]; then
  echo "value 2 test string output"


elif [ "$value" = 3 ]; then
  echo "value 3 test string output"


else 
  echo "Only possible choices are 1-3"
  exit 1
fi