use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hits_new() {
        let hits = Hits::new(10);
        assert_eq!(hits.into_inner(), 10);
    }

    #[test]
    fn test_hits_into_inner() {
        let hits = Hits::new(5);
        assert_eq!(hits.into_inner(), 5);
    }
}
