# iro — Representing Colors in Rust

[![crates.io](https://img.shields.io/crates/d/iro.svg)](https://crates.io/crates/iro)
[![crates.io](https://img.shields.io/crates/v/iro.svg)](https://crates.io/crates/iro)
[![crates.io](https://img.shields.io/crates/l/iro.svg)](https://crates.io/crates/iro)

## Goals

- Representation of colors in different [color spaces](https://en.wikipedia.org/wiki/Color_space)
- Conversion of colors between color spaces

and nothing else.

## Example

```rust
use iro::{Cmyk, CmykInt, Hsl, HslInt, Rgb};

use iro::{Cmyk, CmykInt, Hsl, HslInt, Rgb};

fn main() {
    let color = Rgb::new(85, 191, 100);
    println!("{:?}", &color);
    let color: Cmyk = color.into();
    println!("{:?}", &color);
    let color: CmykInt = color.into();
    println!("{:?}", (color.c, color.m, color.y, color.k));
    println!("{:?}", &color);
    let color = Hsl::from(Rgb::from(color));
    println!("{:?}", &color);
    let color: HslInt = color.into();
    println!("{:?}", &color);
}

// Rgb { r: 85, g: 191, b: 100 }
// Cmyk { c: 0.5549738, m: 0.0, y: 0.47643983, k: 0.25098038 }
// (55, 0, 48, 25)
// CmykInt { c: 55, m: 0, y: 48, k: 25 }
// Hsl { h: 127.42857, s: 0.45064378, l: 0.54313725 }
// HslInt { h: 127, s: 45, l: 54 }
```

## About the name

**iro** (色) is the Japanese word for "color".