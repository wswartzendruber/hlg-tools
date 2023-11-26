#!/usr/bin/env bash

#
# Copyright 2023 William Swartzendruber
#
# To the extent possible under law, the person who associated CC0 with this file has waived all
# copyright and related or neighboring rights to this file.
#
# You should have received a copy of the CC0 legalcode along with this work. If not, see
# <http://creativecommons.org/publicdomain/zero/1.0/>.
#
# SPDX-License-Identifier: CC0-1.0
#

set -e

if [ "$#" -ne 5 ]; then
	echo "hlgprev.sh [pq-input] [max-cll] [exposure] [timestamp] [output]"
	exit
fi

LUT="$(mktemp --suffix=.cube)"

pq2hlg --preview --max-cll "$2" --exposure "$3" --size 64 "$LUT"

ffmpeg -ss "$4" -i "$1" -vf scale=1920:1080,format=rgb48le,lut3d="$LUT",format=yuv420p \
	-color_primaries bt709 -color_trc bt709 -colorspace bt709 \
	-vframes 1 "$5"

rm -f "$LUT"
