
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
