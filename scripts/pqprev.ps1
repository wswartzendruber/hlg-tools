#
# Copyright 2022 William Swartzendruber
#
# To the extent possible under law, the person who associated CC0 with this file has waived all
# copyright and related or neighboring rights to this file.
#
# You should have received a copy of the CC0 legalcode along with this work. If not, see
# <http://creativecommons.org/publicdomain/zero/1.0/>.
#
# SPDX-License-Identifier: CC0-1.0
#

if ($args.count -ne 3) {
	Write-Output "pqprev.ps1 [pq-input] [timestamp] [output]"
	return
}

ffmpeg.exe -ss $args[1] -i $args[0] -vf format=gray,scale=1920:1080 `
	-color_primaries bt709 -color_trc bt709 -colorspace bt709 `
	-vframes 1 $args[2]
