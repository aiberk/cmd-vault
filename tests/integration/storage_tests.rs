use cmd_vault::{models::Command, storage::Storage};
use tempfile::tempdir;

#[test]
fn test_storage_roundtrip() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test.json");
    
    let mut storage = Storage::new(storage_path.clone());
    
    // Add some test commands
    let cmd1 = Command {
        name: "test1".to_string(),
        command: "echo hello".to_string(),
        desc: "Test command 1".to_string(),
        created_at: 123456789,
    };
    
    let cmd2 = Command {
        name: "test2".to_string(),
        command: "ls -la".to_string(),
        desc: "Test command 2".to_string(),
        created_at: 123456790,
    };
    
    storage.add_command(cmd1.clone()).unwrap();
    storage.add_command(cmd2.clone()).unwrap();
    
    // Save and reload
    storage.save().unwrap();
    let loaded_storage = Storage::load(storage_path).unwrap();
    
    let commands = loaded_storage.get_commands();
    assert_eq!(commands.len(), 2);
    assert_eq!(commands[0].name, "test1");
    assert_eq!(commands[1].name, "test2");
}

#[test]
fn test_duplicate_command_names() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test.json");
    
    let mut storage = Storage::new(storage_path);
    
    let cmd = Command {
        name: "duplicate".to_string(),
        command: "echo hello".to_string(),
        desc: "Test command".to_string(),
        created_at: 123456789,
    };
    
    // First add should succeed
    assert!(storage.add_command(cmd.clone()).is_ok());
    
    // Second add with same name should fail
    assert!(storage.add_command(cmd).is_err());
}

#[test]
fn test_command_search() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test.json");
    
    let mut storage = Storage::new(storage_path);
    
    let commands = vec![
        Command {
            name: "docker-build".to_string(),
            command: "docker build -t myapp .".to_string(),
            desc: "Build Docker image".to_string(),
            created_at: 1,
        },
        Command {
            name: "git-push".to_string(),
            command: "git push origin main".to_string(),
            desc: "Push to main branch".to_string(),
            created_at: 2,
        },
        Command {
            name: "ffmpeg-convert".to_string(),
            command: "ffmpeg -i input.mp4 output.avi".to_string(),
            desc: "Convert video format".to_string(),
            created_at: 3,
        },
    ];
    
    for cmd in commands {
        storage.add_command(cmd).unwrap();
    }
    
    // Test search functionality
    let docker_results = storage.search("docker");
    assert_eq!(docker_results.len(), 1);
    assert_eq!(docker_results[0].name, "docker-build");
    
    let git_results = storage.search("git");
    assert_eq!(git_results.len(), 1);
    assert_eq!(git_results[0].name, "git-push");
    
    let no_results = storage.search("nonexistent");
    assert_eq!(no_results.len(), 0);
}