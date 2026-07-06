use std::collections::BTreeSet;

pub const PERMISSION_HOST_LOG: &str = "host.log";
pub const PERMISSION_CONFIG_READ: &str = "config.read";
pub const PERMISSION_DM_ACCESS: &str = "dm.access";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptPermissions {
    allow_all: bool,
    permissions: BTreeSet<String>,
}

impl ScriptPermissions {
    pub fn allow_all() -> Self {
        Self {
            allow_all: true,
            permissions: BTreeSet::new(),
        }
    }

    pub fn from_list(permissions: impl IntoIterator<Item = String>) -> Self {
        Self {
            allow_all: false,
            permissions: permissions.into_iter().collect(),
        }
    }

    pub fn allows(&self, permission: &str) -> bool {
        self.allow_all || self.permissions.contains(permission)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listed_permissions_only_allow_requested_items() {
        let permissions = ScriptPermissions::from_list(["host.log".to_string()]);

        assert!(permissions.allows(PERMISSION_HOST_LOG));
        assert!(!permissions.allows(PERMISSION_CONFIG_READ));
    }
}
