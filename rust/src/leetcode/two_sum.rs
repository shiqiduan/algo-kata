use crate::leetcode::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32];
            }
            map.insert(num, i);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_two_sum() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);

        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);

        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
