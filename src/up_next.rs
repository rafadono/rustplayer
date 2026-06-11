use crate::playlist::Track;
use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct QueueItem {
    pub path: PathBuf,
    pub title: String,
}

impl QueueItem {
    pub fn from_track(t: &Track) -> Self {
        Self {
            path: t.path.clone(),
            title: t.title.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpNextQueue {
    items: VecDeque<QueueItem>,
}

impl UpNextQueue {
    pub fn enqueue_next(&mut self, item: QueueItem) {
        self.items.push_front(item);
    }

    pub fn enqueue_last(&mut self, item: QueueItem) {
        self.items.push_back(item);
    }

    pub fn dequeue(&mut self, idx: usize) -> Option<QueueItem> {
        self.items.remove(idx)
    }

    pub fn pop_next(&mut self) -> Option<QueueItem> {
        self.items.pop_front()
    }

    pub fn clear_queue(&mut self) {
        self.items.clear();
    }

    pub fn as_slice(&self) -> Vec<QueueItem> {
        self.items.iter().cloned().collect()
    }
}
