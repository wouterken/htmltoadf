/**
 * A structure to describe which child types are permitted.
 * Some types are only valid as the first element within a collection.
 */
#[derive(Clone, Debug, Default)]
pub struct AdfPermittedChildren {
    pub starts_with_types: Vec<String>,
    pub permitted_types: Vec<String>,
}

impl AdfPermittedChildren {
    pub fn any_starts_with(
        starts_with_types: &[&str],
        permitted_types: &[&str],
    ) -> AdfPermittedChildren {
        AdfPermittedChildren {
            permitted_types: permitted_types.iter().map(|s| s.to_string()).collect(),
            starts_with_types: starts_with_types.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn any(permitted_types: &[&str]) -> AdfPermittedChildren {
        AdfPermittedChildren {
            permitted_types: permitted_types.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        }
    }
}
