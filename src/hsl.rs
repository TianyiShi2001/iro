//! # HSL
//!
//! # References
//!
//! - Conversion with RGB: https://stackoverflow.com/questions/39118528/rgb-to-hsl-conversion

use std::convert::From;

use min_max::*;

use crate::rgb::*;

#[derive(Debug, PartialEq)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

/// Integer representation of CMYK color space.
///
/// `h` is a `16` between 0 and 360, and `s` and `l` are a `u8` between 0 and 255.
#[derive(Debug, PartialEq)]
pub struct HslInt {
    pub h: u16,
    pub s: u8,
    pub l: u8,
}

impl From<Hsl> for HslInt {
    fn from(hsl: Hsl) -> Self {
        Self {
            h: hsl.h.round() as u16,
            s: (hsl.s * 100.0).round() as u8,
            l: (hsl.l * 100.0).round() as u8,
        }
    }
}

impl From<HslInt> for Hsl {
    fn from(hsl: HslInt) -> Self {
        Self {
            h: hsl.h as f32,
            s: hsl.s as f32 / 100.,
            l: hsl.l as f32 / 100.,
        }
    }
}

impl From<RgbScaled> for Hsl {
    fn from(rgb: RgbScaled) -> Self {
        let RgbScaled { r, g, b } = rgb;
        let (min, max) = (min_partial!(r, g, b), max_partial!(r, g, b));
        let chroma = max - min;
        let hue = if chroma < 1e-3 {
            0f32
        } else {
            60f32
                * if (max - r).abs() < 1e-3 {
                    let segment = (g - b) / chroma;
                    let mut shift = 0 / 60;
                    if segment < 0.0 {
                        shift = 360 / 60
                    }
                    segment + shift as f32
                // ((g - b) / chroma) % 6 ???
                } else if (max - g).abs() < 1e-3 {
                    let segment = (b - r) / chroma;
                    let shift = 120 / 60;
                    segment + shift as f32
                } else if (max - b).abs() < 1e-3 {
                    let segment = (r - g) / chroma;
                    let shift = 240 / 60;
                    segment + shift as f32
                } else {
                    unreachable!()
                }
        };
        let lightness = (min + max) / 2.0;
        let saturation = if chroma < 1e-3 {
            chroma
        } else {
            chroma / (1.0 - (2.0 * lightness - 1.0))
        };
        Self {
            h: hue,
            s: saturation,
            l: lightness,
        }
    }
}

impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Self {
        Self::from(Hsl::from(RgbScaled::from(rgb)))
    }
}

// // expects R, G, B, Cmin, Cmax and chroma to be in number interval [0, 1]
// // type is by default 'bi-hexcone' equation
// // set 'luma601' or 'luma709' for alternatives
// // see: https://en.wikipedia.org/wiki/Luma_(video)
// // returns a number interval [0, 1]
// function lightness(R, G, B, Cmin, Cmax, type = "bi-hexcone") {
//     if (type === "luma601") {
//       return 0.299 * R + 0.587 * G + 0.114 * B;
//     }
//     if (type === "luma709") {
//       return 0.2126 * R + 0.7152 * G + 0.0772 * B;
//     }
//     return average(Cmin, Cmax);
//   }
