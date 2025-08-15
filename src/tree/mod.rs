use std::collections::HashMap;

mod containers;
mod system;
mod tests;

use crate::tree::containers::single_result::SingleResult;
use containers::basic::{Candidacy, Region};
use containers::region_results::RegionResults;

#[derive(Debug, Clone)]
pub enum ErrorTree {
    RegionNotFound,
    CandidacyNotFound,
    NotParentable(String),
}

#[derive(Debug, Clone)]
struct RegionBox {
    pub region: Region,
    pub results: RegionResults,
    pub parent: Option<u32>,
    pub children: Vec<u32>,
}

impl RegionBox {
    fn new(name: &str) -> RegionBox {
        RegionBox {
            region: Region::new(name),
            results: RegionResults::new(),
            parent: None,
            children: Vec::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ElectionTree {
    root: RegionBox,
    regions: HashMap<u32, RegionBox>,
    candidacies: HashMap<u32, Candidacy>,

    _region_next_id: u32,
    _candidacy_next_id: u32,
}

impl Default for ElectionTree {
    fn default() -> ElectionTree {
        ElectionTree {
            root: RegionBox::new("root"),
            regions: HashMap::new(),
            candidacies: HashMap::new(),

            _region_next_id: 0,
            _candidacy_next_id: 0,
        }
    }
}

impl ElectionTree {
    pub fn add_region(&mut self, name: &str) -> u32 {
        self.regions
            .insert(self._region_next_id, RegionBox::new(name));
        self._region_next_id += 1;

        self._region_next_id - 1
    }

    pub fn add_candidacy(&mut self, name: &str) -> u32 {
        self.candidacies
            .insert(self._candidacy_next_id, Candidacy::new(name));
        self._candidacy_next_id += 1;

        self._candidacy_next_id - 1
    }

    pub fn get_region(&self, id: u32) -> Result<&Region, ErrorTree> {
        self.regions
            .get(&id)
            .and_then(|x| Some(&x.region))
            .ok_or(ErrorTree::RegionNotFound)
    }

    pub fn get_results(&self, region_id: u32) -> Result<&RegionResults, ErrorTree> {
        self.regions
            .get(&region_id)
            .and_then(|x| Some(&x.results))
            .ok_or(ErrorTree::RegionNotFound)
    }

    pub fn get_parent(&self, region_id: u32) -> Result<Option<u32>, ErrorTree> {
        self.regions
            .get(&region_id)
            .and_then(|x| Some(x.parent))
            .ok_or(ErrorTree::RegionNotFound)
    }

    pub fn get_candidacy(&self, id: u32) -> Result<&Candidacy, ErrorTree> {
        self.candidacies
            .get(&id)
            .ok_or(ErrorTree::CandidacyNotFound)
    }

    pub fn increase_votes(
        &mut self,
        region_id: u32,
        candidacy_id: u32,
        votes: i32,
    ) -> Result<(), ErrorTree> {
        if self.candidacies.get(&candidacy_id).is_none() {
            return Err(ErrorTree::CandidacyNotFound);
        }

        self.regions
            .get_mut(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .results
            .increase_votes(candidacy_id, votes);

        if let Some(parent_id) = self.get_parent(region_id)? {
            return self.increase_votes(parent_id, candidacy_id, votes);
        } else {
            self.global_increase_votes(candidacy_id, votes);
        }

        Ok(())
    }

    pub fn increase_seats(
        &mut self,
        region_id: u32,
        candidacy_id: u32,
        seats: i8,
    ) -> Result<(), ErrorTree> {
        if self.candidacies.get(&candidacy_id).is_none() {
            return Err(ErrorTree::CandidacyNotFound);
        }

        self.regions
            .get_mut(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .results
            .increase_seats(candidacy_id, seats);

        if let Some(parent_id) = self.get_parent(region_id)? {
            return self.increase_seats(parent_id, candidacy_id, seats);
        } else {
            self.global_increase_seats(candidacy_id, seats);
        }

        Ok(())
    }

    pub fn set_votes(
        &mut self,
        region_id: u32,
        candidacy_id: u32,
        votes: u32,
    ) -> Result<(), ErrorTree> {
        let current_votes = self
            .regions
            .get(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .results
            .get_votes(candidacy_id)
            .unwrap_or(0);

        let diff = votes as i32 - current_votes as i32;

        self.increase_votes(region_id, candidacy_id, diff)
    }

    pub fn set_seats(
        &mut self,
        region_id: u32,
        candidacy_id: u32,
        seats: u32,
    ) -> Result<(), ErrorTree> {
        let current_seats = self
            .regions
            .get(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .results
            .get_seats(candidacy_id)
            .unwrap_or(0);

        let diff = seats as i8 - current_seats as i8;

        self.increase_seats(region_id, candidacy_id, diff)
    }

    pub fn get_votes(&self, region_id: u32, candidacy_id: u32) -> Result<u32, ErrorTree> {
        Ok(self
            .regions
            .get(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .results
            .get_votes(candidacy_id)
            .ok_or(ErrorTree::CandidacyNotFound)?)
    }

    pub fn get_seats(&self, region_id: u32, candidacy_id: u32) -> Result<u32, ErrorTree> {
        Ok(self
            .regions
            .get(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .results
            .get_seats(candidacy_id)
            .ok_or(ErrorTree::CandidacyNotFound)?)
    }

    pub fn set_parent(&mut self, region_id: u32, parent_id: u32) -> Result<(), ErrorTree> {
        if !self.regions.contains_key(&parent_id) || !self.regions.contains_key(&region_id) {
            return Err(ErrorTree::RegionNotFound);
        }

        if !self.is_parentable(region_id, parent_id) {
            return Err(ErrorTree::NotParentable(
                "Region is already an ancestor".to_string(),
            ));
        }

        let current_parent = self.get_parent(region_id).unwrap_or(None);

        let region_results = self.iterate_results(region_id)?;

        if let Some(current_parent_id) = current_parent {
            for (candidacy_id, result) in &region_results {
                let votes = result.votes as i32;
                let seats = result.seats as i8;

                self.increase_votes(current_parent_id, *candidacy_id, -votes)?;
                self.increase_seats(current_parent_id, *candidacy_id, -seats)?;
            }

            self.regions
                .get_mut(&current_parent_id)
                .ok_or(ErrorTree::RegionNotFound)?
                .children
                .retain(|x| *x != region_id);
        } else {
            for (candidacy_id, result) in &region_results {
                let votes = result.votes as i32;
                let seats = result.seats as i8;

                self.global_increase_seats(*candidacy_id, -seats);
                self.global_increase_votes(*candidacy_id, -votes);
            }
        }

        self.regions
            .get_mut(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .parent = Some(parent_id);

        for (candidacy_id, result) in &region_results {
            let votes = result.votes as i32;
            let seats = result.seats as i8;

            self.increase_votes(parent_id, *candidacy_id, votes)?;
            self.increase_seats(parent_id, *candidacy_id, seats)?;
        }

        self.regions
            .get_mut(&parent_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .children
            .push(region_id);

        Ok(())
    }

    pub fn iterate_results(
        &mut self,
        region_id: u32,
    ) -> Result<Vec<(u32, SingleResult)>, ErrorTree> {
        let region_results: Vec<(u32, SingleResult)> = self
            .get_results(region_id)?
            .iter_results()
            .map(|(candidacy_id, result)| (*candidacy_id, result.clone()))
            .collect();
        Ok(region_results)
    }

    pub fn unparent(&mut self, region_id: u32) -> Result<(), ErrorTree> {
        let current_parent = self.get_parent(region_id).unwrap_or(None);

        let region_results: Vec<(u32, SingleResult)> = self.iterate_results(region_id)?;

        if let Some(parent_id) = current_parent {
            for (candidacy_id, result) in &region_results {
                let votes = result.votes as i32;
                let seats = result.seats as i8;

                self.increase_votes(parent_id, *candidacy_id, -votes)?;
                self.increase_seats(parent_id, *candidacy_id, -seats)?;
                self.global_increase_votes(*candidacy_id, votes);
                self.global_increase_seats(*candidacy_id, seats);
            }
        }

        self.regions
            .get_mut(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .parent = None;

        self.regions
            .get_mut(&current_parent.unwrap())
            .ok_or(ErrorTree::RegionNotFound)?
            .children
            .retain(|x| *x != region_id);

        Ok(())
    }

    fn is_parentable(&self, region_id: u32, parent_id: u32) -> bool {
        let ancestors = self.ancestors(parent_id).unwrap();

        !ancestors.contains(&region_id)
    }

    pub fn ancestors(&self, region_id: u32) -> Result<Vec<u32>, ErrorTree> {
        let mut antecessors = Vec::new();
        let mut current_id = region_id;

        if !self.regions.contains_key(&region_id) {
            return Err(ErrorTree::RegionNotFound);
        }

        while let Some(parent_id) = self.get_parent(current_id).unwrap_or(None) {
            antecessors.push(parent_id);
            current_id = parent_id;
        }

        Ok(antecessors)
    }

    pub fn is_leaf(&self, region_id: u32) -> Result<bool, ErrorTree> {
        Ok(self
            .regions
            .get(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .children
            .is_empty())
    }

    pub fn get_children(&self, region_id: u32) -> Result<Vec<u32>, ErrorTree> {
        Ok(self
            .regions
            .get(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?
            .children
            .clone())
    }

    pub fn global_increase_votes(&mut self, candidacy_id: u32, votes: i32) {
        self.root.results.increase_votes(candidacy_id, votes);
    }

    pub fn global_increase_seats(&mut self, candidacy_id: u32, seats: i8) {
        self.root.results.increase_seats(candidacy_id, seats);
    }

    pub fn global_get_votes(&self, candidacy_id: u32) -> u32 {
        self.root.results.get_votes(candidacy_id).unwrap_or(0)
    }

    pub fn global_get_seats(&self, candidacy_id: u32) -> u32 {
        self.root.results.get_seats(candidacy_id).unwrap_or(0)
    }

    pub fn get_global_results(&self) -> &RegionResults {
        &self.root.results
    }

    pub fn global_set_votes(&mut self, candidacy_id: u32, votes: u32) {
        self.root.results.set_votes(candidacy_id, votes);
    }

    pub fn global_set_seats(&mut self, candidacy_id: u32, seats: u32) {
        self.root.results.set_seats(candidacy_id, seats);
    }

    pub fn clear_region(&mut self, region_id: u32) -> Result<(), ErrorTree> {
        let region = self
            .regions
            .get(&region_id)
            .ok_or(ErrorTree::RegionNotFound)?;

        let candidacies: Vec<u32> = region.results.iter_results().map(|(id, _)| *id).collect();

        for candidacy_id in candidacies {
            self.set_seats(region_id, candidacy_id, 0)?;
        }

        Ok(())
    }

    pub fn region_ids(&self) -> Vec<u32> {
        self.regions.keys().cloned().collect()
    }

    pub fn leaf_region_ids(&self) -> Vec<u32> {
        self.regions
            .iter()
            .filter(|(_, region)| region.children.is_empty())
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn remove_region(
        &mut self,
        region_id: u32,
        remove_children: bool,
    ) -> Result<(), ErrorTree> {
        let parent_id = self.get_parent(region_id).unwrap_or(None);

        if let Some(_) = parent_id {
            self.unparent(region_id)?;
        }

        let children = self.get_children(region_id)?;

        for child_id in children {
            self.unparent(child_id)?;

            if remove_children {
                self.remove_region(child_id, true)?;
            }
        }

        self.regions.remove(&region_id);

        Ok(())
    }

    pub fn remove_region_candidacy(
        &mut self,
        region_id: u32,
        candidacy_id: u32,
    ) -> Result<(), ErrorTree> {
        self.set_votes(region_id, candidacy_id, 0)?;
        self.set_seats(region_id, candidacy_id, 0)?;

        self.regions
            .get_mut(&region_id)
            .unwrap()
            .results
            .remove_result(candidacy_id);

        Ok(())
    }

    pub fn remove_candidacy(&mut self, candidacy_id: u32) -> Result<(), ErrorTree> {
        for region_id in self.region_ids() {
            self.remove_region_candidacy(region_id, candidacy_id)?;
        }

        self.candidacies.remove(&candidacy_id);

        Ok(())
    }
}
