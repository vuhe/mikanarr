use chinese_number::{ChineseCountMethod, ChineseToNumber};

#[rustfmt::skip]
static NUMBERS: [char; 43] = [
    '零' , '0' , '〇' ,
    '一' , '壹' , '1' ,
    '二' , '貳' , '贰' , '貮' , '兩' , '2' ,
    '三' , '參' , '叁' , '叄' , '参' , '3' ,
    '四' , '肆' , '4' ,
    '五' , '伍' , '5' ,
    '六' , '陸' , '陆' , '6' ,
    '七' , '柒' , '7' ,
    '八' , '捌' , '8' ,
    '九' , '玖' , '9' ,
    '十' , '拾' ,
    '佰', '百',
    '仟', '千'
];

pub(crate) trait AutoParseU16 {
    fn has_number(&self) -> bool;
    fn auto_parse_u16(&self) -> Option<u16>;
}

impl AutoParseU16 for str {
    fn has_number(&self) -> bool {
        self.contains(&NUMBERS)
    }

    fn auto_parse_u16(&self) -> Option<u16> {
        if let Ok(number) = self.parse() {
            return Some(number);
        }
        self.to_number(ChineseCountMethod::TenThousand).ok()
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
