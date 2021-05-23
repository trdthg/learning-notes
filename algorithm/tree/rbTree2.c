#include<stdio.h>
#include<stdlib.h>
#define max(a,b) ((a > b) ? a : b)

#define red 1
#define black 0
#define leftdirctction 1
#define rightdirctction 0

typedef struct Node {
    int key;
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
    p->key = 0;
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

Node* RightRotation(Node* node, Tree* tree) {
    Node* res = node->left;
    node->left = res->right;
    if (res->right != NULL) 
        res->right->parent = node;
    res->right = node;
    res->parent = node->parent;
    if (node->parent == NULL) {
        tree->root = res;
    } else {
        if (node->parent->left == node) 
            node->parent->left = res;
        else
            node->parent->right = res;
    }
    node->parent = res;
    // if (node == tree->root) {
    //     tree->root = res;
    // }
    return res;
}

Node* LeftRotation(Node* node, Tree* tree) {
    Node* res = node->right;
    node->right = res->left;
    if (res->left != NULL) 
        res->left->parent = node;
    res->left = node;   
    res->parent = node->parent;
    if (node->parent == NULL) {
        tree->root = res;
    } else {
        if (node->parent->left == node) 
            node->parent->left = res;
        else
            node->parent->right = res;
    }
    node->parent = res;
    // if (node == tree->root) {
    //     tree->root = res;
    // }
    return res;
} 

Node* LeftRightRotation(Node* node, Tree* tree) {
    node->left = LeftRotation(node->left, tree);
    node->color = red;
    node->left->color = black;
    RightRotation(node, tree);
}

Node* RightLeftRotation(Node* node, Tree* tree) {
    node->right = RightRotation(node->right, tree);
    node->color = red;
    node->right->color = black;
    LeftRotation(node, tree);
}

Node* fixUp(Node* node, Tree* tree) {
    Node* parent;
    Node* grandparent;
    Node* uncle;
    while (node->parent != NULL && node->parent->color == red) {
        parent = node->parent;
        grandparent = parent->parent;
        if (grandparent == NULL) return NULL;
        if (parent == grandparent->left) {
            uncle = grandparent->right;
        } else {
            uncle = grandparent->left;
        }
        if (uncle != NULL && uncle->color == red) {
            uncle->color = black;
            parent->color = black;
            grandparent->color = red;
            node = grandparent;
            continue;
        } else if (uncle == NULL || uncle->color == black) {
            if (grandparent->left == parent) {
                if (parent->left == node) {
                    parent->color = black;
                    grandparent->color = red;
                    RightRotation(grandparent, tree);
                } else {
                    LeftRightRotation(grandparent, tree);
                    node = node->parent;
                }
            } else {
                if (parent->left == node) {
                    RightLeftRotation(grandparent, tree);
                    node = node->parent;
                } else {
                    parent->color = black;
                    grandparent->color = red;
                    RightRotation(grandparent, tree);
                }
            }
        }
    }
}

Node* put(Node* node, int key, int value) {
    Node* parent = NULL;
    Node* newNode = initNode();
    newNode->key = key; newNode->value = value;
    // 1.添加新节点
    while (node != NULL) {
        parent = node;
        if (value < node->value) {
            node = node->left;
            if (node == NULL) {
                parent->left = newNode;
                newNode->parent = parent;
                break;
            }
        }
        else {
            node = node->right;
            if (node == NULL) {
                parent->right = newNode;
                newNode->parent = parent;
                break;
            }
        }
    } 
    return newNode;
}

void putCheck(Tree* tree, int key, int value) {
    if (tree->init == 0) {
        tree->root->key = key;
        tree->root->value = value;
        tree->init = 1;
        tree->root->color = 0;
    } else {
        fixUp(put(tree->root, key, value), tree);
        tree->root->color = black;
    }
}


Node* removes(Node* node, int key, int value, Tree* tree) {
    Node* parent = NULL;  // 这个parent是指replace的parent
    Node* child = NULL;
    Node* replace = NULL;
    int color;
    while (node != NULL) {
        // 找到了那个点
        if (node->value == value) {
            // 两个子节点都不为空
            /*
            *共有四次对接过程
            * 1. replace(移动后)和 replace.father
            * 2. replace(移动前).right 和 replace.father
            * 3. replace(移动后) 和 node.right
            * 4. replace(移动前) 和 node.left
            */
            if (node->left != NULL && node->right != NULL) { 
                replace = node->right;
                while (replace->left != NULL) {
                    replace = replace->left;
                }
                // 1.1 
                if (node->parent != NULL) {
                    if (node->parent->left == node) {
                        node->parent->left = replace;
                    } else {
                        node->parent->right = replace;
                    }
                } else {
                    tree->root = replace;
                } 
                child = replace->right;
                parent = replace->parent;
                color = replace->color;
                // 2.
                if (parent == node) {
                    parent = replace;
                } else {
                    // 被替换的节点的兄弟节点要被接到replace.parent上
                    if (child != NULL) {child->parent = parent;}
                    parent->left = child;
                    // 3.
                    replace->right = node->right;
                    node->right->parent = replace;
                }
                // 1.2
                replace->parent = node->parent;
                replace->color = node->color;
                // 4.
                replace->left = node->left;
                node->left->parent = replace;
                
                if (color == black) {
                    if (child != NULL) {
                        fixUp(child, tree);
                    } else {
                        fixUp(parent, tree);
                    }
                }
                node = NULL;
                return NULL;
            }



            // 获取子节点
            if (node->left != NULL) {
                child = node->left;
            } else {
                child = node->right;
            }
            if (child != NULL) {
                child->parent = node->parent;
            }
            parent = node->parent;
            color = node->color;
            // 进行对接
            if (node->parent != NULL) {
                if (node->parent->left == node) {
                    parent->left = child;
                } else {
                    parent->right = child;
                }
            } else {
                // 特殊处理若去掉的节点是根节点
                tree->root = child;
            }
            node = NULL;
            if (child != NULL) {
                fixUp(child, tree);
            } else {
                fixUp(parent, tree);
            }
            return NULL;
        }
        // 继续找到要去除的点
        if (value < node->value) {
            node = node->left;
        } else {
            node = node->right;
        }
    } 
    return NULL;
}

void removeChect(Tree* tree, int key,int value) {
    if (tree->init == 0) {
        // 未初始化, 不处理
    } else if (tree->root->left == NULL && tree->root->right == NULL) {
        tree->init = 0;
    } else {
        Node* lastNode = removes(tree->root, key, value, tree);
    }
}


int main(void) {
    Tree* tree = initTree();
    putCheck(tree, 1, 80);
    putCheck(tree, 1, 40);
    putCheck(tree, 1, 160);
    putCheck(tree, 1, 20);
    putCheck(tree, 1, 60);
    // 1.单右旋
    // putCheck(tree, 10);
    // putCheck(tree, 5);
    // 2.左右旋
    // putCheck(tree, 5); putCheck(tree, 11);
    // 3.右左旋
    // putCheck(tree, 30);
    // putCheck(tree, 25);
    // 4.红红
    putCheck(tree, 1, 10);
    putCheck(tree, 1, 30);
    putCheck(tree, 1, 25);
    // 5. 暂时就这样了, 明天再说
    preorderTraversal(tree->root);
    printf("\n");
    // 6. 负载测试
    putCheck(tree, 1, 28);
    putCheck(tree, 1, 22);
    preorderTraversal(tree->root);
    printf("\n");
    putCheck(tree, 1, 23);
    putCheck(tree, 1, 24);
    preorderTraversal(tree->root);
    printf("\n");
    // 删除
    removeChect(tree, 1, 23);
    preorderTraversal(tree->root);
    return 0;
}















