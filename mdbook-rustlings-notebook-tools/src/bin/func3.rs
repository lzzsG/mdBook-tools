use serde::Deserialize;
use std::fs;
use std::io::Write;

// 定义结构体以对应TOML中的数据结构
#[derive(Debug, Deserialize)]
struct Config {
    exercises: Vec<Exercise>,
}

#[derive(Debug, Deserialize)]
struct Exercise {
    num: usize,
    name: String,
    path: String,
    mode: Option<String>,
    hint: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取TOML文件
    let contents = fs::read_to_string("./info.toml")?;
    let config: Config = toml::from_str(&contents)?;

    // 打开一个文件用于写入
    let mut output = fs::File::create("output.md")?;

    // 遍历所有练习
    for exercise in config.exercises {
        // 将.rs后缀更改为.md
        let md_path = exercise.path.replace(".rs", ".md");

        // 写入转换后的信息
        writeln!(
            output,
            "{}.[{}](mdbook/{})",
            exercise.num, exercise.name, md_path
        )?;
    }

    Ok(())
}
