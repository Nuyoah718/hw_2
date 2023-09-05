fn main() {
    let s1: &str = "hello world";
    let s2: &str = "hello world";
    let s3: &str = "abc";
    let s4: &str = "abc123";
    let s5: &str = "abcd12";
    let s6: &str = "abcd34";
    let r1 = compare_string(s1, s2);
    let r2 = compare_string(s3, s4);
    let r3 = compare_string(s5, s6);
    println!("{}", r1);
    println!("{}", r2);
    println!("{}", r3);
}

fn compare_string(x: &str, y: &str) -> bool {
    if x.len() != y.len() {
        return false;
    }
    for i in 0..x.len() {
        if x.chars().nth(i) != y.chars().nth(i) {
            return false;
        }
    }
    return true;
}
