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

if ($args.count -ne 5) {
	Write-Output "hlgprev.ps1 [pq-input] [max-cll] [lum-scale] [timestamp] [output]"
	return
}

$lut = "hlgprev-lut-temp.cube"

pq2hlg.exe --preview --max-cll $args[1] --lum-scale $args[2] --size 64 $lut

ffmpeg.exe -ss $args[3] -i $args[0] -vf scale=1920:1080,format=rgb48le,lut3d=$lut,format=yuv420p `
	-color_primaries bt709 -color_trc bt709 -colorspace bt709 `
	-vframes 1 $args[4]

Remove-Item $lut
