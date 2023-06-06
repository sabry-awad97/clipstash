use crossbeam_channel::Sender;

use crate::{data::DatabasePool, ServiceError, ShortCode};

#[derive(Debug, thiserror::Error)]
enum HitCountError {
    #[error("service error: {0}")]
    Service(#[from] ServiceError),
    #[error("communication error: {0}")]
    Channel(#[from] crossbeam_channel::SendError<HitCountMsg>),
}

enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32),
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>,
}

impl HitCounter {
    pub async fn new(pool: &DatabasePool) -> Self {
        unimplemented!()
    }
}
