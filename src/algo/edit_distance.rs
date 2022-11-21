use std::cmp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit_distance_snow_with_sunny() {
        let source = "snow";
        let target = "sunny";
        let distance = edit_distance(source, target);
        assert_eq!(distance, 3);
    }
    #[test]
    fn edit_distance_hi_with_hello() {
        let source = "hi";
        let target = "hello";
        let distance = edit_distance(source, target);
        assert_eq!(distance, 4);
    }
}
pub fn edit_distance(source: &str, target: &str) -> u32 {
    let mut vec = init_array(source, target);
    let s_len = source.len();
    let t_len = target.len();
    for i in 1..s_len {
        for j in 1..t_len {
            let a = vec[i][j - 1] + 1;
            let b = vec[i - 1][j] + 1;
            let s1 = get_char(source, i);
            let s2 = get_char(target, j);
            let c = vec[i - 1][j - 1] + if s1 == s2 { 0 } else { 1 };
            vec[i][j] = min(a, b, c);
        }
    }
    vec[s_len - 1][t_len - 1] as u32
}

fn get_char(str: &str, index: usize) -> String {
    str.chars().nth(index).map(|x| x.to_string()).unwrap()
}

fn min(a: usize, b: usize, c: usize) -> usize {
    let min = cmp::min(a, b);
    cmp::min(min, c)
}

fn init_array(source: &str, target: &str) -> Vec<Vec<usize>> {
    let s_len = source.len();
    let t_len = target.len();
    let mut vec: Vec<Vec<usize>> = vec![vec![0; t_len]; s_len];
    for i in 0..s_len {
        vec[i][0] = i;
        for j in 0..t_len {
            vec[0][j] = j;
        }
    }
    vec
}
