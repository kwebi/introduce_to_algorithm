pub struct MinHeap {
    size: usize,
    array: Vec<i32>,
}

impl MinHeap {
    pub fn new(array: Vec<i32>) -> MinHeap {
        MinHeap {
            size: array.len(),
            array,
        }
    }
    fn parent(i: usize) -> usize {
        i / 2
    }
    fn left(i: usize) -> usize {
        2 * i
    }
    fn right(i: usize) -> usize {
        2 * i + 1
    }
    fn heap_size(&self) -> usize {
        self.size
    }
    fn swap(&mut self, a: usize, b: usize) {
        self.array.swap(a, b);
    }
    fn min_heapify(&mut self, mut i: usize) {
        loop {
            let l = MinHeap::left(i);
            let r = MinHeap::right(i);
            let mut min;
            if l <= self.size && self.array[i - 1] > self.array[l - 1] {
                min = l;
            } else {
                min = i;
            }
            if r <= self.size && self.array[min - 1] > self.array[r - 1] {
                min = r;
            }
            if min == i {
                break;
            } else {
                self.swap(i - 1, min - 1);
            }
            i = min;
        }
    }
    pub fn build_min_heap(&mut self) {
        for i in (1..self.size + 1).rev() {
            self.min_heapify(i);
        }
    }
    fn heap_exterct_min(&mut self) -> i32 {
        let size = self.size;
        if size < 1 {
            panic!("heap underflow");
        }
        let max = self.array[0];
        self.swap(0, size - 1);
        self.size -= 1;
        self.min_heapify(1);
        max
    }
    pub fn heap_insert_key(&mut self, mut i: usize, key: i32) {
        if key > self.array[i - 1] {
            panic!("the new key is larger");
        }
        self.array[i - 1] = key;
        let mut parent = MinHeap::parent(i);
        while i > 1 && self.array[i - 1] < self.array[parent - 1] {
            self.swap(i - 1, parent - 1);
            i = parent;
            parent = MinHeap::parent(i);
        }
    }
    pub fn min_heap_insert(&mut self, key: i32) {
        self.size += 1;
        self.array.push(i32::max_value());
        let i = self.size;
        self.heap_insert_key(i, key);
    }
}
