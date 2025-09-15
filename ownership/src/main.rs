fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn main() {
    let mut dst = vec![
        String::from("cat"),
        String::from("elephant"),
        String::from("dog"),
    ];

    let src = vec![
        String::from("tiger"),
        String::from("hippopotamus"),
        String::from("ant"),
    ];

    println!("Before: {:?}", dst);
    add_big_strings(&mut dst, &src);
    println!("After:  {:?}", dst);
}