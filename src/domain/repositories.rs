use super::entities::Client;

pub trait ClientRepository {
    fn by_id(&self, id: &str) -> Option<&Client>;
}