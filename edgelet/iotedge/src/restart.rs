// Copyright (c) Microsoft. All rights reserved.

use std::cell::RefCell;
use std::io::Write;
use std::sync::Arc;

use edgelet_core::ModuleRuntime;
use futures::Future;

use error::Error;
use Command;

pub struct Restart<M, W> {
    id: String,
    runtime: M,
    output: Arc<RefCell<W>>,
}

impl<M, W> Restart<M, W> {
    pub fn new(id: String, runtime: M, output: W) -> Self {
        Restart {
            id,
            runtime,
            output: Arc::new(RefCell::new(output)),
        }
    }
}

impl<M, W> Command for Restart<M, W>
where
    M: 'static + ModuleRuntime + Clone,
    M::Error: Into<Error>,
    W: 'static + Write,
{
    type Future = Box<Future<Item = (), Error = Error>>;

    fn execute(&mut self) -> Self::Future {
        let id = self.id.clone();
        let write = self.output.clone();
        let result = self
            .runtime
            .restart(&id)
            .map_err(|e| e.into())
            .and_then(move |_| {
                let mut w = write.borrow_mut();
                writeln!(w, "{}", id)?;
                Ok(())
            });
        Box::new(result)
    }
}
