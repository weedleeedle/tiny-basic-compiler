//! Just a little sanity check test application to make sure I understand how references work.

fn main()
{
    // 1 is stored on the stack as A.
    let a = 1;
    // B is stored on the stack with A's address. B transparently derefs to 1.
    let b = &a;
    // B = 1.
    println!("B = {}", b);
    // B = 1.
    println!("B = {:?}", b);
    // This allows us to get the actual address.
    println!("*const B = {:?}", b as *const i32);
    // So this is a reference to B.
    let c = &b;
    // This also derefs to 1.
    println!("C = {}", c);
    println!("C = {:?}", c);

    println!("*const C = {:?}", c as *const &i32);
    println!("ptr::addr_of(a) = {:?}", std::ptr::addr_of!(a));
}

