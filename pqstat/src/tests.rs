/*
 * Copyright 2021 William Swartzendruber
 *
 * Any copyright is dedicated to the Public Domain.
 *
 * SPDX-License-Identifier: CC0-1.0
 */

use std::io::Cursor;
use super::*;
use byteorder::{LittleEndian, WriteBytesExt};

#[test]
fn test_frame_stats_black() {

    let mut frame = vec![0_u8; 0];

    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();

    let mut cursor = Cursor::new(frame);
    let max_channel = read_frame_max_channel(&mut cursor, 6).unwrap();

    assert_eq!(to_nits(max_channel), 0);
}

#[test]
fn test_frame_stats_red_rw() {

    let mut frame = vec![0_u8; 0];

    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x94A7).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();

    let mut cursor = Cursor::new(frame);
    let max_channel = read_frame_max_channel(&mut cursor, 4).unwrap();

    assert_eq!(to_nits(max_channel), 203);
}

#[test]
fn test_frame_stats_green_rw() {

    let mut frame = vec![0_u8; 0];

    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x94A7).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();

    let mut cursor = Cursor::new(frame);
    let max_channel = read_frame_max_channel(&mut cursor, 4).unwrap();

    assert_eq!(to_nits(max_channel), 203);
}

#[test]
fn test_frame_stats_blue_rw() {

    let mut frame = vec![0_u8; 0];

    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x94A7).unwrap();

    let mut cursor = Cursor::new(frame);
    let max_channel = read_frame_max_channel(&mut cursor, 4).unwrap();

    assert_eq!(to_nits(max_channel), 203);
}

#[test]
fn test_frame_stats_white_rw() {

    let mut frame = vec![0_u8; 0];

    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x94A7).unwrap();
    frame.write_u16::<LittleEndian>(0x94A7).unwrap();
    frame.write_u16::<LittleEndian>(0x94A7).unwrap();

    let mut cursor = Cursor::new(frame);
    let max_channel = read_frame_max_channel(&mut cursor, 4).unwrap();

    assert_eq!(to_nits(max_channel), 203);
}

#[test]
fn test_frame_stats_red_mw() {

    let mut frame = vec![0_u8; 0];

    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0xA000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0xD000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0xFFFF).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();

    let mut cursor = Cursor::new(frame);
    let max_channel = read_frame_max_channel(&mut cursor, 6).unwrap();

    assert_eq!(to_nits(max_channel), 10_000);
}

#[test]
fn test_frame_stats_green_mw() {

    let mut frame = vec![0_u8; 0];

    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0xA000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0xD000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0xFFFF).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();

    let mut cursor = Cursor::new(frame);
    let max_channel = read_frame_max_channel(&mut cursor, 6).unwrap();

    assert_eq!(to_nits(max_channel), 10_000);
}

#[test]
fn test_frame_stats_blue_mw() {

    let mut frame = vec![0_u8; 0];

    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0xA000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0xD000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0xFFFF).unwrap();

    let mut cursor = Cursor::new(frame);
    let max_channel = read_frame_max_channel(&mut cursor, 6).unwrap();

    assert_eq!(to_nits(max_channel), 10_000);
}

#[test]
fn test_frame_stats_white_mw() {

    let mut frame = vec![0_u8; 0];

    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x0000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x4000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0x8000).unwrap();
    frame.write_u16::<LittleEndian>(0xA000).unwrap();
    frame.write_u16::<LittleEndian>(0xA000).unwrap();
    frame.write_u16::<LittleEndian>(0xA000).unwrap();
    frame.write_u16::<LittleEndian>(0xD000).unwrap();
    frame.write_u16::<LittleEndian>(0xD000).unwrap();
    frame.write_u16::<LittleEndian>(0xD000).unwrap();
    frame.write_u16::<LittleEndian>(0xFFFF).unwrap();
    frame.write_u16::<LittleEndian>(0xFFFF).unwrap();
    frame.write_u16::<LittleEndian>(0xFFFF).unwrap();

    let mut cursor = Cursor::new(frame);
    let max_channel = read_frame_max_channel(&mut cursor, 6).unwrap();

    assert_eq!(to_nits(max_channel), 10_000);
}
