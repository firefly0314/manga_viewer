use manga_viewer_lib::fs_ops;
use tempfile::tempdir;

#[tokio::test]
async fn test_read_write_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs_ops::write_file(file_path.to_string_lossy().to_string(), "hello".into())
        .await
        .unwrap();
    let content = fs_ops::read_file(file_path.to_string_lossy().to_string())
        .await
        .unwrap();
    assert_eq!(content, "hello");
}

#[tokio::test]
async fn test_list_directory() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("file.txt");
    fs_ops::write_file(file_path.to_string_lossy().to_string(), "data".into())
        .await
        .unwrap();
    let entries = fs_ops::list_directory(dir.path().to_string_lossy().to_string())
        .await
        .unwrap();
    assert_eq!(entries.len(), 1);
}

#[tokio::test]
async fn test_search_files() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    tokio::fs::create_dir(&sub).await.unwrap();
    let file_path = sub.join("target.txt");
    fs_ops::write_file(file_path.to_string_lossy().to_string(), "data".into())
        .await
        .unwrap();
    let results = fs_ops::search_files(dir.path().to_string_lossy().to_string(), "target".into())
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
}
