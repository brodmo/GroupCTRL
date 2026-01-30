use anyhow::{Context, anyhow, bail};
use block2::RcBlock;
use futures::StreamExt;
use log::info;
use objc2_app_kit::{NSWorkspace, NSWorkspaceOpenConfiguration};
use objc2_foundation::{NSError, NSString};

use super::app::App;
use crate::os::Openable;

impl Openable for App {
    async fn open(&self) -> anyhow::Result<()> {
        info!("opening app {self}");
        let workspace = NSWorkspace::sharedWorkspace();
        let bundle_id = NSString::from_str(&self.bundle_id);
        let Some(app_url) = workspace.URLForApplicationWithBundleIdentifier(&bundle_id) else {
            bail!("could not find app with bundle id '{}'", self.bundle_id);
        };
        let (tx, mut rx) = futures::channel::mpsc::unbounded();
        let app_path = app_url.path().context("app URL is missing path")?;
        let handler = RcBlock::new(move |_app, error: *mut NSError| {
            let _ = tx.unbounded_send(if error.is_null() {
                Ok(())
            } else {
                Err(anyhow!(
                    "could not open app at path '{}': {}",
                    app_path,
                    unsafe { &*error }
                ))
            });
        });
        workspace.openApplicationAtURL_configuration_completionHandler(
            &app_url,
            &NSWorkspaceOpenConfiguration::configuration(),
            Some(&handler),
        );
        rx.next()
            .await
            .context("openApplicationAtUrl completion handler was discarded")?
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;
    use crate::os::{AppQuery, System};

    #[test]
    fn open_finder() {
        let initial_app = System::current_app();
        let app = App {
            bundle_id: "com.apple.finder".to_string(),
        };
        assert!(block_on(app.open()).is_ok());
        if let Ok(Some(restore)) = initial_app {
            block_on(restore.open()).unwrap();
        }
    }

    #[test]
    fn open_fake_app() {
        let fake_app = App {
            bundle_id: "com.test.fake".to_string(),
        };
        let result = block_on(fake_app.open());
        assert_eq!(
            result.unwrap_err().to_string(),
            "could not find app with bundle id 'com.test.fake'"
        );
    }
}
