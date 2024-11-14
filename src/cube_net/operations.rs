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
pub fn plane_fill_top_left(plane_size: usize, right: &Plane, below: &Plane) -> Plane {
    let mut fill_plane: Plane = vec![vec![Color::new(0, 0, 0); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            if line < column {
                fill_plane[line][column] = right[line][0].clone();
            } else {
                fill_plane[line][column] = below[0][column].clone();
            }
        }
    }
    fill_plane
}
pub fn plane_fill_top_right(plane_size: usize, left: &Plane, below: &Plane) -> Plane {
    let mut fill_plane: Plane = vec![vec![Color::new(0, 0, 0); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            if line + column < plane_size {
                fill_plane[line][column] = left[line][plane_size - 1].clone();
            } else {
                fill_plane[line][column] = below[0][column].clone();
            }
        }
    }
    fill_plane
}
pub fn plane_fill_bottom_left(plane_size: usize, above: &Plane, right: &Plane) -> Plane {
    let mut fill_plane: Plane = vec![vec![Color::new(0, 0, 0); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            if line + column < plane_size {
                fill_plane[line][column] = above[plane_size - 1][column].clone();
            } else {
                fill_plane[line][column] = right[line][0].clone();
            }
        }
    }
    fill_plane
}
pub fn plane_fill_bottom_right(plane_size: usize, above: &Plane, left: &Plane) -> Plane {
    let mut fill_plane: Plane = vec![vec![Color::new(0, 0, 0); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            if line < column {
                fill_plane[line][column] = above[plane_size - 1][column].clone();
            } else {
                fill_plane[line][column] = left[line][plane_size - 1].clone();
            }
        }
    }
    fill_plane
}
pub fn plane_fill_left(plane_size: usize, right: &Plane) -> Plane {
    let mut fill_plane: Plane = vec![vec![Color::new(0, 0, 0); plane_size]; plane_size];

    for line in 0..plane_size {
        for column in 0..plane_size {
            fill_plane[line][column] = right[line][0].clone();
        }
    }
    fill_plane
}
pub fn plane_fill_right(plane_size: usize, left: &Plane) -> Plane {
    let mut fill_plane: Plane = vec![vec![Color::new(0, 0, 0); plane_size]; plane_size];
    for line in 0..plane_size {
        for column in 0..plane_size {
            fill_plane[line][column] = left[line][plane_size - 1].clone();
        }
    }
    fill_plane
}

pub fn meta_fill_top_right(plane_size: usize, left: &Plane) -> Plane {
    let color = left[0][plane_size - 1].clone();
    let fill_plane: Plane =
        vec![vec![Color::new(color.red, color.green, color.blue); plane_size]; plane_size];
    fill_plane
}
pub fn meta_fill_bottom_left(plane_size: usize, above: &Plane) -> Plane {
    let color = above[plane_size - 1][0].clone();
    let fill_plane: Plane =
        vec![vec![Color::new(color.red, color.green, color.blue); plane_size]; plane_size];
    fill_plane
}
