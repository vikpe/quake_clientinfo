//! # quake_clientinfo
//! Parse QuakeWorld clientinfo strings

use std::collections::HashMap;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Clientinfo {
    pub name: Option<String>,
    pub team: Option<String>,
    pub topcolor: Option<i32>,
    pub bottomcolor: Option<i32>,
    pub spectator: Option<i32>,
    pub client: Option<String>,
}

impl From<&HashMap<String, String>> for Clientinfo {
    fn from(value: &HashMap<String, String>) -> Self {
        Self {
            client: map_get_string(value, "*client"),
            name: map_get_string(value, "name"),
            team: map_get_string(value, "team"),
            topcolor: map_get_int(value, "topcolor"),
            bottomcolor: map_get_int(value, "bottomcolor"),
            spectator: map_get_int(value, "*spectator"),
        }
    }
}

fn map_get_string(map: &HashMap<String, String>, key: &str) -> Option<String> {
    map.get(key).map(|v| v.to_string())
}

fn map_get_int(map: &HashMap<String, String>, key: &str) -> Option<i32> {
    map.get(key)?.parse().ok()
}

/// # Examples
/// ```
/// use quake_clientinfo::Clientinfo;
///
/// let info = Clientinfo::from(r#"\team\red\name\Alpha\*spectator\1"#);
/// assert_eq!(info.name, Some("Alpha".to_string()));
/// assert_eq!(info.team, Some("red".to_string()));
/// assert_eq!(info.spectator, Some(1));
/// assert_eq!(info.topcolor, None);
/// ```
impl From<&str> for Clientinfo {
    fn from(value: &str) -> Self {
        Self::from(&quake_infostring::to_hashmap(value))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INFO_STR: &str =
        r#"\*client\libqwclient 0.1\*spectator\1\bottomcolor\11\topcolor\12\team\red\name\Alpha"#;

    #[test]
    fn test_from_str() {
        let info = Clientinfo::from(INFO_STR);
        assert_eq!(info.name, Some("Alpha".to_string()));
        assert_eq!(info.team, Some("red".to_string()));
        assert_eq!(info.topcolor, Some(12));
        assert_eq!(info.bottomcolor, Some(11));
        assert_eq!(info.spectator, Some(1));
        assert_eq!(info.client, Some("libqwclient 0.1".to_string()));
    }

    #[test]
    fn test_from_hashmap() {
        let map = HashMap::from([
            ("name".to_string(), "Alpha".to_string()),
            ("team".to_string(), "red".to_string()),
        ]);

        let expected = Clientinfo {
            name: Some("Alpha".to_string()),
            team: Some("red".to_string()),
            ..Default::default()
        };

        assert_eq!(Clientinfo::from(&map), expected);
    }
}
