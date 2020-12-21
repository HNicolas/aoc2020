use std::collections::HashMap;

struct RuleSet<'a> {
    rules: HashMap<&'a str, &'a str>,
}

impl<'a> RuleSet<'a> {
    fn new(input: &'a str, to_update: HashMap<&'a str, &'a str>) -> Self {
        let rules = input
            .lines()
            .map(|rule| {
                let mut rule_parts = rule.split(": ");
                let rule_number = rule_parts.next().unwrap();
                let rule_definition = if to_update.contains_key(rule_number) {
                    to_update.get(rule_number).unwrap()
                } else {
                    rule_parts.next().unwrap()
                };
                (rule_number, rule_definition)
            })
            .collect::<HashMap<_, _>>();
        Self { rules }
    }

    fn match_rule_start(&self, message: &'a str, rule_number: &str) -> Vec<&str> {
        let rule = self.rules.get(rule_number);
        match rule {
            Some(char_rule) if char_rule.contains('"') => {
                if message.starts_with(&char_rule[1..2]) {
                    vec![&message[1..]]
                } else {
                    vec![]
                }
            }
            Some(sub_rules) => sub_rules
                .split(" | ")
                .flat_map(|parts| {
                    parts.split(" ").fold(vec![message], |rests, sub_rule| {
                        rests
                            .iter()
                            .flat_map(|rest| self.match_rule_start(rest, sub_rule))
                            .collect()
                    })
                })
                .collect(),
            _ => panic!("should not happen"),
        }
    }
}

fn solve(rule_set: &RuleSet, messages: &str) -> u32 {
    messages.lines().fold(0, |acc, message| {
        let result = rule_set.match_rule_start(message, "0");
        if result.contains(&"") {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day19").unwrap();
    let mut input_parts = input.split("\n\n");
    let rules = input_parts.next().unwrap();
    let messages = input_parts.next().unwrap();

    let rule_set = RuleSet::new(rules, HashMap::new());
    println!(
        "day 19 solution 1 : {}, {}us",
        solve(&rule_set, messages),
        timer.elapsed().as_micros()
    );
    let rule_set = RuleSet::new(
        rules,
        [("8", "42 | 42 8"), ("11", "42 31 | 42 11 31")]
            .iter()
            .copied()
            .collect(),
    );
    println!(
        "day 19 solution 2 : {}, {}us",
        solve(&rule_set, messages),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb";
        let mut input_parts = input.split("\n\n");
        let rules = input_parts.next().unwrap();
        let messages = input_parts.next().unwrap();

        let rule_set = RuleSet::new(rules, HashMap::new());

        assert_eq!(solve(&rule_set, messages), 2);
    }

    #[test]
    fn test_solution_2() {
        let input = "42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        let mut input_parts = input.split("\n\n");
        let rules = input_parts.next().unwrap();
        let messages = input_parts.next().unwrap();

        let rule_set = RuleSet::new(
            rules,
            [("8", "42 | 42 8"), ("11", "42 31 | 42 11 31")]
                .iter()
                .copied()
                .collect(),
        );

        assert_eq!(solve(&rule_set, messages), 12);
    }
}
