#include <stdio.h>

int main(void) {
    FILE *file = fopen("hello.txt", "r");
    char line[1024];
    // will segfault if file doesn't exist
    fgets(line, sizeof line, file);

    printf("%s", line);
}