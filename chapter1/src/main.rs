use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut arr: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("integer"))
        .collect();
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i: isize = (j - 1) as isize;
        while i >= 0 && arr[i as usize] > key {
            arr[(i + 1) as usize] = arr[i as usize];
            i -= 1;
        }
        arr[(i + 1) as usize] = key;
    }
    for i in arr.iter() {
        println!("{}", i);
    }
}
