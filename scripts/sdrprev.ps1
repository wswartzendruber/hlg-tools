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

if ($args.count -ne 3) {
	Write-Output "sdrprev.ps1 [sdr-input] [timestamp] [output]"
	return
}

$lut = "sdrprev-lut-temp.cube"

mono709.exe --size 64 $lut

ffmpeg.exe -ss $args[1] -i $args[0] -vf format=rgb48le,lut3d=$lut,format=yuv420p `
	-vframes 1 $args[2]

Remove-Item $lut
