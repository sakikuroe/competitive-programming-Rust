use cargo_snippet::snippet;

#[snippet(name = "kaibun")]
fn is_palindrome(s: &Vec<char>) -> bool {
        let length = s.len();
        for i in 0..length {
                if !(s[i] == s[(length - 1) - i]) {
                        return false;
                }
        }

        true
}

fn main() {
        println!("Hello, world!");
}
