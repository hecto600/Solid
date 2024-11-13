use crate::{
    cube_net::{nets::Plane, operations::*, render_net::render_matrix},
    initialization::configuration::Config,
    space::color_space::{Color, OuterPlanes},
};

use super::nets::Matrix;
#[derive(Clone)]
pub struct Cross {
    pub top: Vec<Vec<Color>>,
    pub mid_left: Vec<Vec<Color>>,
    pub mid: Vec<Vec<Color>>,
    pub mid_right: Vec<Vec<Color>>,
    pub mid_bottom: Vec<Vec<Color>>,
    pub bottom: Vec<Vec<Color>>,
}
impl Cross {
    pub fn new(
        top: Vec<Vec<Color>>,
        mid_left: Vec<Vec<Color>>,
        mid: Vec<Vec<Color>>,
        mid_right: Vec<Vec<Color>>,
        mid_bottom: Vec<Vec<Color>>,
        bottom: Vec<Vec<Color>>,
    ) -> Self {
        Self {
            top,
            mid_left,
            mid,
            mid_right,
            mid_bottom,
            bottom,
        }
    }
}
pub fn cross_style(outer_planes: &OuterPlanes, config: &Config) {
    match config.rgb.cut_style.as_str() {
        "a" => {
            let mut cross = Cross::new(
                outer_planes.gb.clone(),
                outer_planes.rg.clone(),
                outer_planes.cy.clone(),
                outer_planes.cm.clone(),
                outer_planes.my.clone(),
                outer_planes.rb.clone(),
            );
            plane_flip_h(&mut cross.mid_right);
            plane_flip_v(&mut cross.mid_bottom);
            plane_flip_v(&mut cross.bottom);
            if config.rgb.flag_fill {
                cross_format_fill(cross.clone());
            } else {
                cross_format(cross.clone());
            }
        }
        "b" => {
            let mut cross = Cross::new(
                outer_planes.my.clone(),
                outer_planes.cy.clone(),
                outer_planes.rg.clone(),
                outer_planes.rb.clone(),
                outer_planes.gb.clone(),
                outer_planes.cm.clone(),
            );
            plane_transpose_reversed(&mut cross.top);
            plane_flip_v_and_h(&mut cross.mid_left);
            plane_flip_v_and_h(&mut cross.mid);
            plane_flip_v(&mut cross.mid_right);
            plane_rotate_90(&mut cross.mid_bottom);
            plane_flip_h(&mut cross.bottom);
            if config.rgb.flag_fill {
                cross_format_fill(cross.clone());
            } else {
                cross_format(cross.clone());
            }
        }
        _ => {
            println!("cross.rs: not a valid cross cut");
        }
    }
}
pub fn cross_format(cross: Cross) {
    let mut display_matrix: Matrix = vec![vec![None; 3]; 4];
    let plane_size: usize = cross.top.len();
    display_matrix[0][1] = Some(cross.top);
    display_matrix[1][0] = Some(cross.mid_left);
    display_matrix[1][1] = Some(cross.mid);
    display_matrix[1][2] = Some(cross.mid_right);
    display_matrix[2][1] = Some(cross.mid_bottom);
    display_matrix[3][1] = Some(cross.bottom);
    render_matrix(&display_matrix, plane_size);
}

pub fn cross_format_fill(cross: Cross) {
    let mut display_matrix: Matrix = vec![vec![None; 3]; 4];
    let plane_size: usize = cross.top.len();
    let mut plane_fill: Plane = vec![vec![Color::new(0, 0, 0); plane_size]; plane_size];
    plane_fill_top_left(&mut plane_fill, &cross.top, &cross.mid_left);
    display_matrix[0][0] = Some(plane_fill.clone());

    plane_fill_top_right(&mut plane_fill, &cross.top, &cross.mid_right);
    display_matrix[0][2] = Some(plane_fill.clone());

    plane_fill_bottom_left(&mut plane_fill, &cross.mid_left, &cross.mid_bottom);
    display_matrix[2][0] = Some(plane_fill.clone());

    plane_fill_bottom_right(&mut plane_fill, &cross.mid_right, &cross.mid_bottom);
    display_matrix[2][2] = Some(plane_fill.clone());

    plane_fill_left(&mut plane_fill, &cross.bottom);
    display_matrix[3][0] = Some(plane_fill.clone());

    plane_fill_right(&mut plane_fill, &cross.bottom);
    display_matrix[3][2] = Some(plane_fill.clone());

    display_matrix[0][1] = Some(cross.top);
    display_matrix[1][0] = Some(cross.mid_left);
    display_matrix[1][1] = Some(cross.mid);
    display_matrix[1][2] = Some(cross.mid_right);
    display_matrix[2][1] = Some(cross.mid_bottom);
    display_matrix[3][1] = Some(cross.bottom);
    render_matrix(&display_matrix, plane_size);
}
