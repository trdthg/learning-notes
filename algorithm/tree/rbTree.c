
// unfished

#include<stdio.h>
#include<stdlib.h>
#define max(a,b) ((a > b) ? a : b)

#define red 1
#define black 0
#define leftdirctction 1
#define rightdirctction 0

typedef struct Node {
    int value;
    int color; // 红为1, 黑为0
    struct Node* parent;
    struct Node* left;
    struct Node* right;
} Node;

typedef struct Tree {
    int init;
    Node* root;
} Tree;

void preorderTraversal(Node* node) {
    if (node != NULL) {
        printf("%d %s ", node->value, node->color ? "r" : "b");
        preorderTraversal(node->left);
        preorderTraversal(node->right);
    }
}

Node* initNode() {
    Node* p = (Node*)malloc(sizeof(Node));
    p->value = 0;
    p->color = red;
    p->parent = NULL;
    p->left = NULL;
    p->right = NULL;
    return p;
}

Tree* initTree() {
    Tree* p = (Tree*)malloc(sizeof(Tree));
    p->root = initNode();    
    p->init = 0;
    return p;
}

Node* RightRotation(Node* node) {
    Node* res = node->left;
    node->left = res->right;
    res->right = node;

    res->parent = NULL;
    node->parent = res;
    if (node->left != NULL) node->left->parent = node;

    int c = res->color;
    res->color = node->color;
    node->color = c;
    return res;
}

