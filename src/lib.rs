//! Bundled Old-School Essentials rule files for the ttrpg DSL.
//!
//! These files are compiled into the binary via `include_str!` and serve as
//! default rule definitions. Users can override individual files by placing
//! custom versions in their game system's rules directory.

/// All bundled OSE rule files as (filename, content) pairs.
pub static BUNDLED_RULES: &[(&str, &str)] = &[
    ("ose_ability.ttrpg", include_str!("../rules/ose_ability.ttrpg")),
    ("ose_chargen.ttrpg", include_str!("../rules/ose_chargen.ttrpg")),
    ("ose_class.ttrpg", include_str!("../rules/ose_class.ttrpg")),
    ("ose_combat.ttrpg", include_str!("../rules/ose_combat.ttrpg")),
    ("ose_core.ttrpg", include_str!("../rules/ose_core.ttrpg")),
    ("ose_equipment.ttrpg", include_str!("../rules/ose_equipment.ttrpg")),
    ("ose_exploration.ttrpg", include_str!("../rules/ose_exploration.ttrpg")),
    ("ose_magic.ttrpg", include_str!("../rules/ose_magic.ttrpg")),
    ("ose_saves.ttrpg", include_str!("../rules/ose_saves.ttrpg")),
    ("ose_spells.ttrpg", include_str!("../rules/ose_spells.ttrpg")),
    ("ose_thief.ttrpg", include_str!("../rules/ose_thief.ttrpg")),
    ("ose_time_economy.ttrpg", include_str!("../rules/ose_time_economy.ttrpg")),
    ("ose_wilderness.ttrpg", include_str!("../rules/ose_wilderness.ttrpg")),
];

/// Get a specific bundled rule file by name. Returns None if not found.
pub fn get_rule(name: &str) -> Option<&'static str> {
    BUNDLED_RULES.iter().find(|(n, _)| *n == name).map(|(_, c)| *c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_rules_load() {
        assert_eq!(BUNDLED_RULES.len(), 13);
        for (name, content) in BUNDLED_RULES {
            assert!(!content.is_empty(), "{} should not be empty", name);
        }
    }

    #[test]
    fn get_rule_by_name() {
        assert!(get_rule("ose_core.ttrpg").is_some());
        assert!(get_rule("nonexistent.ttrpg").is_none());
    }
}
