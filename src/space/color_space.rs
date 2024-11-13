use std::vec;

#[derive(Clone)]
pub struct Color {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
    pub hex_code: String,
}
impl Color {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        let hex_code = format!("#{:02X}{:02X}{:02X}", red, green, blue);
        Self {
            red,
            green,
            blue,
            hex_code,
        }
    }
}
#[derive(Clone)]
pub struct OuterPlanes {
    pub size: usize,
    pub gb: Vec<Vec<Color>>,
    pub rg: Vec<Vec<Color>>,
    pub cy: Vec<Vec<Color>>,
    pub cm: Vec<Vec<Color>>,
    pub my: Vec<Vec<Color>>,
    pub rb: Vec<Vec<Color>>,
}
impl OuterPlanes {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            gb: vec![vec![]; size],
            rg: vec![vec![]; size],
            cy: vec![vec![]; size],
            cm: vec![vec![]; size],
            my: vec![vec![]; size],
            rb: vec![vec![]; size],
        }
    }
}

pub struct ColorSpace {
    pub greyscale: Vec<Color>,
    pub color_pallete: Vec<Vec<Vec<Color>>>,
    pub size: usize,
    pub outer_planes: OuterPlanes,
}
