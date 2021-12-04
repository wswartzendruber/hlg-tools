#!/usr/bin/env bash

set -e

#
# sdrprev.sh [sdr-input] [timestamp] [name]
#
# $1 - sdr-input
# $2 - timestamp
# $3 - name
#

if [ "$#" -ne 3 ]; then
	echo "sdrprev.sh [sdr-input] [timestamp] [name]"
	exit
fi

ffmpeg -ss "$2" -i "$1" -vf format=gray \
	-color_primaries bt709 -color_trc bt709 -colorspace bt709 \
	-vframes 1 "$3-$(echo $2 | sed 's/:/_/g').png"

rm -f "$LUT"