Node* LeftRotation(Node* node) {
    Node* res = node->right;
    node->right = res->left;
    res->left = node;

    res->parent = NULL;
    node->parent = res;
    if (node->right != NULL) node->right->parent = node;

    int c = res->color;
    res->color = node->color;
    node->color = c;
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

Node* reNode(Node* node, int isleftdirection, Tree* tree) {
    Node* res = NULL;
    Node* grandfather = node->parent;
    int isleft = node->value < grandfather->value;
    Node* brother = NULL;
    if (isleft) brother = grandfather->right; else brother = grandfather->left;
    if (brother != NULL && brother->color == red) {
        if (grandfather == tree->root) {
            return node;
        }
        brother->color = black;
        node->color = black;
        grandfather->color = red;
        return NULL;
    } else {
        if (grandfather->parent == NULL) {
            return node;
        } 
        if (grandfather->color == red) {
            // 选择上移吧要不
            Node* grandgrandfather = grandfather->parent;
            if (isleft) {
                res = RightRotation(grandgrandfather);
            } else if (!isleft) {
                res = LeftRotation(grandgrandfather);
            }
            // if (grandgrandfather->right->color == red) {
            //     if (grandfather == tree->root) {
            //         return node;
            //     }
            //     grandgrandfather->right->color = black;
            //     grandfather->color = black;
            //     grandgrandfather->color = red;
            //     return NULL;
            // } 
            // if (grandgrandfather->left == grandfather) {
            //     res = RightRotation(grandgrandfather);
            // } else {
            //     res = LeftRotation(grandgrandfather);
            // }
            return res;
        } else {
            if (isleft) {
                if (isleftdirection && node->left->color == red) {
                    res = RightRotation(grandfather);
                } else {
                    res = LeftRightRotation(grandfather);
                }
            } else {
                if (isleftdirection) {
                    res = RightLeftRotation(grandfather);
                } else {
                    res = LeftRotation(grandfather);
                }
            }
            return res;
        }
    }
}

Node* add(Node* node, int value, Tree* tree) {
    if (node == NULL) {
        Node* newNode = initNode();
        newNode->value = value;
        return newNode;
    }
    if (value < node->value) {
        Node* newNode = add(node->left, value, tree);
        if (newNode == tree->root) {
            return newNode;
        } else if (node->parent == newNode) {
            return newNode;
        }
        newNode->parent = node;
        node->left = newNode;
        if (node->color != black && node != tree->root) {
            Node* newNode = reNode(node, leftdirctction, tree);
            if (newNode == NULL) {return node;}
            else if (newNode->right == tree->root) {
                newNode->parent = NULL;
                newNode->color = black;
                tree->root = newNode;
            } else {
                return newNode;
            } 
        }
    } else {
        Node* newNode = add(node->right, value, tree);
        if (newNode == tree->root) {
            return newNode;
        } else if (node->parent == newNode || node == newNode) {
            return newNode;
        }
        newNode->parent = node;
        node->right = newNode;
        if (node->color != black && node != tree->root) {
            Node* newNode = reNode(node, rightdirctction, tree);
            if (newNode == NULL) {return node;}
            else if (newNode->right == tree->root) {
                newNode->parent = NULL;
                newNode->color = black;
                tree->root = newNode;
            } else {
                return newNode;
            } 
        }
    }
    tree->root->color = 0;
    return node;
}

// Node* reNode(Node* node, int isleftdirection, Tree* tree) {
//     Node* res = NULL;
//     Node* grandfather = node->parent;
//     int isleft = node->value < grandfather->value;
//     Node* brother = NULL;
//     if (isleft) brother = grandfather->right; else brother = grandfather->left;
//     if (brother != NULL && brother->color == red) {
//         brother->color = black;
//         node->color = black;
//         grandfather->color = red;
//         return NULL;
//     } else {
//         if (node->color != red) {
//             if (isleftdirection && node->left->color == red) {
//                 res = RightRotation(node);
//             } else if (!isleftdirection && node->right->color == red) {
//                 res = LeftRotation(node);
//             } else {
//                 return NULL;
//             }
//             return res;
//         }
//         return NULL;
//     }
// }

// Node* add(Node* node, int value, Tree* tree) {
//     if (node == NULL) {
//         Node* newNode = initNode();
//         newNode->value = value;
//         return newNode;
//     }
//     if (value < node->value) {
//         Node* newNode = add(node->left, value, tree);
//         // if (newNode == tree->root) {
//         //     return newNode;
//         // } else if (node->parent == newNode) {
//         //     return newNode;
//         // }
//         newNode->parent = node;
//         node->left = newNode;
//         if (node->color != black && node != tree->root) {
//             Node* newNode = reNode(node, leftdirctction, tree);
//             if (newNode == NULL) {return node;}
//             else if (newNode->right == tree->root) {
//                 newNode->parent = NULL;
//                 newNode->color = black;
//                 tree->root = newNode;
//             } else {
//                 return newNode;
//             } 
//         }
//     } else {
//         Node* newNode = add(node->right, value, tree);
//         if (newNode == tree->root) {
//             return newNode;
//         } else if (node->parent == newNode) {
//             return newNode;
//         }
//         newNode->parent = node;
//         node->right = newNode;
//         if (node->color != black && node != tree->root) {
//             Node* newNode = reNode(node, rightdirctction, tree);
//             if (newNode == NULL) {return node;}
//             else if (newNode->right == tree->root) {
//                 newNode->parent = NULL;
//                 newNode->color = black;
//                 tree->root = newNode;
//             } else {
//                 return newNode;
//             } 
//         }
//     }
//     tree->root->color = 0;
//     return node;
// }

Node* addCheck(Tree* tree, int value) {
    if (tree->init == 0) {
        tree->init = 1;
        tree->root->value = value;
        tree->root->color = black;
        return tree->root;
    } else {
        return add(tree->root, value, tree);
    }
}

int main(void) {
    Tree* tree = initTree();
    // 1.
    addCheck(tree, 80);
    addCheck(tree, 40);
    addCheck(tree, 160);
    addCheck(tree, 20);
    addCheck(tree, 60);

    // 1.单右旋
    // addCheck(tree, 10);
    // addCheck(tree, 5);
    // 2.左右旋
    // addCheck(tree, 5); addCheck(tree, 11);
    // 3.右左旋
    // addCheck(tree, 30);
    // addCheck(tree, 25);
    // 4.红红
    addCheck(tree, 10);
    addCheck(tree, 30);
    addCheck(tree, 25);
    // 5. 暂时就这样了, 明天再说

    // 6. 负载测试

    addCheck(tree, 28);
    preorderTraversal(tree->root);
    printf("\n");
    addCheck(tree, 22);


    preorderTraversal(tree->root);

    return 0;
}

