use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Exercise {
    name: String,
    path: String,
    mode: String,
    hint: String,
    #[serde(default)]
    num: i32, // 新增num字段
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    exercises: Vec<Exercise>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./info.toml";
    let content = fs::read_to_string(path)?;
    let mut config: Config = toml::from_str(&content)?;

    let mut num = 1;
    for exercise in &mut config.exercises {
        exercise.num = num;
        num += 1;
    }

    let toml = toml::to_string_pretty(&config)?;
    fs::write(path, toml.as_bytes())?;
    Ok(())
}
