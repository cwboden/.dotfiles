use std::collections::{BinaryHeap, HashMap};
use std::vec::Vec;

fn mean(vector: &[i32]) -> i32 {
    if vector.is_empty() {
        return 0;
    }

    let mut sum = 0;

    for element in vector {
        sum += element
    }

    sum / (vector.len() as i32)
}

fn pop_from_heap(heap: &mut BinaryHeap<i32>) -> i32 {
    match heap.pop() {
        Some(value) => value,
        None => panic!("Should never reach here"),
    }
}

fn median(vector: &[i32]) -> i32 {
    let vector_copy = Vec::from(vector);
    let mut heap = BinaryHeap::from(vector_copy);
    let mut popped_count = 0;
    let mut last_popped = 0;

    while popped_count < heap.len() {
        last_popped = pop_from_heap(&mut heap);
        popped_count += 1;
    }

    if popped_count == heap.len() {
        if heap.is_empty() {
            return 0;
        }

        let heap_top = pop_from_heap(&mut heap);
        (last_popped + heap_top) / 2
    } else {
        last_popped
    }
}

fn mode(vector: &[i32]) -> i32 {
    let mut hash_map = HashMap::new();

    for element in vector {
        let entry = hash_map.entry(element).or_insert(0);
        *entry += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;
    for (&entry, &count) in hash_map.iter() {
        if count > max_count {
            max_count = count;
            mode = *entry
        }
    }

    mode
}

fn main() {
    let a = vec![1, 2, 3, 3, 4, 5];
    let b = vec![7, 13, 23, 29, 37, 41];
    let c = vec![0, 0, 0, -1, 7];
    let d = vec![-15, -12, 37, 14, -1, 9, 133];
    let e = vec![42];
    let f = Vec::new();
    let g = vec![i32::max_value(), i32::min_value()];
    let h = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let i = vec![1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 2, 3, 1];
    let j = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    let vectors = vec![a, b, c, d, e, f, g, h, i, j];

    for vector in vectors.iter() {
        println!("Generating stats for Vector:");
        println!("\tMean:\t{}", mean(vector));
        println!("\tMedian:\t{}", median(vector));
        println!("\tMode:\t{}", mode(vector));
        println!();
    }
}
