//! [Id]s are used as unique identifiers for symbols in rules and stuff.
use std::{fmt::Debug, hash::Hash};
use std::sync::atomic::AtomicUsize;

/// A generator for creating [Id]s. Only [Id]s from the same [IdGenerator] can be compared to each
/// other. 
///
/// To create an [IdGenerator], use the [id_generator!] macro.
///

pub struct IdGenerator
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
pub struct Id
{
    generator_id: usize,
    id: usize,
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn test_ids_equal()
    {
        let mut id_generator = IdGenerator::new();
        let id_1 = id_generator.id();
        let id_2 = id_1;
        assert_eq!(id_1, id_2);
    }

    #[test]
    pub fn test_ids_not_equal()
    {
        let mut id_generator = IdGenerator::new();
        let id_1 = id_generator.id();
        let id_2 = id_generator.id();
        assert_ne!(id_1, id_2);
    }

    #[test]
    pub fn test_cant_compare_different_symbols()
    {
        let mut gen_1 = IdGenerator::new();
        let mut gen_2 = IdGenerator::new();
        let id_1 = gen_1.id();
        let id_2 = gen_2.id();
        assert_ne!(id_1, id_2);
    }
}
