use chinese_number::{ChineseCountMethod, ChineseToNumber};

use crate::tokens::Token;

#[rustfmt::skip]
static NUMBERS: [char; 44] = [
    '零', '〇', '0', 
    '一', '壹', '1',
    '二', '貳', '贰', '貮', '兩', '两', '2',
    '三', '參', '叁', '叄', '参', '3',
    '四', '肆', '4',
    '五', '伍', '5',
    '六', '陸', '陆', '6',
    '七', '柒', '7',
    '八', '捌', '8',
    '九', '玖', '9',
    '十', '拾',
    '百', '佰',
    '千', '仟', 
];

pub(crate) trait CnAnNum {
    fn has_number(&self) -> bool;
    fn auto_parse_u16(&self) -> Option<u16>;
}

impl CnAnNum for Token {
    fn has_number(&self) -> bool {
        self.to_text().has_number()
    }

    fn auto_parse_u16(&self) -> Option<u16> {
        self.to_text().auto_parse_u16()
    }
}

impl CnAnNum for str {
    fn has_number(&self) -> bool {
        self.contains(&NUMBERS)
    }

    fn auto_parse_u16(&self) -> Option<u16> {
        self.parse()
            .ok()
            .or_else(|| self.to_number(ChineseCountMethod::TenThousand).ok())
            .or_else(|| self.to_number_naive().ok())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chinese_num_lib_test() {
        let num: i32 = "三万五".to_number(ChineseCountMethod::TenThousand).unwrap();
        println!("{num}");
    }

    #[test]
    fn parse_chinese_num_test() {
        assert_eq!("一千零一十三".auto_parse_u16(), Some(1013u16));
    }
}
