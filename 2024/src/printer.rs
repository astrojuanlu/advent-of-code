use core::fmt::Debug;

use petgraph::algo::{toposort, Cycle};
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::{
    Data, DfsPostOrder, GraphBase, GraphProp, IntoNeighbors, IntoNeighborsDirected,
    IntoNodeIdentifiers, Visitable,
};
use petgraph::Directed;

pub type Page = usize;
pub type Ruleset = DiGraphMap<Page, ()>;
pub type Update = Vec<Page>;

pub fn validate_update<G>(update: &Update, ruleset: G) -> bool
where
    G: GraphBase<NodeId = Page> + Data + Visitable + IntoNeighbors + GraphProp<EdgeType = Directed>,
    <G as Visitable>::Map: Debug,
{
    // Graph might contain cycles, so we use post-order (backwards) depth-first search
    let mut visitor = DfsPostOrder::new(ruleset, update[0]);
    let mut seq_iter = update.iter().rev();

    while let Some(&sequence_node) = seq_iter.next() {
        loop {
            if let Some(visited_node) = visitor.next(ruleset) {
                if sequence_node == visited_node {
                    break;
                } else {
                    continue;
                }
            } else {
                return false;
            }
        }
    }
    return true;
}

pub fn add_middle_pages(updates: &Vec<Update>) -> Page {
    let mut middle_pages: Vec<Page> = Vec::new();
    for update in updates {
        middle_pages.push(update[update.len() / 2]);
    }
    return middle_pages.iter().fold(0, |acc, p| acc + p);
}

pub fn generate_valid_update<G>(ruleset: G) -> Result<Vec<G::NodeId>, Cycle<G::NodeId>>
where
    G: GraphBase<NodeId = Page>
        + Data
        + Visitable
        + IntoNeighborsDirected
        + IntoNodeIdentifiers
        + GraphProp<EdgeType = Directed>,
    <G as Visitable>::Map: Debug,
{
    return toposort(ruleset, None);
}

pub fn parse_input_05(contents: String) -> (DiGraphMap<Page, ()>, Vec<Update>) {
    let mut lines = contents.lines();
    let mut rules: Vec<(Page, Page)> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let rule_strs = line.split_once("|").unwrap();
        rules.push((
            rule_strs.0.parse::<Page>().unwrap(),
            rule_strs.1.parse::<Page>().unwrap(),
        ));
    }
    loop {
        if let Some(line) = lines.next() {
            let update: Update = line
                .split(",")
                .map(|s| s.parse::<Page>().unwrap())
                .collect();
            updates.push(update);
        } else {
            break;
        }
    }
    let rule_dag = DiGraphMap::from_edges(rules);
    return (rule_dag, updates);
}

#[cfg(test)]
mod test {
    use super::*;

    use petgraph::visit::{IntoEdgeReferences, NodeFiltered};

    #[test]
    fn validate_update_works() {
        let rules = Ruleset::from_edges(&[(47, 53), (75, 53), (47, 13), (75, 47), (75, 13)]);
        let valid_update = Update::from_iter([75, 47, 13]);
        let invalid_update = Update::from_iter([47, 13, 75]);

        assert!(validate_update(&valid_update, &rules));
        assert!(!validate_update(&invalid_update, &rules));
    }

    #[test]
    fn validate_update_with_cyclic_rules_works() {
        let rules = Ruleset::from_edges(&[
            (47, 53),
            (75, 53),
            (47, 13),
            (75, 47),
            (13, 75), // Cycle
        ]);
        let valid_update = Update::from_iter([75, 47, 53]);
        let invalid_update = Update::from_iter([75, 53, 47]);
        let valid_update_traversing_cycle = Update::from_iter([47, 13, 75, 53]);

        assert!(validate_update(&valid_update, &rules));
        assert!(validate_update(&valid_update_traversing_cycle, &rules));
        assert!(!validate_update(&invalid_update, &rules));
    }

    #[test]
    fn validate_update_with_circular_rules_works() {
        let rules = Ruleset::from_edges(&[(47, 75), (75, 13), (13, 53), (53, 47)]);
        let valid_update1 = Update::from_iter([53, 47, 75]);
        let valid_update2 = Update::from_iter([75, 13, 53]);
        let invalid_update = Update::from_iter([53, 75, 47]);

        assert!(validate_update(&valid_update1, &rules));
        assert!(validate_update(&valid_update2, &rules));
        assert!(!validate_update(&invalid_update, &rules));
    }

    #[test]
    fn validate_update_with_tournament_graph_rules_works() {
        let rules =
            Ruleset::from_edges(&[(47, 75), (75, 13), (13, 53), (53, 47), (47, 13), (75, 53)]);
        let valid_update1 = Update::from_iter([47, 75, 13]);
        let valid_update2 = Update::from_iter([13, 53]);
        let invalid_update = Update::from_iter([53, 75, 47]);

        assert!(validate_update(&valid_update1, &rules));
        assert!(validate_update(&valid_update2, &rules));
        assert!(!validate_update(&invalid_update, &rules));
    }

    #[test]
    fn validate_update_with_wicked_cycle_fails() {
        let rules = Ruleset::from_edges([
            (29, 16), // Cycle
            (52, 16), // Rule of interest
            (16, 47), // Rule of interest
            (47, 29), // Cycle
            (52, 47), // Rule of interest
        ]);

        let valid_update = Update::from_iter([52, 16, 47]);

        // This fails! Hence the need to ignore certain rules
        assert!(!validate_update(&valid_update, &rules));

        let filtered_rules = NodeFiltered::from_fn(&rules, |node| valid_update.contains(&node));
        let filtered_edges = filtered_rules
            .edge_references()
            .map(|e| (e.0, e.1))
            .collect::<Vec<(Page, Page)>>();

        // In the end the rules of interest are fewer
        assert_eq!(filtered_edges, [(52, 16), (16, 47), (52, 47)]);
        // With the filtered rules, the update is properly considered valid
        assert!(validate_update(&valid_update, &filtered_rules));
    }

    #[test]
    fn add_middle_pages_works() {
        let updates: Vec<Update> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ];
        let result = add_middle_pages(&updates);
        assert_eq!(result, 143);
    }
}
