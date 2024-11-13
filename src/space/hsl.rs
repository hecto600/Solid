pub fn render_hsl_light_const(step_hue: i32, step_sat: i32) {
    let mut hue_prime: f64;
    let mut chroma: f64;
    let mut x: f64;
    let mut rgb: (i32, i32, i32);
    let mut col: i32 = 0;
    let nor_l: f64 = 0.50;
    let mut nor_s: f64 = 0.00;

    for h in (0..360).step_by(step_hue as usize) {
        for _s in (0..=100).step_by(step_sat as usize) {
            chroma = (1.0 - (2.0 * nor_l - 1.0).abs()) * nor_s;

            hue_prime = h as f64 / 60.0;
            x = chroma * (1.0 - ((hue_prime % 2.0) - 1.0).abs());

            let m = nor_l - (chroma / 2.0);

            let mut rgb_f = match hue_prime.trunc() as i32 {
                0..1 => (chroma, x, 0.0),
                1..2 => (x, chroma, 0.0),
                2..3 => (0.0, chroma, x),
                3..4 => (0.0, x, chroma),
                4..5 => (x, 0.0, chroma),
                5..6 => (chroma, 0.0, x),
                _ => (-1.0, -1.0, -1.0),
            };

            rgb_f = (
                (rgb_f.0 + m) * 255.0,
                (rgb_f.1 + m) * 255.0,
                (rgb_f.2 + m) * 255.0,
            );
            rgb = (
                ((rgb_f.0).round() as i32),
                ((rgb_f.1).round() as i32),
                ((rgb_f.2).round() as i32),
            );

            print!(
                "\x1b[48;2;{};{};{}m \x1b[30m#{}{}{} \x1b[0m",
                rgb.0,
                rgb.1,
                rgb.2,
                format!("{:02X}", rgb.0),
                format!("{:02X}", rgb.1),
                format!("{:02X}", rgb.2)
            );
            if col >= 100 / step_sat {
                println!("");
                col = 0;
            } else {
                col += 1;
            }

            nor_s += 0.01 * step_sat as f64;
        }
        nor_s = 0.0;
    }
}

pub fn render_hsl(step_hue: i32, step_sat: i32, step_lig: i32) {
    let mut hue_prime: f64;
    let mut chroma: f64;
    let mut x: f64;
    let mut rgb: (i32, i32, i32);
    let mut col: i32 = 0;
    let mut nor_l: f64 = 0.00;
    let mut nor_s: f64 = 0.00;

    for h in (0..360).step_by(step_hue as usize) {
        for _s in (0..100).step_by(step_sat as usize) {
            for _l in (0..100).step_by(step_lig as usize) {
                chroma = (1.0 - (2.0 * nor_l - 1.0).abs()) * nor_s;

                hue_prime = h as f64 / 60.0;
                x = chroma * (1.0 - ((hue_prime % 2.0) - 1.0).abs());

                let m = nor_l - (chroma / 2.0);

                let mut rgb_f = match hue_prime.trunc() as i32 {
                    0..1 => (chroma, x, 0.0),
                    1..2 => (x, chroma, 0.0),
                    2..3 => (0.0, chroma, x),
                    3..4 => (0.0, x, chroma),
                    4..5 => (x, 0.0, chroma),
                    5..6 => (chroma, 0.0, x),
                    _ => (-1.0, -1.0, -1.0),
                };

                rgb_f = (
                    (rgb_f.0 + m) * 255.0,
                    (rgb_f.1 + m) * 255.0,
                    (rgb_f.2 + m) * 255.0,
                );
                rgb = (
                    ((rgb_f.0).round() as i32),
                    ((rgb_f.1).round() as i32),
                    ((rgb_f.2).round() as i32),
                );

                print!("\x1b[48;2;{};{};{}m  \x1b[0m", rgb.0, rgb.1, rgb.2,);
                if col >= 100 / step_lig {
                    // println!("HSL:{h},{:.2},{:.2} {:?}", s, l, rgb);
                    println!("");
                    col = 0;
                } else {
                    col += step_lig;
                }

                nor_l += 0.01 * step_lig as f64;
            }
            nor_l = 0.0;
            nor_s += 0.01 * step_sat as f64;
        }
        nor_s = 0.0;
    }
}
