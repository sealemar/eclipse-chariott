// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.
// SPDX-License-Identifier: MIT

//! A simple consumer for a sample of Chariott Service Discovery.
//!
//! This consumer "discovers" the hello world service through the registry, and then
//! directly calls the SayHello method on it, using a known interface. This returns
//! a message containing "Hello, " followed by the string provided in the request.

// Tells cargo to warn if a doc comment is missing and should be provided.
#![warn(missing_docs)]

use samples_proto::hello_world::v1::hello_world_client::HelloWorldClient;
use samples_proto::hello_world::v1::HelloRequest;
use service_discovery_proto::service_registry::v1::service_registry_client::ServiceRegistryClient;
use service_discovery_proto::service_registry::v1::DiscoverRequest;
use tonic::Request;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

/// URL for the service registry
const SERVICE_REGISTRY_URL: &str = "http://0.0.0.0:50000";
/// Expected provider communication kind. Validate against this to ensure we can communicate with the service that the service registry returns
const EXPECTED_COMMUNICATION_KIND: &str = "grpc+proto";
/// Expected provider communication reference. Validate against this to ensure we can communicate with the service that the service registry returns
const EXPECTED_COMMUNICATION_REFERENCE: &str = "hello_world_service.v1.proto";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up tracing
    let collector = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(tracing::Level::INFO.into())
                .from_env_lossy(),
        )
        .finish();

    collector.init();

    // Create a registry client
    let mut service_registry_client = ServiceRegistryClient::connect(SERVICE_REGISTRY_URL).await?;
    let discover_request = Request::new(DiscoverRequest {
        namespace: String::from("sdv.samples"),
        name: String::from("hello-world"),
        version: String::from("1.0.0.0"),
    });

    // Discover the simple provider service
    let service_option =
        service_registry_client.discover(discover_request).await?.into_inner().service;
    match service_option {
        Some(service) => {
            info!("Discovered service {service:?}");
            if service.communication_kind != EXPECTED_COMMUNICATION_KIND
                || service.communication_reference != EXPECTED_COMMUNICATION_REFERENCE
            {
                return Err("Simple Discover Consumer does not recognize communication_kind or communication_reference of provider; cannot communicate")?;
            }

            // Call the provider application directly, since we recognize the communication kind and reference
            let mut provider_client = HelloWorldClient::connect(service.uri).await?;
            let hello_request = Request::new(HelloRequest { name: String::from("World") });
            let hello_response = provider_client.say_hello(hello_request).await?.into_inner();
            info!(hello_response.message);
        }
        None => info!("No service found."),
    };
    Ok(())
}
