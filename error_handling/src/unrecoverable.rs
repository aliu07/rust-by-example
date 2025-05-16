pub fn panic() {
    panic!("Crash and burn!");
}

pub fn out_of_bounds() {
    let v = vec![1, 2, 3];
    v[99];
}
