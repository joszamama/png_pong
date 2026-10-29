use std::io::Cursor;

use png_pong::{Decoder, PngRaster};

fn decode_gray8(file: Vec<u8>) -> Vec<u8> {
    let mut decoder = Decoder::new(Cursor::new(file))
        .expect("Not PNG")
        .into_steps();

    match decoder.next().unwrap().unwrap().raster {
        PngRaster::Gray8(raster) => raster.as_u8_slice().to_vec(),
        _ => panic!("Not decoded as Gray8"),
    }
}

/// Rows of PngSuite basn0g01.png, most significant bit first
const BASN0G01: [u32; 32] = [
    0xFFFFFFFE, 0xFFFFFFFC, 0xFFFFFFF8, 0xFFFFFFF0, 0xF3F3FFE0, 0xF3F3FFC0,
    0xF3F3FF80, 0xF333FF00, 0xF333FE00, 0xF333FC00, 0xF807F800, 0xF807F000,
    0xFCCFE000, 0xFCCFC000, 0xFFFF8000, 0xFFFF0000, 0xFFFE0000, 0xFFFC0000,
    0xFFF80FE0, 0xFFF00FE0, 0xFFE00C30, 0xFFC00C30, 0xFF800FE0, 0xFF000FE0,
    0xFE000C30, 0xFC000C30, 0xF8000FE0, 0xF0000FE0, 0xE0000000, 0xC0000000,
    0x80000000, 0x00000000,
];

#[test]
fn gray1() {
    let file = std::fs::read("tests/pngsuite-basic/basn0g01.png").unwrap();
    let pixels = decode_gray8(file);

    assert_eq!(pixels.len(), 32 * 32);
    for (i, pixel) in pixels.into_iter().enumerate() {
        let (x, y) = (i % 32, i / 32);
        let bit = (BASN0G01[y] >> (31 - x)) & 1;
        assert_eq!(pixel, bit as u8 * 255, "x = {x}, y = {y}");
    }
}

#[test]
fn gray2() {
    let file = std::fs::read("tests/pngsuite-basic/basn0g02.png").unwrap();
    let pixels = decode_gray8(file);

    assert_eq!(pixels.len(), 32 * 32);
    for (i, pixel) in pixels.into_iter().enumerate() {
        let (x, y) = (i % 32, i / 32);
        let level = (x / 4 + y / 4) % 4;
        assert_eq!(pixel, level as u8 * 85, "x = {x}, y = {y}");
    }
}

#[test]
fn gray4() {
    let file = std::fs::read("tests/pngsuite-basic/basn0g04.png").unwrap();
    let pixels = decode_gray8(file);

    assert_eq!(pixels.len(), 32 * 32);
    for (i, pixel) in pixels.into_iter().enumerate() {
        let (x, y) = (i % 32, i / 32);
        let level = (x / 4 + y / 4) % 16;
        assert_eq!(pixel, level as u8 * 17, "x = {x}, y = {y}");
    }
}

#[test]
fn gray1_partial_byte() {
    // 1x1 greyscale image at bit depth 1, scanline byte 0x80
    let file = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
        0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x37, 0x6E, 0xF9, 0x24, 0x00, 0x00, 0x00,
        0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x68, 0x00, 0x00, 0x00,
        0x82, 0x00, 0x81, 0x77, 0xCD, 0x72, 0xB6, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    assert_eq!(decode_gray8(file), [255]);
}
