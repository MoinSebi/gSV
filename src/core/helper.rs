use std::collections::HashSet;


/// Bool to direction
///
/// Path information (+/-)
pub fn bool2string_dir(b: bool) -> String{
    if b{
        return "+".to_string();
    } else {
        return "-".to_string();
    }
}


#[allow(dead_code)]
/// Vector to String
pub fn vec2string(input: &Vec<u32>, sep:  &str) -> String{
    let j:Vec<String> = input.iter().map(|i| i.to_string()).collect();
    j.join(sep)
}


/// HashSet to String
/// This is not always in the same order
pub fn hashset2string(input: &HashSet<u32>, sep:  &str) -> String{
    let j:Vec<String> = input.iter().map(|i| i.to_string()).collect();
    j.join(sep)
}

#[cfg(test)]
mod tests {
    use crate::core::helper::{bool2string_dir, vec2string, hashset2string};
    use std::collections::HashSet;

    #[test]
    fn helpers() {
        assert_eq!(bool2string_dir(true), "+");
        let k: Vec<u32> = vec![1,2,3,4];
        let mut k2: HashSet<u32> = HashSet::new();
        k2.insert(10);
        k2.insert(11);
        assert_eq!(vec2string(&k, "."), "1.2.3.4".to_string());
    }
}