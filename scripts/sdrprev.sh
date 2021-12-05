#!/usr/bin/env bash

#
# SPDX-FileCopyrightText: 2021 William Swartzendruber <wswartzendruber@gmail.com>
#
# SPDX-License-Identifier: CC0-1.0
#

set -e

#
# sdrprev.sh [sdr-input] [timestamp] [output]
#
# $1 - sdr-input
# $2 - timestamp
# $3 - output
#

if [ "$#" -ne 3 ]; then
	echo "sdrprev.sh [sdr-input] [timestamp] [output]"
	exit
fi

ffmpeg -ss "$2" -i "$1" -vf format=gray \
	-color_primaries bt709 -color_trc bt709 -colorspace bt709 \
	-vframes 1 "$3"
