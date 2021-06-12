#include<stdio.h>
#include<stdlib.h>


typedef struct Node {
    int data;
    struct Node* left;
    struct Node* right;
} Node;

typedef struct Tree {
    int init;
    int length;
    Node* root;
} Tree;

Node* initNode() {
    Node* p = (Node*)malloc(sizeof(Node));
    p -> left = NULL;
    p -> right = NULL;
    p -> data = 0;
    return p;
}

Tree* initTree() {
    Tree* p = (Tree*)malloc(sizeof(Tree));
    p->root = initNode();    
    p->init = 0;
    return p;
}

Node* add(Tree* tree, int data) {
    Node* current = tree->root;
    Node* parent;
    tree->length += 1;
    if (tree->init == 0) {
        tree->init = 1;
        tree->root->data = data;
    } else {
        Node* node = initNode();
        node->data = data;
        while (1) {
            parent = current;
            if (data <= current->data) {
                current = current->left;
                if (current == NULL) {
                    parent->left = node;
                    break;
                }
            } else {
                current = current->right;
                if (current == NULL) {
                    parent->right = node;
                    break;
                }
            }
        }
    }
    return 0;
}

void preorderTraversal(Node* node) {
    if (node != NULL) {
        printf("%d ", node->data);
        preorderTraversal(node->left);
        preorderTraversal(node->right);
    }
}

void inorderTraversal(Node* node) {
    if (node != NULL) {
        inorderTraversal(node->left);
        printf("%d ", node->data);
        inorderTraversal(node->right);
    }
} 

void postorderTraversal(Node* node) {
    if (node != NULL) {
        postorderTraversal(node->left);
        postorderTraversal(node->right);
        printf("%d ", node->data);
    }
}

void levalorderTraversal(Tree* tree) {
    printf("%d\n", tree->length);
    int arr[100];
    int i=0, j=0; // i记录已经加入的元素数, j记录最后输出的节点
    int size = 1; // 记录实际元素个数
    Node* nodes[100] = {NULL};
    Node* node = tree->root;
    nodes[0] = node;
    while (size > 0) {
        printf("%d ", nodes[j]->data);
        printf("%d  ", node->data);
        j += 1;
        size -= 1;
        if (node->left != NULL)  {
            i += 1;
            nodes[i] = node->left;
            size += 1;
        }
        if (node->right != NULL) {
            i += 1;
            nodes[i] = node->right;
            size += 1;
        }
        node = nodes[j];
    }
}

Node* findMax(Node* node) {
    if (node->right != NULL) {
        findMax(node->right);
    } else {
        return node;
    }
}

Node* findMin(Node* node) {
    if (node->left != NULL) {
        findMax(node->left);
    } else {
        return node;
    }
}

int find(Node* node, int data) {
    if (node != NULL) {
        if (node->data == data) return node->data;
        else if (data < node->data) find(node->left, data);
        else find(node->right, data);
    } else {
        return -1;
    }
}

int delete(Node* root, Node* node, int data) {
    Node* parent = node;
    if (node != NULL && node != root) {
        if (data < node->data) {
            node = node->left;
        } else {
            node = node->right;
        }
    }
    if (data == node->data) {
        if (node->left == NULL && node->right == NULL) {
            if (data <= parent->data) parent->left = NULL; else parent->right = NULL;
        } else if (node->left == NULL) {
            if (data > parent->data) parent->right = node->right; else parent->left = node->right;
        } else if (node->right == NULL) {
            if (data < parent->data) parent->left = node->left; else parent->right = node->left;
        } else {
            Node* innerparent = node;
            Node* innernode = node->right;
            while (innernode->left != NULL) {
                innerparent = innernode;
                innernode = node->left;
            }
            node->data = innernode->data;
            if (innerparent == node) {
                node->right = innernode->right;
            } else {
                innerparent->left = NULL;
            }
        }
        return 1;
    } else if (data < node->data) {
        delete(root, node, data);
    } else {
        delete(root, node, data);
    }
}

int main(void) {
    Tree* tree = initTree();
    add(tree, 8);
    add(tree, 10);
    add(tree, 5);
    add(tree, 42);
    add(tree, 6);
    add(tree, 3);
    add(tree, 2);
    add(tree, 4);
    add(tree, 7);
    preorderTraversal(tree->root);
    printf("\n");
    inorderTraversal(tree->root);
    printf("\n");
    postorderTraversal(tree->root);
    printf("\n");
    levalorderTraversal(tree);
    printf("\n");
    printf("max: %d min: %d\n", findMax(tree->root)->data, findMin(tree->root)->data);
    printf("find 3: %d \n", find(tree->root, 3));
    printf("delete: %d \n", delete(tree->root, tree->root, 8));
    preorderTraversal(tree->root);
    return 0;
}