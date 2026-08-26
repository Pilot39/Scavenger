use soroban_sdk::{contracttype, Env, String};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ApiVersion {
    V1 = 1,
    V2 = 2,
}

impl ApiVersion {
    pub fn current() -> Self {
        ApiVersion::V2
    }

    pub fn is_deprecated(&self) -> bool {
        matches!(self, ApiVersion::V1)
    }

    pub fn deprecation_message(&self) -> &'static str {
        match self {
            ApiVersion::V1 => "API v1 is deprecated. Please migrate to v2.",
            ApiVersion::V2 => "",
        }
    }

    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            1 => Some(ApiVersion::V1),
            2 => Some(ApiVersion::V2),
            _ => None,
        }
    }
}

#[contracttype]
pub struct VersionInfo {
    pub current: u32,
    pub supported: soroban_sdk::Vec<u32>,
    pub deprecated: soroban_sdk::Vec<u32>,
}

pub fn get_version_info(env: &Env) -> VersionInfo {
    let mut supported = soroban_sdk::Vec::new(env);
    supported.push_back(1);
    supported.push_back(2);

    let mut deprecated = soroban_sdk::Vec::new(env);
    deprecated.push_back(1);

    VersionInfo {
        current: 2,
        supported,
        deprecated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_version() {
        assert_eq!(ApiVersion::current(), ApiVersion::V2);
    }

    #[test]
    fn test_v1_deprecated() {
        assert!(ApiVersion::V1.is_deprecated());
    }

    #[test]
    fn test_v2_not_deprecated() {
        assert!(!ApiVersion::V2.is_deprecated());
    }

    #[test]
    fn test_deprecation_message() {
        assert!(!ApiVersion::V1.deprecation_message().is_empty());
        assert!(ApiVersion::V2.deprecation_message().is_empty());
    }

    #[test]
    fn test_from_u32() {
        assert_eq!(ApiVersion::from_u32(1), Some(ApiVersion::V1));
        assert_eq!(ApiVersion::from_u32(2), Some(ApiVersion::V2));
        assert_eq!(ApiVersion::from_u32(3), None);
    }
}
