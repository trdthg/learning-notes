#include<stdio.h>
#include<stdlib.h>

#define M 1

typedef struct Node
{
    int d; // 节点中元素数目
    int key[2 * M];
    char* value[2 * M];
    struct Node* children[2 * M + 1];
} Node;


Node* node_init();
Node* search(int key, Node* root);

Node* insert(Node* root, Node* node, int key, char* value) {
    Node* newNode = node_init(key, value);
    Node* parent = NULL;
    while (node != NULL) {
        if (node->d == 1) {
            // 一个节点不是满的时候
            if (node->children[0] == node->children[1]) {
                // 没有子节点, 再本节点插入
                if (key < node->key[0]) {
                    node->key[1] = node->key[0]; 
                    node->key[0] = key;
                } else if (key == node->key[0]) {
                    return node;
                } else if (key > node->key[0]) {
                    node->key[1] = key;
                }
            } else {
                // 有子节点, 尝试在下一层插入
                if (key < node->key[0]) {parent = node; node = node->children[0];}
                else if (key == node->key[0]) {
                    node->value[0] = value;
                    return node;
                } else {parent = node; node = node->children[1];}
            }
        } else if (node->d == 2) {
            // 节点满了, 尝试找到子节点, 找不到则需要分裂
            if (node->children[0] == NULL) {
                // 没有子节点, 向上分裂
                if (parent != NULL) {
                    if (parent->d == 1) {
                        // 判断中间节点应该插入哪里, 哪里呢?
                        if (key < node->key[0]) {
                            
                        } else if (key == node->key[0]) {
                            node->value[0] = value;
                            return node;
                        } else if (key < node->key[1]) {

                        } else if (key == node->key[1]) {
                            node->value[1] = value;
                            return node;
                        } else if (key > node->key[1]) {
                            if (node->key[1] < parent->key[0]) {
                                parent->key[1] = parent->key[0]; parent->value[1] = parent->value[0];
                                parent->key[0] = node->key[1]; parent->value[0] = node->value[1];
                                parent->children[2] = parent->children[0];
                                parent->children[0] = node_init(node->key[0], node->value[0]);
                                parent->children[1] = node_init(key, value);
                            } else {
                                parent->key[1] = node->key[1]; parent->value[1] = node->value[1];
                                parent->children[0] = node_init(node->key[0], node->value[0]);
                                parent->children[2] = parent->children[0];
                                parent->children[1] = node_init(key, value);
                            }
                        }
                        
                    }
                }
            } else {
                // 有子节点, 尝试在下一层插入
                if (key < node->key[0]) {
                    parent = node; node = node->children[0];
                } else if (key == node->key[0]) {
                    node->value[0] = value;
                    return node;
                } else if (key > node->key[0] && key < node->key[1]) {
                    parent = node; node = node->children[1];
                } else if (key == node->key[1]) {
                    node->value[1] = value;
                    return node;
                } else {
                    parent = node; node = node->children[2];
                }
            }
            if (parent == NULL) {
                // 根节点分裂
                int a, b, c;
                // Node* newNodeLeft, newNodeRight;
                if (key > node->key[0] && key < node->key[1]) {
                    a = 0; b = 1; c = 2;
                    // 确定左右节点
                    Node* newNodeLeft = node_init(node->key[0], node->value[0]);
                    Node* newNodeRight = node_init(node->key[1], node->value[1]);    
                    // 重置自身
                    node->key[0] = key; node->value[0] = value;
                    node->d = 1;
                    // 添加子节点
                    node->children[0] = newNodeLeft; node->children[1] = newNodeRight;
                } else if (key < node->key[0]) {
                    Node* newNodeLeft = node_init(key, value);
                    Node* newNodeRight = node_init(node->key[1], node->value[1]);  
                    node->d = 1;
                    node->children[0] = newNodeLeft; node->children[1] = newNodeRight;
                } else if (key > node->key[0]) {
                    Node* newNodeLeft = node_init(node->key[0], node->value[0]);  
                    Node* newNodeRight = node_init(key, value);
                    node->key[0] = node->key[1]; node->value[0] = node->value[1];
                    node->d = 1;
                    node->children[0] = newNodeLeft; node->children[1] = newNodeRight;
                }
                
            }
        }
    }

}

Node* findChild(Node* node, int key, char* value) {

}

int main(void) {
    printf("----------start----------\n");
    Node* root = node_init(1, "root");
    printf("%d", search(1, root)->d);

    printf("\n-----------end-----------");
    return 0;
}






























Node* node_init(int key, char* value) {
    Node* root = (Node*)malloc(sizeof(Node));
    root->d = 1;
    for (int i = 0; i < 2 * M; i++) {
        root->key[i] = 0;
        root->value[i] = NULL;
        root->children[i] = NULL;
    }
    root->children[2 * M] = NULL;
    root->key[0] = key;
    root->value[0] = value;
    return root;
}
Node* search(int key, Node* root) {
    Node* node = root;
    while (node != NULL) {
        if (node->d == 1) {
            if (key < node->key[0]) {node = node->children[0];}
            else if (key == node->key[0]) {return node;}
            else {node = node->children[1];}
        } else if (node->d == 2) {
            if (key < node->key[0]) {node = node->children[0];}
            else if (key == node->key[0]) {return node;}
            else if (key > node->key[0] && key < node->key[1]) {node = node->children[1];}
            else if (key == node->key[1]) {return node;}
            else {node = node->children[2];}
        } 
    }
    return NULL;
}