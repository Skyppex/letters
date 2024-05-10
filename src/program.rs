use std::{collections::HashMap, vec};

use crate::args::LettersArgs;

pub fn run(input: String, args: LettersArgs) -> String {
    let input = input.replace('\r', " ").replace('\n', " ");
    let mut items = get_items(input);

    if args.equals.is_some() {
        let equals = args.clone().equals.unwrap();

        if args.case_sensitive {
            items = items.into_iter().filter(|s| s == &equals).collect();
        } else {
            items = items.into_iter().filter(|s| s.to_ascii_lowercase() == equals.to_ascii_lowercase()).collect();
        }
    }
    
    if args.first.is_some() {
        items = items.into_iter().take(args.first.unwrap().unwrap_or(1) as usize).collect();
    }

    if args.last.is_some() {
        items = items.into_iter().rev().take(args.last.unwrap().unwrap_or(1) as usize).collect();
    }

    if items.len() == 0 {
        return String::new();
    }
    
    let items = alter_output(items, args.clone());

    if args.output.count {
        return items.len().to_string();
    }

    if args.output.group {
        let groups = group(items);
        let mut output = vec![];

        for (key, value) in groups {
            output.push(format!("{}: {}", key, value));
        }

        return join_items_str(output, args);
    }

    join_items_char(items, args)
}

fn get_items<'a>(input: String) -> Vec<char> {
    return input.chars().collect();
}

fn group(s: Vec<char>) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.into_iter() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

fn alter_output(items: Vec<char>, args: LettersArgs) -> Vec<char> {
    let mut items = items;
    let output = args.output;

    if output.no_punctuation {
        items = items.into_iter()
            .filter(|c| !c.is_ascii_punctuation())
            .collect();
    }

    if output.trim_whitespace {
        items = items.into_iter()
            .filter(|s| !s.is_whitespace())
            .collect();
    }

    if output.lowercase {
        items = items.into_iter()
            .map(|s| s.to_ascii_lowercase())
            .collect();
    }

    return items;
}

fn join_items_char(items: Vec<char>, args: LettersArgs) -> String {
    let output = args.output;

    let items = &items.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    if output.list {
        return items.join("\n");
    }

    if output.json {
        return format!("[\"{}\"]", items.join("\", \""));
    }

    return items.join("");
}

fn join_items_str(items: Vec<String>, args: LettersArgs) -> String {
    let output = args.output;

    if output.list {
        return items.join("\n");
    }

    if output.json {
        return format!("[\"{}\"]", items.join("\", \""));
    }

    return items.join(" ");
}

#[cfg(test)]
mod tests {
    
}