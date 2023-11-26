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

if [ "$#" -ne 3 ]; then
	echo "sdrprev.sh [sdr-input] [timestamp] [output]"
	exit
fi

ffmpeg -ss "$2" -i "$1" -vf format=gray \
	-color_primaries bt709 -color_trc bt709 -colorspace bt709 \
	-vframes 1 "$3"
