# Introduction

This tool suite consists of the basics needed to generate 3D LUTs that will convert video
streams from Perceptual Quantizer (PQ) HDR into Hybrid-Log Gamma (HLG) HDR. This is intended
to facilitate individuals in converting their physical HDR10 video libraries into HLG, thereby
allowing more universal viewing on devices including SDR displays.

Most of the work here is based on ITU-R BT.2390.

Note that nothing here addresses the topic of decrypting physical media.

# Rationale

## PQ Weaknesses

While it is true that PQ offers a rather large amount of dynamic range, the format has two main
weaknesses:

- PQ video streams **are** mastered for a specific viewing environment.
- PQ video streams **cannot** be reliably rendered on SDR displays with good results.

Regarding the first point above, PQ video streams are typically mastered for a viewing
environment of just 5 nits. Any increase in ambient lighting causes the displayed image to
appear darker than originally intended. This inherently limits what environments a PQ video can
be watched in. Another way to look at this is that the viewer is expected to accommodate what is
being shown.

Regarding the second point above, PQ is inherently difficult here because nothing about the
signal data indicates how SDR downconversion should be handled. This is necessary because
applying the PQ gamma curve without processing will produce a picture that is simply too dim.
Static HDR10 metadata cannot help us here, either, because what we really need for SDR
downconversion is the reference white level, but that is not signaled.

## HLG Strengths

In light of these two issues, the United Kingdom's BBC and Japan's NHK cooperated to create an
entirely new HDR format. This resulted in HLG which has two main strengths:

- HLG video streams **are not** mastered for a specific viewing environment.
- HLG video streams **can** be reliably rendered on SDR displays with good results.

Regarding the first point above, HLG video streams are mastered in relative brightness. This
means that instead of mastering the signal for a fixed viewing environment, HLG streams contain
more generic signal data that the display then alters according to its settings. Ergo, if the
viewing environment changes, the display can simply be adjusted and the same video can then be
viewed again with these new settings.

Regarding the second point above, HLG is inherently easy here because its gamma curve was
designed for it. That is, a HLG video signal can be naively displayed on a SDR device with more
acceptable results. However, since HLG *does* define a fixed reference white level, HLG-aware
players can modify the picture accordingly. The end result is that given the combination of a
correctly mastered (or converted) HLG stream and a HLG-aware player, it can be extremely
difficult to tell that the picture wasn't natively mastered for SDR. Some players are also good
at downconverting BT.2020 color to BT.709 color as a part of this process. MPV is one such
player.

# How It Works

The tooling here works to facilitate the following procedure:

- Gather color data on the PQ stream.
- Determine its reference white level.
- Adjust the brightness to bring reference white to 203 nits.
- Tone map the PQ stream to be within HLG's dynamic range.
- Apply the PQ-to-HLG conversion algorithm.

First, color data needs to be gathered on the PQ stream. The one and only thing we really care
about here is what the tooling refers to as `max-channel`. This is the highest ratio of linear
brightness between all three color channels. A value of `0.0` represents pitch black while a
value of `1.0` represents full brightness. Effectively, `max-channel` is the value of the
brightest pixel's highest color channel in linear space. The `pqstat` utility provides this
information.

Second, the reference white level needs to be determined. This is not as important if the HLG
output is going to be viewed on HDR displays only, but it is absolutely critical for SDR
compatibility. If `ref-white` is set too low, the SDR downconversion will appear too bright. If
`ref-white` is set too high, the SDR downconversion will appear too dim. This is the only part
of the process that requires human judgment and will be covered in detail below. This step is
unfortunately necessary because PQ content, especially from 4K UltraHD Blu-rays, tends to be
anywhere between 100 and 203 nits.

Third, the linear brightness of the PQ stream needs to be adjusted so that `ref-white` sits at
203 nits. The `max-channel` value is also going to be adjusted accordingly, even preserving
values higher than `1.0`. This step is handled by each 3D LUT that `pq2hlg` generates.

Fourth, each color channel will be tone mapped such that:

- All red subpixels fall within 262.7 nits.
- All green subpixels fall within 678 nits.
- All blue subpixels fall within 59.3 nits.

This will permit a pure white pixel to hit exactly 1,000 nits in accordance with BT.2390. This
step is also covered by the 3D LUT.

Fifth, the video signal finally gets converted from PQ to HLG. This is the last step convered by
the 3D LUT and the conclusion to the process.
