#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_chinese_number() {
        let number = 4123;
        let result = chinese_number(number);
        assert_eq!(result, "四千一百二十三");
    }

    #[test]
    fn to_chinese_num() {
        let number = 1234567;
        let result = chinese_number(number);
        assert_eq!(result, "一百二十三万四千五百六十七");
    }
    #[test]
    fn to_chinese_number_with_many_zero() {
        let number = 100001;
        let result = chinese_number(number);
        assert_eq!(result, "一十万零一");
    }

    #[test]
    fn section_to_chinese_test() {
        let number = 123;
        let result = section_to_chinese(number);
        assert_eq!(result, "一百二十三");
    }

    #[test]
    fn section_to_chinese_two_thousand() {
        let number = 2000;
        let result = section_to_chinese(number);
        assert_eq!(result, "二千")
    }
}

enum ChineseNumberUnit {
    Simple,
    Complex,
    SimpleMoney,
    ComplexMoney,
}

struct ChineseNumber<'a> {
    value: &'a str,
    unit: ChineseNumberUnit,
}

pub fn chinese_number(mut number: u32) -> String {
    let section_unit_char = ["", "万", "亿", "万亿"];
    let mut unit_pos = 0;
    let mut need_zero = false;
    let mut result = String::new();
    while number > 0 {
        let section = number % 10000;
        let s = section_to_chinese(section);
        result = if need_zero {
            format!("{s}{}{}{result}", section_unit_char[unit_pos], "零")
        } else {
            format!("{s}{}{result}", section_unit_char[unit_pos])
        };
        need_zero = is_need_zone(section);
        unit_pos += 1;
        number /= 10000;
    }
    return result;

    fn is_need_zone(number: u32) -> bool {
        number > 0 && number < 1000
    }
}

fn section_to_chinese(mut number: u32) -> String {
    let list = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
    let unit_char = ["", "十", "百", "千"];
    let mut unit_pos = 0;
    let mut result = String::new();
    let mut need_zone = true;
    while number > 0 {
        let pos = (number % 10) as usize;
        if pos == 0 {
            if number == 0 || !need_zone {
                need_zone = true;
                result = format!("{}{result}", list[0]);
            }
        } else {
            need_zone = false;
            result = format!("{}{}{result}", list[pos], unit_char[unit_pos]);
        }
        unit_pos += 1;
        number /= 10;
    }
    result
}
