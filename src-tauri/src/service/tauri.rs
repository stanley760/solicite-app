use crate::config::*;
use crate::domain::*;
use crate::error::*;
use crate::service::*;
use crate::utils::*;
use crate::{domain::ResultWrapper, Config};
use ::tauri::command;
use hashbrown::HashMap;
use std::path::{Path, PathBuf};
use tracing::info;
use umya_spreadsheet::{writer, Worksheet};

#[command]
pub async fn handle_excel<'a>(params: ExcelParams<'a>) -> Result<(), String> {
    let params = convert_param(params)?;
    // 1.读取yml配置
    let project_root = env!("CARGO_MANIFEST_DIR");
    let config_path = Path::new(project_root)
        .join("assets")
        .join("yml")
        .join("config.yaml");
    let config_path = config_path.to_str().ok_or("文件不存在".to_string())?;

    let config: Config = read_file_yml(config_path).map_err(|_| "读取yaml文件失败")?;
    let client: elasticsearch::Elasticsearch =
        create_client(&config).await.map_err(|e| e.to_string())?;
    // 2.judge the field of stat which is the type of
    //   analysis to different excel file that read and write data.
    match params.stat {
        1 => handle_excel_count_all_channel(config, params, client).await,
        2 => handle_excel_agg_all_categories(config, params, client, project_root).await,
        _ => Err("未知的类型".to_string()),
    }
}

async fn handle_excel_count_all_channel<'a>(
    config: Config,
    params: ExcelParams<'a>,
    client: elasticsearch::Elasticsearch,
) -> Result<(), String> {
    let mut excel_config = config.clone().excel;

    let start_column_name = excel_config
        .clone()
        .channel_insert_start_column
        .ok_or("配置渠道插入数据起始列不能为空".to_string())?;
    excel_config.sheet_name = Some(String::from(params.sheet_name));

    let channel_location = excel_config
        .clone()
        .channel_location
        .ok_or("配置渠道起始单元位置不能为空".to_string())?
        .trim()
        .to_string();

    let mut sheet = get_excel_book(params.path, params.password).map_err(|e| e.to_string())?;
    let worksheet: &mut Worksheet =
        get_excel_workbook(&mut sheet, &excel_config).map_err(|e| e.to_string())?;
    let vec: Vec<String> = read_excel_content_by_column(worksheet, channel_location.clone());
    // =======================================================
    // 6. get the data from the elasticsearch by one day
    let mut count_config = config.clone().count;
    let times = params.time_range;
    count_config.start_time = times.get(0).cloned();
    count_config.end_time = times.get(1).cloned();

    let days = calculated_days(
        count_config.clone().start_time,
        count_config.clone().end_time,
    )
    .map_err(|e| e.to_string())?;
    for k in 1..=days {
        let (start_datetime, end_datetime) =
            calculated_range_time(times.get(0).unwrap(), times.get(1).unwrap(), k)
                .map_err(|e| e.to_string())?;
        count_config.start_time = Some(start_datetime);
        count_config.end_time = Some(end_datetime);
        let arr = count_all_channel(client.clone(), &mut count_config)
            .await
            .map_err(|e| e.to_string())?;

        // 7. handle the data from the elasticsearch with compare with excel title,
        // and get the result by the sort of the excel file.
        let res: Vec<String> = handle_data(vec.clone(), arr);
        let result: Vec<String> =
            assemble_column(res, count_config.start_time).map_err(|e| e.to_string())?;
        info!("统计数据结果: {:?}", result);
        // 插入指定列对应行数的数据
        let (start_row_numb, end_row_numb, _) =
            get_excel_column_range_index(worksheet, channel_location.clone());
        let start_index = column_name_to_index(start_column_name.as_str())
            .ok_or("列名转换索引失败".to_string())?;
        let current_column_name = column_index_to_name(start_index + (k as u32) - 1);

        // 获取行
        for i in start_row_numb..=end_row_numb {
            let current_unit = format!("{}{}", current_column_name, i);
            let cell_value = worksheet.get_cell_mut(current_unit);
            let data = result.get(i - start_row_numb);
            if data.is_some() {
                cell_value.set_value(data.unwrap());
            }
        }
    }
    let result = writer::xlsx::write(&sheet, params.path)
        .map_err(|_| "写入excel失败,当前文档已被其他程序进程占用".to_string());

    result
}

