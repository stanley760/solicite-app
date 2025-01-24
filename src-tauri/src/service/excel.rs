use crate::{config::ExcelConfig, error::ExcelError};
use anyhow::{anyhow, Context, Ok};
use office_crypto::{decrypt_from_bytes, decrypt_from_file};
use tracing::info;
use std::io::Cursor;
use std::path::Path;
use umya_spreadsheet::{reader, CellRawValue, Spreadsheet, Worksheet};

pub fn get_excel_book(file_path: &str, password: &str) -> anyhow::Result<Spreadsheet> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(ExcelError::FileNotFound.into());
    }
    if !path.is_file() {
        return Err(ExcelError::NotAFile.into());
    }
    let book: Result<Spreadsheet, anyhow::Error> =
        reader::xlsx::read(path).context(ExcelError::ReadError);
    let sheet: Spreadsheet = if book.is_ok() {
        book?
    } else {
        let ret = decrypt_from_file(path, password).context(ExcelError::DecryptError)?;

        reader::xlsx::read_reader(Cursor::new(ret), true).context(ExcelError::DecryptError)?
    };
    Ok(sheet)
}
pub fn get_excel_book_by_content(content: Vec<u8>, password: &str) -> Spreadsheet {
    let read = reader::xlsx::read_reader(Cursor::new(content.clone()), true);
    let book = if read.is_err() {
        let ret = decrypt_from_bytes(content, password).unwrap();
        reader::xlsx::read_reader(Cursor::new(ret), true).unwrap()
    } else {
        read.unwrap()
    };
    book
}

pub fn get_excel_workbook<'a>(
    book: &'a mut Spreadsheet,
    excel_config: &ExcelConfig,
) -> anyhow::Result<&'a mut Worksheet> {
    let sheet_name = match excel_config.clone().sheet_name {
        Some(name) => name,
        None => return Err(anyhow!(ExcelError::WorkbookNotNull)),
    };
    let position = book
        .get_sheet_collection()
        .iter()
        .position(|sheet| sheet.get_name() == sheet_name)
        .ok_or_else(|| anyhow!(ExcelError::WorkbookNotExisted))?;

    let worksheet = book
        .get_sheet_mut(&position)
        .ok_or_else(|| anyhow!(ExcelError::WorkbookError("".to_string())))?;

    Ok(worksheet)
}

pub fn read_excel_content_by_column(worksheet: &Worksheet, location: String) -> Vec<String> {
    let (_, last_column_idx, column_code) =
        get_excel_column_range_index(worksheet, location.clone());
    let cell_range_location = format!("{}:{}{}", location, column_code, last_column_idx);
    info!("统计单元格范围: {}", cell_range_location);
    let mut title_vec = Vec::new();
    for row in worksheet.get_cell_value_by_range(cell_range_location.as_str()) {
        let value = row.get_raw_value();

        match value {
            CellRawValue::String(va) => {
                title_vec.push((*va).trim().to_string());
            }
            CellRawValue::RichText(val) => {
                title_vec.push(val.get_text().trim().to_string());
            }
            _ => {}
        }
    }
    title_vec
}

pub fn get_excel_column_range_index(
    worksheet: &Worksheet,
    location: String,
) -> (usize, usize, String) {
    let (column_code, column_numb): (String, String) =
        location.chars().partition(|c| c.is_alphabetic());
    let idx = column_name_to_index(&column_code).unwrap();
    let start_column_numb = column_numb.parse::<usize>().unwrap();
    let last_column_idx = worksheet.get_collection_by_column(&idx).len();
    (start_column_numb, last_column_idx, column_code)
}

pub(crate) fn column_name_to_index(name: &str) -> Option<u32> {
    let mut index = 0;
    // AZ 26 + 25
    for (i, c) in name.trim().chars().rev().enumerate() {
        let current_val: u32 = (c.to_ascii_uppercase() as u8 - b'A' as u8 + 1) as u32;
        index += 26_u32.pow(i as u32) * current_val;
    }
    Some(index - 1)
}

pub fn column_index_to_name(index: u32) -> String {
    let mut result = String::new();
    let mut index = index + 1;
    while index > 0 {
        let c = ((index - 1) % 26) as u8 + b'A';
        result.push(c as char);
        index = (index - 1) / 26;
    }
    result.chars().rev().collect()
}

pub fn get_excel_sheets(book: &Spreadsheet) -> Vec<String> {
    let all_sheet = book.get_sheet_collection();
    all_sheet
        .into_iter()
        .map(|x| x.get_name().to_string())
        .collect()
}
