#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_chinese_number_test() {
        let number = 4123;
        let result = ChineseNumber::from(number);
        assert_eq!(result.to_string(), "四千一百二十三");
        let result = result.to_complex();
        assert_eq!(result.to_string(), "肆仟壹佰贰拾叁")
    }

    #[test]
    fn to_chinese_num() {
        let number = 1234567;
        let result = ChineseNumber::from(number);
        assert_eq!(result.to_string(), "一百二十三万四千五百六十七");
        let n = ChineseNumber::from(result.to_string());
        assert_eq!(n.value, number);
    }
    #[test]
    fn to_chinese_number_with_many_zero() {
        let number = 100001;
        let result = ChineseNumber::from(number).to_string();
        assert_eq!(result, "一十万零一");
        let result = ChineseNumber::from(result);
        assert_eq!(result.value, number);
    }

    #[test]
    fn chinese_to_number_test() {
        let number = 1324;
        let chinese = ChineseNumber::from(number).to_string();
        let result = ChineseNumber::from(chinese);
        assert_eq!(result.value, number);
    }

    #[test]
    fn chinese_to_number_two_thousand() {
        let number = 2000;
        let chinese = ChineseNumber::from(number).to_string();
        let result = ChineseNumber::from(chinese);
        assert_eq!(result.value, number);
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ChineseNumberUnit {
    Simple,
    Complex,
    SimpleMoney,
    ComplexMoney,
}

#[derive(Clone, Debug)]
pub struct ChineseNumber {
    value: u32,
    unit: ChineseNumberUnit,
}

impl ToString for ChineseNumber {
    fn to_string(&self) -> String {
        match self.unit {
            ChineseNumberUnit::Simple => to(self.value, self.unit),
            ChineseNumberUnit::Complex => to(self.value, self.unit),
            ChineseNumberUnit::SimpleMoney => {
                let v = to(self.value, ChineseNumberUnit::Simple);
                format!("{v}元整")
            }
            ChineseNumberUnit::ComplexMoney => {
                let v = to(self.value, ChineseNumberUnit::Complex);
                format!("{v}元整")
            }
        }
    }
}

impl From<String> for ChineseNumber {
    fn from(str: String) -> Self {
        let string = format!("{str} ");
        let mut result = 0;
        let mut tmp = 0;
        for x in string.chars() {
            let char = &x.to_string();
            if is_number_char(char) {
                let n = index_number(char);
                if n.is_some() {
                    let v = index_number(char);
                    tmp = v.unwrap() as u32;
                }
            } else {
                let chinese_unit = get_chinese_unit(char);
                if let Some(..) = chinese_unit {
                    let cu = chinese_unit.unwrap();
                    let ChineseUnit(_, v, is_unit) = cu;
                    if is_unit {
                        let t = result + tmp;
                        result = t * v;
                    } else {
                        let t = tmp * v;
                        tmp = 0;
                        result += t;
                    }
                }
            }
        }
        ChineseNumber {
            value: result,
            unit: ChineseNumberUnit::Simple,
        }
    }
}

impl ChineseNumber {
    pub fn to_simple(&self) -> ChineseNumber {
        ChineseNumber {
            value: self.value,
            unit: ChineseNumberUnit::Simple,
        }
    }
    pub fn to_complex(&self) -> ChineseNumber {
        ChineseNumber {
            value: self.value,
            unit: ChineseNumberUnit::Complex,
        }
    }
    pub fn to_simple_money(&self) -> ChineseNumber {
        ChineseNumber {
            value: self.value,
            unit: ChineseNumberUnit::SimpleMoney,
        }
    }
    pub fn to_complex_money(&self) -> ChineseNumber {
        ChineseNumber {
            value: self.value,
            unit: ChineseNumberUnit::ComplexMoney,
        }
    }
}

impl From<u32> for ChineseNumber {
    fn from(i: u32) -> Self {
        ChineseNumber {
            value: i,
            unit: ChineseNumberUnit::Simple,
        }
    }
}

#[derive(Clone, Eq, PartialOrd, PartialEq, Ord)]
struct ChineseUnit(&'static str, u32, bool);

const CHINESE_UNITS: [ChineseUnit; 6] = [
    ChineseUnit(" ", 1, false),
    ChineseUnit("十", 10, false),
    ChineseUnit("百", 100, false),
    ChineseUnit("千", 1000, false),
    ChineseUnit("万", 10000, true),
    ChineseUnit("亿", 100000000, true),
];

fn get_chinese_unit(str: &str) -> Option<ChineseUnit> {
    CHINESE_UNITS.iter().find(|&x| x.0 == str).cloned()
}

const NUMBERS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
fn is_number_char(str: &str) -> bool {
    NUMBERS.contains(&str)
}

fn index_number(str: &str) -> Option<usize> {
    NUMBERS.iter().position(|&v| v == str)
}

fn to(mut number: u32, unit: ChineseNumberUnit) -> String {
    let section_unit_char = ["", "万", "亿", "万亿"];
    let mut unit_pos = 0;
    let mut need_zero = false;
    let mut result = String::new();
    while number > 0 {
        let section = number % 10000;
        let s = section_to(section, unit);
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

fn section_to(mut number: u32, unit: ChineseNumberUnit) -> String {
    let list = get_list_by_unit(unit);
    let unit_char = get_unit_char_by_unit(unit);
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

fn get_unit_char_by_unit(unit: ChineseNumberUnit) -> [&'static str; 4] {
    if unit == ChineseNumberUnit::Simple || unit == ChineseNumberUnit::SimpleMoney {
        ["", "十", "百", "千"]
    } else {
        ["", "拾", "佰", "仟"]
    }
}

fn get_list_by_unit(unit: ChineseNumberUnit) -> [&'static str; 10] {
    if unit == ChineseNumberUnit::Simple || unit == ChineseNumberUnit::SimpleMoney {
        ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"]
    } else {
        ["零", "壹", "贰", "叁", "肆", "伍", "陆", "柒", "捌", "玖"]
    }
}
