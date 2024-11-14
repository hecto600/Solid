use super::{
    nets::{Matrix, NetShapeAndFx, Plane},
    operations::*,
    render_net::render_matrix,
};
use crate::{initialization::configuration::Config, space::color_space::OuterPlanes};

pub struct Ladder {
    pub top_left: Plane,
    pub top_right: Plane,
    pub mid_left: Plane,
    pub mid_right: Plane,
    pub bottom_left: Plane,
    pub bottom_right: Plane,
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
    let mut display_matrix: Matrix = vec![vec![None; 4]; 3];
    let plane_size: usize = outer_planes.size;
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
                render_matrix(
                    &mut display_matrix,
                    plane_size,
                    NetShapeAndFx::LadderFill(ladder),
                )
            } else {
                render_matrix(
                    &mut display_matrix,
                    plane_size,
                    super::nets::NetShapeAndFx::Ladder(ladder),
                )
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
                render_matrix(
                    &mut display_matrix,
                    plane_size,
                    NetShapeAndFx::LadderFill(ladder),
                )
            } else {
                render_matrix(
                    &mut display_matrix,
                    plane_size,
                    NetShapeAndFx::Ladder(ladder),
                );
            }
        }
        _ => {
            println!("ladder.rs: not a valid ladder cut");
        }
    }
}
