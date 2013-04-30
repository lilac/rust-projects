use core::rand;
use core::vec;

fn generate_random_array(size: uint) -> ~[uint] {
    let ret = vec::build_sized(size, |push| {
        for size.times {
            push(rand::random());
        }
    });

    return ret;
}

fn bubble_sort(mut arr: ~[uint]) -> ~[uint] {
    let mut left = 0;
    let mut right: uint;
    let max = arr.len();
    while left < max {
        right = left + 1;
        while right < max {
            if arr[right] < arr[left] {
                arr[left] <-> arr[right];
            }
            right += 1;
        }

        left += 1;
    }

    return arr;
}

fn main() {
    let vals = generate_random_array(10);
    let sorted_vals = bubble_sort(vals);
    for sorted_vals.each |v| {
        core::io::println(fmt!("%?", *v));
    }
}
