use lazy_regex::regex_captures;

/// 中文或者阿拉伯数字转 u16
/// 由于集数最大设定为 1890，因此本函数会在此限制条件下进行解析
pub(crate) fn chinese_or_ascii_num_to_u16(num: &str) -> Option<u16> {
    if let Ok(number) = num.parse() {
        return Some(number);
    }
    if num == "零" {
        return Some(0);
    }

    let (_, u1, u2, n2, u3, n3, n4) = match regex_captures!(
        "(一?千)?零?\
        (([一二三四五六七八九])?百)?零?\
        (([一二三四五六七八九])?十)?\
        ([一二三四五六七八九])?",
        num
    ) {
        None => return None,
        Some(it) => it,
    };

    let mut number: u16 = 0;
    if !u1.is_empty() {
        number = number + 1000;
    }
    if !u2.is_empty() {
        number = number + 100 * number_map(n2).unwrap_or(1);
    }
    if !u3.is_empty() {
        number = number + 10 * number_map(n3).unwrap_or(1);
    }
    if !n4.is_empty() {
        number = number + number_map(n4).unwrap_or(1);
    }
    return Some(number);
}

fn number_map(str: &str) -> Option<u16> {
    match str {
        "一" => Some(1),
        "二" => Some(2),
        "三" => Some(3),
        "四" => Some(4),
        "五" => Some(5),
        "六" => Some(6),
        "七" => Some(7),
        "八" => Some(8),
        "九" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_chinese_num_test() {
        let num = super::chinese_or_ascii_num_to_u16("一千零十三");
        assert_eq!(num, Some(1013u16));
    }
}
