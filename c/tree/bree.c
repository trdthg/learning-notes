#include<stdio.h>
#include<stdlib.h>

#define M 1

// 准备写一个5阶B树, 每个节点可以存储4个数据, 对应5个指针
typedef struct Node {
    int n;                       // 存储已经存储的节点数量
    int keys[5];                 // 存储数字, 用来排序, 保持平衡之类的
    int values[5];               // 用来存储数据
    struct Node* children[5];    // 储存下一阶的节点的指针
    struct Node* father;
} Node;

typedef struct Tree {
    Node* root;  // 树的根节点
} Tree;

Node* borrow_from_brother(int index, Node* node);


// 初始化Node
Node* init_node(int key, int value);
// 初始化Tree
Tree* init_tree();
// 获得插入位置的索引-1
int get_insert_index(int new_key, Node* node);


// 拆分没有children的节点
Node* split_node(int new_key, int new_value, Node* node, int index);
// 拆分有children的节点
Node* split_node_full(Node* child, Node* node, int index);
// 插入普通数值
void sort_and_insert(int new_key, int new_value, Node* node, int index);
// merge时与父节点合并
void sort_and_insert_full(Node* child, Node* father, int index);
// 合并节点
Node* merge_node(Node* node);
// 插入时的状态分析
Node* get_insert_node(int new_key, int new_value, Node* node);
// 判断是否没有插入过
void insert(int key, int value, Tree* tree);

// 查询某个key是否存在
Node* search(int key, Node* node) {
    int i;
    for (i = 1; i <= node->n; i++) {
        if (key == node->keys[i]) {
            return node;
        } else if (key > node->keys[i]) {
            continue;
        } else {
            break;
        }
    }
    // 得到插入位置的index => i
    int child_index = i - 1;
    if (node->children[child_index] == NULL) {
        return NULL;
    } else {
        node = node->children[child_index];
        return search(key, node);
    }
}

// 返回该节点在父节点中的索引位置
int find_child_index(Node* child) {
    Node* father = child->father;
    for (int i = 0; i <= father->n; i++) {
        if (child == father->children[i]) {
            return i;
        }
    }
    return -1;
}

int get_index(int key, Node* node) {
    for (int i = 1; i <= node->n; i++) {
        if (key == node->keys[i]) {
            return i;
        }
    }
    return 0;
}

Node* delete_node(int index, Node* node) {
    Node* deleted_node = init_node(node->keys[index], node->values[index]);
    for (int i = index; i <= node->n - 1; i++) {
        node->keys[i] = node->keys[i + 1];
        node->values[i] = node->values[i + 1];
    }
    node->n -= 1;
    return deleted_node;
}

Node* merge_two_small_nodes(Node* a, Node* b) {
    int start = a->n + 1;
    int end = a->n + b->n;
    for (int i = start, j = 1; i <= end; i++, j++) {
        a->n += 1;
        a->keys[i] = b->keys[j];
        a->values[j] = b->values[j];
    }
    return a;
}

Node* merge_three_nodes(Node* left, Node* middle, Node* right) {
    Node* a = merge_two_small_nodes(left, middle);
    for (int i = left->n, j = 0; i <= left->n + right->n; i++, j++) {
        a->children[i] = right->children[j];
    }
    a = merge_two_small_nodes(a, right);
    return a;
}

Node* find_left(Node* node) {
    if (node->children[0] == NULL) {
        return node;
    } else {
        find_left(node->children[node->n]);
    }
}

Node* find_right(Node* node) {
    if (node == NULL) return NULL;
    if (node->children[0] == NULL) {
        return node;
    } else {
        find_left(node->children[0]);
    }
}

Node* delete_and_replace(int index, Node* node, Tree* tree) {
    // 数据准备
    Node* deleted_node;
    int deleted_index;

    Node* left_child_node = find_left(node->children[index - 1]);
    Node* right_child_node = find_right(node->children[index]);

    int n_left = left_child_node->n;
    int n_right = right_child_node->n;

    // PART I: 借不到节点
    if (n_left <= 2 && n_right <= 2) {
        Node* merged_node = merge_two_small_nodes(node->children[index - 1], node->children[index]);
        for (int i = index; i <= node->n; i++) {
            node->children[i] = node->children[i + 1]; 
        }
        if (node->father == NULL) {
            node->children[0]->father = NULL;
            tree->root = merged_node;
        }
        Node* deleted_node = delete_node(index, node);
        return deleted_node;
    }

    // PART II: 借的到节点
    if (n_left > n_right) {
        deleted_index = left_child_node->n;
        deleted_node = left_child_node;
    } else {
        deleted_index = 1;
        deleted_node = right_child_node;
    }
    Node* tmp_node = delete_node(deleted_index, deleted_node);
    node->keys[index] = tmp_node->keys[1];
    node->values[index] = tmp_node->values[1];
}

