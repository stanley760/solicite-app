mod solicite;

mod data;
mod excel;
mod tauri;

pub use data::*;
pub use excel::*;
pub use solicite::*;
pub use tauri::*;

#[cfg(test)]
mod tests {
    use crate::column_index_to_name;
    use crate::config::ExcelConfig;
    use crate::service::excel::{
        column_name_to_index, get_excel_book, get_excel_sheets, get_excel_workbook,
        read_excel_content_by_column,
    };
    use anyhow::Result;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use lazy_static::lazy_static;
    use serde_json::json;
    use std::path::Path;
    use umya_spreadsheet::{writer, Spreadsheet};

    lazy_static! {
        static ref CONFIG: ExcelConfig = ExcelConfig {
            channel_insert_start_column: None,
            channel_location: Some("B2".to_string()),
            sheet_name: Some("网络媒体平台".to_string()),
            category_insert_start_column: Some("C".to_string()),
            category_location: Some("B2".to_string()),
        };
    }

    #[test]
    fn test_parse_date_str() {
        let date_str = "2025-01-20T01:00:01.841Z";
        let datetime: DateTime<Utc> = date_str.parse().unwrap();
        let naive_datetime: NaiveDateTime = datetime.naive_utc();
        let date = naive_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{:?}", date);
    }

    #[test]
    fn test_serde_json() {
        let param = vec![1, 2, 3];
        let json_str = json!({"res": param});
        println!("{}", json_str)
    }

    #[test]
    fn test_excel_file() {
        let sheet: Spreadsheet = get_excel_book(r"C:\\Users\\sakura\\Desktop\\solicite-app\\src-tauri\\assets\\excel\\2025分渠道留言统计表.xlsx", "123").unwrap();
        let vec = get_excel_sheets(&sheet);
        println!("当前excel文件工作薄名称列表: {:?}", vec);
    }

    #[test]
    fn test_excel_cell() -> Result<()> {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let excel = Path::new(project_root)
            .join("assets")
            .join("excel")
            .join("2025省份数据统计表.xlsx");
        let mut sheet: Spreadsheet = get_excel_book(excel.to_str().unwrap(), "123abc")?;
        let worksheet = get_excel_workbook(&mut sheet, &CONFIG)?;
        let channel_location = CONFIG.clone().channel_location.unwrap().trim().to_string();
        let vec = read_excel_content_by_column(worksheet, channel_location);
        println!("当前省份内容:{:?}", vec);
        for i in 2..=115 {
            let cell_value = worksheet.get_cell_mut(format!("{}{}", "F", i));
            cell_value.set_value("测试");
        }

        let result =
            writer::xlsx::write(&sheet, excel.to_str().unwrap()).map_err(|e| e.to_string());

        if result.is_ok() {
            println!(">>>>>>>>>>>>>写入成功");
        } else {
            println!("写入失败");
        }
        Ok(())
    }

    #[test]
    fn test_column_name_to_index() {
        assert_eq!(25, column_name_to_index("Z").unwrap());
        assert_eq!(26, column_name_to_index("AA").unwrap());
        assert_eq!(51, column_name_to_index("AZ").unwrap());
        assert_eq!(77, column_name_to_index("BZ").unwrap());
        assert_eq!(702, column_name_to_index("AAA").unwrap());
    }

    #[test]
    fn test_column_index_to_name() {
        assert_eq!("Z", column_index_to_name(25));
        assert_eq!("AA", column_index_to_name(26));
        assert_eq!("AZ", column_index_to_name(51));
        assert_eq!("BZ", column_index_to_name(77));
        assert_eq!("AAA", column_index_to_name(702));
    }
}
