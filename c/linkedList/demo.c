#include<stdio.h>
#include<stdlib.h>

typedef struct LinkedList {
    int data;
    struct LinkedList* previous;
    struct LinkedList* next;
} LinkedList;

typedef struct Link {
    int length;
    LinkedList* head;
    LinkedList* tail;
} Link;

Link* init(int data);
int add(Link*, int data);
void listAll(Link* link);

int main(void) {
    // 初始化
    Link* p = init(1);
    // 插入末尾
    int res = add(p, 2);
    printf("%d", p->length);
    // 删除

    // 修改

    // 查找

    // all
    listAll(p);
    return 0;
}

Link* init(int data) {
    Link* pl = (Link*)malloc(sizeof(Link));
    LinkedList* p = (LinkedList*)malloc(sizeof(LinkedList));
    p->next = NULL;
    p->previous = NULL;
    p->data = data;
    pl->head = p;
    pl->length = 1;
    pl->tail = p;
    return pl;
}

int add(Link* link, int data) {
    LinkedList* p = (LinkedList*)malloc(sizeof(LinkedList));
    p->next = link->head;
    p->previous = link->tail;
    p->data = data;
    link->tail->next = p;
    link->tail = p;
    link->length += 1;
    return 1;
}

void listAll(Link* link) {
    LinkedList* p = link->head->next;
    printf("长度: %d\n", link->length);
    printf("%d ", link->head->data);
    while (p != link->head) {
        printf("%d ", p->data);
        p = p->next;
    }
}