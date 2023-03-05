#include <cstring>
#include <iostream>

int main() {
  int number = 50;
  uint8_t *buffer = new uint8_t[10];
  memset(buffer, 0, 10);
  memcpy(buffer, &number, sizeof(number));
  delete[] buffer;
}
