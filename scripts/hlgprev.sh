#!/usr/bin/env bash

set -e

#
# hlgprev.sh [pq-input] [timestamp] [name]
#
# $1 - pq-input
# $2 - timestamp
# $3 - name
#

if [ "$#" -ne 3 ]; then
	echo "hlgprev.sh [pq-input] [timestamp] [name]"
	exit
fi

LUT="$(mktemp --suffix=.cube)"

pq2hlg "$LUT"

ffmpeg -ss "$2" -i "$1" -vf scale=1920:1080,format=rgb48le,lut3d="$LUT",format=gray,format=yuv420p \
	-color_primaries bt2020 -color_trc bt709 -colorspace bt2020nc \
	-vframes 1 "$3-$(echo $2 | sed 's/:/_/g').png"

rm -f "$LUT"
