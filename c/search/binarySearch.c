#include <stdio.h>


int Bin_Search(int *num,int cnt,int target)
{
	int left = 0;
    int right = cnt - 1;
    int middle;
    while (left < right) {
        middle = (left + right) / 2;
        if (num[middle] < target) {
            left = middle;
        } else if (num[middle] > target) {
            right = middle;
        } else {
            return 0;
        }
    }
	return 1;
}


int main(void)
{
	int flag = 0,target = 3;
	int num[10] = {0,1,2,4,5,6,7,8};
    flag = Bin_Search(num,8,target);
    printf("%d\n", flag);
	return 0;
}

