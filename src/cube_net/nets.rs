use super::cross::cross_style;
use crate::{
    cube_net::ladder::ladder_style,
    initialization::configuration::Config,
    space::color_space::{Color, OuterPlanes},
};
pub type Plane = Vec<Vec<Color>>;
pub type Matrix = Vec<Vec<Option<Vec<Vec<Color>>>>>;

pub fn select_net(outer_planes: &OuterPlanes, config: &Config) {
    match config.rgb.net_type.as_str() {
        "cross" => {
            cross_style(outer_planes, &config);
        }
        "ladder" => {
            ladder_style(outer_planes, &config);
        }
        _ => {
            println!("Val:{}", config.rgb.net_type);
            println!(
                r#"
            nets.rs: config has an invalid net type. Current valid types are: ladder, \
            and cross.
            "#
            )
        }
    }
}
