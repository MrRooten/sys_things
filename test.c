#include <stdio.h>
#include <stdlib.h>
#include <string.h>
void func_start() {
    asm (
        "push rbp\n"
        ""
    )
}

void func_end() {

}

int main() {
    unsigned char* code = malloc(func_end - func_start);
    memcpy(code,func_start,func_end - func_start);
    FILE* f = fopen("./test.bin","wb");
    fwrite(code,func_end - func_start,func_end - func_start,f);
}