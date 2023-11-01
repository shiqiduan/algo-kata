public class LongestCommonPrefix14 {
    public String longestCommonPrefix(String[] strs) {
        if (strs.length == 0)
            return "";
        if (strs.length == 1)
            return strs[0];
        String f = strs[0];
        for (int i = 0; i < f.length(); i++) {
            char c = f.charAt(i);

            for (int j = 1; j < strs.length; j++) {
                if (i >= strs[j].length() || c != strs[j].charAt(i)) {
                    return f.substring(0, i);
                }
            }
        }
        return f;
    }
}
