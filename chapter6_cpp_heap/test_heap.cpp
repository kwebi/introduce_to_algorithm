#include "binheap.hpp"
#include <iostream>
using namespace std;

int main()
{
    Heap<int> h(100);
    h.insert(10);
    h.insert(2);
    h.insert(4);
    h.insert(5);
    h.insert(7);
    h.insert(8);
    for (int i = 0; i < 3; ++i)
        cout << h.deleteMin() << endl;
    return 0;
}