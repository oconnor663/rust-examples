#include<stdio.h>

// The pointer y is only valid inside this function, but C will let us return
// it anyway!
int* bad_ptr(int x) {
  int* y = &x;
  return y;
}

void main() {
  int* x = bad_ptr(1);
  // Calling bad_ptr() again changes what x is pointing to!
  bad_ptr(2);
  printf("I expect this to be 1, but it's actually 2!\n=> %d\n", *x);
}
