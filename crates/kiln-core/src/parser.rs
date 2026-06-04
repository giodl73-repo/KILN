use crate::{Capability, Declaration, Field};

pub fn parse_declaration_text(text: &str) -> Declaration {
    let fields = collect_fields(text);
    let sections = collect_sections(&fields);
    let kiln_version = value_for(&fields, "kiln.version");
    let capability = capability_from(&fields);

    Declaration {
        kiln_version,
        capability,
        sections,
        fields,
    }
}

fn collect_fields(text: &str) -> Vec<Field> {
    let mut fields = Vec::new();
    let mut current_section: Option<String> = None;

    for raw_line in text.lines() {
        let line = raw_line.trim_end();
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }

        let indent = leading_spaces(line);
        let trimmed = line.trim_start();
        if indent == 0 {
            if let Some((key, value)) = split_key_value(trimmed) {
                let section = key.to_owned();
                current_section = Some(section.clone());
                fields.push(Field {
                    path: section,
                    value: value.to_owned(),
                });
            }
            continue;
        }

        if let Some(section) = current_section.as_deref() {
            if let Some((key, value)) = split_key_value(trimmed) {
                fields.push(Field {
                    path: format!("{section}.{key}"),
                    value: value.to_owned(),
                });
            } else if let Some(item) = trimmed.strip_prefix("- ") {
                fields.push(Field {
                    path: format!("{section}[]"),
                    value: item.trim().to_owned(),
                });
            }
        }
    }

    fields
}

fn collect_sections(fields: &[Field]) -> Vec<String> {
    let mut sections = Vec::new();
    for field in fields {
        if field.path.contains('.') || field.path.ends_with("[]") {
            continue;
        }
        if !sections.iter().any(|section| section == &field.path) {
            sections.push(field.path.clone());
        }
    }
    sections
}

fn capability_from(fields: &[Field]) -> Option<Capability> {
    let version = value_for(fields, "capability.version").unwrap_or_default();
    let summary = value_for(fields, "capability.summary");
    let owner = value_for(fields, "capability.owner");

    value_for(fields, "capability.id").map(|id| Capability {
        id,
        version,
        summary,
        owner,
    })
}

fn value_for(fields: &[Field], path: &str) -> Option<String> {
    fields
        .iter()
        .find(|field| field.path == path)
        .map(|field| field.value.clone())
}

fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(':')?;
    let key = key.trim();
    if key.is_empty() {
        return None;
    }
    Some((key, value.trim()))
}

fn leading_spaces(line: &str) -> usize {
    line.chars().take_while(|c| *c == ' ').count()
}
