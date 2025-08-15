#[cfg(test)]
mod tests {
    use crate::tree::ElectionTree;

    #[test]
    fn create_tree() {
        let _ = ElectionTree::default();
    }

    #[test]
    fn add_region() {
        let mut tree = ElectionTree::default();

        let coruscant = tree.add_region("Coruscant");
        let trantor = tree.add_region("Trántor");

        assert_eq!(coruscant, 0);
        assert_eq!(trantor, 1);

        let reg = tree.get_region(coruscant).unwrap();
        assert_eq!(reg.name(), "Coruscant");

        let result = tree.get_region(42);
        assert!(result.is_err());
    }

    #[test]
    fn add_candidacy() {
        let mut tree = ElectionTree::default();

        let pt = tree.add_candidacy("Partido Tecnócrata");
        let psd = tree.add_candidacy("Pacto Social Demócrata");

        assert_eq!(pt, 0);
        assert_eq!(psd, 1);

        let cand = tree.get_candidacy(pt).unwrap();
        assert_eq!(cand.name(), "Partido Tecnócrata");

        let result = tree.get_candidacy(72);
        assert!(result.is_err());
    }

    #[test]
    fn increase_seats() {
        let mut tree = ElectionTree::default();

        let coruscant = tree.add_region("Coruscant");
        let pt = tree.add_candidacy("Partido Tecnócrata");

        tree.increase_seats(coruscant, pt, 50).unwrap();

        let results = tree.get_results(coruscant).unwrap();

        assert_eq!(tree.get_seats(coruscant, pt).unwrap(), 50);
        assert_eq!(results.get_seats(pt).unwrap(), 50);

        assert!(tree.increase_seats(42, pt, 1).is_err());
        assert!(tree.increase_seats(coruscant, 24, 1).is_err());
    }

    #[test]
    fn increase_votes() {
        let mut tree = ElectionTree::default();

        let coruscant = tree.add_region("Coruscant");
        let pt = tree.add_candidacy("Partido Tecnócrata");

        tree.increase_votes(coruscant, pt, 2350).unwrap();

        let results = tree.get_results(coruscant).unwrap();

        assert_eq!(tree.get_votes(coruscant, pt).unwrap(), 2350);
        assert_eq!(results.get_votes(pt).unwrap(), 2350);

        assert!(tree.increase_votes(42, pt, 50).is_err());
        assert!(tree.increase_votes(coruscant, 24, 50).is_err());
    }

    #[test]
    fn set_votes_and_seats() {
        let mut tree = ElectionTree::default();

        let coruscant = tree.add_region("Coruscant");
        let pt = tree.add_candidacy("Partido Tecnócrata");

        tree.set_votes(coruscant, pt, 2350).unwrap();
        tree.set_seats(coruscant, pt, 50).unwrap();

        let results = tree.get_results(coruscant).unwrap();

        assert_eq!(results.get_votes(pt).unwrap(), 2350);
        assert_eq!(results.get_seats(pt).unwrap(), 50);

        tree.set_votes(coruscant, pt, 150).unwrap();
        tree.set_seats(coruscant, pt, 10).unwrap();

        let results = tree.get_results(coruscant).unwrap();

        assert_eq!(results.get_votes(pt).unwrap(), 150);
        assert_eq!(results.get_seats(pt).unwrap(), 10);
    }

    #[test]
    fn set_parent() {
        let mut tree = ElectionTree::default();

        let coruscant = tree.add_region("Coruscant");
        let trantor = tree.add_region("Trántor");
        let tatooine = tree.add_region("Tatooine");

        tree.set_parent(coruscant, tatooine).unwrap();
        tree.set_parent(trantor, tatooine).unwrap();

        let children = tree.get_children(tatooine).unwrap();

        assert_eq!(tree.get_parent(coruscant).unwrap(), Some(tatooine));
        assert!(tree.get_parent(tatooine).unwrap().is_none());
        assert!(tree.is_leaf(coruscant).unwrap());
        assert!(!tree.is_leaf(tatooine).unwrap());
        assert!(tree.is_leaf(trantor).unwrap());
        assert!(children.contains(&coruscant));
        assert!(children.contains(&trantor));
    }

    #[test]
    fn set_parent_and_increase_votes_and_seats() {
        let mut tree = ElectionTree::default();

        let coruscant = tree.add_region("Coruscant");
        let trantor = tree.add_region("Trántor");

        tree.set_parent(coruscant, trantor).unwrap();

        let pt = tree.add_candidacy("Partido Tecnócrata");
        let psd = tree.add_candidacy("Pacto Social Demócrata");

        tree.increase_votes(coruscant, pt, 100).unwrap();
        tree.increase_votes(trantor, psd, 90).unwrap();

        let current_votes_pt = tree.get_votes(trantor, pt).unwrap();
        let current_votes_psd = tree.get_votes(trantor, psd).unwrap();

        assert_eq!(current_votes_pt, 100);
        assert_eq!(current_votes_psd, 90);
    }

    #[test]
    fn force_not_parentable_error() {
        let mut tree = ElectionTree::default();

        let coruscant = tree.add_region("Coruscant");
        let trantor = tree.add_region("Trántor");
        let corellia = tree.add_region("Corellia");
        let tatooine = tree.add_region("Tatooine");

        tree.set_parent(coruscant, trantor).unwrap();
        tree.set_parent(corellia, trantor).unwrap();
        tree.set_parent(tatooine, corellia).unwrap();

        let result = tree.set_parent(trantor, tatooine);

        assert!(result.is_err());
    }

    #[test]
    fn change_parent_with_votes() {
        let mut tree = ElectionTree::default();

        let coruscant = tree.add_region("Coruscant");
        let trantor = tree.add_region("Trántor");
        let corellia = tree.add_region("Corellia");

        let pt = tree.add_candidacy("Partido Tecnócrata");

        tree.set_parent(coruscant, trantor).unwrap();

        tree.increase_votes(coruscant, pt, 103).unwrap();

        tree.set_parent(coruscant, corellia).unwrap();

        let current_votes = tree.get_votes(corellia, pt).unwrap();

        assert_eq!(current_votes, 103);
    }

    #[test]
    fn get_global_results() {
        let mut tree = ElectionTree::default();
        let coruscant = tree.add_region("Coruscant");
        let trantor = tree.add_region("Trántor");
        let corellia = tree.add_region("Corellia");

        let pt = tree.add_candidacy("Partido Tecnócrata");
        let psd = tree.add_candidacy("Pacto Social Demócrata");

        tree.set_parent(coruscant, trantor).unwrap();
        tree.set_parent(corellia, trantor).unwrap();

        tree.increase_votes(coruscant, pt, 100).unwrap();
        tree.increase_votes(coruscant, psd, 90).unwrap();

        tree.increase_votes(corellia, pt, 50).unwrap();
        tree.increase_votes(corellia, psd, 60).unwrap();

        let results = tree.get_global_results();

        assert_eq!(results.get_votes(pt).unwrap(), 150);
        assert_eq!(results.get_votes(psd).unwrap(), 150);
    }
}
