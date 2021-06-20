#include <iostream>

int foo();

int A = foo();
int B = foo();
int C = A + B;

int foo() {
  return C + 1;
}

int main() {
  std::cout << "Hello from C++!" << std::endl;
  std::cout << "A: " << A << std::endl;
  std::cout << "B: " << B << std::endl;
  std::cout << "C: " << C << std::endl;
  return 0;
}