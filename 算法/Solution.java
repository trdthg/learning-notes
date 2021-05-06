
public class Solution {
    public static void main(String[] args) {
        int[] nums1 = {1,2};
        int[] nums2 = {3,6};
        double ans;
        ans = findMedianSortedArrays(nums1,nums2);
        System.out.println(ans);
        return;
    }
    public static double findMedianSortedArrays(int[] nums1, int[] nums2) {
        int len1 = nums1.length, len2 = nums2.length;
        int total_len = len1 + len2;
        double midian = 0;
        if (total_len % 2 == 1) {
            // 中位数只有一个数
            int mid_index = total_len;
            midian = getKthElement(nums1, nums2, mid_index + 1);
        } else {
            int mid_index1 = total_len / 2 - 1;
            int mid_index2 = total_len / 2;
            midian = (getKthElement(nums1, nums2, mid_index1 + 1) + getKthElement(nums1, nums2, mid_index2 + 1)) / 2.0;
        }
        return midian;
    }

    public static int getKthElement(int[] num1, int[] num2, int mid_index) {
        int len1 = num1.length, len2 = num2.length;
        int index1 = 0, index2 = 0;
        while (true) {
            if (index1 == len1) {
                return num2[index2 + mid_index -1];
            }
            if (index2 == len2) {
                return num1[index1 + mid_index - 1];
            }
            if (mid_index == 1) {
                return Math.min(num1[index1], num2[index2]);
            }
            int half = mid_index / 2;
            int new_index1 = Math.min(index1 + half, len1) - 1;
            int new_index2 = Math.min(index2 + half, len2) - 1;
            int pivot1 = num1[new_index1], pivot2 = num2[new_index2];
            if (pivot1 <= pivot2) {
                mid_index -= (new_index1 - index1 + 1);
                index1 = new_index1 + 1;
            } else {
                mid_index -= (new_index2 - index2 + 1);
                index2 = new_index2 + 1;
            }
        }
    }
}
