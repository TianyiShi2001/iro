use iro::{Cmyk, CmykInt, Hsl, HslInt, Rgb};

fn main() {
    let color = Rgb::new(85, 191, 100);
    println!("{:?}", &color);
    let color: Cmyk = color.into();
    println!("{:?}", &color);
    let color: CmykInt = color.into();
    println!("{:?}", &color);
    let color = Hsl::from(Rgb::from(color));
    println!("{:?}", &color);
    let color: HslInt = color.into();
    println!("{:?}", &color);
}
