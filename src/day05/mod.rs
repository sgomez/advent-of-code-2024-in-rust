use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

pub(crate) fn run() {
    let filename = "data/day05/input.txt";
    let (part1, part2) = parse_file(filename);

    let rules = PageOrderingRules::from_lines(part1);
    let result_part_1: u32 = part2
        .iter()
        .filter_map(|item| rules.check_pages_to_update(item))
        .sum();
    println!("Result of Day 05, Part 1 is {}", result_part_1);

    let result_part_2: u32 = part2
        .iter()
        .filter(|pages| rules.check_pages_not_ordered(pages))
        .filter_map(|item| {
            let ordered = rules.order_pages(item);
            rules.check_pages_to_update(&ordered)
        })
        .sum();
    println!("Result of Day 05, Part 2 is {}", result_part_2);
}

fn parse_file(filename: &str) -> (Vec<String>, Vec<Vec<u32>>) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut sections = contents.split("\n\n");

    let part1: Vec<String> = sections
        .next()
        .expect("Missing first part")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let part2: Vec<Vec<u32>> = sections
        .next()
        .expect("Missing last part")
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<Vec<u32>>() // Recolecta en un Vec<u32>
        })
        .collect(); // Recolecta todas las líneas en un Vec<Vec<u32>>

    (part1, part2)
}

#[derive(PartialEq, Debug)]
pub struct PageOrderingRules {
    rules: HashMap<u32, Vec<u32>>,
}

impl PageOrderingRules {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    fn from_lines(lines: Vec<String>) -> Self {
        let mut rules = Self::new();
        for line in lines {
            rules.add_order(line.as_str());
        }
        rules
    }

    fn add(&mut self, index: u32, value: u32) -> &mut Self {
        self.rules.entry(index).or_insert(vec![]).push(value);

        self
    }

    fn add_order(&mut self, order: &str) -> &mut Self {
        if let Some((index_str, value_str)) = order.split_once("|") {
            if let (Ok(index), Ok(value)) = (index_str.parse::<u32>(), value_str.parse::<u32>()) {
                return self.add(index, value);
            }
        }
        panic!("Invalid order format: {}", order);
    }

    fn check_pages_to_update(&self, pages: &Vec<u32>) -> Option<u32> {
        for pair in pages.windows(2) {
            if let [from, to] = pair {
                if !self.check_pages(from, to) {
                    return None;
                }
            }
        }
        pages.get(pages.len() / 2).cloned()
    }

    fn check_pages(&self, from: &u32, to: &u32) -> bool {
        if let Some(rules) = self.rules.get(&to) {
            return rules.contains(&from) == false;
        }

        true
    }

    fn check_pages_not_ordered(&self, pages: &Vec<u32>) -> bool {
        for pair in pages.windows(2) {
            if let [from, to] = pair {
                if !self.check_pages(from, to) {
                    return true;
                }
            }
        }
        false
    }

    fn order_pages(&self, pages: &Vec<u32>) -> Vec<u32> {
        let mut sorted_pages = pages.clone();
        sorted_pages.sort_by(|from, to| {
            if self.check_pages(from, to) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        sorted_pages
    }
}

#[cfg(test)]
mod tests {
    use crate::day05::PageOrderingRules;
    use test_case::test_case;

    fn create_page_ordering_rules() -> PageOrderingRules {
        let mut rules = PageOrderingRules::new();

        rules
            .add_order("47|53")
            .add_order("97|13")
            .add_order("97|61")
            .add_order("97|47")
            .add_order("75|29")
            .add_order("61|13")
            .add_order("75|53")
            .add_order("29|13")
            .add_order("97|29")
            .add_order("53|29")
            .add_order("61|53")
            .add_order("97|53")
            .add_order("61|29")
            .add_order("47|13")
            .add_order("75|47")
            .add_order("97|75")
            .add_order("47|61")
            .add_order("75|61")
            .add_order("47|29")
            .add_order("75|13")
            .add_order("53|13");

        rules
    }

    #[test]
    fn test_add_order() {
        // Arrange
        let rules = create_page_ordering_rules();
        // Act
        // Assert
        assert_eq!(rules.rules.get(&47), Some(&vec![53, 13, 61, 29]));
        assert_eq!(rules.rules.get(&97), Some(&vec![13, 61, 47, 29, 53, 75]));
    }

    #[test_case(29, 13, true)]
    #[test_case(13, 29, false)]
    fn test_rules(from: u32, to: u32, expected: bool) {
        // Arrange
        let rules = create_page_ordering_rules();
        // Act
        let result = rules.check_pages(&from, &to);
        // Assert
        assert_eq!(expected, result);
    }

    #[test_case(vec![75,47,61,53,29], Some(&61))]
    #[test_case(vec![97,61,53,29,13], Some(&53))]
    #[test_case(vec![75,29,13], Some(&29))]
    #[test_case(vec![75,97,47,61,53], None)]
    #[test_case(vec![61,13,29], None)]
    #[test_case(vec![97,13,75,29,47], None)]
    fn test_pages_to_order(pages: Vec<u32>, expected: Option<&u32>) {
        // Arrange
        let rules = create_page_ordering_rules();
        // Act
        let result = rules.check_pages_to_update(&pages);
        // Assert
        assert_eq!(result, expected.copied());
    }

    #[test_case(vec![75,47,61,53,29], vec![75,47,61,53,29])]
    #[test_case(vec![97,61,53,29,13], vec![97,61,53,29,13])]
    #[test_case(vec![75,29,13], vec![75,29,13])]
    #[test_case(vec![75,97,47,61,53], vec![97,75,47,61,53])]
    #[test_case(vec![61,13,29], vec![61,29,13])]
    #[test_case(vec![97,13,75,29,47], vec![97,75,47,29,13])]
    fn test_pages_ordering(pages: Vec<u32>, expected: Vec<u32>) {
        // Arrange
        let rules = create_page_ordering_rules();
        // Act
        let result = rules.order_pages(&pages);
        // Assert
        assert_eq!(result, expected);
    }
}
