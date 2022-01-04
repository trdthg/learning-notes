#include<stdio.h>
#include<stdlib.h>
#include<string.h>

typedef int Status;
#define OK 1
#define Err 0
#define OVERFLOW -2

#define DEFAULT_SIZE 100
#define STACK_INCREMENT 10

typedef struct Stack{
    char** base;
    char** top;
    int stacksize;
} Stack;

Status init_stack(Stack* s) {
    s->base = (char**)malloc(DEFAULT_SIZE * sizeof(char*));
    if (s->base) {
        s->top = s->base;
        s->stacksize = DEFAULT_SIZE;
        return OK;
    } else {
        exit(OVERFLOW);
    }
    return OK;
}

Status push(Stack* s, char* elem) {
    if (s->top - s->base >= s->stacksize) {
        // 字符串
        s->base = (char**)realloc(s->base, (s->stacksize + STACK_INCREMENT) * sizeof(char*));
        if (s->base) {
            s->top = s->base + s->stacksize;
            s->stacksize += STACK_INCREMENT;
            *s->top = elem;
            s->top++;
            return OK;
        } else {
            exit(OVERFLOW);
        }
    } else {
        *s->top++ = elem;
        return Err;
    }
} 

Status pop(Stack* s, char** elem) {

    if (s->base == s->top) {
        return Err;
    } else {
        s->top--;
        *elem = *s->top;
        return OK;
    }
}

Status read(Stack* s) {
    char** a = s->base;
    while (a != s->top)
    {
        printf("%s\n", *a);
        a++;
    }
    return OK;
}

int main(void) {
    Stack* s = (Stack*)malloc(sizeof(Stack));
    init_stack(s);
    // push(s, (char*)"aa");
    // push(s, (char*)"bb");
    // push(s, (char*)"cc");
    char* tmp = NULL;
    // pop(s, &tmp);
    // printf("%s", tmp);
    // read(s);
    // printf("-------end------\n");
    // int a = 0;


}
