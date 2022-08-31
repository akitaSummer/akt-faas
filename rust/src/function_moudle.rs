use std::fs::{remove_file, File};
use std::io::prelude::Write;
use std::path::Path;
use std::process::Command;

use serde_derive::Deserialize;

use hyper::{Body, Response};

use hyper::http::request::Parts;

use crate::router::Router;
use crate::trigger::Trigger;

#[derive(Clone, Debug, Deserialize)]
pub struct FunctionMoudle {
    name: String,
    language: String,
    source: String,
    method: String,
    path: String,
    cpu: String,
    memory: String,
    uptime: String,
}

impl FunctionMoudle {
    pub fn from_json(b: &[u8]) -> Option<FunctionMoudle> {
        serde_json::from_slice(b).ok()
    }

    pub fn trigger(&self) -> Trigger {
        Trigger::new(self.method.as_str(), self.path.as_str())
    }

    pub fn build(self) -> Result<FunctionMoudle, &'static str> {
        todo!()
    }

    pub async fn run(&self, _headers: Parts, body: Body) -> Response<Body> {
        todo!()
    }

    pub fn delete(&self, router: Router) -> Response<Body> {
        todo!()
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
