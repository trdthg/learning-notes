#include<stdio.h>
#include<stdlib.h>

// int add(int a) {
//     return a;
// }
// int add(int a, int b) {
//     return a + b;
// }
typedef struct Node {
    int data;
    struct Node* left;
    struct Node* right;
} Node;
Node* initNode() {
    Node* p = (Node*)malloc(sizeof(Node));
    p -> left = NULL;
    p -> right = NULL;
    p -> data = 0;
    return p;
}



int main(void) {
    // Node* a = initNode();
    // Node* b = initNode();
    // a->left = b;
    // free(b);
    // a->data = 1;

    int a = 1, b= 2, c;
    c = a<b;
    printf("%d", c);
    // int a, b, c;
    // scanf("%d %d %d", &a, &b, &c);
    // printf("%d", (int)(a*0.2 + b * 0.3 + c * 0.5));

    // printf("%d", add(1));
    // printf("%d", add(1,2));
    return 0;
}