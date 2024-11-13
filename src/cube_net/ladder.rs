use crate::{initialization::configuration::Config, space::color_space::OuterPlanes};

use super::{
    nets::{Matrix, Plane},
    operations::*,
    render_net::render_matrix,
};
pub struct Ladder {
    top_left: Plane,
    top_right: Plane,
    mid_left: Plane,
    mid_right: Plane,
    bottom_left: Plane,
    bottom_right: Plane,
}

impl Ladder {
    pub fn new(
        top_left: Plane,
        top_right: Plane,
        mid_left: Plane,
        mid_right: Plane,
        bottom_left: Plane,
        bottom_right: Plane,
    ) -> Self {
        Self {
            top_left,
            top_right,
            mid_left,
            mid_right,
            bottom_left,
            bottom_right,
        }
    }
}

pub fn ladder_style(outer_planes: &OuterPlanes, config: &Config) {
    match config.rgb.cut_style.as_str() {
        "a" => {
            let mut ladder = Ladder::new(
                outer_planes.rg.clone(),
                outer_planes.gb.clone(),
                outer_planes.cy.clone(),
                outer_planes.cm.clone(),
                outer_planes.my.clone(),
                outer_planes.rb.clone(),
            );
            plane_rotate_90(&mut ladder.top_left);
            plane_flip_h(&mut ladder.mid_right);
            plane_transpose_reversed(&mut ladder.bottom_left);
            plane_transpose_reversed(&mut ladder.bottom_right);
            if config.rgb.flag_fill {
                println!("Fill effect not availble to ladder cut yet.");
            } else {
                ladder_format(ladder);
            }
        }
        "b" => {
            let mut ladder = Ladder::new(
                outer_planes.cy.clone(),
                outer_planes.my.clone(),
                outer_planes.rg.clone(),
                outer_planes.rb.clone(),
                outer_planes.gb.clone(),
                outer_planes.cm.clone(),
            );
            plane_rotate_90ccw(&mut ladder.top_left);
            plane_transpose_reversed(&mut ladder.top_right);
            plane_flip_v_and_h(&mut ladder.mid_left);
            plane_flip_v(&mut ladder.mid_right);
            plane_transpose(&mut ladder.bottom_right);

            if config.rgb.flag_fill {
                println!("Fill effect not availble to ladder cut yet.");
            } else {
                ladder_format(ladder);
            }
        }
        _ => {
            println!("ladder.rs: not a valid ladder cut");
        }
    }
}

pub fn ladder_format(ladder: Ladder) {
    let mut display_matrix: Matrix = vec![vec![None; 4]; 3];
    let plane_size: usize = ladder.top_left.len();
    display_matrix[0][0] = Some(ladder.top_left);
    display_matrix[0][1] = Some(ladder.top_right);
    display_matrix[1][1] = Some(ladder.mid_left);
    display_matrix[1][2] = Some(ladder.mid_right);
    display_matrix[2][2] = Some(ladder.bottom_left);
    display_matrix[2][3] = Some(ladder.bottom_right);
    render_matrix(&display_matrix, plane_size);
}
