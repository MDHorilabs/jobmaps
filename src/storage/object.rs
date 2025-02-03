// (offset, size, head_size, head, body)
pub type Object = (u64, u32, u16, Vec<u8>, Vec<u8>);

// (offset, size, head_size, head)
pub type IndexObject = (u64, u32, u16, Vec<u8>);

const CAP_SIZE: usize = 14;

pub fn create_object(offset: u64, head: &[u8], body: &[u8]) -> Object {
    (
        offset,
        (body.len() + head.len() + CAP_SIZE) as u32,
        head.len() as u16,
        head.to_vec(),
        body.to_vec(),
    )
}

pub fn object_to_index(object: Object) -> IndexObject {
    (object.0, object.1, object.2, object.3)
}

pub fn index_to_object(index: IndexObject, store: &memmap2::MmapMut) -> Object {
    let start = index.0 as usize + index.2 as usize + CAP_SIZE;
    (
        index.0,
        index.1,
        index.2,
        index.3,
        store[start..(start + index.1 as usize) - (CAP_SIZE + index.2 as usize)].to_vec(),
    )
}

pub fn object_to_bytes(object: &Object) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&object.0.to_le_bytes());
    bytes.extend_from_slice(&object.1.to_le_bytes());
    bytes.extend_from_slice(&object.2.to_le_bytes());
    bytes.extend_from_slice(&object.3);
    bytes.extend_from_slice(&object.4);
    bytes
}

pub fn bytes_to_object(bytes: &[u8]) -> Object {
    let size = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
    let head_size = u16::from_le_bytes(bytes[12..14].try_into().unwrap());
    (
        u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
        size,
        head_size,
        bytes[14..(14 + head_size as usize)].to_vec(),
        bytes[(14 + head_size as usize)..(14 + head_size as usize + size as usize)].to_vec(),
    )
}

pub fn bytes_to_index(bytes: &[u8]) -> IndexObject {
    let size = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
    let head_size = u16::from_le_bytes(bytes[12..14].try_into().unwrap());
    (
        u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
        size,
        head_size,
        bytes[14..(14 + head_size as usize)].to_vec(),
    )
}

pub fn offset_to_object(offset: u64, store: &memmap2::MmapMut) -> Option<Object> {
    let b_offset = store[offset as usize..offset as usize + 8].try_into();
    match b_offset {
        Err(_) => return None,
        Ok(u64_bytes) => {
            let self_offset = u64::from_le_bytes(u64_bytes);
            if self_offset != offset {
                return None;
            }
        }
    }
    let size = u32::from_le_bytes(
        store[offset as usize + 8..offset as usize + 12]
            .try_into()
            .unwrap(),
    );
    let head_size = u16::from_le_bytes(
        store[offset as usize + 12..offset as usize + 14]
            .try_into()
            .unwrap(),
    );
    Some((
        offset,
        size,
        head_size,
        store[offset as usize + 14..offset as usize + 14 + head_size as usize].to_vec(),
        store[offset as usize + 14 + head_size as usize
            ..offset as usize + 14 + head_size as usize + size as usize]
            .to_vec(),
    ))
}

pub fn offset_to_index(offset: u64, store: &memmap2::MmapMut) -> Option<IndexObject> {
    let b_offset = store[offset as usize..offset as usize + 8].try_into();
    match b_offset {
        Err(_) => return None,
        Ok(u64_bytes) => {
            let self_offset = u64::from_le_bytes(u64_bytes);
            if self_offset != offset {
                return None;
            }
        }
    }
    let size = u32::from_le_bytes(
        store[offset as usize + 8..offset as usize + 12]
            .try_into()
            .unwrap(),
    );
    let head_size = u16::from_le_bytes(
        store[offset as usize + 12..offset as usize + 14]
            .try_into()
            .unwrap(),
    );
    Some((
        offset,
        size,
        head_size,
        store[offset as usize + 14..offset as usize + 14 + head_size as usize].to_vec(),
    ))
}
