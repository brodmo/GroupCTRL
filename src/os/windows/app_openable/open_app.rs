use anyhow::bail;

use super::win32::focus;

pub fn open_app(exe_path: &str) -> anyhow::Result<()> {
    if !focus(exe_path)? {
        bail!("couldn't find window");
    };
    Ok(())
}
