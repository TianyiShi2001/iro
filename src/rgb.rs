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

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(Rgb {r: 80, g: 191, b: 100} => CmykInt {c : 58, m: 0, y: 48, k: 25}; "no_m")]
    #[test_case(Rgb {r: 255, g: 255, b: 255} => CmykInt {c : 0, m: 0, y: 0, k: 0}; "white")]
    #[test_case(Rgb {r: 0, g: 0, b: 0} => CmykInt {c : 0, m: 0, y: 0, k: 100}; "black")]
    fn test_rgb_to_cmyk(rgb: Rgb) -> CmykInt {
        CmykInt::from(Cmyk::from(rgb))
    }

    #[test_case(Cmyk {c : 0.58, m: 0.0, y: 0.47, k: 0.25} => Rgb {r: 80, g: 191, b: 101} ; "no_m")]
    #[test_case(Cmyk {c : 0.0, m: 0.0, y: 0.0, k: 0.0} => Rgb {r: 255, g: 255, b: 255}; "white")]
    #[test_case( Cmyk {c : 0.0, m: 0.0, y: 0.0, k: 1.0} => Rgb {r: 0, g: 0, b: 0}; "black")]
    fn test_cmyk_to_rgb(cmyk: Cmyk) -> Rgb {
        cmyk.into()
    }

    #[test_case(Hsl {h: 131.0, s: 0.46, l: 0.53} => Rgb {r: 80, g: 190, b: 100} ; "1")]
    #[test_case(Hsl {h: 0.0, s: 0.0, l: 1.0} => Rgb {r: 255, g: 255, b: 255}; "white")]
    #[test_case( Hsl {h: 0.0, s: 0.0, l: 0.0} => Rgb {r: 0, g: 0, b: 0}; "black")]
    fn test_hsl_to_rgb(hsl: Hsl) -> Rgb {
        hsl.into()
    }

    #[test_case(Rgb {r: 80, g: 190, b: 100} => HslInt {h: 131, s: 46, l: 53}  ; "11")]
    #[test_case(Rgb {r: 255, g: 255, b: 255} => HslInt {h: 0, s: 0, l: 100} ; "white")]
    #[test_case(Rgb {r: 0, g: 0, b: 0} => HslInt {h: 0, s: 0, l: 0}; "black")]
    fn test_rgb_to_hsl(rgb: Rgb) -> HslInt {
        HslInt::from(Hsl::from(rgb))
    }
}
