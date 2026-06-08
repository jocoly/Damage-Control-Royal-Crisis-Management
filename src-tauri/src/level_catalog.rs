use std::sync::OnceLock;

const MAX_CATALOG_LEVEL: u64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelDefinition {
    pub level: u64,
    pub xp_required: u64,
    pub xp_required_for_next_level: u64,
    pub xp_to_next_level: u64,
}

static LEVEL_CATALOG: OnceLock<Vec<LevelDefinition>> = OnceLock::new();

pub fn level_catalog() -> &'static [LevelDefinition] {
    LEVEL_CATALOG.get_or_init(|| {
        (1..=MAX_CATALOG_LEVEL)
            .map(|level| {
                let xp_required = calculate_xp_required_for_level(level);
                let xp_required_for_next_level = calculate_xp_required_for_level(level + 1);

                LevelDefinition {
                    level,
                    xp_required,
                    xp_required_for_next_level,
                    xp_to_next_level: xp_required_for_next_level - xp_required,
                }
            })
            .collect()
    })
}

pub fn level_for_xp(xp: u64) -> u64 {
    let mut level = 1;

    while xp >= xp_required_for_level(level + 1) {
        level += 1;
    }

    level
}

pub fn xp_required_for_level(level: u64) -> u64 {
    if let Some(definition) = level_catalog().get(level.saturating_sub(1) as usize) {
        return definition.xp_required;
    }

    calculate_xp_required_for_level(level)
}

fn calculate_xp_required_for_level(level: u64) -> u64 {
    let mut xp = 0_u64;

    for completed_level in 1..level {
        xp = xp.saturating_add(xp_needed_to_advance_from_level(completed_level));
    }

    xp
}

pub fn xp_needed_to_advance_from_level(level: u64) -> u64 {
    const EARLY_REQUIREMENTS: [u64; 17] = [
        0, 500, 2_000, 6_250, 10_000, 8_750, 7_500, 8_750, 11_250, 12_500, 22_500, 23_750, 23_750,
        25_000, 27_500, 31_250, 35_000,
    ];

    if let Some(requirement) = EARLY_REQUIREMENTS.get(level as usize) {
        return *requirement;
    }

    let mut requirement = EARLY_REQUIREMENTS[16];

    for _ in 17..level {
        requirement = requirement.saturating_mul(108) / 100;
    }

    requirement
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_curve_matches_reference_play_days_then_grows() {
        assert_eq!(level_for_xp(0), 1);
        assert_eq!(level_for_xp(24_000), 5);
        assert_eq!(level_for_xp(48_000), 8);
        assert_eq!(level_for_xp(72_000), 10);
        assert_eq!(level_for_xp(96_000), 11);
        assert_eq!(level_for_xp(120_000), 12);
        assert_eq!(level_for_xp(144_000), 13);
        assert!(xp_needed_to_advance_from_level(18) > xp_needed_to_advance_from_level(17));
        assert!(xp_needed_to_advance_from_level(30) > xp_needed_to_advance_from_level(18));
    }

    #[test]
    fn level_catalog_contains_level_specific_xp_attributes() {
        let level_five = level_catalog()
            .get(4)
            .expect("level five should be present in the catalog");

        assert_eq!(level_five.level, 5);
        assert_eq!(level_five.xp_required, 18_750);
        assert_eq!(
            level_five.xp_required_for_next_level,
            xp_required_for_level(6)
        );
        assert_eq!(
            level_five.xp_to_next_level,
            level_five.xp_required_for_next_level - level_five.xp_required
        );
        assert_eq!(level_catalog().len(), MAX_CATALOG_LEVEL as usize);
    }
}