fn convert_param(mut params: ExcelParams) -> Result<ExcelParams, String> {
    let time_range: Vec<String> = params
        .time_range
        .iter()
        .map(|item| parse_date_by_pattern(item, "%Y-%m-%d %H:%M:%S"))
        .collect::<Result<Vec<String>, String>>()?;
    params.time_range = time_range;
    Ok(params)
}

async fn handle_excel_agg_all_categories<'a>(
    config: Config,
    params: ExcelParams<'a>,
    client: elasticsearch::Elasticsearch,
    project_root: &str,
) -> Result<(), String> {
    let mut excel = config.clone().excel;
    let mut count = config.count;

    let data_path = count
        .clone()
        .category_path
        .ok_or("配置分类文件路径不能为空".to_string())?;

    let start_column_name = excel
        .clone()
        .category_insert_start_column
        .ok_or("配置分类插入数据起始列不能为空".to_string())?;

    excel.sheet_name = Some(String::from(params.sheet_name));

    let category_location = excel
        .clone()
        .category_location
        .ok_or("配置分类起始单元位置不能为空".to_string())?
        .trim()
        .to_string();

    // 1. read the file that named resource.json
    let resource_path = Path::new(project_root)
        .join("assets")
        .join("json")
        .join(data_path);
    let resource_path = resource_path.to_str().ok_or("文件不存在".to_string())?;

    let classify_val = read_file_json(resource_path).map_err(|_| "读取json文件失败".to_string())?;
    let classify: ClassifyCode =
        serde_json::from_value(classify_val).map_err(|_| "解析json文件失败".to_string())?;
    // 2. covert the map which contains codes and names
    // to mapping the file that the column in title .
    let children: HashMap<String, String> = classify
        .children
        .into_iter()
        .map(|item| (item.code, item.name))
        .collect();
    // 3. read the file and get the data by the column
    let times = params.time_range;
    count.start_time = times.get(0).cloned();
    count.end_time = times.get(1).cloned();

    let data = agg_all_categories(client, count)
        .await
        .map_err(|e| e.to_string())?;
    // the final data that would be filled into the excel file
    let data_map: HashMap<String, i64> = children
        .clone()
        .into_iter()
        .map(|(code, name)| {
            let val = data.get(&code).unwrap_or(&0).clone();
            (name, val)
        })
        .collect();

    let total: i64 = data.values().sum();

    let percentage_map: HashMap<String, f64> = data
        .into_iter()
        .map(|(key, value)| {
            let percentage = (value as f64 / total as f64) * 100.0;
            let formatted = format!("{:.3}", percentage);
            let val: f64 = formatted.parse().unwrap();
            (key, val)
        })
        .collect();

    let percentage_data_map: HashMap<String, f64>  = children
        .into_iter()
        .map(|(code, name)| {
            let val = percentage_map.get(&code).unwrap_or(&0_f64).clone();
            (name, val)
        })
        .collect();

    info!("percent map: {:?}", percentage_data_map);
    // 4. set the data to fill the unit in excel file
    let mut sheet = get_excel_book(params.path, params.password).map_err(|e| e.to_string())?;

    let worksheet: &mut Worksheet =
        get_excel_workbook(&mut sheet, &excel).map_err(|e| e.to_string())?;

    let data_excel: Vec<String> =
        read_excel_content_by_column(worksheet, category_location.clone());

    let (start_row_numb, end_row_numb, _) =
        get_excel_column_range_index(worksheet, category_location.clone());
    for i in start_row_numb..=end_row_numb {
        let current_unit = format!("{}{}", start_column_name, i);
        let cell_value = worksheet.get_cell_mut(current_unit);
        let data = data_excel.get(i - start_row_numb);
        if data.is_some() {
            let name: &String = data.unwrap();
            let value: i64 = *data_map.get(name).unwrap();
            cell_value.set_value(value.to_string());
        }
        // calculated the percent of each category.
        let start_index = column_name_to_index(start_column_name.as_str())
            .ok_or("列名转换索引失败".to_string())?;
        let current_column_name = column_index_to_name(start_index + 1_u32);
        let per_unit = format!("{}{}", current_column_name, i);
        let per_cell_value = worksheet.get_cell_mut(per_unit);

        let data = data_excel.get(i - start_row_numb);
        if data.is_some() {
            let name: &String = data.unwrap();
            let value = percentage_data_map.get(name);
            if value.is_some() {
                per_cell_value.set_value(format!("{}%", value.unwrap()));
            }
        }
    }

    let result = writer::xlsx::write(&sheet, params.path)
        .map_err(|_| "写入excel失败,当前文档已被其他程序进程占用".to_string());

    result
}

