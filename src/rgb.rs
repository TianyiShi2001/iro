use crate::cmyk::*;
use crate::hsl::*;
use min_max::*;
use std::convert::From;

pub(crate) const RGB_SCALE: u8 = 255;
pub(crate) const RGB_SCALE_FLOAT: f32 = 255.0;

#[derive(Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, PartialEq)]
pub struct RgbScaled {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<Rgb> for RgbScaled {
    fn from(rgb: Rgb) -> Self {
        Self {
            r: rgb.r as f32 / RGB_SCALE_FLOAT,
            g: rgb.g as f32 / RGB_SCALE_FLOAT,
            b: rgb.b as f32 / RGB_SCALE_FLOAT,
        }
    }
}

impl From<RgbScaled> for Rgb {
    fn from(rgb: RgbScaled) -> Self {
        Self {
            r: (rgb.r * 255.0) as u8,
            g: (rgb.g * 255.0) as u8,
            b: (rgb.b * 255.0) as u8,
        }
    }
}

impl From<Cmyk> for RgbScaled {
    fn from(cmyk: Cmyk) -> Self {
        Self {
            r: (1.0 - cmyk.c) * (1.0 - cmyk.k),
            g: (1.0 - cmyk.m) * (1.0 - cmyk.k),
            b: (1.0 - cmyk.y) * (1.0 - cmyk.k),
        }
    }
}

impl From<Cmyk> for Rgb {
    fn from(cmyk: Cmyk) -> Self {
        Self::from(RgbScaled::from(cmyk))
    }
}

impl From<CmykInt> for Rgb {
    fn from(cmyk: CmykInt) -> Self {
        Self::from(RgbScaled::from(Cmyk::from(cmyk)))
    }
}

impl From<Hsl> for RgbScaled {
    // https://stackoverflow.com/questions/36721830/convert-hsl-to-rgb-and-hex/54014428#54014428
    fn from(hsl: Hsl) -> Self {
        fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
            if t < 0. {
                t += 1.
            } else if t > 1. {
                t -= 1.
            }
            if t < 1. / 6. {
                p + (q - p) * 6. * t
            } else if t < 1. / 2. {
                q
            } else if t < 2. / 3. {
                p + (q - p) * (2. / 3. - t) * 6.
            } else {
                p
            }
        }

        let Hsl { mut h, s, l } = hsl;

        h /= 360.0;

        if s < 1e-3 {
            Self { r: l, g: l, b: l }
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            Self {
                r: hue_to_rgb(p, q, h + 1. / 3.),
                g: hue_to_rgb(p, q, h),
                b: hue_to_rgb(p, q, h - 1. / 3.),
            }
        }
    }
}

impl From<Hsl> for Rgb {
    fn from(hsl: Hsl) -> Self {
        Self::from(RgbScaled::from(hsl))
    }
}

impl From<HslInt> for Rgb {
    fn from(hsl: HslInt) -> Self {
        Self::from(RgbScaled::from(Hsl::from(hsl)))
    }
}
