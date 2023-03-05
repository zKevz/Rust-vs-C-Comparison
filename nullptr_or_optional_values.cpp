#include <iostream>

int main() {
  int *ptr = nullptr;
  if (ptr == nullptr) {
    std::cout << "Null" << std::endl;
  } else {
    std::cout << "Not null, safe to use" << std::endl;
  }
}
