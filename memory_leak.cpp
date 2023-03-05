#include <cstring>
#include <iostream>

int main() {
  char *string = new char[6];
  memset(string, 0, 6);
  strncpy(string, "Hello", 5);
  std::cout << string << std::endl;
}
