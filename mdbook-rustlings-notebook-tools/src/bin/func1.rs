use serde::Deserialize;
use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct Exercise {
    name: String,
    path: String,
    hint: String,
    num: i32,
}

#[derive(Deserialize, Debug)]
struct Config {
    exercises: Vec<Exercise>,
}

fn main() -> Result<()> {
    let path = "./info.toml";
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;

    for exercise in config.exercises {
        let mut md_path = PathBuf::from("mdbook");
        md_path.push(&exercise.path.replace(".rs", ".md"));

        // 确保目标目录存在
        if let Some(parent) = md_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(md_path)?;

        // // 格式化并写入内容
        // writeln!(file, "# Exercise {}\n", exercise.num)?;
        // writeln!(file, "- Name: ```{}```", exercise.name)?;
        // writeln!(file, "- Path: ```{}```", exercise.path)?;
        // writeln!(file, "#### Hint: \n```\n{}\n```\n", exercise.hint)?;
        // writeln!(file, "---\n\n\n")?;
        // writeln!(file, "---\n")?;

        writeln!(file, "# Exercise {}\n", exercise.num)?;
        writeln!(file, "- Name: ```{}```", exercise.name)?;
        writeln!(file, "- Path: ```{}```", exercise.path)?;
        writeln!(file, "#### Hint: \n\n{}\n\n", exercise.hint)?;
        writeln!(file, "---\n\n\n")?;
    }

    Ok(())
}
