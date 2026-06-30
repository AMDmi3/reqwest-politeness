// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This crate provides `reqwest` middleware which enforces delay between
//! requests to a single host.
//!
//! ```no_run
//! # use std::time::Duration;
//! use reqwest_politeness::ReqwestPoliteness;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let middleware = ReqwestPoliteness::default()
//!     .with_delay(Duration::from_secs(1));
//!
//! // Base clent
//! let client = reqwest::Client::builder()
//!     .build()?;
//!
//! // With added middleware
//! let client = reqwest_middleware::ClientBuilder::new(client)
//!     .with(middleware)
//!     .build();
//!
//! client.get("https://foo.com/").send().await?;
//! client.get("https://foo.com/").send().await?; // waits for 1 second
//! client.get("https://bar.com/").send().await?; // does not wait, other host
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

use async_trait::async_trait;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Next, Result};
use tokio::sync::Mutex as TokioMutex;

use std::collections::HashMap;
use std::sync::{Arc, Mutex as StdMutex};
use std::time::{Duration, Instant};

#[derive(Default)]
struct HostState {
    last_freed: Option<Instant>,
}

/// Middleware for client politeness enforcement.
#[derive(Default)]
pub struct ReqwestPoliteness {
    delay: Duration,
    hosts: StdMutex<HashMap<String, Arc<TokioMutex<HostState>>>>,
}

impl ReqwestPoliteness {
    /// Sets delay between requests to a single host.
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }
}

#[async_trait]
impl reqwest_middleware::Middleware for ReqwestPoliteness {
    async fn handle(
        &self,
        req: Request,
        extensions: &'_ mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let key = req
            .url()
            .host_str()
            .map(|host| host.to_owned())
            .unwrap_or_default();

        // With minimal contention on whole hashmap, obtain Arc-pointer
        // to a lock for a given host.
        let host_lock = Arc::clone(
            self.hosts
                .lock()
                .expect("mutex is expected not to be poisoned")
                .entry(key)
                .or_default(),
        );

        // First, prevent concurrent access to this host entry.
        let mut host_guard = host_lock.lock_owned().await;

        // Next, under the exclusive access to the host, ensure
        // delay since the moment the host was freed.
        if let Some(last_freed) = host_guard.last_freed
            && Instant::now() < last_freed + self.delay
        {
            tokio::time::sleep_until((last_freed + self.delay).into()).await;
        }

        let res = next.run(req, extensions).await;

        host_guard.last_freed = Some(Instant::now());

        res
    }
}
