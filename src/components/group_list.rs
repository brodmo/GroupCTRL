use std::collections::HashSet;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::list::List;
use crate::components::list_cell::ListCell;
use crate::models::Group;

#[component]
pub fn GroupList(groups: Vec<Group>, selected: Signal<HashSet<Uuid>>) -> Element {
    rsx! {
        List {
            elements: groups,
            selected,
        }
    }
}

impl ListCell<Uuid> for Group {
    fn render(&self) -> Element {
        rsx! {
            span { "{self.name}" }
        }
    }
}
