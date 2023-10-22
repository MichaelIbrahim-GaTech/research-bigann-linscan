use std::collections::HashMap;

mod index;

fn main() {
    let mut ind = index::Index::new();
    let v1 = HashMap::from([(1_u32, 4_f32), (5, 6.0)]);
    let v2 = HashMap::from([(2_u32, 4_f32), (5, 9.0)]);
    let q = HashMap::from([(13_u32, 4_f32), (5, 12.0)]);
    ind.insert(&v1);
    ind.insert(&v2);

    println!("Index built: {}", ind);
    let dur = std::time::Duration::new(5, 0);
    let r = ind.retrieve(&q, 4, 0.2, Some(dur));
    println!("{:?}", &r);
    let mut my_int = 1024;
    my_int = std::cmp::max(1,my_int >> 2);
    println!("{}", &my_int);
}
