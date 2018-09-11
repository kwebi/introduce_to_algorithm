## 堆排序

堆是一个数组，可以看成一颗二叉树

```rust
struct Heap(Vec<i32>);

impl Heap {
    //父节点索引
    fn parent(i: usize) -> usize {
        i / 2
    }
    //左子节点索引
    fn left(i: usize) -> usize {
        2 * i
    }
    //右子节点索引
    fn right(i: usize) -> usize {
        2 * i + 1
    }
}
```

### 首先要能维护最大堆(排序通常用最大堆)

思路就是找出该节点和两个子节点的最大节点
如果该节点为最大，则结束，否则，先交换，然后继续判断子节点
递归实现起来更简单
具体实现如下

```rust
impl Heap {
    fn heap_size(&self) -> usize {
        self.0.len()
    }
    fn max_heapify(&mut self, mut i: usize) {
        loop {
            let l = Heap::left(i);
            let r = Heap::right(i);
            let mut largest;
            if l <= self.heap_size() && self.0[l - 1] > self.0[i - 1] {
                largest = l;
            } else {
                largest = i;
            }
            if r <= self.heap_size() && self.0[r - 1] > self.0[largest - 1] {
                largest = r;
            }

            if largest == i {
                break;
            } else {
                let tmp = self.0[i - 1];
                self.0[i - 1] = self.0[largest - 1];
                self.0[largest - 1] = tmp;
            }
            i = largest;
        }
    }
}
```

### 构建最大堆

依次对每个节点都进行维护即可构建最大堆

```rust
impl Heap {
    fn build_max_heap(&mut self) {
        for i in (1..self.heap_size() + 1).rev() {
            self.max_heapify(i);
        }
    }
}
```

### 堆排序

1. 先构建一个最大堆
2. 交换第一个和最后一个元素的值
3. 缩小堆空间
4. 对第一个节点进行堆维护

```rust
impl Heap {
    fn heap_sort(&mut self) {
        let len = self.heap_size();
        self.build_max_heap();
        let mut v = Vec::with_capacity(self.heap_size());
        for i in (2..len + 1).rev() {
            let tmp = self.0[0];
            self.0[0] = self.0[i - 1];
            v.insert(0, tmp);
            let len = self.heap_size();
            self.0.truncate(len - 1);
            self.max_heapify(1);
        }
        self.0.append(&mut v);
    }
}
```

+ 整个堆排序时间复杂度为O(nlgn)
+ 构建堆时间复杂度为O(n)
+ 每次维护堆为O(lgn)， 总共调用n-1次