Node* borrow_from_father(int index, Node* node) {
    Node* father = node->father;
    int child_index = find_child_index(node);

    Node* left_brother = father->children[child_index - 1];
    Node* right_brother = father->children[child_index + 1];
    if (child_index - 1 < 0) {
        // 向右合并
        Node* from_father = delete_node(child_index + 1, father);
        if (node->children[0] == NULL) {
            Node* merged_node = merge_two_small_nodes(node, from_father);
            merged_node = merge_two_small_nodes(merged_node, right_brother);
            for (int i = child_index; i <= node->n; i++) {
                father->children[i] = father->children[i + 1]; 
            }
        } else {
            Node* merged_node = merge_three_nodes(node, from_father, right_brother);
        }
    } else {
        // 向左合并
        Node* from_father = delete_node(child_index, father);
        if (node->children[0] == NULL) {
            // Node* from_father = init_node(father->keys[child_index], father->values[child_index]);
            Node* merged_node = merge_two_small_nodes(left_brother, from_father);
            merged_node = merge_two_small_nodes(merged_node, node);
            for (int i = child_index; i <= node->n; i++) {
                father->children[i] = father->children[i + 1]; 
            }
        } else {
            Node* merged_node = merge_three_nodes(left_brother, from_father, node);
        }
    }

    if (father->n == 1) {
        borrow_from_brother(1, father);
    } else if (father->n == 0) {
        return node;
    } else {
        return NULL;
    }
}

Node* borrow_from_brother(int index, Node* node) {
    Node* father = node->father;
    int child_index = find_child_index(node);
    Node* left_brother = father->children[child_index - 1];
    Node* right_brother = father->children[child_index + 1];
    int n_left, n_right;
    if (child_index - 1 < 0) {
        n_left = 0;
    } else {
        n_left = left_brother->n;
    }
    if (child_index + 1 > 4) {
        n_right = 0;
    } else {
        n_right = right_brother->n;
    }
    if (n_left <= 2 && n_right <= 2) {
        // 接不到兄弟节点
        Node* new_root = borrow_from_father(index, node);
        if (new_root != NULL) {
            return new_root;
        }
    } else {
        // 借的到兄弟节点
        if (n_left <= n_right) {
            Node* temp_node = delete_node(1, right_brother);
            node->keys[index] = father->keys[child_index + 1];
            node->values[index] = father->values[child_index + 1];
            father->keys[child_index + 1] = temp_node->keys[1];
            father->values[child_index + 1] = temp_node->values[1];
        } else {
            Node* temp_node = delete_node(left_brother->n, left_brother);
            node->keys[index] = father->keys[child_index];
            node->values[index] = father->values[child_index];
            father->keys[child_index] = temp_node->keys[1];
            father->values[child_index] = temp_node->values[1];
        }
    }
    // 都小于最小关键字数量, 先删去随便一个点, 在尝试从父节点拉下来一个节点, 并于兄弟节点3块合并
    // 若父节点小于最小关键字数量, 递归像父节点拉下来
    // find_left_brother
    return NULL;
}

Node* delete(int key, Tree* tree) {
    Node* node = tree->root;
    Node* node_for_delete = search(key, node);
    int index = get_index(key, node_for_delete);
    if (node_for_delete->children[0] == NULL) {
        // 没有子分支节点, 安心看看能否直接删除
        if (node_for_delete->n > 2) {
            // 满足最小关键字数量, 直接删除
            return delete_node(index, node_for_delete);
        } else {
            delete_node(index, node_for_delete);
            // 不满足最小关键字数量, 尝试向上合并节点, 或者借兄弟节点
            Node* new_root = borrow_from_brother(index, node_for_delete);
            if (new_root != NULL) {
                tree->root = new_root;
            }
        }
    } else {
        // 尝试替换节点
        return delete_and_replace(index, node_for_delete, tree);
    }
}

