pub const INFO_HELP: &str = r#"
Usage: solid [command][flag][options]...

Commands
rgb
Shows a 2d mapping rgb cube, with differents nets.
Example: solid rgb
    Options:
        -b <number>: Bits quantitiy of each r,g,b channel. The range interval is [1,8].
                     Example: solid -b 3. Shows (axis^3)*(axis^3)*(cube faces).
                     2^3 * 2^3 * 6 = 384 colors for the cube net.
        -n <name>: Type of 2d representation of the rgb cube.
                   Available nets <name> are "ladder" and "cross".
                   Example: solid -n cross 
        -c <name>: Variation of where the cube is opened. It defines which planes are separeted or not.
                   Available <name> are "a" and "b".
                   Example: solid -c a
    Flag:
        -f: Activate fill effect. Repeat the color to fullfill a rectangle

hsl
Shows colors with Hue, saturation and lightness parameters.\
Example: solid hsl
"#;

pub const INFO_VERSION: &str = r#"solid 0.1.0"#;
