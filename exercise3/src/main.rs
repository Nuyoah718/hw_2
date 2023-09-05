fn main() {
    let v = vec!['a', 'b', 'c', 'd', 'e'];
    let v: Vec<char> = v
        .iter()
        .map(|c| {
            let mut num: u8 = *c as u8;
            num += 1;
            num as char
        })
        .collect();
    assert_eq!(v, vec!['b', 'c', 'd', 'e', 'f']);
}