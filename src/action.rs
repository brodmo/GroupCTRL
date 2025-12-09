use crate::app::App;
use crate::open::Open;

pub trait Action: Send + Sync {
    fn execute(&self);
}

pub struct OpenApp(pub App);

impl Action for OpenApp {
    fn execute(&self) {
        self.0.open().unwrap();
    }
}
