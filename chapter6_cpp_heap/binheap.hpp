#pragma once
#include <vector>
using namespace std;

template <class T>
class Heap
{
  public:
    Heap(size_t maxElems)
    {
        h = new HeapStruct;
        h->Elems = new T[maxElems + 1];
        h->Capacity = maxElems;
        h->size = 0;
    }
    ~Heap()
    {
        destroy();
    }
    void insert(T x)
    {
        size_t i;
        if (isFull())
        {
            return;
        }
        for (i = ++h->size; i / 2 > 0 && h->Elems[i / 2] > x; i /= 2)
        {
            h->Elems[i] = h->Elems[i / 2];
        }
        h->Elems[i] = x;
    }
    T deleteMin()
    {
        size_t i, child;
        T minElems, lastElems;
        if (isEmpty())
            return h->Elems[0];
        minElems = h->Elems[1];
        lastElems = h->Elems[h->size--];
        for (i = 1; i * 2 <= h->size; i = child)
        {
            child = i * 2;
            if (child != h->size && h->Elems[child + 1] < h->Elems[child])
                ++child;
            if (lastElems > h->Elems[child])
                h->Elems[i] = h->Elems[child];
            else
                break;
        }
        h->Elems[i] = lastElems;
        return minElems;
    }
    bool isFull()
    {
        return h->size == h->Capacity;
    }
    bool isEmpty()
    {
        return h->size == 0;
    }
    T findMin()
    {
        return h->Elems[1];
    }

  private:
    void destroy()
    {
        delete h->Elems;
        delete h;
    }
    void makeEmpty() {}

    struct HeapStruct
    {
        size_t Capacity;
        size_t size;
        T *Elems;
    };
    HeapStruct* h;
};