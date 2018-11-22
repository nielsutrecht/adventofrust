use std::vec::Vec;

pub fn string_to_digits(s: String) -> Vec<u32> {
    let mut vec = Vec::with_capacity(s.len());

    for c in s.chars() {
        vec.push(c.to_digit(10).expect("Parse error"))
    }

    return vec;
}

pub fn vec_to_string<T : ToString>(v: &[T]) -> String {
    let mut out = String::new();

    for e in v {
        out += &e.to_string()
    }

    return out
}

#[test]
fn string_to_digits_test() {
    let vec = string_to_digits("1234".to_owned());
    assert_eq!(vec, vec![1,2,3,4]);
}

#[test]
fn vec_to_string_test() {
    let s = vec_to_string(vec![1,2,3,4].as_slice());

    assert_eq!(s, "1234");
}
