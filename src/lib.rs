mod dp;
mod dp_array;
mod dp_array_opt;
mod dp_memo;
mod utils;

pub struct Config {
    pub mode: String,
    pub s1: String,
    pub s2: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let mode = args[1].clone();
        let s1 = args[2].clone();
        let s2 = args[3].clone();
        Ok(Config { mode, s1, s2 })
    }
}

pub fn min_edit_distance(config: &Config) -> Result<isize, &str> {
    let s1 = config.s1.as_str();
    let s2 = config.s2.as_str();
    match config.mode.as_str() {
        "dp" => Ok(dp::min_edit_distance(s1, s2)),
        "dp_memo" => Ok(dp_memo::min_edit_distance(s1, s2)),
        "dp_array" => Ok(dp_array::min_edit_distance(s1, s2)),
        "dp_array_opt" => Ok(dp_array_opt::min_edit_distance(s1, s2)),
        _ => Err("Not that mode, only support 'dp', 'dp_memo', 'dp_array', and 'dp_array_opt'"),
    }
}
