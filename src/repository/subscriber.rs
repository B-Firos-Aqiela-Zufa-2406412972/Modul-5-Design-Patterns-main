use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

pub struct SubscriberRepository;

lazy_static! {
    pub static ref SUBSCRIBERS: DashMap<String, DashMap<String,Subscriber>> = DashMap::new();
}

impl SubscriberRepository{
    
}

