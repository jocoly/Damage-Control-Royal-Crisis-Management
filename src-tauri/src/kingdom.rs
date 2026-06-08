use crate::game_state::{InputCounts, InputSnapshot};

pub(crate) const MAX_KINGDOM_NAME_LEN: usize = 12;

impl InputCounts {
    pub(crate) fn set_kingdom_name(&self, name: &str) -> Result<InputSnapshot, String> {
        let name = normalize_kingdom_name(name)?;
        *self
            .kingdom_name
            .lock()
            .expect("kingdom name lock poisoned") = Some(name);
        self.mark_dirty();

        Ok(self.snapshot())
    }
}

pub(crate) fn normalize_kingdom_name(name: &str) -> Result<String, String> {
    let normalized = name.split_whitespace().collect::<Vec<_>>().join(" ");

    if normalized.is_empty() {
        return Err("Enter a kingdom name.".to_string());
    }

    if normalized.len() > MAX_KINGDOM_NAME_LEN {
        return Err(format!(
            "Kingdom names can be at most {MAX_KINGDOM_NAME_LEN} characters."
        ));
    }

    if !normalized
        .bytes()
        .any(|character| character.is_ascii_alphanumeric())
    {
        return Err("Include at least one letter or number.".to_string());
    }

    if !normalized.bytes().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, b' ' | b'\'' | b'-')
    }) {
        return Err("Use only letters, numbers, spaces, apostrophes, and hyphens.".to_string());
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kingdom_names_are_trimmed_and_spaces_are_collapsed() {
        assert_eq!(
            normalize_kingdom_name("  New   Vale  "),
            Ok("New Vale".to_string())
        );
    }

    #[test]
    fn kingdom_names_reject_unsupported_characters_and_excess_length() {
        assert!(normalize_kingdom_name("Avalon!").is_err());
        assert!(normalize_kingdom_name("---").is_err());
        assert!(normalize_kingdom_name("This Kingdom Name Is Far Too Long").is_err());
    }
}
