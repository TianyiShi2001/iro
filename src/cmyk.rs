use std::convert::From;

use min_max::*;

use crate::rgb::*;

#[derive(Debug, PartialEq)]
pub struct Cmyk {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
}

impl Cmyk {
    fn new(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self { c, m, y, k }
    }
}

impl From<RgbScaled> for Cmyk {
    fn from(rgb: RgbScaled) -> Self {
        let mut c = 1.0 - rgb.r;
        let mut m = 1.0 - rgb.g;
        let mut y = 1.0 - rgb.b;
        let k = min_partial!(c, m, y);
        c = (c - k) / (1.0 - k);
        m = (m - k) / (1.0 - k);
        y = (y - k) / (1.0 - k);
        Self { c, m, y, k }
    }
}

impl From<Rgb> for Cmyk {
    fn from(rgb: Rgb) -> Self {
        Self::from(RgbScaled::from(rgb))
    }
}

/// Integer representation of CMYK color space.
///
/// Each of `c`, `m`, `y` and `k` is an `u8` between 0 and 255 inclusive.
#[derive(Debug, PartialEq)]
pub struct CmykInt {
    pub c: u8,
    pub m: u8,
    pub y: u8,
    pub k: u8,
}

impl CmykInt {
    fn new(c: u8, m: u8, y: u8, k: u8) -> Self {
        Self { c, m, y, k }
    }
}

impl From<Cmyk> for CmykInt {
    fn from(cmyk: Cmyk) -> CmykInt {
        Self {
            c: (cmyk.c * 100.0).round() as u8,
            m: (cmyk.m * 100.0).round() as u8,
            y: (cmyk.y * 100.0).round() as u8,
            k: (cmyk.k * 100.0).round() as u8,
        }
    }
}

impl From<CmykInt> for Cmyk {
    fn from(cmyk: CmykInt) -> Cmyk {
        Self {
            c: cmyk.c as f32 / 100.,
            m: cmyk.m as f32 / 100.,
            y: cmyk.y as f32 / 100.,
            k: cmyk.k as f32 / 100.,
        }
    }
}
