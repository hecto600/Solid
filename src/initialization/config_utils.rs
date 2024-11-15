use std::collections::HashMap;

#[derive(Debug)]
enum TextStyle {
    Bold,
    Underline,
    Red,
    Green,
    BoldUnderline,
    BoldGreen,
    BoldUnderlineGreen,
    Reset,
}

impl TextStyle {
    fn to_ansi_code(&self) -> &'static str {
        match self {
            TextStyle::Bold => "\x1b[1m",
            TextStyle::Underline => "\x1b[4m",
            TextStyle::Red => "\x1b[31m",
            TextStyle::Green => "\x1b[32m",
            TextStyle::BoldUnderline => "\x1b[1;4m",
            TextStyle::BoldGreen => "\x1b[1;32m",
            TextStyle::BoldUnderlineGreen => "\x1b[1;4;32m",
            TextStyle::Reset => "\x1b[0m",
        }
    }
}

fn format_with_styles(template: &str, styles: &HashMap<&str, TextStyle>) -> String {
    let mut result = template.to_string();

    for (placeholder, style) in styles {
        let ansi_code = style.to_ansi_code();
        let placeholder_with_braces = format!("{{{}}}", placeholder);
        result = result.replace(&placeholder_with_braces, ansi_code);
    }

    result
}

pub fn info_help() {
    let info_help = r#"
{bold}Usage:{reset} solid [command][flag][options...]

{bold}COMMANDS{reset}
    {bold_green}rgb{reset}
        The default command. It shows a 2d mapping rgb cube, with differents nets.
        Example: 
            {bold}solid rgb{reset}

            {bold}Options{reset}:
                {bold}-b <number>:{reset} 
                    Bits quantitiy for each r,g, and b channel. The range interval is {bold}[1,8]{reset}.
                    It defines the amount of colors the cube net has. 

                    As a example if the option is set with value 3, then: 
                        Plane = (axis^3)*(axis^3)
                        Cube = Plane * 6
                    Therefore, the total amount of colors displayed is 384 

                    Example: 
                        {bold}solid -b 3{reset}

                {bold}-n <name>:{reset}
                    Type of 2d representation of the rgb cube.
                    Available nets {bold}<name>{reset} are {underline}ladder{reset} and {underline}cross{reset}.
                    Example:
                        {bold}solid -n cross{reset}  

                {bold}-c <name>:{reset} 
                    Variation of where the cube is opened. It defines which planes are separeted or not.
                    Available {bold}<name>{reset} are {underline}a{reset} and {underline}b{reset}.
                    Example:
                        {bold}solid -c a {reset}

            {bold}Flag:{reset}
                {bold}-f{reset}:
                    Activate fill effect. Repeat colors to fullfill a rectangle.
                    Example: 
                        {bold}solid -f{reset}

    {bold_green}hsl{reset}
        Shows colors with Hue, saturation and lightness parameters.
        Example: 
            {bold}solid hsl{reset}
"#;

    let mut styles = HashMap::new();
    styles.insert("bold", TextStyle::Bold);
    styles.insert("underline", TextStyle::Underline);
    styles.insert("red", TextStyle::Red);
    styles.insert("green", TextStyle::Green);
    styles.insert("bold_underline", TextStyle::BoldUnderline);
    styles.insert("bold_green", TextStyle::BoldGreen);
    styles.insert("bold_underline_green", TextStyle::BoldUnderlineGreen);
    styles.insert("reset", TextStyle::Reset);
    let formatted_text = format_with_styles(&info_help, &styles);

    println!("{}", formatted_text);
}

pub fn info_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Solid version: {}", version);
}
