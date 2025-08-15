use crate::tree::system::configs::auxiliar::Candidacy;
use crate::tree::system::configs::ElectionConfig;
use crate::tree::system::events::ElectionEvent;
use crate::tree::system::ElectionSystem;
use crate::tree::ElectionTree;
use crate::{election, Method};
use std::collections::HashMap;

pub struct DistrictConfig {
    method: Method,
    seats: u32,
    cutoff: f32,
}

impl DistrictConfig {
    #[allow(dead_code)]
    pub fn new(method: Method, seats: u32, cutoff: f32) -> Self {
        DistrictConfig {
            method,
            seats,
            cutoff,
        }
    }
}

pub struct MultiDistrictConfig {
    pub configs: HashMap<u32, DistrictConfig>,
}

impl MultiDistrictConfig {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MultiDistrictConfig {
            configs: HashMap::new(),
        }
    }
}

impl MultiDistrictConfig {
    fn compute_all(&self, tree: &mut ElectionTree) {
        for region_id in self.configs.keys() {
            self.compute(tree, *region_id);
        }
    }
    fn compute(&self, tree: &mut ElectionTree, region_id: u32) {
        let config = self.configs.get(&region_id).unwrap();
        let results: Vec<_> = tree
            .get_results(region_id)
            .unwrap()
            .iter_results()
            .map(|(id, result)| Candidacy {
                id: *id,
                votes: result.votes,
                seats: result.seats as u16,
            })
            .collect();

        let mut ele = election!(results, config.seats as u16, config.method, config.cutoff);

        ele.compute().unwrap();

        for res in ele.results {
            tree.set_seats(region_id, res.id, res.seats as u32).unwrap();
        }
    }
}

impl ElectionConfig for MultiDistrictConfig {
    fn on_event(&self, tree: &mut ElectionTree, event: ElectionEvent) {
        match event {
            ElectionEvent::Create => {
                self.compute_all(tree);
            }
            ElectionEvent::RegionModified(region_id) => {
                if self.configs.contains_key(&region_id) {
                    self.compute(tree, region_id);
                } else {
                    tree.clear_region(region_id).unwrap();
                }
            }
            ElectionEvent::IncreaseVotes {
                region: region_id, ..
            } => {
                if self.configs.contains_key(&region_id) {
                    self.compute(tree, region_id);
                }
            }
            _ => {}
        }
    }
}

#[allow(dead_code)]
impl ElectionSystem<MultiDistrictConfig> {
    pub fn set_config(&mut self, region_id: u32, config: DistrictConfig) {
        self.config.configs.insert(region_id, config);
        self.config
            .on_event(&mut self.tree, ElectionEvent::RegionModified(region_id));
    }

    pub fn remove_config(&mut self, region_id: u32) {
        self.config.configs.remove(&region_id);
        self.config
            .on_event(&mut self.tree, ElectionEvent::RegionModified(region_id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Method;

    struct Example {
        pub system: ElectionSystem<MultiDistrictConfig>,
        pub avila: u32,
        pub cyl: u32,
        pub segovia: u32,
        pub madrid: u32,
        pub pp: u32,
        pub psoe: u32,
        pub vox: u32,
        pub sumar: u32,
    }

    fn base() -> Example {
        let tree = ElectionTree::default();
        let config = MultiDistrictConfig::new();
        let mut system = ElectionSystem::create(tree, config);

        let avila = system.create_region("Ávila");
        let cyl: u32 = system.create_region("Castilla y León");
        let segovia = system.create_region("Segovia");
        let madrid = system.create_region("Madrid");

        system.set_parent(avila, cyl).unwrap();
        system.set_parent(segovia, cyl).unwrap();

        let pp = system.create_candidacy("Partido Popular");
        let psoe = system.create_candidacy("Partido Socialista Obrero Español");
        let vox = system.create_candidacy("Vox");
        let sumar = system.create_candidacy("Sumar");

        system.increase_votes(avila, pp, 42_369).unwrap();
        system.increase_votes(avila, psoe, 26_828).unwrap();
        system.increase_votes(avila, vox, 15_068).unwrap();
        system.increase_votes(avila, sumar, 5_027).unwrap();

        system.increase_votes(segovia, pp, 39_894).unwrap();
        system.increase_votes(segovia, psoe, 27_113).unwrap();
        system.increase_votes(segovia, vox, 12_510).unwrap();
        system.increase_votes(segovia, sumar, 7_137).unwrap();

        system.increase_votes(madrid, pp, 1_463_183).unwrap();
        system.increase_votes(madrid, psoe, 1_004_599).unwrap();
        system.increase_votes(madrid, sumar, 557_780).unwrap();
        system.increase_votes(madrid, vox, 506_164).unwrap();

        Example {
            system,
            avila,
            cyl,
            segovia,
            madrid,
            pp,
            psoe,
            vox,
            sumar,
        }
    }

    #[test]
    fn test_multi_district() {
        let mut example = base();

        example
            .system
            .set_config(example.avila, DistrictConfig::new(Method::DHONDT, 3, 0.03));
        example.system.set_config(
            example.segovia,
            DistrictConfig::new(Method::DHONDT, 3, 0.03),
        );
        example.system.set_config(
            example.madrid,
            DistrictConfig::new(Method::DHONDT, 37, 0.03),
        );

        assert_eq!(example.system.global_get_seats(example.pp), 20);
        assert_eq!(example.system.global_get_seats(example.psoe), 12);
        assert_eq!(example.system.global_get_seats(example.vox), 5);
        assert_eq!(example.system.global_get_seats(example.sumar), 6);

        for party in [example.pp, example.psoe, example.vox, example.sumar] {
            assert_eq!(
                example.system.get_seats(example.cyl, party).unwrap(),
                example.system.get_seats(example.avila, party).unwrap()
                    + example.system.get_seats(example.segovia, party).unwrap()
            )
        }
    }

    #[test]
    fn test_change_method() {
        let mut example = base();

        example
            .system
            .set_config(example.avila, DistrictConfig::new(Method::DHONDT, 3, 0.03));

        // Change method for Madrid
        example.system.set_config(
            example.avila,
            DistrictConfig::new(Method::WINNERTAKESALL, 5, 0.03),
        );

        for (party, seats) in [
            (example.pp, 5),
            (example.psoe, 0),
            (example.vox, 0),
            (example.sumar, 0),
        ] {
            assert_eq!(
                example.system.get_seats(example.avila, party).unwrap(),
                seats
            );
        }
    }

    #[test]
    fn test_change_votes() {
        let mut example = base();

        example
            .system
            .set_config(example.avila, DistrictConfig::new(Method::DHONDT, 3, 0.03));

        example
            .system
            .set_votes(example.avila, example.pp, 26_828)
            .unwrap();

        example
            .system
            .set_votes(example.avila, example.psoe, 42_369)
            .unwrap();

        for (party, seats) in [(example.psoe, 2), (example.pp, 1)] {
            assert_eq!(
                example.system.get_seats(example.avila, party).unwrap(),
                seats
            );
        }
    }
}
