use regex::Regex;

fn compare(str: &str, target: &str) -> i32 {
    let n = str.len();
    let m = target.len();

    let mut d = vec![vec![0; m + 1]; n + 1];

    // 初始化第一列
    for i in 0..=n {
        d[i][0] = i as i32;
    }
    // 初始化第一行
    for j in 0..=m {
        d[0][j] = j as i32;
    }

    for i in 1..=n {
        if let Some(ch1) = str.chars().nth(i - 1) {
            for j in 1..=m {
                if let Some(ch2) = target.chars().nth(j - 1) {
                    let temp = if ch1 == ch2 || ch1.to_ascii_lowercase() == ch2.to_ascii_lowercase()
                    {
                        0
                    } else {
                        1
                    };
                    d[i][j] = min(d[i - 1][j] + 1, d[i][j - 1] + 1, d[i - 1][j - 1] + temp);
                } else {
                    // 处理 target 字符串为空的情况
                    d[i][j] = d[i - 1][j] + 1;
                }
            }
        } else {
            // 处理 str 字符串为空的情况
            for j in 1..=m {
                d[i][j] = d[i][j - 1] + 1;
            }
        }
    }

    d[n][m]
}

fn min(one: i32, two: i32, three: i32) -> i32 {
    one.min(two).min(three)
}

pub fn get_similarity_ratio(str: &str, target: &str) -> i32 {
    let reg_ex = Regex::new(r"[^a-zA-Z_\u4e00-\u9fa5]").unwrap();
    let str_clean = reg_ex.replace_all(str, "").trim().to_string();
    let target_clean = reg_ex.replace_all(target, "").trim().to_string();
    let max = str_clean.len().max(target_clean.len());
    let similarity = 1.0 - compare(&str_clean, &target_clean) as f32 / max as f32;
    (similarity.abs() * 100.0) as i32
}
