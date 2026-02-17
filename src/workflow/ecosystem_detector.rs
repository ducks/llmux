//! Auto-detect ecosystem and current project from working directory

use crate::config::EcosystemConfig;
use std::collections::HashMap;
use std::path::Path;

/// Detect which ecosystem the working directory belongs to
///
/// Looks through all configured ecosystems and checks if working_dir
/// is within any project path.
pub fn detect_ecosystem(
    working_dir: &Path,
    ecosystems: &HashMap<String, EcosystemConfig>,
) -> Option<(String, String)> {
    // Convert working_dir to canonical path for comparison
    let working_canonical = working_dir.canonicalize().ok()?;

    for (ecosystem_name, ecosystem_config) in ecosystems {
        for (project_name, project_config) in &ecosystem_config.projects {
            // Expand ~ in project path
            let project_path_str = project_config.path.display().to_string();
            let expanded_path = shellexpand::tilde(&project_path_str);
            let project_path = Path::new(expanded_path.as_ref());

            // Try to canonicalize project path (may not exist)
            if let Ok(project_canonical) = project_path.canonicalize() {
                // Check if working_dir is within this project
                if working_canonical.starts_with(&project_canonical) {
                    return Some((ecosystem_name.clone(), project_name.clone()));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProjectConfig;
    use tempfile::TempDir;

    #[test]
    fn test_detect_ecosystem_in_project() {
        let tmp = TempDir::new().unwrap();
        let project_dir = tmp.path().join("myproject");
        std::fs::create_dir(&project_dir).unwrap();

        let mut ecosystems = HashMap::new();
        let mut projects = HashMap::new();
        projects.insert(
            "myproject".to_string(),
            ProjectConfig {
                path: project_dir.clone(),
                project_type: Some("rust".into()),
                depends_on: vec![],
                tags: vec![],
                description: String::new(),
            },
        );

        let mut config = EcosystemConfig::default();
        config.projects = projects;
        ecosystems.insert("test".to_string(), config);

        let result = detect_ecosystem(&project_dir, &ecosystems);
        assert_eq!(result, Some(("test".to_string(), "myproject".to_string())));
    }

    #[test]
    fn test_detect_ecosystem_in_subdirectory() {
        let tmp = TempDir::new().unwrap();
        let project_dir = tmp.path().join("myproject");
        let subdir = project_dir.join("src");
        std::fs::create_dir_all(&subdir).unwrap();

        let mut ecosystems = HashMap::new();
        let mut projects = HashMap::new();
        projects.insert(
            "myproject".to_string(),
            ProjectConfig {
                path: project_dir.clone(),
                project_type: Some("rust".into()),
                depends_on: vec![],
                tags: vec![],
                description: String::new(),
            },
        );

        let mut config = EcosystemConfig::default();
        config.projects = projects;
        ecosystems.insert("test".to_string(), config);

        let result = detect_ecosystem(&subdir, &ecosystems);
        assert_eq!(result, Some(("test".to_string(), "myproject".to_string())));
    }

    #[test]
    fn test_detect_ecosystem_not_found() {
        let tmp = TempDir::new().unwrap();
        let other_dir = tmp.path().join("other");
        std::fs::create_dir(&other_dir).unwrap();

        let ecosystems = HashMap::new();

        let result = detect_ecosystem(&other_dir, &ecosystems);
        assert_eq!(result, None);
    }
}
