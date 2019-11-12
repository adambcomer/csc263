
/// A Vector based Max Heap implementation
/// 
/// Should satisfy the MaxHeap Property, that is, `MaxHeap.parent(i) >= MaxHeap.get(i)`.
pub struct MaxHeap<T: PartialOrd> {
    data: Vec<T>,
}

impl<T: PartialOrd> MaxHeap<T> {

    /// Creates a new empty `MaxHeap`
    pub fn new() -> MaxHeap<T> {
        MaxHeap { data: Vec::new() }
    }

    /// Creates a new `MaxHeap` from an existing vector
    /// 
    /// # Arguments
    /// 
    /// * `vec` - Vector to create a max heap from
    pub fn from_vec(vec: Vec<T>) -> MaxHeap<T> {
        MaxHeap { data: MaxHeap::create_max_heap(vec) }
    }

    /// Internal function to create a new `MaxHeap` from a vector
    /// 
    /// # Arguments
    /// 
    /// * `vec` - The vector to modify to satisfy the Max Heap Property
    fn create_max_heap(mut vec: Vec<T>) -> Vec<T> {
        for j in (0..((vec.len() as f32 / 2.0).floor() as usize)).rev() {
            let mut i = j;
            let mut largest = i;
            while { // Hacky Do-While loop
                let l = (2 * i) + 1;
                let r = (2 * i) + 2;
                if l < vec.len() && vec[l] > vec[i] {
                    largest = l;
                }
                if r < vec.len() && vec[r] > vec[largest] {
                    largest = r;
                }
                if i != largest {
                    vec.swap(i, largest);
                }

                i != largest
            } {
                i = largest;
            }
        }
        vec
    }

    /// Uses the heapsort algorithm to sort a vector
    /// 
    /// Sorts a vector, smallest to largest, using the heapsort algorithm.
    /// 
    /// # Arguments
    /// 
    /// * `vec` - Vector to sort
    /// 
    pub fn heapsort(vec: Vec<T>) -> Vec<T> {
        let mut vec = MaxHeap::create_max_heap(vec);

        let mut c = 1;
        for j in (1..vec.len()).rev() {
            vec.swap(0, j);

            let mut i = 0;
            let mut largest = i;
            while { // Hacky Do-While loop
                let l = (2 * i) + 1;
                let r = (2 * i) + 2;
                if l < (vec.len() - c) && vec[l] > vec[largest] {
                    largest = l;
                }
                if r < (vec.len() - c) && vec[r] > vec[largest] {
                    largest = r;
                }
                if i != largest {
                    vec.swap(i, largest);
                }

                i != largest
            } {
                i = largest;
                c += 1;
            }
        }
        vec
    }

    /// Gets an element at index i
    /// 
    /// # Arguments
    /// 
    /// * `i` - The index to look to
    pub fn get(&self, i: usize) -> Option<&T> {
        self.data.get(i)
    }

    /// Gets the parent element of an element's index
    /// 
    /// # Arguments
    /// 
    /// * `i` - Index to find the parent
    /// 
    /// If i is 0 or greater than the last index of the max heap, then the result will be None
    pub fn parent(&self, i: usize) -> Option<&T> {
        if i <= 0 || i >= self.data.len() {
            return None
        }
        let pos = (i as f32 / 2.0).ceil() as usize;
        self.data.get(pos - 1)
    }

    /// Gets the left element of an element's index
    /// 
    /// # Arguments
    /// 
    /// * `i` - Index to find the left element of
    /// 
    /// If (2 * i) + 1 is greater than the last index of the max heap, then the result will be None
    pub fn left(&self, i: usize) -> Option<&T> {
        self.data.get((2 * i) + 1)
    }

    /// Gets the right element of an element's index
    /// 
    /// # Arguments
    /// 
    /// * `i` - Index to find the right element of
    /// 
    /// If (2 * i) + 2 is greater than the last index of the max heap, then the result will be None
    pub fn right(&self, i: usize) -> Option<&T> {
        self.data.get((2 * i) + 2)
    }

    /// Inserts a new element into the `MaxHeap`
    /// 
    /// `MaxHeap` will automatically rebalance after insert, to satisfy the Max Heap Property.
    /// 
    /// # Arguments
    /// 
    /// * `d` - New data to insert
    pub fn insert(&mut self, d: T) {
        self.data.insert(0, d);
        self.max_heapify(0);
    }

