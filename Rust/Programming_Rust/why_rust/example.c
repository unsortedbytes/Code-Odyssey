#include<stdio.h>
#include<stdlib.h>
#include<string.h>

// Function that expects the an int pointer but we will pass the wrong types 
void modify_value (int* ptr){
    *ptr = 42;
}

// int get_string(){
//     return "Hello World";// This return string literal as int!
// }

int main(){
    printf("=== C Type Safety Issue Demo ===\n\n");

    // Issue 1: Implicit pointer conversions
    printf("1. Dangerous pointer conversions:\n");
    char buffer[10] = "test";
    int* int_ptr = (int*) buffer;// Cast char array into int pointer
    printf("Buffer before : %s\n", buffer);
    *int_ptr = 0x41424344;
    printf("Buffer :%s\n", buffer);
    printf("After int write: %s\n", buffer);
    printf("Buffer corruption likely occured!\n\n");
}
