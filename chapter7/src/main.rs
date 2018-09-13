fn quick_sort(arr: &mut [i32], p: usize, r: usize) {
    if p < r {
        let q = partition(arr, p, r);
        quick_sort(arr, p, q - 1);
        quick_sort(arr, q + 1, r);
    }
}

fn partition(arr: &mut [i32], p: usize, r: usize) -> usize {
    let x = arr[r];
    let mut i = p;
    for j in p..r {
        if arr[j] <= x {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, r);
    i
}

fn main() {
    let mut arr = [4,1,3,5,2,9,6,8,7];
    let len = arr.len();
    quick_sort(&mut arr, 0, len-1);
    println!("{:?}", arr);
}
