use crate::space::color_space::Color;

use super::{
    cross::Cross,
    ladder::Ladder,
    nets::{Matrix, NetShapeAndFx, Plane},
    operations::*,
};

pub fn matrix_cross(cross: &Cross, display_matrix: &mut Matrix) {
    display_matrix[0][1] = Some(cross.top.clone());
    display_matrix[1][0] = Some(cross.mid_left.clone());
    display_matrix[1][1] = Some(cross.mid.clone());
    display_matrix[1][2] = Some(cross.mid_right.clone());
    display_matrix[2][1] = Some(cross.mid_bottom.clone());
    display_matrix[3][1] = Some(cross.bottom.clone());
}
pub fn matrix_add_cross_fill(cross: Cross, display_matrix: &mut Matrix) {
    let plane_size: usize = cross.top.len();

    let mut plane_fill: Plane = plane_fill_top_left(plane_size, &cross.top, &cross.mid_left);
    display_matrix[0][0] = Some(plane_fill);

    plane_fill = plane_fill_top_right(plane_size, &cross.top, &cross.mid_right);
    display_matrix[0][2] = Some(plane_fill);

    plane_fill = plane_fill_bottom_left(plane_size, &cross.mid_left, &cross.mid_bottom);
    display_matrix[2][0] = Some(plane_fill);

    plane_fill = plane_fill_bottom_right(plane_size, &cross.mid_right, &cross.mid_bottom);
    display_matrix[2][2] = Some(plane_fill);

    plane_fill = plane_fill_left(plane_size, &cross.bottom);
    display_matrix[3][0] = Some(plane_fill);

    plane_fill = plane_fill_right(plane_size, &cross.bottom);
    display_matrix[3][2] = Some(plane_fill);
}
fn matrix_ladder(ladder: &Ladder, display_matrix: &mut Matrix) {
    display_matrix[0][0] = Some(ladder.top_left.clone());
    display_matrix[0][1] = Some(ladder.top_right.clone());
    display_matrix[1][1] = Some(ladder.mid_left.clone());
    display_matrix[1][2] = Some(ladder.mid_right.clone());
    display_matrix[2][2] = Some(ladder.bottom_left.clone());
    display_matrix[2][3] = Some(ladder.bottom_right.clone());
}
fn matrix_add_ladder_fill(ladder: Ladder, display_matrix: &mut Matrix) {
    let plane_size: usize = ladder.top_left.len();

    let mut plane_fill: Plane =
        plane_fill_top_right(plane_size, &ladder.top_right, &ladder.mid_right);
    display_matrix[0][2] = Some(plane_fill);

    plane_fill = meta_fill_top_right(plane_size, &ladder.top_right);
    display_matrix[0][3] = Some(plane_fill);

    plane_fill = plane_fill_bottom_left(plane_size, &ladder.top_left, &ladder.mid_left);
    display_matrix[1][0] = Some(plane_fill);

    plane_fill = plane_fill_top_right(plane_size, &ladder.mid_right, &ladder.bottom_right);
    display_matrix[1][3] = Some(plane_fill);

    plane_fill = meta_fill_bottom_left(plane_size, &ladder.mid_left);
    display_matrix[2][0] = Some(plane_fill);

    plane_fill = plane_fill_bottom_left(plane_size, &ladder.mid_left, &ladder.bottom_left);
    display_matrix[2][1] = Some(plane_fill);
}

pub fn render_matrix(block_matrix: &mut Matrix, plane_size: usize, net_shape: NetShapeAndFx) {
    match net_shape {
        NetShapeAndFx::Cross(cross) => {
            matrix_cross(&cross, block_matrix);
        }
        NetShapeAndFx::CrossFill(cross_fill) => {
            matrix_cross(&cross_fill, block_matrix);
            matrix_add_cross_fill(cross_fill, block_matrix);
        }
        NetShapeAndFx::Ladder(ladder) => matrix_ladder(&ladder, block_matrix),
        NetShapeAndFx::LadderFill(ladder_fill) => {
            matrix_ladder(&ladder_fill, block_matrix);
            matrix_add_ladder_fill(ladder_fill, block_matrix);
        }
    }

    for l in 0..block_matrix.len() {
        for line in 0..plane_size {
            for c in 0..block_matrix[0].len() {
                for column in 0..plane_size {
                    match &block_matrix[l][c] {
                        Some(plane) => {
                            render_color(&plane[line][column]);
                        }
                        None => {
                            print!("  ");
                        }
                    }
                }
            }
            println!();
        }
    }
}
fn render_color(color: &Color) {
    print!(
        "\x1b[48;2;{};{};{}m  \x1b[0m",
        color.red, color.green, color.blue
    );
    // print!(
    //     "\x1b[48;2;{};{};{}m #{}{}{}  \x1b[0m",
    //     r,
    //     g,
    //     b,
    //     format!("{:02X}", r),
    //     format!("{:02X}", g),
    //     format!("{:02X}", b)
    // );
}
