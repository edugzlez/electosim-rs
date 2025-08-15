use crate::tree::system::configs::auxiliar::Candidacy;
use crate::tree::system::configs::ElectionConfig;
use crate::tree::system::events::ElectionEvent;
use crate::tree::ElectionTree;
use crate::{election, Method};

pub struct UniqueDistrict {
    method: Method,
    seats: u32,
    cutoff: f32,
}

impl UniqueDistrict {
    #[allow(dead_code)]
    pub fn new(method: Method, seats: u32, cutoff: f32) -> Self {
        UniqueDistrict {
            method,
            seats,
            cutoff,
        }
    }

    fn compute(&self, tree: &mut ElectionTree) {
        let results: Vec<_> = tree
            .get_global_results()
            .iter_results()
            .map(|(id, result)| Candidacy {
                id: *id,
                votes: result.votes,
                seats: result.seats as u16,
            })
            .collect();

        let mut ele = election!(results, self.seats as u16, self.method, self.cutoff);

        ele.compute().unwrap();

        for res in ele.results {
            tree.global_set_seats(res.id, res.seats as u32)
        }
    }
}
impl ElectionConfig for UniqueDistrict {
    fn on_event(&self, tree: &mut ElectionTree, event: ElectionEvent) {
        match event {
            ElectionEvent::IncreaseVotes { .. } | ElectionEvent::Create => {
                self.compute(tree);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::system::ElectionSystem;

    #[test]
    fn test_election_system() {
        let tree = ElectionTree::default();

        let config = UniqueDistrict::new(Method::DHONDT, 13, 0.03);

        let mut system = ElectionSystem::create(tree, config);

        let candeleda = system.create_region("Candeleda");
        let pp = system.create_candidacy("Partido Popular");
        let psoe = system.create_candidacy("Partido Socialista Obrero Español");
        let pcal = system.create_candidacy("Partido de Castilla y León");

        system.increase_votes(candeleda, pp, 1846).unwrap();
        system.increase_votes(candeleda, psoe, 1114).unwrap();
        system.increase_votes(candeleda, pcal, 466).unwrap();

        assert_eq!(system.global_get_seats(pp), 7);
        assert_eq!(system.global_get_seats(psoe), 4);
        assert_eq!(system.global_get_seats(pcal), 2);
    }
}
