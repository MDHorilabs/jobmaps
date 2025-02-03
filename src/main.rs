mod storage;
use storage::object::index_to_object;
use storage::Storage;

#[tokio::main]

async fn main() {
    let mut storage = Storage::new("test", 3).await;
    // storage.add(b"ini 1".to_vec(), b"one one one".to_vec());
    // storage.add(b"ini 2".to_vec(), b"two two two".to_vec());
    // storage.add(b"ini 3".to_vec(), b"two two two".to_vec());
    // storage.add(b"ini 4".to_vec(), b"two two two".to_vec());
    // storage.sync_all().await;
    // storage.add(b"ini 5".to_vec(), b"two two two".to_vec());
    // storage.add(b"ini 6".to_vec(), b"two two two".to_vec());
    // storage.add(b"ini 7".to_vec(), b"two two two".to_vec());
    // storage.add(b"ini 8".to_vec(), b"two two two".to_vec());
    // storage.add(b"ini 9".to_vec(), b"two two two".to_vec());
    // storage.sync_all().await;
    println!("{:?}, {:?}", storage.indexes.len(), storage.last_offset);
    storage.indexes.iter().for_each(|(offset, index)| {
        let object = index_to_object(index.clone(), storage.get_store());
        println!(
            "offset: {}, size: {}, head_size: {}, head: {:?}, body: {:?},",
            offset,
            object.1,
            object.2,
            String::from_utf8(object.3.clone()),
            String::from_utf8(object.4.clone()),
        );
    });
}
