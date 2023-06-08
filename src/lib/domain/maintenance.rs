use crate::data::DatabasePool;
use crate::service;
use std::time::Duration;

pub struct Maintenance;

impl Maintenance {
    pub fn spawn(pool: &DatabasePool) -> Self {
        let pool = pool.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Err(e) = service::action::delete_expired(&pool).await {
                    eprintln!("Failed to delete expired clips: {}", e);
                }
            }
        });
        Self
    }
}
