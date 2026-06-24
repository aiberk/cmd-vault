use regex::Regex;

/// Extracts placeholder names from a command string.
/// Placeholders are defined as `<name>` patterns.
/// Returns unique placeholder names in order of first occurrence.
///
/// # Example
/// ```
/// let cmd = "ffmpeg -i <input> -o <output>";
/// let placeholders = extract_placeholders(cmd);
/// assert_eq!(placeholders, vec!["input", "output"]);
/// ```
pub fn extract_placeholders(command: &str) -> Vec<String> {
    let re = Regex::new(r"<([^>]+)>").unwrap();
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for cap in re.captures_iter(command) {
        let name = cap[1].to_string();
        if seen.insert(name.clone()) {
            result.push(name);
        }
    }
    result
}

/// Replaces all `<placeholder>` tokens in the command with the provided values.
/// Each unique placeholder name maps to one value; all occurrences are replaced.
pub fn fill_placeholders(command: &str, placeholders: &[String], values: &[String]) -> String {
    let mut result = command.to_string();
    for (placeholder, value) in placeholders.iter().zip(values.iter()) {
        let token = format!("<{}>", placeholder);
        result = result.replace(&token, value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_placeholders() {
        let cmd = "scp <source_file> user@<host>:<remote_path>";
        let result = extract_placeholders(cmd);
        assert_eq!(result, vec!["source_file", "host", "remote_path"]);
    }

    #[test]
    fn test_no_placeholders() {
        let cmd = "ls -la";
        let result = extract_placeholders(cmd);
        assert!(result.is_empty());
    }

    #[test]
    fn test_fill_placeholders() {
        let cmd = "ffmpeg -i <input> -o <output>";
        let placeholders = vec!["input".to_string(), "output".to_string()];
        let values = vec!["video.mp4".to_string(), "compressed.mp4".to_string()];
        let result = fill_placeholders(cmd, &placeholders, &values);
        assert_eq!(result, "ffmpeg -i video.mp4 -o compressed.mp4");
    }

    #[test]
    fn test_duplicate_placeholder_deduplicates_and_fills_all() {
        let cmd = "cp <file> /backup/<file>";
        let placeholders = extract_placeholders(cmd);
        // Should only ask once
        assert_eq!(placeholders, vec!["file"]);
        // But fill all occurrences
        let values = vec!["data.txt".to_string()];
        let result = fill_placeholders(cmd, &placeholders, &values);
        assert_eq!(result, "cp data.txt /backup/data.txt");
    }

    #[test]
    fn test_complex_placeholder_command() {
        let cmd = "scp <local_file> <user>@<host>:<remote_path>";
        let placeholders = extract_placeholders(cmd);
        assert_eq!(
            placeholders,
            vec!["local_file", "user", "host", "remote_path"]
        );
        let values = vec![
            "app.tar.gz".to_string(),
            "deploy".to_string(),
            "10.0.0.1".to_string(),
            "/opt/app/".to_string(),
        ];
        let result = fill_placeholders(cmd, &placeholders, &values);
        assert_eq!(result, "scp app.tar.gz deploy@10.0.0.1:/opt/app/");
    }
}
