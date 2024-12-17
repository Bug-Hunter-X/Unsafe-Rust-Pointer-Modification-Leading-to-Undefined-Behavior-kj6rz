fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0;
    v[index] = 10;
    println!("Modified Vector: {:?}", v);
    
    // Alternatively, using iterators
    let mut v2 = vec![1, 2, 3];
    v2.iter_mut().nth(0).map(|x| *x = 10);
    println!("Modified Vector (iterators): {:?}", v2);
} 