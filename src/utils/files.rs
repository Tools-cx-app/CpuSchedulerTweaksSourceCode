use std::{
    fs::{Permissions, set_permissions, write},
    os::unix::fs::PermissionsExt,
    path::Path,
};

use anyhow::{Context, Result};

pub fn write_with_locked<T: AsRef<Path>>(path: T, content: &str) -> Result<()> {
    let path = path.as_ref();
    set_permissions(&path, Permissions::from_mode(0o644)).context("无法设置最大频率权限")?;
    write(&path, content).context("无法写入最大频率")?;
    set_permissions(&path, Permissions::from_mode(0o400)).context("无法恢复最大频率权限")?;
    Ok(())
}
