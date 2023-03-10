pub struct Solution;

impl Solution {
    fn reverse(x: i32) -> i32 {
        let mut input = x as i64;
        let mut output = 0 as i64;
        let mut digit = 0;

        while input != 0 {
            digit = input % 10;
            output = output * 10 + digit;
            input = input / 10;
        }
        let base: i64 = 2;
        let upper_bound: i64 = base.pow(31) - 1;
        let lower_bound: i64 = -base.pow(31);
        if output > upper_bound || output < lower_bound {
            return 0;
        }
        output as i32
    }
}

fn main() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(-123000), -321);
    let base: i64 = 2;
    assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
    assert_eq!(Solution::reverse((-base.pow(31)) as i32), 0);
}
