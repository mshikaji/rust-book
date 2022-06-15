use rand::Rng;
// use partition::partition;

fn qsort1(vec: &mut [i32]) {
    println!("{:?}", vec);
    if vec.len() <= 1 {
        return;
    }
    let pivot_idx = rand::thread_rng().gen_range(0..vec.len());
    let pivot = vec[pivot_idx];
    println!("pivot is {}", pivot);
    let (lesser, greater) = vec.iter().partition(|&x| x < pivot);
    //let (lesser, greater) = vec.split_at_mut(mid);
    qsort1(lesser);
    qsort1(greater);
}

fn main() {
    let mut v = vec![0, 9, 8, 7, 6, 4, 3, 2, 5, 1];
    qsort1(&mut v);
    println!("{:?}", v);
}
