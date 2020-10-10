use std::convert::From;

use min_max::*;

use crate::rgb::*;

pub const CMYK_SCALE: u8 = 100;
pub(crate) const CMYK_SCALE_FLOAT: f32 = 100.0;

#[derive(Debug, PartialEq, Eq)]
pub struct Cmyk {
    pub c: u8,
    pub m: u8,
    pub y: u8,
    pub k: u8,
}

pub(crate) struct CmykFloat {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
}

impl From<Cmyk> for CmykFloat {
    fn from(cmyk: Cmyk) -> Self {
        Self {
            c: cmyk.c as f32,
            m: cmyk.m as f32,
            y: cmyk.y as f32,
            k: cmyk.k as f32,
        }
    }
}

impl From<CmykFloat> for Cmyk {
    fn from(cmyk: CmykFloat) -> Self {
        Self {
            c: cmyk.c as u8,
            m: cmyk.m as u8,
            y: cmyk.y as u8,
            k: cmyk.k as u8,
        }
    }
}

impl From<RgbFloat> for CmykFloat {
    fn from(rgb: RgbFloat) -> Self {
        let mut c = 1.0 - rgb.r / RGB_SCALE_FLOAT;
        let mut m = 1.0 - rgb.g / RGB_SCALE_FLOAT;
        let mut y = 1.0 - rgb.b / RGB_SCALE_FLOAT;
        let k = min_partial!(c, m, y);
        c = (c - k) / (1.0 - k);
        m = (m - k) / (1.0 - k);
        y = (y - k) / (1.0 - k);
        Self {
            c: c * CMYK_SCALE_FLOAT,
            m: m * CMYK_SCALE_FLOAT,
            y: y * CMYK_SCALE_FLOAT,
            k: k * CMYK_SCALE_FLOAT,
        }
    }
}

impl From<Rgb> for Cmyk {
    fn from(rgb: Rgb) -> Self {
        Self::from(CmykFloat::from(RgbFloat::from(rgb)))
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(Rgb {r: 80, g: 191, b: 100} => Cmyk {c : 58, m: 0, y: 47, k: 25}; "no_m")]
    #[test_case(Rgb {r: 255, g: 255, b: 255} => Cmyk {c : 0, m: 0, y: 0, k: 0}; "white")]
    #[test_case(Rgb {r: 0, g: 0, b: 0} => Cmyk {c : 0, m: 0, y: 0, k: 100}; "black")]
    fn test_rgb_to_cmyk(rgb: Rgb) -> Cmyk {
        rgb.into()
    }
}
