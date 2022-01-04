#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

#define INFINITY INT_MAX
#define MAX_VERTEX_NUM 20
typedef int VRType;
int g = INFINITY;
int g_i = 0;

typedef enum { DG, DN, UDG, UDN } GraphKind;

typedef struct VectexType {
    int data;
} VectexType;

typedef struct ArcCell {
    VRType adj;  // 权值
} ArcCell, Path[MAX_VERTEX_NUM], AdjMatrix[MAX_VERTEX_NUM][MAX_VERTEX_NUM];

typedef struct MGraph {
    VectexType vexs[MAX_VERTEX_NUM];  // 顶点集
    AdjMatrix arcs;                   // 边集
    int vexnum;                       // 顶点数
    int arcnum;                       // 边数
    GraphKind kind;                   // 图的类型
} MGraph;

// VectexType[6] 存顶点集
// AdjMatrix[6][6] 存边集
// int[6] 存权值和
typedef int PathMatrix;
typedef int ShortPathTable;
void DIG(MGraph* G,
         int v0,
         PathMatrix* P[MAX_VERTEX_NUM][MAX_VERTEX_NUM],
         ShortPathTable* D[MAX_VERTEX_NUM]) {
    int added[MAX_VERTEX_NUM];
    // 刷新为0先，之后确定那些是通路，、
    // P记录了通路矩阵
    // D记录了v0到各点的距离，会不断更新
    for (int i = 0; i < G->vexnum; i++) {
        added[i] = 0;
        D[i] = G->arcs[v0][i].adj;
        for (int j = 0; j < G->vexnum; j++) {
            P[i][j] = 0;
        }
        if (D[i] < INFINITY) {
            P[i][v0] = 1;
            P[i][i] = 1;
        }
    }
    // printf("DIG: 第一层循环\n");
    // for (int k = 0; k < G->vexnum; k++) {
    //     printf("%d ", D[k]);
    // }
    // printf("\n");
    // for (int L = 0; L < G->vexnum; L++) {
    //     for (int k = 0; k < G->vexnum; k++) {
    //         printf("%d ", P[L][k]);
    //     }
    //     printf("\n");
    // }

    // 初始化v0
    D[v0] = 0;
    added[v0] = 1;
    // 主循环
    // printf("DIG: 主循环\n");

    int v, min;
    for (int i = 0; i < G->vexnum; ++i) {
        if (i == v0) {
            continue;
        }

        min = INFINITY;
        for (int j = 0; j < G->vexnum; ++j) {
            if (!added[j] && D[j] < min) {
                v = j;
                min = D[j];
            }
        }
        // printf("=================================\n");
        // printf("min: %d v: %d \n", min, v);
        added[v] = 1;
        // for (int i = 1; i < G->vexnum; i++) {
        //     printf("%d ", D[i]);
        // }
        // printf("\n");
        for (int j = 0; j < G->vexnum; ++j) {
            if (!added[j] && (min + G->arcs[v][j].adj < INFINITY) &&
                (min + G->arcs[v][j].adj < D[j])) {
                D[j] = min + G->arcs[v][j].adj;
                for (int k = 0; k < G->vexnum; k++) {
                    P[j][k] = P[v][k];
                    // printf("%d ", P[i][j]);
                }
                P[j][j] = 1;  // P1[w] = P1[v] + P1[w]
            }
        }
        // printf("\n");
    }
    int aaa = 0;
    int aaa_i = 0;
    for (int i = 0; i < G->vexnum; i++) {
        if (D[i] > aaa && D[i] < INFINITY) {
            aaa = D[i];
            aaa_i = i;
        }
        printf("%d ", D[i]);
    }
    if (g > aaa) {
        g_i = aaa_i;
    }
    printf("\n%d %d %d", aaa, aaa_i, g_i);
    printf("\n");
}

int main(int argc, char const* argv[]) {
    int a[6][6] = {
        {INFINITY, INFINITY, 10, INFINITY, 30, 100},
        {INFINITY, INFINITY, 5, INFINITY, INFINITY, INFINITY},
        {INFINITY, INFINITY, INFINITY, 50, INFINITY, INFINITY},
        {INFINITY, INFINITY, INFINITY, INFINITY, INFINITY, 10},
        {INFINITY, INFINITY, INFINITY, 20, INFINITY, 60},
        {INFINITY, INFINITY, INFINITY, INFINITY, INFINITY, INFINITY},
    };

    for (int i = 0; i < 6; i++) {
        int tmp;
        MGraph m;

        m.vexnum = 6;
        printf("===========================\n");
        // ===================== 求最短路径 =======================
        VectexType finalvexs[MAX_VERTEX_NUM];  // 最短路径顶点集
        int v0;
        m.vexnum = 6;
        for (int i = 0; i < m.vexnum; i++) {
            for (int j = 0; j < m.vexnum; j++) {
                if (i == j) {
                    m.arcs[i][j].adj = INFINITY;
                    continue;
                }
                // scanf("%d", &tmp);
                tmp = a[i][j];
                if (tmp < 0) {
                    m.arcs[i][j].adj = INFINITY;
                } else {
                    m.arcs[i][j].adj = tmp;
                }
            }
        }
        v0 = i;
        PathMatrix P[MAX_VERTEX_NUM][MAX_VERTEX_NUM];
        ShortPathTable D[MAX_VERTEX_NUM];
        DIG(&m, v0, P, D);
        printf("\n");
        printf("%d", g_i);
    }
    printf("\n应该选择节点%d\n", g_i);
    return 0;
}
