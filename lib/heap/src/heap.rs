use core::fmt::Display;

pub struct Heap<T: PartialOrd + Clone> {
    heap: Vec<T>,
}

impl<T: PartialOrd + Clone> Clone for Heap<T> {
    fn clone(&self) -> Self {
        Heap {
            heap: self.heap.clone(),
        }
    }
}

impl<T: PartialOrd + Clone> Heap<T> {
    pub fn new() -> Heap<T> {
        Heap { heap: vec![] }
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn insert(&mut self, value: T) {
        self.heap.push(value);
        let mut index = self.len() - 1;
        while index != 0 && self.heap[index] < self.heap[(index - 1) / 2] {
            self.heap.swap(index, (index - 1) / 2);
            index = (index - 1) / 2;
        }
    }

    fn push_down(&mut self) {
        let length = self.len();
        if length == 2 {
            if self.heap[0] > self.heap[1] {
                self.heap.swap(0, 1);
            }
            return;
        }
        let mut index = 0;

        while 2 * index + 1 <= length - 1 {
            if 2 * index + 2 <= length - 1 {
                let left_child_index = 2 * index + 1;
                let right_child_index = 2 * index + 2;
                let left_child = self.heap[left_child_index].clone();
                let right_child = self.heap[right_child_index].clone();
                if self.heap[index]
                    > if left_child < right_child {
                        left_child.clone()
                    } else {
                        right_child.clone()
                    }
                {
                    if left_child < right_child {
                        self.heap.swap(index, left_child_index);
                        index = left_child_index;
                    } else {
                        self.heap.swap(index, right_child_index);
                        index = right_child_index;
                    }
                } else {
                    break;
                }
            } else {
                let left_child_index = 2 * index + 1;
                let left_child = self.heap[left_child_index].clone();
                if self.heap[index] > left_child {
                    self.heap.swap(index, left_child_index);
                    index = left_child_index;
                } else {
                    break;
                }
            }
        }
    }

    pub fn pop_min(&mut self) -> Option<T> {
        if let Some(last) = self.heap.pop() {
            if self.len() > 0 {
                let min = self.heap[0].clone();
                self.heap[0] = last;
                self.push_down();
                Some(min)
            } else {
                Some(last)
            }
        } else {
            None
        }
    }

    pub fn heap_sort(&self) -> Vec<T> {
        let mut tmp = self.clone();
        let mut sorted_vec: Vec<T> = vec![];
        while let Some(value) = tmp.pop_min() {
            sorted_vec.push(value);
        }
        sorted_vec
    }
}

impl<T: PartialOrd + Clone + Display> Heap<T> {
    pub fn display(&self) {
        let mut row_length = 1;
        let mut count = 0;
        for v in self.heap.clone() {
            print!("{} ", v);
            count += 1;
            if row_length == count {
                println!("");
                row_length *= 2;
                count = 0;
            }
        }
        if count > 0 {
            println!("");
        }
    }
}
