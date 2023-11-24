mod heap;
use heap::*;

fn main() {
    let mut sample = Heap::<f32>::new();
    let sample_data = vec![2.0, 4.5, 2.8, 1.3, 4.1, 52.41, 2.41, 62.41, 6.42, 3.62];
    for i in sample_data {
        sample.insert(i);
    }
    sample.display();
    println!("min: {}", sample.pop_min().unwrap());
    sample.display();
    println!("{:?}", sample.heap_sort());
}