#[command]
pub fn update_config(key: u8, content: &str) -> ResultWrapper<bool> {
    info!("更新文件类型: {}, 更新内容: {}", key, content);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");

    let source = match_file_path(root, key);
    let path_str = source.to_str().unwrap();
    let result = update_file_by_content(path_str, content);

    match result {
        Ok(res) => ResultWrapper {
            code: 0,
            message: res,
            data: Some(true),
        },
        Err(err) => {
            let code = match err.downcast_ref::<FileError>() {
                Some(FileError::FileNotFound) => 201,
                Some(FileError::ConfigFileNotFound) => 201,
                Some(FileError::NotAFile) => 202,
                Some(FileError::FileNameUTF8Error) => 203,
                Some(FileError::SaveContentNotNull) => 201,
                Some(FileError::WriteError) => 201,
                Some(FileError::ReadError) => 201,
                Some(FileError::JsonDeserializeError(_)) => 204,
                Some(FileError::YamlDeserializeError(_)) => 205,
                None => 200,
            };
            ResultWrapper {
                code,
                message: err.to_string(),
                data: None,
            }
        }
    }
}

#[command]
pub fn read_excel_sheet_names(path: &str, password: &str) -> ResultWrapper<Vec<String>> {
    info!(
        "invoking the sheet‘s reader, path: {}, password: {}",
        path, password
    );
    match get_excel_book(path, password) {
        Ok(sheet) => {
            let vec = get_excel_sheets(&sheet);
            ResultWrapper {
                code: 0,
                message: "ok".to_string(),
                data: Some(vec),
            }
        }
        Err(e) => {
            let code = match e.downcast_ref::<ExcelError>() {
                Some(ExcelError::FileNotFound) => 101,
                Some(ExcelError::NotAFile) => 102,
                Some(ExcelError::ReadError) => 103,
                Some(ExcelError::DecryptError) => 104,
                Some(ExcelError::WorkbookNotNull) => 105,
                Some(ExcelError::WorkbookNotExisted) => 106,
                Some(ExcelError::WorkbookError(_)) => 107,
                None => 100,
            };
            ResultWrapper {
                code,
                message: e.to_string(),
                data: None,
            }
        }
    }
}

#[command]
pub fn read_config_by_type(key: u8) -> ResultWrapper<String> {
    info!("读取文件类型: {}", key);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");

    let source = match_file_path(root, key);

    match read_file_content(source.as_path()) {
        Ok(content) => ResultWrapper {
            code: 0,
            message: "ok".to_string(),
            data: Some(content),
        },
        Err(e) => ResultWrapper {
            code: 100,
            message: e.to_string(),
            data: None,
        },
    }
}

fn match_file_path(root: PathBuf, key: u8) -> PathBuf {
    match key {
        1 => root.join("json").join("resources.json"),
        2 => root.join("json").join("classify.json"),
        3 => root.join("yml").join("config.yaml"),

        _ => root.to_path_buf(),
    }
}
