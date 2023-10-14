import java.util.HashMap;
import java.util.Map;

public class TwoSum {

}

class Solution {
    public int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> map = new HashMap<>();
        for (int i = 0; i < nums.length; i++) {
            int complement = target - nums[i];
            Integer index = map.get(complement);
            if (index != null) {
                return new int[] { index, i };
            }
            map.put(nums[i], i);
        }
        return null;
    }
}
