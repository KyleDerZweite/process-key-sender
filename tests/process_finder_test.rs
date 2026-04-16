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
