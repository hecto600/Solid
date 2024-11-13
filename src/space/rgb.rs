use crate::{
    cube_net::nets::select_net,
    initialization::configuration::Config,
    space::{color_space::OuterPlanes, rgb_utils},
};

pub fn rgb_net_render(config: &Config) {
    const MAX_CHANNEL_ELEMENTS: usize = 256;
    let size = usize::pow(2, config.rgb.bits_qty);
    let mut outer_planes = OuterPlanes::new(size);
    let incr = MAX_CHANNEL_ELEMENTS / size;
    rgb_utils::create_planes_colors(&mut outer_planes, incr);

    select_net(&outer_planes, &config);
}
