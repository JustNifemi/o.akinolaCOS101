fn main() {

    let v = vec![101, 250, 330, 400];
    //vector v owns the object in heap

    //only a single variable owns the heap memory at any given time

    let v2 = v;
    //here two variables owns heap value,
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memeory access, so it detectsa race condition
    //as two variables point to same heap

    println!("{:?}",v2);
}
