#include <iostream>
using namespace std;

int main() {
    int* ptr = nullptr; // initializing pointer to null
    *ptr = 10; // dereferencing a null pointer, which will cause a null pointer exception
    cout << *ptr << endl;
    return 0;
}