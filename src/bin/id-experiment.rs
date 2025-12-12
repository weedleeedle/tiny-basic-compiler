use std::sync::atomic::AtomicUsize;

struct IdGenerator
{
    id: usize,
    idx: usize,
}

impl IdGenerator
{
    pub fn new() -> Self
    {
        static GEN_ID: AtomicUsize = AtomicUsize::new(0);
        Self
        {
            id: GEN_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            idx: 0
        }
    }

    pub fn id(&mut self) -> Id
    {
        let old_id = self.idx;
        self.idx += 1;
        Id 
        {
            generator_id: self.id,
            id: old_id,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Id
{
    generator_id: usize,
    id: usize,
}

fn main()
{
    let mut id_generator = IdGenerator::new();
    let id_1 = id_generator.id();
    let id_2 = id_generator.id();
    let mut id_generator_2 = IdGenerator::new();
    let id_3 = id_generator_2.id();
    let id_4 = id_1;
    let id_5 = id_2;

    assert_ne!(id_1, id_2);
    assert_ne!(id_1, id_3);
    println!("{:?}", id_1);
    println!("{:?}", id_2);
    println!("{:?}", id_3);
    assert_eq!(id_1, id_4);
    assert_eq!(id_2, id_5);
}
