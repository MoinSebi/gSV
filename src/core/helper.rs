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
pub fn hashset2string(input: &HashSet<u32>, sep:  &str) -> String{
    let j:Vec<String> = input.iter().map(|i| i.to_string()).collect();
    j.join(sep)
}