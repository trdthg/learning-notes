#include <arpa/inet.h>
#include <netinet/in.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define DEFAULT_PORT 1234

void handle_connect(int serverfd) {
    char buffer[1024];
    memset(buffer, 0, sizeof(buffer));

    recv(serverfd, buffer, sizeof(buffer), 0);
    printf("recv: %s\n", buffer);

    memset(buffer, 0, sizeof(buffer));
    strcpy(buffer, "this is client!");
    printf("send %ld bytes\n", sizeof(buffer));
    send(serverfd, buffer, sizeof(buffer), 0);
    for(;;) {
        memset(buffer, 0, sizeof(buffer));
        scanf("%s", buffer);
        int p = strlen(buffer);
        buffer[p + 1] = '\0';
        send(serverfd, buffer, sizeof(buffer), 0);
    }

}

int main(int argc, char *argv[])
{
    int serverfd;
    struct sockaddr_in client_addr;

    client_addr.sin_family = AF_INET;
    client_addr.sin_port = htons(DEFAULT_PORT);
    client_addr.sin_addr.s_addr = inet_addr("127.0.0.1");
    serverfd = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (serverfd < 0) {
        perror("[ERROR] create socket fd failed!");
        return -2;
    }

    if (connect(serverfd, (struct sockaddr*)&client_addr, sizeof(client_addr)) < 0) {
        perror("[ERROR] connect failed!");
        return -3;
    }

    handle_connect(serverfd);
    close(serverfd);

    return 0;
}
