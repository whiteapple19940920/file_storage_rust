use file_storage_manager::storage_manager::StorageManager;

fn main() {
    let mut storage_manager = StorageManager::new();

    let file_contents = b"Hello, world!";
    let file_name = "hello.txt".to_string();
    let file_type = "txt".to_string();
    let replicas = 3;

    storage_manager.add_file(file_name.clone(), file_contents, file_type, replicas);
    if let Some(locations) = storage_manager.check_redundancy(&file_name) {
        println!("File '{}' is stored in nodes: {:?}", file_name, locations);
    }

    storage_manager.delete_file(&file_name);
    if storage_manager.check_redundancy(&file_name).is_none() {
        println!("File '{}' successfully deleted from all nodes.", file_name);
    }
}