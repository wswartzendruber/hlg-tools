#
# Copyright 2022 William Swartzendruber
#
# Any copyright is dedicated to the Public Domain.
#
# SPDX-License-Identifier: CC0-1.0
#

if ($args.count -ne 3) {
	Write-Output "sdrprev.sh [sdr-input] [timestamp] [output]"
	return
}

ffmpeg -ss $args[1] -i $args[0] -vf format=gray `
	-color_primaries bt709 -color_trc bt709 -colorspace bt709 `
	-vframes 1 $args[2]