int main(void) {
    printf("----------start----------\n");
    Tree* tree = init_tree();
    insert(1, 2, tree);
    insert(2, 2, tree);
    insert(6, 2, tree);
    insert(7, 2, tree);
    insert(11, 2, tree);
    insert(4, 2, tree);
    insert(8, 2, tree);
    insert(13, 2, tree);
    insert(10, 2, tree);
    insert(5, 2, tree);
    insert(17, 2, tree);
    insert(9, 2, tree);
    insert(16, 2, tree);
    insert(20, 2, tree);
    insert(3, 2, tree);
    insert(12, 2, tree);
    insert(14, 2, tree);
    insert(18, 2, tree);
    insert(19, 2, tree);
    insert(15, 2, tree);

    insert(30, 2, tree);
    insert(31, 2, tree);
    insert(32, 2, tree);
    // insert(33, 2, tree);
    insert(34, 2, tree);
    // insert(45, 2, tree);
    // insert(36, 2, tree);

    printf("%d", tree->root->keys[1]);
    Node* find = search(9, tree->root);
    printf("%d", find->keys[1]);

    delete(8, tree);
    delete(16, tree);
    delete(15, tree);
    delete(4, tree);
    printf("\n-----------end-----------");
    return 0;
}

int get_insert_index(int new_key, Node* node) {
    int n = node->n;
    int* keys = node->keys;
    int n_ = 0;
    for (int i = 1; i <= n; i++) {
        int key = keys[i];
        if (new_key > key) {
            n_ += 1;
        } else {
            break;
        }
    }
    return n_;
}

Node* init_node(int key, int value) {
    // 设定第一对k, v的值, 其他的全为null
    Node* root = (Node*)malloc(sizeof(Node));
    root->n = 1;
    for (int i = 0; i < 5; i++) {
        root->keys[i] = 0;
        root->values[i] = 0;
        root->children[i] = NULL;
    }
    root->keys[1] = key;
    root->values[1] = value;
    root->father = NULL;
    return root;
}

Tree* init_tree() {
    Tree* tree = (Tree*)malloc(sizeof(Tree));
    tree->root = NULL;
    return tree;
}

Node* split_node(int new_key, int new_value, Node* node, int index) {
    // 只有满员了才会分离节点, 找到中间节点
    int n = node->n;
    int last_key;
    int last_value;
    // 重排k, v, 第5个放外面保存
    // 找到插入的位置
    index += 1;
    if (index < 4) {
        last_key = node->keys[4];
        last_value = node->values[4];
        for (int i = node->n + 1; i >= index; i--) {
            node->keys[i] = node->keys[i - 1];
            node->values[i] = node->values[i - 1];
        }
        node->keys[index] = new_key;
        node->values[index] = new_value;
    } else if (index == 4) {
        last_key = node->keys[4];
        last_value = node->values[4];
        node->keys[4] = new_key;
        node->values[4] = new_value;
    } else {
        last_key = new_key;
        last_value = new_value;
    }
    int top_key = node->keys[3];
    int top_value = node->values[3];
    Node* new_top = init_node(top_key, top_value);

    // 重排children
    // node->children[index - 1] = 


    // 左右分支重新生成
    Node* new_left = init_node(node->keys[1], node->values[1]);
    new_left->keys[2] = node->keys[2];
    new_left->values[2] = node->values[2];
    new_left->father = new_top;
    new_left->n = 2;

    Node* new_right = init_node(node->keys[4], node->values[4]);
    new_right->keys[2] = last_key;
    new_right->values[2] = last_value;
    new_right->father = new_top;
    new_right->n = 2;

    new_top->children[0] = new_left;
    new_top->children[1] = new_right;
    new_top->father = node->father;

    return new_top;
}

Node* split_node_full(Node* child, Node* node, int index) {
    // child是新插入的节点, node是需要被拆分的父节点
    int new_key = child->keys[1];
    int new_value = child->values[1];
    child->children[0]->father = node;
    child->children[1]->father = node;

    // 只有满员了才会分离节点, 找到中间节点
    int n = node->n;
    int last_key = node->keys[4];
    int last_value = node->values[4];

    // 重排key, value
    index += 1;
    for (int i = node->n + 1; i >= index; i--) {
        node->keys[i] = node->keys[i - 1];
        node->values[i] = node->values[i - 1];
    }
    node->keys[index] = new_key;
    node->values[index] = new_value;
    node->n += 1;

    // 生成新的top_node
    int top_key = node->keys[3];
    int top_value = node->values[3];
    Node* new_top = init_node(top_key, top_value);
    new_top->father = node->father;

    // 重排children
    Node* last_child = node->children[4];
    for (int i = node->n + 1; i > index; i--) {
        node->children[i] = node->children[i - 1];
    }
    node->children[index - 1] = child->children[0];
    node->children[index] = child->children[1];


    // 左右分支重新生成
    Node* new_left = init_node(node->keys[1], node->values[1]);
    new_left->keys[2] = node->keys[2];
    new_left->values[2] = node->values[2];
    new_left->father = new_top;
    new_left->n += 1;

    Node* new_right = init_node(node->keys[4], node->values[4]);
    new_right->keys[2] = last_key;
    new_right->values[2] = last_value;
    new_right->father = new_top;
    new_right->n += 1;

    // 左右分支的children
    new_left->children[0] = node->children[0];
    new_left->children[1] = node->children[1];
    new_left->children[2] = node->children[2];
    new_right->children[0] = node->children[3];
    new_right->children[1] = node->children[4];
    new_right->children[2] = last_child;
    for (int i = 0; i < new_left->n + 1; i++) {
        new_right->children[i]->father = new_right;
    }
    for (int i = 0; i < new_left->n + 1; i++) {
        new_left->children[i]->father = new_left;
    }
    new_top->children[0] = new_left;
    new_top->children[1] = new_right;

    return new_top;
}

