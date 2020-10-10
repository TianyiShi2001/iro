pub mod cmyk;
pub mod hsl;
pub mod rgb;

pub use cmyk::{Cmyk, CmykInt};
pub use hsl::{Hsl, HslInt};
pub use rgb::{Rgb, RgbScaled};

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
