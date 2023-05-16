pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut char = s.chars().collect::<Vec<char>>();
        
    }
}

fn main() {
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI"
    );
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR"
    );
    assert_eq!(Solution::convert("A".to_string(), 1), "A");
    assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
}