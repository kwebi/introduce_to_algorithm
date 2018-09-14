# 计数排序

计数排序假设`n`个输入元素都是`0`到`k`区间的一个整数(`k`为某整数)
当`k`为`O(n)`时，排序的时间为`O(n)`

计数排序基本思想是：对于每个输入元素`x`, 确定小于`x`的元素个数

### 先新建一个可变数组c, 初始化为0

```rust
let mut c: Vec<usize> = Vec::with_capacity(k);
for i in 0..k+1 {
    c.push(0);
}
```

### c记录a中每个元素出现的个数

```rust
for i in 0..a.len() {
    c[a[i]] = c[a[i]] + 1;
}
```

### 然后计算对于`i`从`0..k`, 有多少个元素是小于等于`i`

```rust
for i in 1..k+1 {
    c[i] = c[i] + c[i-1];
}
```

### 最后把元素a[i]放入数组b的正确位置上

```rust
for i in (0..a.len()).rev() {
    b[c[a[i]]-1] = a[i];
    c[a[i]] = c[a[i]] - 1;
}
```

+ 计数排序的时间代价为`O(k+n)`, 当`k=O(n)`时, 一般会采用计数排序
+ 该排序不是比较排序, 而是根据元素的值来确定元素的位置
+ 计数排序是稳定的

### 全部代码如下：

```rust
fn count_sort(a: &mut [usize], b: &mut [usize], k: usize) {
    let mut c: Vec<usize> = Vec::with_capacity(k+1);
    for _ in 0..k+1 {
        c.push(0);
    }
    for i in 0..a.len() {
        c[a[i]] = c[a[i]] + 1;
    }
    for i in 1..k+1 {
        c[i] = c[i] + c[i-1];
    }
    for i in (0..a.len()).rev() {
        b[c[a[i]]-1] = a[i];
        c[a[i]] = c[a[i]] - 1;
    }
}

fn main() {
    let mut a: [usize; 8] = [2,5,3,0,2,3,0,3];
    let mut b = [0usize; 8];
    count_sort(&mut a, &mut b, 5);
    println!("{:?}", b);
}

```
