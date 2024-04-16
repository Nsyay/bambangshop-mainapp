use dashmap::Dashmap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;
pub struct SubscriberRepository;

lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, Subscriber>> = DashMap::new();
}

implSubscriberRepository {
    
}