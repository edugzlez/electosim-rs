use crate::tree::ErrorTree;

#[derive(Debug)]
pub enum SystemError {
    NotLeafRegion,
    RegionNotFound,
    CandidacyNotFound,
    NotParentable,
}

impl From<ErrorTree> for SystemError {
    fn from(error: ErrorTree) -> Self {
        match error {
            ErrorTree::RegionNotFound => SystemError::RegionNotFound,
            ErrorTree::CandidacyNotFound => SystemError::CandidacyNotFound,
            ErrorTree::NotParentable(_) => SystemError::NotParentable,
        }
    }
}
