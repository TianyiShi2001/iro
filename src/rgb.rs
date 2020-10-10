use crate::cmyk::*;
use std::convert::From;

pub(crate) const RGB_SCALE: u8 = 255;
pub(crate) const RGB_SCALE_FLOAT: f32 = 255.0;

#[derive(Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub(crate) struct RgbFloat {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<Rgb> for RgbFloat {
    fn from(rgb: Rgb) -> Self {
        Self {
            r: rgb.r as f32,
            g: rgb.g as f32,
            b: rgb.b as f32,
        }
    }
}

impl From<RgbFloat> for Rgb {
    fn from(rgb: RgbFloat) -> Self {
        Self {
            r: rgb.r as u8,
            g: rgb.g as u8,
            b: rgb.b as u8,
        }
    }
}

impl From<CmykFloat> for RgbFloat {
    fn from(cmyk: CmykFloat) -> Self {
        Self {
            r: RGB_SCALE_FLOAT
                * (1.0 - cmyk.c / CMYK_SCALE_FLOAT)
                * (1.0 - cmyk.k / CMYK_SCALE_FLOAT),
            g: RGB_SCALE_FLOAT
                * (1.0 - cmyk.m / CMYK_SCALE_FLOAT)
                * (1.0 - cmyk.k / CMYK_SCALE_FLOAT),
            b: RGB_SCALE_FLOAT
                * (1.0 - cmyk.y / CMYK_SCALE_FLOAT)
                * (1.0 - cmyk.k / CMYK_SCALE_FLOAT),
        }
    }
}

impl From<Cmyk> for Rgb {
    fn from(cmyk: Cmyk) -> Self {
        Self::from(RgbFloat::from(CmykFloat::from(cmyk)))
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(Cmyk {c : 58, m: 0, y: 47, k: 25} => Rgb {r: 80, g: 191, b: 101} ; "no_m")]
    #[test_case(Cmyk {c : 0, m: 0, y: 0, k: 0} => Rgb {r: 255, g: 255, b: 255}; "white")]
    #[test_case( Cmyk {c : 0, m: 0, y: 0, k: 100} => Rgb {r: 0, g: 0, b: 0}; "black")]
    fn test_cmyk_to_rgb(cmyk: Cmyk) -> Rgb {
        cmyk.into()
    }
}
