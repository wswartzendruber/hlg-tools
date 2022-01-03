#!/usr/bin/env bash

#
# Any copyright is dedicated to the Public Domain.
#
# Copyright 2022 William Swartzendruber
#
# SPDX-License-Identifier: CC0-1.0
#

set -e

if [ "$#" -ne 5 ]; then
	echo "hlgprev.sh [pq-input] [max-cll] [lum-scale] [timestamp] [output]"
	exit
fi

LUT="$(mktemp --suffix=.cube)"

pq2hlg --preview --max-cll "$2" --lum-scale "$3" --size 64 "$LUT"

ffmpeg -ss "$4" -i "$1" -vf scale=1920:1080,format=rgb48le,lut3d="$LUT",format=yuv420p \
	-color_primaries bt709 -color_trc bt709 -colorspace bt709 \
	-vframes 1 "$5"

rm -f "$LUT"
