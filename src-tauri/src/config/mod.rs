mod config;
mod es;

pub use es::create_client;

pub use config::{Config, CountConfig, EsConfig, ExcelConfig};

#[cfg(test)]
mod tests {
    #[test]
    fn test_join() -> anyhow::Result<(), String> {
        let channel_total_location = vec!["B2", "B115"];
        let column_range_str = channel_total_location.join(":");
        println!("{}", column_range_str);
        Ok(())
    }
}
