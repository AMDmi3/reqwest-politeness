// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::time::{Duration, Instant};

use reqwest_politeness::ReqwestPoliteness;

#[tokio::test]
async fn sequential() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("GET", "/").with_status(200).expect(3).create();

    let client = reqwest::Client::builder().build().unwrap();
    let client = reqwest_middleware::ClientBuilder::new(client)
        .with(ReqwestPoliteness::default().with_delay(Duration::from_millis(100)))
        .build();

    let start = Instant::now();

    client.get(server.url()).send().await.unwrap();
    client.get(server.url()).send().await.unwrap();
    client.get(server.url()).send().await.unwrap();

    assert!(start.elapsed() > Duration::from_millis(200));
    mock.assert();
}

#[tokio::test]
async fn parallel() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("GET", "/").with_status(200).expect(3).create();

    let client = reqwest::Client::builder().build().unwrap();
    let client = reqwest_middleware::ClientBuilder::new(client)
        .with(ReqwestPoliteness::default().with_delay(Duration::from_millis(100)))
        .build();

    let start = Instant::now();

    let _ = tokio::join!(
        client.get(server.url()).send(),
        client.get(server.url()).send(),
        client.get(server.url()).send(),
    );

    assert!(start.elapsed() > Duration::from_millis(200));
    mock.assert();
}
