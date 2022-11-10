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
        assert_eq!(result, "十万零一");
    }
}

fn chinese_number(number: i32) -> String {
    let list = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
    let utils = [
        "", "十", "百", "千", "万", "十", "百", "千", "亿", "十", "百", "千", "万", "十", "百",
        "千",
    ];
    let s = reverse(&number.to_string());
    let mut s = s
        .chars()
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .map(|x| list.get(x).unwrap())
        .zip(utils.iter())
        .map(|(&a, &b)| format!("{a}{b}"))
        .collect::<Vec<String>>();
    s.reverse();
    let s = s.join("");
    println!("{:?}", s);
    s
}

fn reverse(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars().rev() {
        result.push(c);
    }
    result
}
