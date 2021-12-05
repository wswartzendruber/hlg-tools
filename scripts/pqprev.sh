#!/usr/bin/env bash

#
# SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
#
# SPDX-License-Identifier: CC0-1.0
#

set -e

#
# pqprev.sh [pq-input] [timestamp] [output]
#
# $1 - pq-input
# $2 - timestamp
# $3 - output
#

if [ "$#" -ne 3 ]; then
	echo "pqprev.sh [pq-input] [timestamp] [output]"
	exit
fi

ffmpeg -ss "$2" -i "$1" -vf scale=1920:1080,format=rgb48le,lut3d="$LUT",format=gray,format=yuv420p \
	-color_primaries bt2020 -color_trc bt709 -colorspace bt2020nc \
	-vframes 1 "$3"
