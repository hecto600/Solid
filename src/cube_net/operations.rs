use crate::space::color_space::Color;

use super::nets::Plane;

pub fn plane_flip_v(plane: &mut Vec<Vec<Color>>) {
    plane.reverse();
}
pub fn plane_flip_h(plane: &mut Vec<Vec<Color>>) {
    for column in plane.iter_mut() {
        column.reverse();
    }
}
pub fn plane_flip_v_and_h(plane: &mut Vec<Vec<Color>>) {
    plane.reverse();
    for column in plane.iter_mut() {
        column.reverse();
    }
}
pub fn plane_transpose(plane: &mut Vec<Vec<Color>>) {
    let plane_size = plane.len();
    let mut transposed = vec![vec![plane[0][0].clone(); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            transposed[column][line] = plane[line][column].clone();
        }
    }
    *plane = transposed;
}
pub fn plane_transpose_reversed(plane: &mut Vec<Vec<Color>>) {
    let plane_size = plane.len();
    let mut transposed = vec![vec![plane[0][0].clone(); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            transposed[column][line] =
                plane[plane_size - line - 1][plane_size - column - 1].clone();
        }
    }
    *plane = transposed;
}

pub fn plane_transpose_minor_diagonal(plane: &mut Vec<Vec<Color>>) {
    let plane_size = plane.len();
    let mut transposed = vec![vec![plane[0][0].clone(); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            transposed[column][line] =
                plane[plane_size - column - 1][plane_size - line - 1].clone();
        }
    }
    *plane = transposed;
}

pub fn plane_rotate_90(plane: &mut Vec<Vec<Color>>) {
    let plane_size = plane.len();
    let mut rotated = vec![vec![plane[0][0].clone(); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            rotated[line][column] = plane[plane_size - column - 1][line].clone();
        }
    }
    *plane = rotated;
}
pub fn plane_rotate_90ccw(plane: &mut Vec<Vec<Color>>) {
    let plane_size = plane.len();
    let mut rotated = vec![vec![plane[0][0].clone(); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            rotated[line][column] = plane[column][plane_size - line - 1].clone();
        }
    }
    *plane = rotated;
}
pub fn plane_fill_top_left(target: &mut Plane, right: &Plane, below: &Plane) {
    let size = target.len();
    for line in 0..size {
        for column in 0..size {
            if line < column {
                target[line][column] = right[line][0].clone();
            } else {
                target[line][column] = below[0][column].clone();
            }
        }
    }
}
pub fn plane_fill_top_right(target: &mut Plane, left: &Plane, below: &Plane) {
    let size = target.len();
    for line in 0..size {
        for column in 0..size {
            if line + column < size {
                target[line][column] = left[line][size - 1].clone();
            } else {
                target[line][column] = below[0][column].clone();
            }
        }
    }
}
pub fn plane_fill_bottom_left(target: &mut Plane, above: &Plane, right: &Plane) {
    let size = target.len();
    for line in 0..size {
        for column in 0..size {
            if line + column < size {
                target[line][column] = above[size - 1][column].clone();
            } else {
                target[line][column] = right[line][0].clone();
            }
        }
    }
}
pub fn plane_fill_bottom_right(target: &mut Plane, above: &Plane, left: &Plane) {
    let size = target.len();
    for line in 0..size {
        for column in 0..size {
            if line < column {
                target[line][column] = above[size - 1][column].clone();
            } else {
                target[line][column] = left[line][size - 1].clone();
            }
        }
    }
}
pub fn plane_fill_left(target: &mut Plane, right: &Plane) {
    let size = target.len();
    for line in 0..size {
        for column in 0..size {
            target[line][column] = right[line][0].clone();
        }
    }
}
pub fn plane_fill_right(target: &mut Plane, left: &Plane) {
    let size = target.len();
    for line in 0..size {
        for column in 0..size {
            target[line][column] = left[line][size - 1].clone();
        }
    }
}
