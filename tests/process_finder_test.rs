use process_key_sender::ProcessFinder;

#[test]
fn test_process_finder_creation() {
    let finder = ProcessFinder::new();
    let finder2 = finder.clone();
    drop(finder);
    drop(finder2);
}

#[test]
fn test_process_finder_default() {
    let finder = ProcessFinder::default();
    drop(finder);
}

#[test]
fn test_process_finder_nonexistent_process() {
    let mut finder = ProcessFinder::new();
    let result = finder.find_process_window("nonexistent_process_xyz_123456");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_process_finder_finds_running_process_case_insensitively() {
    let executable = std::env::current_exe().unwrap();
    // Linux process names may be truncated to 15 characters.
    let process_name: String = executable
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .chars()
        .take(15)
        .collect();
    let mut finder = ProcessFinder::new();
    // Another concurrent test run may have the same process name.
    assert!(finder
        .find_process_window(&process_name.to_uppercase())
        .unwrap()
        .is_some());
}
