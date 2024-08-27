use std::process::Command;

#[derive(Clone, Debug)]
pub struct Flag {
    pub name: String,
    pub value: Option<String>,
}

pub fn flag_merge(flags1: &Vec<Flag>, flags2: &Vec<Flag>) -> Vec<Flag> {
    let mut final_flags = flags1.to_vec();

    for flag2 in flags2 {
        let mut found = false;
        for flag1 in &mut final_flags {
            if flag1.name == flag2.name {
                flag1.value = flag2.value.clone();
                found = true;
                break;
            }
        }

        if !found {
            final_flags.push(flag2.clone());
        }
    }

    final_flags
}

pub fn parse_flags(flags: String) -> Result<Vec<Flag>, String> {
    let mut result = Vec::new();

    for flag in flags.split(';') {
        if flag.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = flag.splitn(2, ' ').collect();

        // Check if the flag name is missing
        if parts.is_empty() || parts[0].trim().is_empty() {
            return Err(format!("Syntax error: Missing flag name in '{}'", flag));
        }

        let name = parts[0].to_string();

        // Check if there is an unbalanced quote
        if parts.len() > 1 && parts[1].matches('"').count() % 2 != 0 {
            return Err(format!(
                "Syntax error: Unbalanced quotes in value '{}'",
                parts[1]
            ));
        }

        let value = if parts.len() > 1 {
            Some(parts[1].trim().to_string())
        } else {
            None
        };

        result.push(Flag { name, value });
    }

    Ok(result)
}

pub fn stringify_flags(flags: Vec<Flag>) -> String {
    flags
        .into_iter()
        .map(|flag| {
            if let Some(value) = flag.value {
                format!("{} \"{}\"", flag.name, value.trim_matches('"'))
            } else {
                flag.name
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn add_flags(command: &mut Command, flags: Vec<Flag>) {
    for flag in flags {
        command.arg(flag.name);
        if flag.value.is_some() {
            command.arg(flag.value.unwrap().trim_matches('"'));
        }
    }
}
