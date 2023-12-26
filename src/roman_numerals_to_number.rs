/*
13. 罗马数字转整数

罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。

字符          数值
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
例如， 罗马数字 2 写做 II ，即为两个并列的 1 。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。

通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：

I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
给定一个罗马数字，将其转换成整数。
*/

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut iter = s.chars().peekable();

        while let Some(c) = iter.next() {
            let next_c = iter.peek();

            let value = match (c, next_c) {
                ('I', Some('V')) => {
                    iter.next();
                    4
                }
                ('I', Some('X')) => {
                    iter.next();
                    9
                }
                ('X', Some('L')) => {
                    iter.next();
                    40
                }
                ('X', Some('C')) => {
                    iter.next();
                    90
                }
                ('C', Some('D')) => {
                    iter.next();
                    400
                }
                ('C', Some('M')) => {
                    iter.next();
                    900
                }
                ('I', _) => 1,
                ('V', _) => 5,
                ('X', _) => 10,
                ('L', _) => 50,
                ('C', _) => 100,
                ('D', _) => 500,
                ('M', _) => 1000,
                _ => 0,
            };
            result += value;
        }
        result
    }
}
