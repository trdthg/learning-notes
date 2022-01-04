#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAX_SIZE 5

typedef struct {
    int key;
    char name[5];
} Data;

typedef struct {
    int length;
    Data data[MAX_SIZE + 1];
} List;

void ShellInsert(List* L, int dk) {
    int i, j;
    for (i = dk + 1; i <= L->length; ++i) {
        if (L->data[i].key >= L->data[i - dk].key) {
            L->data[0].key = L->data[i].key;
            strcpy(L->data[0].name, L->data[i].name);
            for (j = i - dk; j > 0 && L->data[0].key > L->data[j].key; j -= dk) {
                L->data[j + dk].key = L->data[j].key;
                strcpy(L->data[j + dk].name, L->data[j].name);
            }
            L->data[j + dk].key = L->data[0].key;
            strcpy(L->data[j + dk].name, L->data[0].name);
            printf("-- %s\n", L->data[j+dk].name);
        }
    }
}

void ShellSort(List* L, int dlta[3], int t) {
    for (int i = 0; i < t; ++i) {
        ShellInsert(L, dlta[i]);
    }
}

int BinarySearch(List* L, int length, int n) {
    int low = 1;
    int high = length;
    int mid;
    while (low <= high) {
        mid = (low + high) / 2;
        // printf("%d %d\n", L->data[mid].key, n);
        if (n == L->data[mid].key) {
            return mid;
        } if (n > L->data[mid].key) {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    return 0;
}

int main(int argc, char *argv[])
{
    List list = {5, {{0, ""},{5, "aa"},{2, "bb"},{8, "cc"},{3, "dd"},{6, "ee"}}};
    for (int i = 1; i <= 5; ++i) {
        printf("%s %d ", list.data[i].name, list.data[i].key);
    }

    int dlta[3] = {3, 2, 1};
    ShellSort(&list, dlta, 3);
    printf("\nlength: %d\n", list.length);
    for (int i = 1; i <= 5; ++i) {
        printf("%s %d ", list.data[i].name, list.data[i].key);
    }
    printf("\n");
    printf("----- search -----\n");
    int res = BinarySearch(&list, 5, 3);
    printf("%d\n", res);
    return 0;
}
