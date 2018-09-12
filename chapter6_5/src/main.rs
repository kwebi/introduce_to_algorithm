mod min_heap;
use min_heap::MinHeap;

struct Heap {
    size: usize,
    array: Vec<i32>,
}

impl Heap {
    fn new(array: Vec<i32>) -> Heap {
        Heap {
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
    fn max_heapify(&mut self, mut i: usize) {
        loop {
            let l = Heap::left(i);
            let r = Heap::right(i);
            let mut largest;
            if l <= self.heap_size() && self.array[l - 1] > self.array[i - 1] {
                largest = l;
            } else {
                largest = i;
            }
            if r <= self.heap_size() && self.array[r - 1] > self.array[largest - 1] {
                largest = r;
            }

            if largest == i {
                break;
            } else {
                let tmp = self.array[i - 1];
                self.array[i - 1] = self.array[largest - 1];
                self.array[largest - 1] = tmp;
            }
            i = largest;
        }
    }
    fn heap_maximum(&self) -> i32 {
        self.array[0]
    }
    fn heap_extract_max(&mut self) -> i32 {
        let size = self.heap_size();
        if size < 1 {
            panic!("heap underflow");
        }
        let max = self.array[0];
        self.array[0] = self.array[size - 1];
        self.size -= 1;
        self.max_heapify(1);
        max
    }
    fn heap_insert_key(&mut self, mut i: usize, key: i32) {
        if key < self.array[i - 1] {
            panic!("new key is smaller than current key");
        }
        self.array[i - 1] = key;
        while i > 1 && self.array[Heap::parent(i)] < self.array[i - 1] {
            let tmp = self.array[i - 1];
            self.array[i - 1] = self.array[Heap::parent(i)];
            self.array[Heap::parent(i)] = tmp;
            i = Heap::parent(i);
        }
    }
    fn max_heap_insert(&mut self, key: i32) {
        self.size += 1;
        self.array.push(i32::min_value());
        let i = self.size;
        self.heap_insert_key(i, key);
    }
}

fn main() {
    let mut h1 = Heap::new(vec![15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1]);
    h1.max_heap_insert(10);
    let mut h2 = MinHeap::new(vec![15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1]);
    h2.build_min_heap();
    h2.min_heap_insert(1);
}
