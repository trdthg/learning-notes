#include<stdio.h>
#include<stdlib.h>
typedef struct Node {
    int value;
    int height;
    struct Node* left;
    struct Node* right;
} Node;
#define max(a,b) ((a > b) ? a : b)
typedef struct Tree {
    int length;
    int init;
    Node* root;
} Tree;
// 初听古典的小伙伴，老司机给你们带带路：
// 巴赫抗躁动、海顿抗抑郁、莫扎特抗失眠、贝多芬抗萎靡、
// 柴科夫斯基抗饥饿、马勒抗瞌睡、拉赫玛尼诺夫抗寂寞...
// 最后还必须得指出一个最管用的:布鲁克纳，抗吃醋后的不良情绪反应。
void preorderTraversal(Node* node);

// 初始化
Node* initNode();
Tree* initTree();

// 添加
int getHeight(Node* node);
Node* add(Node* node, int value);
Node* addCheck(Tree* tree, int value);

// 删除
Node* reHeightRight(Node* node);
Node* removeNode(Node* node, int value, Tree* tree);
void removeChect(Tree* tree, int value);

// // 旋转
int getBalanceFactor(Node* node) {
    // 获取平衡因子(左子树-右子树, 注意保留负号)
    if (node == NULL) {
        return 0;
    }
    int a = getHeight(node->left) - getHeight(node->right);
    return a;
}

Node* RightRotation(Node* node) {
    Node* res = node->left;
    node->left = node->left->right;
    res->right = node;
    node->height = getHeight(node);
    res->height = getHeight(res);
    return res;
}
Node* LeftRotation(Node* node) {
    Node* res = node->right;
    node->right = node->right->left;
    res->left = node;
    node->height = getHeight(node);
    // res->height = getHeight(res);
    return res;
}

Node* LeftRightRotation(Node* node) {
    node->left = LeftRotation(node->left);
    return RightRotation(node);
}

Node* RightLeftRotation(Node* node) {
    node->right = RightRotation(node->right);
    return LeftRotation(node);
}

int main(void) {
    Tree* tree = initTree();
    addCheck(tree, 12);
    addCheck(tree, 8);
    addCheck(tree, 18);
    addCheck(tree, 5);
    addCheck(tree, 11);
    addCheck(tree, 17);

    // // addCheck(tree, 4);
    addCheck(tree, 7);
    addCheck(tree, 9);
    // R
    // addCheck(tree, 2);
    // RL
    // addCheck(tree, 6);
    removeChect(tree, 17);

    preorderTraversal(tree->root);
    
    return 0;
}



Node* initNode() {
    Node* p = (Node*)malloc(sizeof(Node));
    p->left = NULL;
    p->right = NULL;
    p->value = 0;
    p->height = 1;
    return p;
}
Tree* initTree() {
    Tree* p = (Tree*)malloc(sizeof(Tree));
    p->root = initNode();    
    p->init = 0;
    p->length = 0;
    return p;
}

void preorderTraversal(Node* node) {
    if (node != NULL) {
        printf("%d %d  ", node->value, node->height);
        preorderTraversal(node->left);
        preorderTraversal(node->right);
    }
}

int getHeight(Node* node) {
    if (node == NULL) {
        return 0;
    }
    int hl, hr;
    if (node->left == NULL) {
        hl = 0;
    } else {
        hl = node->left->height;
    }
    if (node->right == NULL) {
        hr = 0;
    } else {
        hr = node->right->height;
    }
    return 1 + max(hl, hr);
}
Node* add(Node* node, int value) {
    
    if (node == NULL) {
        Node* newNode = initNode();
        newNode->value = value;
        return newNode;
    } else if (value < node->value) {
        node->left = add(node->left, value);
        node->height = getHeight(node);
        if (getBalanceFactor(node) == 2) {
            if (value < node->left->value) {
                node = RightRotation(node);
            } else {
                node = LeftRightRotation(node);
            }
        }
    } else {
        node->right  = add(node->right, value);
        node->height = getHeight(node);
        if (getBalanceFactor(node) == -2) {
            if (value < node->right->value) {
                node = LeftRotation(node);
            } else {
                node = RightLeftRotation(node);
            }
        }
    }
    node->height = getHeight(node);
    return node;
}
Node* addCheck(Tree* tree, int value) {
    if (tree->init == 0) {
        tree->init = 1;
        tree->root->value = value;
        return tree->root;
    } else {
        return add(tree->root, value);
    }
}

Node* reHeightRight(Node* node) {
    Node* parent = node;
    if (node->left != NULL) {
        node = node->left;
        Node* newNode = reHeightRight(node);
        node->height = getHeight(node);
        return newNode;
    } else {
        return node;
    }
}
Node* removeNode(Node* node, int value, Tree* tree) {
    if (node == NULL) {
        return NULL;
    }
    if (value == node->value) {
        if (node->left == NULL && node->right == NULL) {
            return NULL;
        } else if (node->right == NULL) {
            return node->left;
        } else if (node->left == NULL) {
            return node->right;
        } else {
            Node* newNode = reHeightRight(node->right);
            node->value = newNode->value;
            node->right = newNode->right;
        }
    } else if (value < node->value) {
        node->left = removeNode(node->left, value, tree);
        node->height = getHeight(node);
        if (getBalanceFactor(node) == -2) {
            if (getHeight(node->left) > 0) {
                node = RightRotation(node);
            } else {
                node = RightLeftRotation(node);
            }
        }
    } else {
        node->right = removeNode(node->right, value, tree);
        node->height = getHeight(node);
        if (getBalanceFactor(node) == 2) {
            Node* temp = NULL;
            if (getHeight(node->left) > 0) {
                temp = LeftRightRotation(node);
            } else {
                temp = LeftRotation(node);
            }
            if (node == tree->root) {
                tree->root = temp;
            } else {
                node = temp;
            }
        }
    }
    node->height = getHeight(node);
}
void removeChect(Tree* tree, int value) {
    if (tree->init == 0) {
    } else {
        removeNode(tree->root, value, tree);
    }
}

// 失败的标志
// while (node != NULL) {
//     if (node->left == NULL && node->right == NULL) {
//         if (value < node->value) node->left = newNode; else node->right = newNode;
//         node->height += 1;
//         break;
//     } else if (node->left == NULL || node->right == NULL) {
//         if (value < node->value) {
//             if (node->left == NULL) {
//                 node->left = newNode;
//                 break;
//             } else {
//                 node->height = 1 + node->right->height;
//                 node = node->left;
//             }
//         } else {
//            if (node->right == NULL) {
//                 node->right = newNode;
//                 break;
//             } else {
//                 node->height = 1 + node->left->height;
//                 node = node->right;
//             }
//         }
//     } else {
//         node->height = 1 + max(node->left->value, node->right->value);
//         if (value < node->value) {
//             node = node->left;
//         } else {
//             node = node->right;
//         }
//     }
// }