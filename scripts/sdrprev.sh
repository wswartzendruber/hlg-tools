#!/usr/bin/env bash

#
# Copyright 2024 William Swartzendruber
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

if [ "$#" -ne 3 ]; then
	echo "sdrprev.sh [sdr-input] [timestamp] [output]"
	exit
fi

LUT="$(mktemp --suffix=.cube)"

mono709 --size 64 "$LUT"

ffmpeg -ss "$2" -i "$1" -vf format=rgb48le,lut3d="$LUT",format=yuv420p \
	-vframes 1 "$3"

rm -f "$LUT"
