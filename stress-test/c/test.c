#include <stdio.h>
void foo(){
    int i,j;
    for(i=0;i<1000;i++)
        j+=2;
    printf("%d\n", j);
}
int main(void) {
    int i;
    for(i=0;i<100000000;i++){
        foo();
    }
    return 0;
}
