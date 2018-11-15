#!/bin/env zsh

DAY=01
time -p cargo run --release -- $DAY < inputs/day$DAY.txt
