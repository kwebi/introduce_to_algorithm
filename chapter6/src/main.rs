struct Heap(Vec<i32>);

impl Heap {
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
    fn build_max_heap(&mut self) {
        for i in (1..self.heap_size() + 1).rev() {
            self.max_heapify(i);
        }
    }
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
    fn heap_maximum(&self) -> i32 {
        self.0[0]
    }
    fn heap_extract_max(&mut self) -> i32{
        let size = self.heap_size();
        if size < 1 {
            panic!("heap underflow");
        }
        let max = self.0[0];
        self.0[0] = self.0[size-1];
        self.0.truncate(size-1);
        self.max_heapify(1);
        max
    }
}

fn main() {
    let mut h = Heap(vec![4, 16, 3, 14, 7, 9, 10, 8, 2, 1]);
    h.heap_sort();
    for i in h.0.iter() {
        print!("{} ", i);
    }
}
