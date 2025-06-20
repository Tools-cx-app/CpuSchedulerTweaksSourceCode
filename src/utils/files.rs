use std::{
    fs::{Permissions, set_permissions, write},
    os::unix::fs::PermissionsExt,
    path::Path,
};

use anyhow::{Context, Result};

pub fn write_with_locked<T: AsRef<Path>>(path: T, content: &str) -> Result<()> {
    let path = path.as_ref();
    set_permissions(&path, Permissions::from_mode(0o644))
        .context(format!("无法设置644权限:{}", path.display()))?;
    write(&path, content).context(format!("无法写入{}:{}", content, path.display()))?;
    log::debug!("已写入{}到{}", content, path.display());
    set_permissions(&path, Permissions::from_mode(0o400))
        .context(format!("无法设置400权限:{}", path.display()))?;
    Ok(())
}
