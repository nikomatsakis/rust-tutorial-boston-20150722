fn copy_vec(mut to: Vec<i32>, from: Vec<i32>) -> Vec<i32> {
    for i in 0 .. from.len() {
        to.push(from[i]);
    }
    to
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let vec3 = copy_vec(vec1, vec2);
    println!("{:?}", vec3);
}

// Exercise 1. Convert `copy_vec` to use borrowing
// rather than taking ownership.

// Exercise 2. What happens if you try to call your
// new borrowing version of `copy_vec` with the same
// vector for both arguments (e.g., `copy_vec(&mut vec1, &vec1)`).
// Why? What bug is being prevented here?
