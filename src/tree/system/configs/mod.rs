mod auxiliar;
pub mod multi_district;
pub mod unique_district;

use crate::tree::system::events::ElectionEvent;
use crate::tree::ElectionTree;

pub trait ElectionConfig {
    #[allow(dead_code)]
    fn on_event(&self, tree: &mut ElectionTree, event: ElectionEvent);
}
