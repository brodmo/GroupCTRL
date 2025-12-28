use std::hash::Hash;

pub trait Identifiable<I: Clone + Eq + Hash> {
    fn id(&self) -> I;
}
