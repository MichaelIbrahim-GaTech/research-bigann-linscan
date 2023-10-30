use std::collections::HashMap;

mod index;

fn main() {
    let mut ind = index::Index::new();
    let v1 = HashMap::from([(1_u32, 4_u32), (5, 6)]);
    let v2 = HashMap::from([(2_u32, 4_u32), (5, 9)]);
    let q = HashMap::from([(13_u32, 4_u32), (5, 12)]);
    ind.insert(&v1);
    ind.insert(&v2);

    println!("Index built: {}", ind);
    let dur = std::time::Duration::new(5, 0);
    let r = ind.retrieve(&q, 4, 1, Some(dur));
    println!("{:?}", &r);
    let mut my_int = 1024;
    my_int = std::cmp::max(1,my_int >> 2);
    println!("{}", &my_int);
    let table = 100000 / 65536_u32;
    let id   = (std::u16::MAX as u32 % 65536_u32) as u16;
    let new_id = 2 * 65536_u32 + id as u32;
    println!("{}", table);
    println!("{}", id);
    println!("{}", new_id);
}
