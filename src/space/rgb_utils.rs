use super::color_space::{Color, OuterPlanes};
#[derive(Debug)]
pub enum Axis {
    Red,
    Green,
    Blue,
}
struct PlaneColorizeArgs {
    incr: usize,
    size: usize,
    axis_val: usize,
}

struct PlaneValues {
    outer_idx: usize,
    outer_val: usize,
    inner_idx: usize,
    inner_val: usize,
}
impl PlaneValues {
    fn new() -> Self {
        Self {
            outer_idx: 0,
            outer_val: 0,
            inner_idx: 0,
            inner_val: 0,
        }
    }
}
pub fn create_planes_colors(all_planes: &mut OuterPlanes, incr: usize) {
    let size = all_planes.size;
    let red_planes: Vec<Vec<Vec<Color>>>;
    let green_planes: Vec<Vec<Vec<Color>>>;
    let blue_planes: Vec<Vec<Vec<Color>>>;

    red_planes = planes_of_axis(incr, size, Axis::Red);
    green_planes = planes_of_axis(incr, size, Axis::Green);
    blue_planes = planes_of_axis(incr, size, Axis::Blue);

    all_planes.gb = red_planes[0].clone();
    all_planes.my = red_planes[1].clone();
    all_planes.rb = green_planes[0].clone();
    all_planes.cy = green_planes[1].clone();
    all_planes.rg = blue_planes[0].clone();
    all_planes.cm = blue_planes[1].clone();
}
pub fn planes_of_axis(incr: usize, size: usize, axis: Axis) -> Vec<Vec<Vec<Color>>> {
    let mut planes: Vec<Vec<Vec<Color>>> = vec![vec![vec![]; size]; 2];

    let mut pca: PlaneColorizeArgs = PlaneColorizeArgs {
        incr,
        size,
        axis_val: 0,
    };

    planes[0] = plane_colorize(&pca, &axis);
    pca.axis_val = 255;
    planes[1] = plane_colorize(&pca, &axis);

    planes
}

fn plane_colorize(pca: &PlaneColorizeArgs, axis: &Axis) -> Vec<Vec<Color>> {
    let mut plane: Vec<Vec<Color>> = vec![vec![]; pca.size];
    let mut vw = PlaneValues::new();

    for i in 0..pca.size {
        vw.outer_idx = i;
        if i == 0 {
            vw.outer_val += pca.incr - 1;
        } else {
            vw.outer_val += pca.incr;
        }

        for j in 0..pca.size {
            vw.inner_idx = j;
            if j == 0 {
                vw.inner_val += pca.incr - 1;
            } else {
                vw.inner_val += pca.incr;
            }

            match axis {
                Axis::Red => {
                    red_axis(&mut plane, &vw, pca.axis_val);
                }

                Axis::Green => {
                    green_axis(&mut plane, &vw, pca.axis_val);
                }

                Axis::Blue => {
                    blue_axis(&mut plane, &vw, pca.axis_val);
                }
            }
        }
        vw.inner_val = 0
    }
    plane
}

fn red_axis(plane: &mut Vec<Vec<Color>>, vw: &PlaneValues, axis_val: usize) {
    plane[vw.outer_idx].push(Color::new(axis_val, vw.outer_val, vw.inner_val));
}
fn green_axis(plane: &mut Vec<Vec<Color>>, vw: &PlaneValues, axis_val: usize) {
    plane[vw.outer_idx].push(Color::new(vw.outer_val, axis_val, vw.inner_val));
}
fn blue_axis(plane: &mut Vec<Vec<Color>>, vw: &PlaneValues, axis_val: usize) {
    plane[vw.outer_idx].push(Color::new(vw.outer_val, vw.inner_val, axis_val));
}
