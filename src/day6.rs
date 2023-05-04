use std::{ str:: from_utf8, collections::HashSet};
fn check_if_unique( four_bytes : &[u8]) -> bool{
    let mut hs = HashSet ::new();
    for elem in four_bytes.iter() {
        if !(hs.insert(elem)) {
            return false;
        }
    }
    let s = from_utf8(four_bytes);
    println!("{}",s.unwrap());
    return true;
}
pub fn day6() {
    let input =include_str!("./day6.txt");
    let answer = input.as_bytes()
        .windows(14).position(|window| check_if_unique(window));
    println!("{}",answer.unwrap())
}
