use std::vec::Vec;

pub fn string_to_digits(s: String) -> Vec<u32> {
    let mut vec = Vec::with_capacity(s.len());

    for c in s.chars() {
        vec.push(c.to_digit(10).expect("Parse error"))
    }

    return vec;
}

pub fn string_vec_to_ints(v: Vec<String>) -> Vec<Vec<i32>> {
    return v.into_iter()
        .map(|s| {
            s.split_whitespace()
                .into_iter()
                .map(|e | e.parse::<i32>().expect("Parse error"))
                .collect()
        }).collect()
}

pub fn vec_to_string<T : ToString>(v: &[T]) -> String {
    let mut out = String::new();

    for e in v {
        out += &e.to_string()
    }

    return out
}

#[cfg(test)]
mod tests {
    #[test]
    fn string_to_digits() {
        let vec = super::string_to_digits("1234".to_owned());
        assert_eq!(vec, vec![1, 2, 3, 4]);
    }

    #[test]
    fn vec_to_string() {
        let s = super::vec_to_string(vec![1, 2, 3, 4].as_slice());

        assert_eq!(s, "1234");
    }

    #[test]
    fn string_vec_to_ints() {
        let vec: Vec<String> = vec!["1234 456".to_string(), "890 -123".to_string()];
        let result = super::string_vec_to_ints(vec);

        assert_eq!(1234, result[0][0]);
        assert_eq!(-123, result[01][1]);
    }
}
