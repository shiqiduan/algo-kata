use crate::leetcode::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut index = 0;
        'o: loop {
            let mut s = None;
            for x in strs.iter() {
                match x.get(index..index + 1) {
                    Some(c) if s.is_none() => {
                        s = Some(c);
                    }
                    Some(c) if s.unwrap() != c => {
                        break 'o;
                    }
                    Some(_c) => {}
                    None => {
                        break 'o;
                    }
                }
            }
            index += 1;
        }

        strs[0].get(0..index).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::Solution;

    #[test]
    fn test_longest_common_prefix() {
        let vec = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];

        let s = Solution::longest_common_prefix(vec);
        assert_eq!("fl", s);

        let vec = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];

        let s = Solution::longest_common_prefix(vec);
        assert_eq!("", s);
    }
}
