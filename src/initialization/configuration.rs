use std::{collections::HashSet, fs};

use serde::Deserialize;
use toml;

use crate::{
    initialization::config_utils,
    space::{hsl, rgb::rgb_net_render},
};

#[derive(Deserialize)]
pub struct RgbConfig {
    #[serde(default = "default_bits")]
    pub bits_qty: u32,
    #[serde(default = "default_net")]
    pub net_type: String,
    #[serde(default = "default_cut")]
    pub cut_style: String,
    pub flag_fill: bool,
}
impl Default for RgbConfig {
    fn default() -> Self {
        Self {
            bits_qty: 3,
            net_type: String::from("ladder"),
            cut_style: String::from("a"),
            flag_fill: false,
        }
    }
}
pub fn default_bits() -> u32 {
    3
}
pub fn default_net() -> String {
    String::from("ladder")
}

pub fn default_cut() -> String {
    String::from("a")
}

#[derive(Deserialize)]
pub struct HslConfig {
    #[serde(default = "Default::default")]
    pub hue_increment: usize,
    #[serde(default = "Default::default")]
    pub saturation_increment: usize,
    #[serde(default = "Default::default")]
    pub light_increment: f64,
}
impl Default for HslConfig {
    fn default() -> Self {
        Self {
            hue_increment: 10,
            saturation_increment: 10,
            light_increment: 0.2,
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "Default::default")]
    mode: String,
    #[serde(default = "Default::default")]
    pub rgb: RgbConfig,
    #[serde(default = "Default::default")]
    pub hsl: HslConfig,
    #[serde(default = "Default::default")]
    pub show_info_on: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: String::from("rgb"),
            rgb: RgbConfig::default(),
            hsl: HslConfig::default(),
            show_info_on: false,
        }
    }
}
impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
enum Token {
    Command(String),
    Option(String),
    KeyValueOption(String, String),
}

fn tokenize(args: Vec<String>) -> Result<Vec<Token>, String> {
    let valid_commands = ["rgb", "hsl"];
    let valid_options = ["-a", "-h", "-v", "-f"];
    let key_value_options = ["-b", "-n", "-c"];
    let mut tokens = Vec::new();
    let mut words = args.iter().peekable();

    while let Some(word) = words.next() {
        if valid_commands.contains(&word.as_str()) {
            tokens.push(Token::Command(word.to_string()));
        } else if valid_options.contains(&word.as_str()) {
            tokens.push(Token::Option(word.to_string()));
        } else if key_value_options.contains(&word.as_str()) {
            if let Some(value) = words.peek() {
                tokens.push(Token::KeyValueOption(word.to_string(), value.to_string()));
                words.next();
            } else {
                return Err(format!("Option '{}' requires a value", word));
            }
        } else {
            return Err(format!("Unknown token '{}'", word));
        }
    }

    Ok(tokens)
}
fn analyze_syntax(tokens: Vec<Token>) -> Result<Config, String> {
    let mut seen_options = HashSet::new();
    let mut cfg = Config::default();

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Command(cmd) => {
                if i > 0 {
                    return Err("Command must be the first argument".to_string());
                }
                cfg.mode = cmd.clone();
            }
            Token::Option(opt) => {
                if !seen_options.insert(opt.clone()) {
                    return Err(format!("Duplicate option '{}'", opt));
                }
                match opt.as_str() {
                    "-h" => {
                        if tokens.len() > 1 {
                            return Err(format!("-v can't have other arguments"));
                        }
                        config_utils::info_help();
                        cfg.show_info_on = true;
                    }
                    "-v" => {
                        if tokens.len() > 1 {
                            return Err(format!("-v can't have other arguments"));
                        }
                        config_utils::info_version();
                        cfg.show_info_on = true;
                    }
                    "-f" => cfg.rgb.flag_fill = true,
                    _ => return Err(format!("Unknown option '{}'", opt)),
                }
            }
            Token::KeyValueOption(opt, val) => {
                if !seen_options.insert(opt.clone()) {
                    return Err(format!("Duplicate option '{}'", opt));
                }
                match opt.as_str() {
                    "-b" => {
                        let parsed_value = val.parse::<u32>().map_err(|_| {
                            format!("Option '-b' requires a u32 value, found '{}'", val)
                        })?;
                        if parsed_value < 1 || parsed_value > 8 {
                            return Err(format!(
                                "Option '-b' requires a value between 1 and 8, found '{}'",
                                parsed_value
                            ));
                        }
                        cfg.rgb.bits_qty = parsed_value;
                    }
                    "-n" => {
                        if val != "ladder" && val != "cross" {
                            return Err(format!(
                                "Option '-n' requires 'ladder' or 'cross', found '{}'",
                                val
                            ));
                        }
                        cfg.rgb.net_type = val.clone();
                    }
                    "-c" => {
                        if val != "a" && val != "b" && val != "c" {
                            return Err(format!(
                                "Option '-c' requires 'a', 'b', or 'c', found '{}'",
                                val
                            ));
                        }
                        cfg.rgb.cut_style = val.clone();
                    }

                    _ => return Err(format!("Unknown option '{}'", opt)),
                }
            }
        }
    }
    Ok(cfg)
}
pub fn start(mut args: Vec<String>) {
    args.remove(0);
    if args.len() > 0 {
        let tokens = tokenize(args).expect("Failed to tokenize args");
        match analyze_syntax(tokens) {
            Ok(cfg) => {
                if cfg.mode == "hsl" {
                    hsl::render_hsl_light_const(5, 5);
                } else if !cfg.show_info_on {
                    rgb_net_render(&cfg)
                }
            }

            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        let config: Config;
        match Config::from_file("config/config.toml") {
            Ok(valid) => config = valid,
            Err(_) => config = Config::default(),
        }
        match config.rgb.bits_qty {
            1..=8 => {
                rgb_net_render(&config);
            }

            _ => {
                println!("That is a invalid number!");
            }
        }
    }
}
