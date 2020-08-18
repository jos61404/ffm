use crate::{filter::FilterDataStruct, log};
use console::style;
use fs::File;
use indicatif::ProgressBar;
use std::fs;
use std::path::Path;
use std::{io, path::PathBuf};

#[warn(unused_must_use)]
pub fn check_dir(dir_name: &String) -> std::io::Result<&Path> {
    let filter_path = Path::new(dir_name);
    // 確認隔離資料夾是否存在，如沒有直接創建
    if filter_path.exists() == false {
        fs::DirBuilder::new().recursive(true).create(filter_path)?;
    }
    return Ok(filter_path);
}

fn copy_file(
    pb_copy_file: &ProgressBar,
    original_path: &Path,
    target_path: &Path,
) -> std::io::Result<()> {
    pb_copy_file.set_message(&format!(
        "{} 資料搬移 : {:#?} - {}",
        style("[6/6]").bold().dim(),
        target_path.file_stem().unwrap(),
        "執行中..."
    ));

    // 判斷檔案是否存不存在
    if target_path.exists() == false {
        let pb_length = &pb_copy_file.length(); // 讀取原本進度條暫存
        let mut source = File::open(original_path)?;
        let target = File::create(target_path)?;
        pb_copy_file.set_length(source.metadata()?.len());
        io::copy(&mut source, &mut pb_copy_file.wrap_write(target))?;

        pb_copy_file.set_position(pb_length + 1); // 還原原始進度條
        pb_copy_file.set_length(pb_length + 1); // 還原原始進度條
        pb_copy_file.set_message(&format!(
            "{} 資料搬移 : {:#?} - {}",
            style("[6/6]").bold().dim(),
            target_path.file_stem().unwrap(),
            "處理完成"
        ));
    }
    return Ok(());
}

fn remove_file(path: &Path) -> std::io::Result<()> {
    if path.exists() == true {
        fs::remove_file(path)?;
    }
    return Ok(());
}

fn get_path(path_name: &Path, paths_list: &mut Vec<PathBuf>) -> std::io::Result<Vec<PathBuf>> {
    for entry in fs::read_dir(path_name)? {
        let dir = entry?;
        let path = dir.path();
        paths_list.push(dir.path());

        if path.is_file() == false {
            get_path(&path, paths_list)?;
        }
    }
    let paths = paths_list.to_vec();
    return Ok(paths);
}

pub fn get_list(pb_file_list: ProgressBar, path_name: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut pb_len = 0;
    pb_file_list.set_message(&format!(
        "{} 讀取路徑 : {}",
        style("[3/6]").bold().dim(),
        "執行中..."
    ));

    for fs_entry in fs::read_dir(&path_name)? {
        let dir = fs_entry?;
        let path = dir.path();
        if path.is_file() == false {
            pb_len = pb_len + 1;
        }
    }

    pb_file_list.set_length(pb_len);

    for entry in fs::read_dir(&path_name)? {
        let dir = entry?;
        let path = dir.path();
        paths.push(dir.path());
        if path.is_file() == false {
            get_path(&path, &mut paths)?;
            pb_file_list.inc(1);
        }
    }

    pb_file_list.set_message(&format!(
        "{} 讀取路徑 : {}",
        style("[3/6]").bold().dim(),
        "完成"
    ));

    pb_file_list.finish();
    return Ok(paths);
}

// 隔離文件
pub fn isolation_name(
    pb_filter_name: &ProgressBar,
    pb_copy_file: &ProgressBar,
    filter_path: &String,
    file: FilterDataStruct, // file_name: &str,
) -> std::io::Result<()> {
    let original_path = file.path.as_path();
    let dir_name = file.dir_name.to_owned();

    pb_filter_name.set_message(&format!(
        "{} 過濾名稱 : {:#?} - {}",
        style("[4/6]").bold().dim(),
        dir_name,
        "檢查隔離資料夾"
    ));

    // 檢查隔離資料夾
    let mut path = filter_path.to_owned();
    path.push_str("/");
    path.push_str(&file.dir_name);
    let filter_file_path = path.to_owned();
    check_dir(&filter_file_path)?;

    // 搬移隔離檔案
    pb_filter_name.set_message(&format!(
        "{} 過濾名稱 : {:#?} - {}",
        style("[4/6]").bold().dim(),
        dir_name,
        "檔案搬移"
    ));

    path.push_str("/");
    path.push_str(&file.file_name);
    let target_path = Path::new(&path);
    copy_file(pb_copy_file, original_path, target_path)?;

    pb_filter_name.set_message(&format!(
        "{} 過濾名稱 : {:#?} - {}",
        style("[4/6]").bold().dim(),
        file.dir_name,
        "建立記錄檔"
    ));

    // 記錄隔離資訊
    log::add(original_path, filter_file_path, file.file_name)?;

    // 移除原始檔案
    pb_filter_name.set_message(&format!(
        "{} 過濾名稱 : {:#?} - {}",
        style("[4/6]").bold().dim(),
        dir_name,
        "移除原始資料"
    ));
    remove_file(original_path)?;
    return Ok(());
}

// 隔離文件
pub fn isolation_extension(
    pb_filter_extension: &ProgressBar,
    pb_copy_file: &ProgressBar,
    filter_path: &String,
    file: FilterDataStruct, // file_name: &str,
) -> std::io::Result<()> {
    let original_path = file.path.as_path();
    let dir_name = file.dir_name.to_owned();
    pb_filter_extension.set_message(&format!(
        "{} 過濾副名 : {:#?} - {}",
        style("[5/6]").bold().dim(),
        dir_name,
        "檢查隔離資料夾"
    ));

    // 檢查隔離資料夾
    let mut path = filter_path.to_owned();
    path.push_str("/");
    path.push_str(&file.dir_name);
    let filter_file_path = path.to_owned();
    check_dir(&filter_file_path)?;

    // 搬移隔離檔案
    pb_filter_extension.set_message(&format!(
        "{} 過濾副名 : {:#?} - {}",
        style("[5/6]").bold().dim(),
        dir_name,
        "檔案搬移"
    ));

    path.push_str("/");
    path.push_str(&file.file_name);
    let target_path = Path::new(&path);
    copy_file(pb_copy_file, original_path, target_path)?;

    pb_filter_extension.set_message(&format!(
        "{} 過濾副名 : {:#?} - {}",
        style("[5/6]").bold().dim(),
        file.dir_name,
        "建立記錄檔"
    ));

    // 記錄隔離資訊
    log::add(original_path, filter_file_path, file.file_name)?;

    // 移除原始檔案
    pb_filter_extension.set_message(&format!(
        "{} 過濾副名 : {:#?} - {}",
        style("[5/6]").bold().dim(),
        dir_name,
        "移除原始資料"
    ));
    remove_file(original_path)?;
    return Ok(());
}
