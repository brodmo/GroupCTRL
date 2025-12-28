use dioxus::prelude::*;

use crate::os::{App, AppDialog, AppSelection};

#[component]
pub(super) fn AppSelector() -> Element {
    let selection_sender = use_context::<UnboundedSender<App>>();
    let select_app = move |_| {
        let my_sender = selection_sender.clone();
        spawn(async move {
            if let Ok(Some(app)) = AppDialog::select_app().await {
                let _ = my_sender.unbounded_send(app);
            }
        });
    };

    rsx! {
        button {
            class: "btn btn-primary",
            onclick: select_app,
            "Pick App"
        }
    }
}
