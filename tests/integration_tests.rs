use file_storage_manager::storage_manager::StorageManager;

#[test]
fn test_add_and_retrieve_file() {
    let mut storage_manager = StorageManager::new();
    let contents = b"Hello, world!";
    let file_name = "testfile.txt".to_string();
    let file_type = "txt".to_string();
    let replicas = 3;

    storage_manager.add_file(file_name.clone(), contents, file_type, replicas);

    let locations = storage_manager.check_redundancy(&file_name).unwrap();
    assert_eq!(locations.len(), replicas);
    println!("Test passed: File is stored across {} nodes as expected.", replicas);

    for node_id in &locations {
        let node = storage_manager.get_node(*node_id).unwrap();
        let file_in_node = node.get_files().iter().find(|f| f.name() == file_name).is_some();
        assert!(file_in_node, "Test failed: File not found in node {}", node_id);
    }
}

#[test]
fn test_delete_file() {
    let mut storage_manager = StorageManager::new();
    let contents = b"Goodbye, world!";
    let file_name = "deletefile.txt".to_string();
    let file_type = "txt".to_string();
    let replicas = 2;

    // Adding file to ensure it exists before deletion
    storage_manager.add_file(file_name.clone(), contents, file_type, replicas);

    // Deleting the file
    assert!(storage_manager.delete_file(&file_name), "Test failed: Deletion returned false.");
    
    // Verify that the file is removed from all nodes
    if let Some(locations) = storage_manager.check_redundancy(&file_name) {
        for node_id in locations {
            let node = storage_manager.get_node(node_id).unwrap();
            let file_in_node = node.get_files().iter().find(|f| f.name() == file_name).is_none();
            assert!(file_in_node, "Test failed: File still present in node {} after deletion.", node_id);
        }
        panic!("Test failed: Redundancy check should have returned None.");
    }

    println!("Test passed: File successfully deleted from all nodes.");
}