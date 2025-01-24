mod date;
mod file;
mod text;

pub use date::*;

pub use file::*;

pub use text::*;

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::utils::{file, text};
    use crate::{calculated_range_time, parse_date_by_pattern, ClassifyCode};
    use anyhow::Ok;
    use serde_json::{json, Value};
    use std::path::Path;

    #[test]
    fn test_similarity_ratio() {
        let str = "weixin";
        let target = "weixin-client";
        let ratio = text::get_similarity_ratio(str, target);
        println!("相似度:{}", ratio)
    }

    #[test]
    fn test_read_file_yml() -> anyhow::Result<()> {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let config_path = Path::new(project_root).join("config.yaml");
        let path = config_path.to_str().unwrap();

        let config: Config = file::read_file_yml(path)?;
        println!("{:?}", config);
        Ok(())
    }

    #[test]
    fn test_convert_pathbuf2str() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        let path_str = path.to_str().expect("Failed to convert path to &str");
        // /Users/sakura/Desktop/rust/solicite-app/src-tauri/assets //macos
        println!("Path as &str: {}", path_str);
    }

    #[test]
    fn test_read_json_to_covert_obj() -> anyhow::Result<()> {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let config_path = Path::new(project_root)
            .join("assets")
            .join("json")
            .join("classify.json");
        let path = config_path.to_str().unwrap();

        let config: Value = file::read_file_json(path)?;

        let classify: ClassifyCode = serde_json::from_value(config)?;

        println!("{:?}", classify);
        Ok(())
    }

    #[test]
    fn test_read_file_json() -> anyhow::Result<()> {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let config_path = Path::new(project_root)
            .join("assets")
            .join("json")
            .join("resources.json");
        let path = config_path.to_str().unwrap();

        let config: Value = file::read_file_json(path)?;
        println!("{:?}", config);
        assert!(
            config["channel"].as_array().is_some(),
            "expected a non-empty array"
        );
        assert!(
            config["resources"].as_array().is_some(),
            "expected a non-empty array."
        );
        let channels = config["channel"].as_array().unwrap();
        channels.iter().for_each(|channel| {
            println!("channel: {:?}", channel);
        });

        let resources = config["resources"]
            .as_array()
            .expect("expected a non-empty array.");
        resources.into_iter().for_each(|resource| {
            let json = json!({
                 "query": {
                    "bool": {
                        "must": [
                            {
                                "terms": {
                                    "resource": resource
                                }
                            }
                        ]
                    }
                }
            });
            println!("{:#}", json);
        });
        Ok(())
    }

    #[test]
    fn test_parse_date_str() {
        let date_str = "2024-12-25";
        let date = parse_date_by_pattern(date_str, "%m月%d日").unwrap();
        println!("date:{}", date);
        let date_str = "2024-12-25 12:00:00";
        let date = parse_date_by_pattern(date_str, "%m月%d日 %H:%M:%S").unwrap();
        println!("date:{}", date);
    }

    #[test]
    fn test_calculated_days() {
        let start = Some(String::from("2025-01-20 09:00:00"));
        let end = Some(String::from("2025-01-26 09:00:01"));
        let start_str = start.clone().unwrap();
        let end_str = end.clone().unwrap();
        let days = crate::utils::calculated_days(start, end).unwrap();
        println!("days:{}", days);
        for i in 1..days {
            let (start_datetime, end_datetime) =
                calculated_range_time(start_str.as_str(), end_str.as_str(), i).unwrap();
            println!(
                "start_datetime:{}, end_datetime:{}",
                start_datetime, end_datetime
            );
        }
    }
}
