use crate::os::{App, AppQuery, System};

impl AppQuery for System {
    fn current_app() -> anyhow::Result<Option<App>> {
        Ok(None)
    }
}
