#include <stdio.h>
#include <stdlib.h>
 
#define BUF_SIZE 10
 
void display(int array[], int maxlen)
{
    int i;
 
    for(i = 0; i < maxlen; i++)
    {
        printf("%-3d", array[i]);
    }
    printf("\n");
 
    return ;
}

void QuickSort(int *array, int lo, int hi) {
    if (lo < hi) {
        int left = lo, right = hi;
        int target = array[lo];
        // while循环每次排两个
        while (left < right) {
            while (left < right && array[right] >= target) { 
                right -= 1;
            } if (left < right) {
                array[left] = array[right];
                left += 1;
            }
            while (left < right && array[left] <= target) {
                left += 1;
            } if (left < right) {
                array[right] = array[left];
                right -= 1;
            } 
        }
        array[left] = target;
        QuickSort(array, lo, left - 1);
        QuickSort(array, left + 1, hi);
    }
}
// 主函数
int main()
{
    int array[BUF_SIZE] = {12,85,25,16,34,23,49,95,17,61};
    int maxlen = BUF_SIZE;
    
    printf("排序前的数组\n");
    display(array, maxlen);
    
    QuickSort(array, 0, maxlen-1);  // 快速排序
    
    printf("排序后的数组\n");
    display(array, maxlen);
    
    return 0;
}