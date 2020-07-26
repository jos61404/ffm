use console::style;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub extension: Vec<String>,
    pub name: Vec<String>,
}

pub fn get(pb_config: ProgressBar) -> std::io::Result<Filter> {
    pb_config.set_message(&format!(
        "{} 讀取設定 : {}",
        style("[2/6]").bold().dim(),
        "執行中..."
    ));
    pb_config.inc(1);

    let mut file_list = fs::File::open("./filter_list.json")?;
    pb_config.set_message(&format!(
        "{} 讀取設定 : {}",
        style("[2/6]").bold().dim(),
        "取得原始檔"
    ));
    pb_config.inc(1);

    pb_config.set_message(&format!(
        "{} 讀取設定 : {}",
        style("[2/6]").bold().dim(),
        "資料轉換"
    ));
    pb_config.inc(1);

    let mut contents = String::new();
    file_list.read_to_string(&mut contents)?;
    let filter_list: Filter = serde_json::from_str(&contents)?;

    pb_config.set_message(&format!(
        "{} 讀取設定 : {}",
        style("[2/6]").bold().dim(),
        "完成"
    ));
    pb_config.inc(1);
    pb_config.finish();

    return Ok(filter_list);
}
