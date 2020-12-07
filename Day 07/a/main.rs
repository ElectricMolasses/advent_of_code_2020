use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

struct Bag {
    colour: String,
    contains: HashMap<String, i32>,
}

fn day_7() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut bag_rules: HashMap<String, Bag> = HashMap::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut current_bag = Bag {
            colour: "".to_string(),
            contains: HashMap::new(),
        };

        let colour = line.split(" bags ").collect::<Vec<&str>>()[0];
        
        current_bag.colour = colour.to_string();

        let rules: Vec<&str> = line.split(" contain ").collect::<Vec<&str>>()[1]
                                .split(", ").collect::<Vec<&str>>();

        let mut rules_strings: Vec<String> = Vec::new();

        for rule in rules {
            rules_strings.push(rule.replace(".", ""));
        }

        for rule in rules_strings {
            if rule == "no other bags" {
                break;
            }
            let mut split_rule: Vec<&str> = Vec::new();
            split_rule.push(&rule[..1]);
            split_rule.push(&rule[2..]);

            let parsed_name: String = remove_bag_from_string(split_rule[1]);

            current_bag.contains.insert(parsed_name, split_rule[0].parse::<i32>().unwrap());
        }

        bag_rules.insert(colour.to_string(), current_bag);
    }

    let answer = count_contained("shiny gold".to_string(), &bag_rules) - 1;
    //let answer = count_can_contain_gold(&bag_rules);

    println!("{}", answer);
    Ok(())
}

fn count_contained(bag: String, all_bags: &HashMap<String, Bag>) -> i32 {
    let mut count: i32 = 0;

    if all_bags[&bag].contains.len() as i32 == 0 {
        return 1;
    }

    for inner_bag in &all_bags[&bag].contains {
        count += count_contained(inner_bag.0.to_string(), all_bags) * inner_bag.1;
    }

    count += 1;

    return count;
}

fn remove_bag_from_string(string: &str) -> String {
    if string.to_string().contains(" bags") {
        return string.to_string().replace(" bags", "");
    }
    if string.to_string().contains(" bag") {
        return string.to_string().replace(" bag", "");
    }
    return "".to_string();
}

fn _print_colour(bag: &Bag) {
    println!("{}", bag.colour);
}

fn count_can_contain_gold(all_bags: &HashMap<String, Bag>) -> i32 {
    let mut answer: i32 = 0;

    for bag in all_bags.keys() {
        println!("{}", bag);
        if can_contain_gold(bag.to_string(), all_bags) {
            answer += 1;
        }
    }

    return answer - 1;
}

fn can_contain_gold(bag: String, all_bags: &HashMap<String, Bag>) -> bool {

    if (&bag == "shiny gold") {
        return true;
    }

    for inner_bag in &all_bags[&bag].contains {
        if (can_contain_gold(inner_bag.0.to_string(), all_bags)) {
            return true;
        }
    }
    return false;
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = day_7();

    Ok(())
}