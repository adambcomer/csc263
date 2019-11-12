# Heaps
A heap is a complete binary tree datastructure used to build priority queues and sort data, sorting is done using heapsort. 

## Datastructure
A heap is a complete binary tree, that is, each level, that is not the last, the tree is filled and on the last level the nodes are pushed leftward. 

## Heapsort
A heap based sorting algorithm that runs in `O(n*log(n))`[1]. Although it has good performance, a standard QuickSort can beat out heapsort.[2]

## Implementation
`max_heap.rs` is a Rust implementation of a Max Heap using a Vector to store the data. The stdlib implementation of the `Vec` has an amortized cost of `O(1)` for inserts, and shouldn't effect the runtime of the textbook version of a MaxHeap or Heapsort[3]. Implementation could be improved using an array implementation, to give finer control over growth and shrinkage in the MaxHeap.

## Sources
[1] Cormen, Thomas H.., et al. <i>Introduction to Algorithms<i>.
[2] https://stackoverflow.com/questions/2467751/quicksort-vs-heapsort
[3] https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees