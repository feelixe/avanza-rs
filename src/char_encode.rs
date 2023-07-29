pub(crate) fn char_encode(num: u32) -> String {
    let mut s = String::new();
    let mut n = num;
    while n > 0 {
        let digit = (n % 10) as u8;
        s.push(('a' as u8 + digit) as char);
        n /= 10;
    }
    s.chars().rev().collect()
}