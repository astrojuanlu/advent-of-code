use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Topo;

pub type Page = usize;
pub type Ruleset = DiGraphMap<Page, ()>;
pub type Update = Vec<Page>;

pub fn validate_update(update: &Update, ruleset: &Ruleset) -> bool {
    let mut topo = Topo::new(ruleset);
    let mut seq_iter = update.iter();

    while let Some(&sequence_node) = seq_iter.next() {
        loop {
            if let Some(visited_node) = topo.next(ruleset) {
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

    // Check if rules are a DAG
    toposort(&rule_dag, None).unwrap();

    return (rule_dag, updates);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_update_works() {
        let rules = Ruleset::from_edges(&[(47, 53), (75, 53), (47, 13), (75, 47), (75, 13)]);
        let valid_update = Update::from_iter([75, 47, 13]);
        let invalid_update = Update::from_iter([47, 13, 75]);

        assert!(validate_update(&valid_update, &rules));
        assert!(!validate_update(&invalid_update, &rules));
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
