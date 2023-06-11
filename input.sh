#!/bin/sh

YEAR=2015
DAY=$(printf "%00d" "$1")

# Check if day is set
if [ "$DAY" = "0" ]; then
	echo "Usage: $0 <day>"
	exit 1
fi

# Check if cached
if [ -f "/tmp/AoC-$YEAR-$DAY.txt" ]; then
	cat "/tmp/AoC-$YEAR-$DAY.txt"
	exit 0
fi

# Check if session is set
if [ -z "$SESSION" ]; then
	echo "SESSION is not set in .env"
	exit 1
fi

# Download and cache
curl -sSL "https://adventofcode.com/$YEAR/day/$DAY/input" -H "Cookie: session=$SESSION" | tee "/tmp/AoC-$YEAR-$DAY.txt"
