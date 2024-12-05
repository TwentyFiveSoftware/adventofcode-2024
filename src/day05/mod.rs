use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let [raw_rules, raw_manuals] = input.split("\n\n").collect::<Vec<&str>>()[..] else { return; };

    let rules = raw_rules.lines().map(|line| Rule::from_str(line).unwrap()).collect::<Vec<Rule>>();
    let manuals = raw_manuals.lines().map(|line| Manual::from_str(line).unwrap()).collect::<Vec<Manual>>();

    println!("PART 1: {}", part1(&rules, &manuals));
    println!("PART 2: {}", part2(&rules, manuals));
}

fn part1(rules: &Vec<Rule>, manuals: &Vec<Manual>) -> u32 {
    manuals.iter()
        .filter(|manual| manual.satisfies_rules(rules.iter()))
        .map(|manual| manual.get_middle_page())
        .sum()
}

fn part2(rules: &Vec<Rule>, manuals: Vec<Manual>) -> u32 {
    manuals.into_iter()
        .filter(|manual| !manual.satisfies_rules(rules.iter()))
        .map(|mut manual| {
            while !manual.satisfies_rules(rules.iter()) {
                manual = manual.get_relevant_rules(rules.iter())
                    .fold(manual.clone(), |new_manual, rule| new_manual.with_rule_satisfied(rule));
            }

            manual.get_middle_page()
        })
        .sum()
}

struct Rule {
    before: u32,
    after: u32,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [first, second] = s.split("|").collect::<Vec<&str>>()[..] else { return Err(()); };

        Ok(Rule {
            before: first.parse::<u32>().unwrap_or_default(),
            after: second.parse::<u32>().unwrap_or_default(),
        })
    }
}

#[derive(Clone)]
struct Manual {
    pages: Vec<u32>,
}

impl Manual {
    fn satisfies_rules<'a>(&self, rules: impl Iterator<Item=&'a Rule>) -> bool {
        self.get_relevant_rules(rules).all(|rule| self.is_rule_satisfied(rule))
    }

    fn get_relevant_rules<'a>(&self, rules: impl Iterator<Item=&'a Rule>) -> impl Iterator<Item=&'a Rule> {
        rules.filter(|rule| self.pages.contains(&rule.before) && self.pages.contains(&rule.after))
    }

    fn is_rule_satisfied(&self, rule: &Rule) -> bool {
        self.pages.iter().position(|page| *page == rule.before) <
            self.pages.iter().position(|page| *page == rule.after)
    }

    fn get_middle_page(&self) -> u32 {
        *self.pages.get((self.pages.len() - 1) / 2).unwrap()
    }

    fn with_rule_satisfied(&self, rule: &Rule) -> Manual {
        if self.is_rule_satisfied(rule) {
            return self.clone();
        }

        let before_index = self.pages.iter().position(|page| *page == rule.before).unwrap();
        let after_index = self.pages.iter().position(|page| *page == rule.after).unwrap();

        let mut new_pages = self.pages.clone();
        new_pages.insert(before_index + 1, rule.after); // add the page one page after the containing page
        new_pages.remove(after_index); // remove the originally misplaced page

        return Manual { pages: new_pages };
    }
}

impl FromStr for Manual {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Manual {
            pages: s.split(",").map(|page| page.parse().unwrap_or_default()).collect(),
        })
    }
}
