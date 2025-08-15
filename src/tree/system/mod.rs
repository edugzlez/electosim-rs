mod configs;
mod errors;
mod events;

use crate::tree::containers::region_results::RegionResults;
use crate::tree::system::configs::ElectionConfig;
use crate::tree::system::errors::SystemError;
use crate::tree::system::events::ElectionEvent;
use crate::tree::ElectionTree;

pub struct ElectionSystem<T: ElectionConfig> {
    tree: ElectionTree,
    config: T,
}

#[allow(dead_code)]
impl<T> ElectionSystem<T>
where
    T: ElectionConfig,
{
    pub fn create(tree: ElectionTree, config: T) -> ElectionSystem<T> {
        let mut system = ElectionSystem { tree, config };
        system.clear();
        system
    }

    pub fn increase_votes(
        &mut self,
        region: u32,
        candidacy: u32,
        votes: i32,
    ) -> Result<(), SystemError> {
        if !self.tree.is_leaf(region)? {
            return Err(SystemError::NotLeafRegion);
        }

        self.tree.increase_votes(region, candidacy, votes)?;
        self.config.on_event(
            &mut self.tree,
            ElectionEvent::IncreaseVotes {
                region,
                candidacy,
                votes,
            },
        );

        Ok(())
    }

    pub fn set_votes(
        &mut self,
        region: u32,
        candidacy: u32,
        votes: u32,
    ) -> Result<(), SystemError> {
        if !self.tree.is_leaf(region)? {
            return Err(SystemError::NotLeafRegion);
        }

        let current_votes = self.tree.get_votes(region, candidacy)?;
        let offset = votes as i32 - current_votes as i32;

        self.increase_votes(region, candidacy, offset)
    }

    pub fn set_parent(&mut self, region: u32, parent: u32) -> Result<(), SystemError> {
        let current_parent = self.tree.get_parent(region)?;

        self.config.on_event(
            &mut self.tree,
            ElectionEvent::BeforeSetParent { region, parent },
        );

        self.tree.set_parent(region, parent)?;

        self.config.on_event(
            &mut self.tree,
            ElectionEvent::AfterSetParent {
                region,
                parent,
                last_parent: current_parent,
            },
        );

        Ok(())
    }

    fn unset_parent(&mut self, region: u32) -> Result<(), SystemError> {
        self.config
            .on_event(&mut self.tree, ElectionEvent::BeforeRemoveParent(region));
        self.tree.unparent(region)?;
        self.config
            .on_event(&mut self.tree, ElectionEvent::AfterRemoveParent(region));

        Ok(())
    }

    pub fn get_results(&self, region: u32) -> Result<&RegionResults, SystemError> {
        let results = self.tree.get_results(region)?;

        Ok(results)
    }

    pub fn get_seats(&self, region: u32, candidacy: u32) -> Result<u32, SystemError> {
        let seats = self.tree.get_seats(region, candidacy)?;

        Ok(seats)
    }

    pub fn global_results(&self) -> &RegionResults {
        self.tree.get_global_results()
    }

    pub fn global_get_votes(&self, candidacy: u32) -> u32 {
        self.tree.global_get_votes(candidacy)
    }

    pub fn global_get_seats(&self, candidacy: u32) -> u32 {
        self.tree.global_get_seats(candidacy)
    }

    pub fn create_region(&mut self, name: &str) -> u32 {
        self.tree.add_region(name)
    }

    pub fn remove_region(
        &mut self,
        region_id: u32,
        remove_children: bool,
    ) -> Result<(), SystemError> {
        self.tree.remove_region(region_id, remove_children)?;

        self.config.on_event(
            &mut self.tree,
            ElectionEvent::RegionRemoved {
                region: region_id,
                remove_children,
            },
        );

        Ok(())
    }

    pub fn create_candidacy(&mut self, name: &str) -> u32 {
        self.tree.add_candidacy(name)
    }

    pub fn clear(&mut self) {
        self.tree.leaf_region_ids().iter().for_each(|&region_id| {
            self.tree.clear_region(region_id).unwrap();
        });
    }
}

impl<T> Drop for ElectionSystem<T>
where
    T: ElectionConfig,
{
    fn drop(&mut self) {
        self.clear();
    }
}
