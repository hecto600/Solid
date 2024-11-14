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
    let mut plane_vals = PlaneValues::new();

    for line in 0..pca.size {
        plane_vals.outer_idx = line;
        if line == 0 {
            plane_vals.outer_val += pca.incr - 1;
        } else {
            plane_vals.outer_val += pca.incr;
        }

        for columns in 0..pca.size {
            plane_vals.inner_idx = columns;
            if columns == 0 {
                plane_vals.inner_val += pca.incr - 1;
            } else {
                plane_vals.inner_val += pca.incr;
            }

            match axis {
                Axis::Red => {
                    red_axis(&mut plane, &plane_vals, pca.axis_val);
                }

                Axis::Green => {
                    green_axis(&mut plane, &plane_vals, pca.axis_val);
                }

                Axis::Blue => {
                    blue_axis(&mut plane, &plane_vals, pca.axis_val);
                }
            }
        }
        plane_vals.inner_val = 0
    }
    plane
}

fn red_axis(plane: &mut Vec<Vec<Color>>, pv: &PlaneValues, axis_val: usize) {
    plane[pv.outer_idx].push(Color::new(axis_val, pv.outer_val, pv.inner_val));
}
fn green_axis(plane: &mut Vec<Vec<Color>>, pv: &PlaneValues, axis_val: usize) {
    plane[pv.outer_idx].push(Color::new(pv.outer_val, axis_val, pv.inner_val));
}
fn blue_axis(plane: &mut Vec<Vec<Color>>, pv: &PlaneValues, axis_val: usize) {
    plane[pv.outer_idx].push(Color::new(pv.outer_val, pv.inner_val, axis_val));
}
