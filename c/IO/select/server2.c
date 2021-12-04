#include <arpa/inet.h>
#include <bits/types/struct_timeval.h>
#include <netinet/in.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <unistd.h>

#define DEFAULT_PORT 1234
#define MAX_CLIENT_NUMS 10

int main(int argc, char *argv[])
{
    int serverfd, clientfd;
    struct sockaddr_in server_addr;
    struct sockaddr_in client_addr;

    // 创建socket + 初始化
    if ((serverfd = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP)) == -1) {
        perror("[ERROR] socket create failed!");
        return -1;
    }
    printf("[INFO] create socket success!\n");
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(DEFAULT_PORT);

    // 建立链接
    if (bind(serverfd, (struct sockaddr*)&server_addr, sizeof(server_addr)) == -1) {
        perror("[ERROR] bind failed!");
        return -2;
    }
    printf("[INFO] bind success!\n");

    // 监听端口
    if (listen(serverfd, MAX_CLIENT_NUMS) == -1) {
        perror("listening failed!\n");
        return -3;
    }
    printf("[INFO] listen on port %d success!\n", DEFAULT_PORT);

    // 客户端fd_set
    fd_set clientfd_set;
    // 最大的fd号
    int maxsock;
    // 超时时间
    struct timeval tv;
    // 活动的fd_set
    int active_clientfd_set[5];
    memset((void*)active_clientfd_set, 0, sizeof(active_clientfd_set));

    // 描述符数量
    int conn_amount = 0;
    maxsock = serverfd;
    char buffer[1024];
    int ret = 0;

    // 循环中要做的事
    // 1. 判断是否有并建立新的客户端连接
    // 2. 检查是否有客户端连接的数据报文已经准备好，准备好就处理返回，否则continue
    for (;;) {
        // 初始化fd到集合
        FD_ZERO(&clientfd_set);
        // 加入server的fd
        FD_SET(serverfd, &clientfd_set);
        // 设置超时时间;
        tv.tv_sec = 30;
        tv.tv_usec = 0;

        // 把活动的句柄加入到文件描述符中
        for (int i = 0; i < 5; ++i) {
            if (active_clientfd_set[i] != 0) {
                FD_SET(active_clientfd_set[i], &clientfd_set);
            }
        }

        // select, 文件描述符的范围（最大的fd + 1），可读集合，可写集合，监视异常集合，超时时间（NULL阻塞,0非阻塞，>0先阻塞超时则直接返回）
        ret = select(maxsock + 1, &clientfd_set, NULL, NULL, &tv);
        if (ret < 0) {
            perror("[ERROR] select failed!");
            break;
        } else if (ret == 0) {
            printf("[WARN] timeout...\n");
            continue;
        }

        // 轮询文件描述符
        for (int i = 0; i < conn_amount; ++i) {
            if (FD_ISSET(active_clientfd_set[i], &clientfd_set)) {
                // 判断fd是否在集合中
                printf("[INFO] start recv from client[%d]\n", i);
                ret = recv(active_clientfd_set[i], buffer, 1024, 0);
                if (ret <= 0) {
                    printf("[INFO] client[%d] closed\n", i);
                    FD_CLR(active_clientfd_set[i], &clientfd_set);
                    active_clientfd_set[i] = 0;
                } else {
                    printf("[INFO] recv from client[%d]: %s\n", i, buffer);
                }
            }
        }

        // 检查是否有新的连接，如果有就加入到active_clientfd_set里
        if (FD_ISSET(serverfd, &clientfd_set)) {
            // 建立新的clientfd
            struct sockaddr_in new_clientaddr;
            socklen_t new_size = sizeof(struct sockaddr_in);
            int newclientfd = accept(serverfd, (struct sockaddr*)&new_clientaddr, &new_size);
            if (newclientfd < 0) {
                perror("[WARN] accept failed!\n");
                continue;
            }
            // 加入到fdset中
            if (conn_amount < 5) {
                // 找到位置，储存新的连接的fd
                active_clientfd_set[conn_amount] = newclientfd;
                conn_amount += 1;
                memset(buffer, 0, sizeof(buffer));
                strcpy(buffer, "this is server! Welcome!");
                send(newclientfd, buffer, sizeof(buffer), 0);
                printf("[INFO] new connection client[%d] %s: %d\n", conn_amount - 1, inet_ntoa(new_clientaddr.sin_addr), ntohs(new_clientaddr.sin_port));
                memset(buffer, 0, sizeof(buffer));
                // 没有收到回复，客户端连接已关闭
                ret = recv(newclientfd, buffer, sizeof(buffer), 0);
                if (ret < 0) {
                    perror("[ERROR] recv failed!\n");
                    close(newclientfd);
                    return -1;
                }
                printf("client[%d]: %s\n", conn_amount, buffer);
                if (newclientfd > maxsock) {
                    maxsock = newclientfd;
                }
            } else {
                printf("[WARN] max connection! quit now!\n");
                break;
            }
        }
    }
    for (int i = 0; i < 5; ++i) {
        if (active_clientfd_set[i] != 0) {
            close(active_clientfd_set[i]);
        }
    }
    close(serverfd);
    return 0;
}
