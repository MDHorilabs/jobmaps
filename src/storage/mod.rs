use memmap2::MmapMut;
use tokio::fs::{File, OpenOptions};

mod indexes;
pub mod object;
use indexes::Indexes;
use object::{create_object, object_to_bytes, object_to_index, offset_to_index};

pub struct Storage {
    file: File,
    pub store: MmapMut,
    batch_size: u32,
    pub indexes: Indexes,
    pub last_offset: u64,
}

impl Storage {
    pub async fn new(path: &str, batch_size: u32) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await
            .unwrap();
        let (store, indexes, last_offset) = Self::init(&file, batch_size).await;
        Self {
            file,
            store,
            indexes,
            last_offset,
            batch_size,
        }
    }

    async fn init(file: &File, batch_size: u32) -> (MmapMut, Indexes, u64) {
        let metadata = file.metadata().await.unwrap();
        if metadata.len() == 0 {
            file.set_len(batch_size as u64 * 1024 * 1024).await.unwrap();
            file.sync_all().await.unwrap();
            return (
                unsafe { MmapMut::map_mut(file).unwrap() },
                Indexes::new(),
                0 as u64,
            );
        }
        let store = unsafe { MmapMut::map_mut(file).unwrap() };
        let (offset, indexes) = Self::traveling(&store).await;
        return (store, indexes, offset);
    }

    pub async fn traveling(store: &MmapMut) -> (u64, Indexes) {
        let mut offset: u64 = 0;
        let mut indexes = Indexes::new();
        loop {
            match offset_to_index(offset, store) {
                Some(index) => {
                    offset += index.1 as u64;
                    indexes.add(index);
                }
                None => break,
            }
        }
        return (offset, indexes);
    }

    pub async fn sync_all(&mut self) {
        self.flush().await;
        self.store = unsafe { MmapMut::map_mut(&self.file).unwrap() };
        let (last_offset, indexes) = Self::traveling(&self.store).await;
        self.indexes = indexes;
        self.last_offset = last_offset;
    }

    pub async fn flush(&mut self) {
        self.store.flush().unwrap();
        self.file.sync_all().await.unwrap();
    }

    pub fn add(&mut self, head: Vec<u8>, body: Vec<u8>) {
        let object = create_object(self.last_offset, &head, &body);
        self.store[self.last_offset as usize..self.last_offset as usize + object.1 as usize]
            .copy_from_slice(&object_to_bytes(&object));
        self.last_offset += object.1 as u64;
        self.indexes.add(object_to_index(object));
    }

    pub fn get_store(&self) -> &MmapMut {
        &self.store
    }
}
