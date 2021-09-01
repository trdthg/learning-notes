#include <stdio.h>


void hello() {
    printf("Hello from C!\n");
}

void greet(const char* name) {
    printf("Hello, %s!\n", name);
}

// void print_app_info() {
//     #ifdef WELCOME
//         printf("Welcome to ");
//     #endif
//         printf("%s - version %s\n", APP_NAME, VERSION);
// }