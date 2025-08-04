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


    // Issue 2 : Unsigned/Signed mixing
    printf("2. Signed/unsigned confusion:\n");
    int signed_val = -1;
    unsigned int unsigned_val = signed_val;
    printf("Signed value : %d\n", signed_val);
    printf("As unsigned: %u\n", unsigned_val);

    if (unsigned_val > 1000){
        printf("Unsigned value appers huge due to conversion!\n\n");
    }


    // Issue 3 : Array bounds not checked
    printf("3. Array bounds violations:\n");
    int arr[5] = {12, 42, 53, 12, 42};
    printf("Valid access arr[4] = %d\n", arr[4]);
    printf("Invalid access arr[10] = %d\n", arr[10]);// out of bounds
    arr[10] = 999;// Writing out of bounds - undeifing behaviour
    printf("Wrote  to arr[10] - memory corruption possible!\n\n");

