mod storage;
use storage::object::{create_object, Object};

struct Job {
    id: u64,
    tags: Vec<String>,
    // state is 0:new, 1:progressing, 2:done, 3:failed, 4:archived, 5:deleted, 6:hold
    state: u8,
    // if priority is 254 is high priority, 0 is low priority and 255 is request/direct
    priority: u8,
    body: Vec<u8>,
}

// (id, tags, state, priority, (offset, size))
type IndexJob = (u64, Vec<String>, u8, u8, (u64, u32));

fn job_to_indexjob(job: Job, offset: u64, size: u32) -> IndexJob {
    (job.id, job.tags, job.state, job.priority, (offset, size))
}

fn job_to_object(job: &Job, offset: u64) -> Object {
    let mut head = Vec::new();
    let mut b_tags = Vec::new();
    job.tags.iter().for_each(|tag| {
        b_tags.extend_from_slice(tag.as_bytes());
        b_tags.push(0);
    });
    head.extend_from_slice(&job.id.to_le_bytes());
    head.extend_from_slice(&job.state.to_le_bytes());
    head.extend_from_slice(&job.priority.to_le_bytes());
    head.extend_from_slice(&(b_tags.len() as u16).to_le_bytes());
    head.extend_from_slice(&b_tags);
    create_object(offset, &head, &job.body)
}

fn object_to_job(object: &Object) -> Job {
    let mut tags: Vec<String> = Vec::new();
    let head: Vec<u8> = object.3.clone();
    let mut i = 0;
    while i < object.2 as usize {
        let mut tag = Vec::new();
        while i < object.2 as usize && head[i] != 0 {
            tag.push(head[i]);
            i += 1;
        }
        tags.push(String::from_utf8(tag).unwrap());
        i += 1;
    }
    Job {
        id: u64::from_le_bytes(head[0..8].try_into().unwrap()),
        tags,
        state: u8::from_le_bytes(head[8..9].try_into().unwrap()),
        priority: u8::from_le_bytes(head[9..10].try_into().unwrap()),
        body: object.4.clone(),
    }
}
#[tokio::main]
async fn main() {
    let job = Job {
        id: 1,
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        state: 0,
        priority: 0,
        body: b"ini body job".to_vec(),
    };
    let object = job_to_object(&job, 1);
}
