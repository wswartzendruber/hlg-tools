#!/usr/bin/env bash

#
# Copyright 2022 William Swartzendruber
#
# Any copyright is dedicated to the Public Domain.
#
# SPDX-License-Identifier: CC0-1.0
#

set -e

if [ "$#" -ne 3 ]; then
	echo "sdrprev.sh [sdr-input] [timestamp] [output]"
	exit
fi

ffmpeg -ss "$2" -i "$1" -vf format=gray \
	-color_primaries bt709 -color_trc bt709 -colorspace bt709 \
	-vframes 1 "$3"
