use crate::space::color_space::Color;

pub fn render_matrix(block_matrix: &Vec<Vec<Option<Vec<Vec<Color>>>>>, plane_size: usize) {
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
