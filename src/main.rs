fn main() {
    let l1 = [1, 2, 3];
    let l2 = [0; 1000];

    println!("{:?}", l1);

    let i: i32 = l1[0];

    let [x, y, z] = l1;

    let l3: &[i32] = &l1[..];
    println!("{:?}", l3);
}