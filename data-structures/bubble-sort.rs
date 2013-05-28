/* Sample bubble sort program in Rust.
   Tested to compile with rust-0.6.
*/
extern mod extra;
extern mod benchmark;
use std::vec;
use benchmark::Benchmark;

fn bubble_sort<T:Ord+Copy>(arr: ~[T]) -> ~[T] {
    let mut result = copy arr;
    let mut left: uint;
    let mut right: uint = arr.len() - 1;
    let mut swap_occurred = true;

    while swap_occurred {
        swap_occurred = false;
        left = 0;
        while left < right {
            if result[left+1] < result[left] {
                // swap the two values
                vec::swap(result, left, left+1);
                swap_occurred = true;
            }
            left += 1;
        }

        right -= 1;
    }

    return result;
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(bubble_sort);
}

