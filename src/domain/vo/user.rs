use crate::domain::core::SysUser;
use crate::domain::vo::SysRoleVO;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUserVO {
    #[serde(flatten)]
    pub inner: SysUser,
    pub role: Option<SysRoleVO>,
}

impl From<SysUser> for SysUserVO {
    fn from(arg: SysUser) -> Self {
        Self {
            inner: arg,
            role: None,
        }
    }
}
