#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int weight;
    int parent;
    int lchild;
    int rchild;
} Node;

int get_n_node() {
    int n;
    printf("请输入结点的个数:\n");
    scanf("%d", &n);  //读入权值
    if (n <= 1)
        exit(9);
    return n;
}

Node* get_node() {
    Node* new_node = (Node*)malloc(sizeof(Node*));
    new_node->parent = 0;
    new_node->rchild = 0;
    new_node->lchild = 0;
    new_node->weight = 0;
    return new_node;
}

//从赫夫曼树存储的前n个结点中，求解双亲为0且权重值最小的两个结点的坐标s1,s2
void select_minium_two_node(Node** HT, int n, int* s1, int* s2) {
    int min1 = 1000, min2 = 1000;
    for (int i = 1; i <= n; i++) {
        printf("i: %d ", i);
        if (HT[i]->parent == 0) {
            if (HT[i]->weight < min1) {
                printf("?1");
                min2 = min1;
                min1 = HT[i]->weight;
                *s2 = *s1;
                *s1 = i;
            } else if (HT[i]->weight < min2) {
                printf("?2");
                min2 = HT[i]->weight;
                *s2 = i;
            }
            printf("%d min1: %d min2: %d s1: %d s2: %d \n", HT[i]->weight, min1,
                   min2, *s1, *s2);
        }
    }
}
//构建包含n个叶子结点的赫夫曼树HT
Node** creat_huffman_tree(int n) {
    //? 2. 读入权值
    int weight;
    int m = 2 * n - 1;
    Node** HT = (Node**)malloc((m + 1) * sizeof(Node*));
    for (int i = 1; i <= m; i++)
        HT[i] = get_node();
    for (int i = 1; i <= n; i++) {
        scanf("%d", &weight);
        HT[i]->weight = weight;
    }

    //? 3. 开始接树, 从新的节点开始
    int s1, s2;
    for (int i = n + 1; i <= m; i++) {
        HT[i]->parent = 0;
        select_minium_two_node(HT, i - 1, &s1, &s2);
        HT[s1]->parent = i;
        HT[s2]->parent = i;
        HT[i]->lchild = s1;
        HT[i]->rchild = s2;
        HT[i]->weight = HT[s1]->weight + HT[s2]->weight;
        for (int i = 1; i <= 2 * n - 1; i++) {
            printf("%d | %d %d %d %d \n", i, HT[i]->weight, HT[i]->parent,
                   HT[i]->lchild, HT[i]->rchild);
        }
        // printf("??? %d %d - %d %d", i, HT[i]->weight, s1, s2);
    }
    printf("\n");
    return HT;
}

char** get_huffman_code(Node** HT, int n) {
    char** HC = (char**)malloc((n + 1) * sizeof(char*));
    char* cd = (char*)malloc(100 * sizeof(char));
    cd[n] = '\0';
    for (int i = 1; i <= n; i++) {
        int start = i + 1;
		int self = i; // 当前节点
		int parent = HT[i]->parent; // 父节点
		printf("i: %d ", i);
        while (parent != 0) {
			start -= 1;
            printf("start: %d ", self);
            if (HT[parent]->lchild == self) {
                cd[start] = '0';
                printf("0 ");
            } else {
                cd[start] = '1';
                printf("1 ");
            }
			self = parent;
			parent = HT[parent]->parent;
        }
        printf("\n");
        HC[i] = (char*)malloc((n - start) * sizeof(char));
        strcpy(HC[i], &cd[start]);
    }
	free(cd);
	return HC;
}

int main(void) {
    int n = get_n_node();
    Node** HT = creat_huffman_tree(n);
    for (int i = 1; i <= 2 * n - 1; i++) {
        printf("%d | %d %d %d %d \n", i, HT[i]->weight, HT[i]->parent,
               HT[i]->lchild, HT[i]->rchild);
    }

    char** HC = get_huffman_code(HT, n);
    printf("\n");
    for (int i = 0; i < n; i++) {
        printf("%s ", HC[i + 1]);
    }

    return 0;
}
