use crate::{
    cube_net::{nets::Plane, operations::*, render_net::render_matrix},
    initialization::configuration::Config,
    space::color_space::{Color, OuterPlanes},
};

use super::nets::{Matrix, NetShapeAndFx};
#[derive(Clone)]
pub struct Cross {
    pub top: Plane,
    pub mid_left: Plane,
    pub mid: Plane,
    pub mid_right: Plane,
    pub mid_bottom: Plane,
    pub bottom: Plane,
}
impl Cross {
    pub fn new(
        top: Plane,
        mid_left: Plane,
        mid: Plane,
        mid_right: Plane,
        mid_bottom: Plane,
        bottom: Plane,
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
    let mut display_matrix: Matrix = vec![vec![None; 3]; 4];
    let plane_size: usize = outer_planes.size;
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
                render_matrix(
                    &mut display_matrix,
                    plane_size,
                    NetShapeAndFx::CrossFill(cross.clone()),
                );
            } else {
                render_matrix(
                    &mut display_matrix,
                    plane_size,
                    NetShapeAndFx::Cross(cross.clone()),
                );
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
                render_matrix(
                    &mut display_matrix,
                    plane_size,
                    NetShapeAndFx::CrossFill(cross.clone()),
                );
            } else {
                render_matrix(
                    &mut display_matrix,
                    plane_size,
                    NetShapeAndFx::Cross(cross.clone()),
                );
            }
        }
        _ => {
            println!("cross.rs: not a valid cross cut");
        }
    }
}