void sort_and_insert(int new_key, int new_value, Node* node, int index) {

    // 不满4个, 直接平移数组, 插入新数据
    // 没有子分支, 不用管children
    // 已经有3个插入到2号位
    //  o o o
    //    o
    //  o o o o 

    // 把index该成插入key, value的索引, 不是children的索引         
    index += 1;
    for (int i = node->n + 1; i >= index; i--) {
        node->keys[i] = node->keys[i - 1];
        node->values[i] = node->values[i - 1];
    }
    node->keys[index] = new_key;
    node->values[index] = new_value;
    node->n += 1;
}

void sort_and_insert_full(Node* child, Node* father, int index) {
    // 不满4个, 直接平移数组, 插入新数据
    int new_key = child->keys[1];
    int new_value = child->values[1];

    // key, value重排列
    index += 1;
    for (int i = father->n + 1; i >= index; i--) {
        father->keys[i] = father->keys[i - 1];
        father->values[i] = father->values[i - 1];
    }
    father->keys[index] = new_key;
    father->values[index] = new_value;
    father->n += 1;

    // 更换father的children
    for (int i = father->n + 1; i > index; i--) {
        father->children[i] = father->children[i - 1];
    }
    father->children[index] = child->children[1];
    father->children[index - 1] = child->children[0];

    // 更换children的father
    father->children[index]->father = father;
    father->children[index - 1]->father = father;
    
}

Node* merge_node(Node* node) {
    Node* father = node->father;
    if (father == NULL) {
        // 就是根节点
        return node;
    } else {
        // 不是根节点, 尝试与父节点合并
        if (father->n == 4) {
            // 父节点也满员了, 需要进行递归
            // 还需要先排列构造新的new_top
            int n_ = get_insert_index(node->keys[1], father);
            Node* new_top = split_node_full(node, father, n_);
            Node* new_root = merge_node(new_top);
            return new_root;
        } else {
            // 父节点不满, 合并节点
            int n_ = get_insert_index(node->keys[1], father);
            sort_and_insert_full(node, father, n_);
        }
    }
    return NULL;
}

Node* insert_node(int new_key, int new_value, Node* node) {
    int* keys = node->keys;
    int n = node->n;

    // 获取插入的child的位置的索引
    int n_ = get_insert_index(new_key, node);
    // 根据情况判断是否向下递归还是直接尝试插入
    Node* child = node->children[n_];
    if (child == NULL) {
        if (n == 4) {
            // 需要向上提, 最下面一层的拆分更简单一些
            Node* new_top = split_node(new_key, new_value, node, n_);
            // 尝试向上合并节点
            return merge_node(new_top);
        } else {
            // 直接插入
            sort_and_insert(new_key, new_value, node, n_);
        }
    } else {
        // 递归查询
        return insert_node(new_key, new_value, child);
    }
    return NULL;
}

void insert(int key, int value, Tree* tree) {
    // insert的注意事项
    // 若本层没有子节点, 且本层已满, 需要向上提一层, 
    //    - 若上层没有节点就改变为根节点, 之后结束
    //    - 若上层有节点就尝试和上层节点合并, 之后递归

    // Node* node = tree->root;
    Node* node = tree->root;
    if (node == NULL) {
        Node* node = init_node(key, value);
        tree->root = node;
    } else {
        // 创建新节点
        Node* new_root = insert_node(key, value, node);
        if (new_root != NULL) {
            tree->root = new_root;
        }
    }
}
