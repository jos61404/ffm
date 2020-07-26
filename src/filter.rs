use crate::file::{isolation_extension, isolation_name};
use console::style;
use indicatif::ProgressBar;
use std::convert::TryInto;
use std::path::*;

#[derive(Debug)]
pub struct FilterDataStruct {
    pub path: PathBuf,
    pub dir_name: String,
    pub file_name: String,
}

// 過濾名稱
pub fn name(
    pb_filter_name: ProgressBar,
    pb_copy_file: &ProgressBar,
    filter_path: &String,
    paths: &Vec<PathBuf>,
    filters: &Vec<String>,
) -> std::io::Result<Vec<FilterDataStruct>> {
    let mut filter_data: Vec<FilterDataStruct> = Vec::new();
    pb_filter_name.set_length(paths.len().try_into().unwrap());

    for path in paths {
        let file_stem = path.file_stem().unwrap();
        pb_filter_name.set_message(&format!(
            "{} 過濾名稱 : {:#?}",
            style("[4/6]").bold().dim(),
            file_stem
        ));
        pb_filter_name.inc(1);

        for filter in filters {
            let filter = filter.as_str();
            let file_stem = file_stem.to_str().unwrap();

            if filter == file_stem {
                let file_name = path.file_name().unwrap().to_str().unwrap();

                filter_data.push(FilterDataStruct {
                    path: path.to_owned(),
                    dir_name: filter.to_string(),
                    file_name: file_name.to_string(),
                });

                isolation_name(
                    &pb_filter_name,
                    &pb_copy_file,
                    filter_path,
                    FilterDataStruct {
                        path: path.to_owned(),
                        dir_name: filter.to_string(),
                        file_name: file_name.to_string(),
                    },
                )?;
            }
        }
    }
    pb_filter_name.set_message(&format!(
        "{} 過濾名稱 : {}",
        style("[4/6]").bold().dim(),
        "完成"
    ));
    pb_filter_name.finish();
    return Ok(filter_data);
}

// 過濾檔案副名稱
pub fn extension(
    pb_filter_extension: ProgressBar,
    pb_copy_file: &ProgressBar,
    filter_path: &String,
    paths: &Vec<PathBuf>,
    filters: &Vec<String>,
) -> std::io::Result<Vec<FilterDataStruct>> {
    let mut filter_data: Vec<FilterDataStruct> = Vec::new();
    pb_filter_extension.set_length(paths.len().try_into().unwrap());

    for path in paths {
        let file_stem = path.file_stem().unwrap();
        pb_filter_extension.set_message(&format!(
            "{} 過濾副名 : {:#?}",
            style("[5/6]").bold().dim(),
            file_stem
        ));
        pb_filter_extension.inc(1);

        // 判斷是否有副檔名
        if path.exists() == true && path.extension().is_some() == true {
            let file_extension = path.extension().unwrap();

            for filter in filters {
                let filter = filter.as_str();
                let file_extension = file_extension.to_str().unwrap();

                if filter == file_extension {
                    filter_data.push(FilterDataStruct {
                        path: path.to_owned(),
                        dir_name: filter.to_string(),
                        file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
                    });

                    isolation_extension(
                        &pb_filter_extension,
                        &pb_copy_file,
                        filter_path,
                        FilterDataStruct {
                            path: path.to_owned(),
                            dir_name: filter.to_string(),
                            file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
                        },
                    )?;
                }
            }
        }
    }

    pb_filter_extension.set_message(&format!(
        "{} 過濾副名 : {}",
        style("[5/6]").bold().dim(),
        "完成"
    ));
    pb_filter_extension.finish();

    return Ok(filter_data);
}
