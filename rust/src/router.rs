use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::function_moudle::FunctionMoudle;
use crate::trigger::Trigger;

#[derive(Clone)]
pub struct Router {
    router: Arc<Mutex<HashMap<Trigger, FunctionMoudle>>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            router: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn select(&self) -> Vec<(Trigger, FunctionMoudle)> {
        self.router
            .lock()
            .unwrap()
            .iter()
            .map(|(t, f)| (t.clone(), f.clone()))
            .collect()
    }

    pub fn insert(&self, t: Trigger, f: FunctionMoudle) -> Option<FunctionMoudle> {
        self.router.lock().unwrap().insert(t, f)
    }

    pub fn get(&self, t: &Trigger) -> Option<FunctionMoudle> {
        self.router.lock().unwrap().get(t).map(|f| f.clone())
    }

    pub fn remove(&self, t: &Trigger) -> Option<FunctionMoudle> {
        self.router.lock().unwrap().remove(t)
    }
}
