use std::hash::Hash;

use dioxus::prelude::Element;

use crate::models::Identifiable;

pub trait ListCell<I: Clone + Eq + Hash>: Identifiable<I> {
    fn render(&self) -> Element;
}