    /// Rebalances the `MaxHeap` to satisfy the Max Heap Property
    /// 
    /// `max_heapify` assumes that the left and right sub-trees are Max Heaps. 
    /// This method is intended to rebalance the `MaxHeap` assuming at most 1 element 
    /// violates the Max Heap property. If the left and right sub-trees are not Max Heaps, 
    /// this method won't properly rebalance the `MaxHeap`.
    /// 
    /// # Arguments
    /// 
    /// * `i` - Index to perform max_heapify from
    fn max_heapify(&mut self, mut i: usize) {
        let mut largest = i;
        while { // Hacky Do-While loop
            let l = (2 * i) + 1;
            let r = (2 * i) + 2;
            if l < self.data.len() && self.data[l] > self.data[largest] {
                largest = l;
            }
            if r < self.data.len() && self.data[r] > self.data[largest] {
                largest = r;
            }
            if i != largest {
                self.data.swap(i, largest);
            }

            i != largest
        } {
            i = largest;
        }
    }

    /// Removes and returns the largest value in the `MaxHeap`, then rebalances the `MaxHeap` 
    /// to satisfy the Max Heap Property.
    pub fn pop(&mut self) -> Option<T> {
        let i = self.data.len() - 1;
        self.data.swap(0, i);
        let e = self.data.pop();
        self.max_heapify(0);

        e
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent() {
        let heap = MaxHeap::from_vec(vec![7, 6, 5, 4, 3, 2, 1]);

        assert_eq!(None, heap.parent(0));
        assert_eq!(Some(&7), heap.parent(1));
        assert_eq!(Some(&7), heap.parent(2));
        assert_eq!(Some(&6), heap.parent(3));
        assert_eq!(Some(&6), heap.parent(4));
        assert_eq!(Some(&5), heap.parent(5));
        assert_eq!(Some(&5), heap.parent(6));
        assert_eq!(None, heap.parent(7));
    }

    #[test]
    fn test_left() {
        let heap = MaxHeap::from_vec(vec![7, 6, 5, 4, 3, 2, 1]);

        assert_eq!(Some(&6), heap.left(0));
        assert_eq!(Some(&4), heap.left(1));
        assert_eq!(Some(&2), heap.left(2));
        assert_eq!(None, heap.left(3));
        assert_eq!(None, heap.left(4));
        assert_eq!(None, heap.left(5));
        assert_eq!(None, heap.left(6));
        assert_eq!(None, heap.left(7));
    }

    #[test]
    fn test_right() {
        let heap = MaxHeap::from_vec(vec![7, 6, 5, 4, 3, 2, 1]);

        assert_eq!(Some(&5), heap.right(0));
        assert_eq!(Some(&3), heap.right(1));
        assert_eq!(Some(&1), heap.right(2));
        assert_eq!(None, heap.right(3));
        assert_eq!(None, heap.right(4));
        assert_eq!(None, heap.right(5));
        assert_eq!(None, heap.right(6));
        assert_eq!(None, heap.right(7));
    }

    #[test]
    fn test_max_heapify() {
        let mut heap = MaxHeap {
            data: vec![1, 2, 0],
        };
        heap.max_heapify(0);
        assert_eq!(heap.data, vec![2, 1, 0]);

        let mut heap = MaxHeap {
            data: vec![1, 0, 2],
        };
        heap.max_heapify(0);
        assert_eq!(heap.data, vec![2, 0, 1]);

        let mut heap = MaxHeap {
            data: vec![1, 2, 0, 4],
        };
        heap.max_heapify(0);
        assert_eq!(heap.data, vec![2, 4, 0, 1]);
    }

    #[test]
    fn test_insert() {
        let mut heap = MaxHeap::new();

        heap.insert(0);
        assert_eq!(heap.data, vec![0]);

        heap.insert(1);
        assert_eq!(heap.data, vec![1, 0]);

        heap.insert(-5);
        assert_eq!(heap.data, vec![1, -5, 0]);

        heap.insert(-1);
        assert_eq!(heap.data, vec![1, 0, -5, -1]);
    }

    #[test]
    fn test_build_max_heap() {
        let v = vec![0, 1, 2, 3];
        let v = MaxHeap::create_max_heap(v);
        assert_eq!(vec![3, 1, 2, 0], v);

        let v = vec![8, 2, 9, 4, 7];
        let v = MaxHeap::create_max_heap(v);
        assert_eq!(vec![9, 7, 8, 4, 2], v);

        let v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let v = MaxHeap::create_max_heap(v);
        assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], v);
    }

    #[test]
    fn test_pop() {
        let mut heap = MaxHeap::from_vec(vec![3, 2, 1]);
        let e = heap.pop();
        assert_eq!(Some(3), e);
        assert_eq!(vec![2, 1], heap.data);

        let mut heap = MaxHeap::from_vec(vec![5, 4, 1, 3]);
        let e = heap.pop();
        assert_eq!(Some(5), e);
        assert_eq!(vec![4, 3, 1], heap.data);
    }

    #[test]
    fn test_heapsort() {
        let v = vec![3, 2, 1];
        let v = MaxHeap::heapsort(v);
        assert_eq!(vec![1, 2, 3], v);

        let v = vec![5, 2, 1, 3];
        let v = MaxHeap::heapsort(v);
        assert_eq!(vec![1, 2, 3, 5], v);
    }
}
