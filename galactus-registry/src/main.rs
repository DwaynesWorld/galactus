#[macro_use]
extern crate lazy_static;

pub mod proto {
    tonic::include_proto!("config.v1");
}

use async_once::AsyncOnce;
use proto::configuration_server::{Configuration, ConfigurationServer};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

use galactus_core::session::{create_session, TcpSession};
use galactus_core::{ConfigEntry, ConfigEntryStore};

lazy_static! {
    static ref SESSION: AsyncOnce<Arc<TcpSession>> = AsyncOnce::new(async {
        let session = create_session().await;
        Arc::new(session)
    });
}

async fn new_session() -> Arc<TcpSession> {
    SESSION.get().await.clone()
}

#[derive(Debug, Default)]
pub struct ConfigurationEndpoints;

#[tonic::async_trait]
impl Configuration for ConfigurationEndpoints {
    async fn list_configs(
        &self,
        request: Request<proto::ListConfigsRequest>,
    ) -> Result<Response<proto::ListConfigsResponse>, Status> {
        let mut store = ConfigEntryStore::new(new_session().await);
        let request = request.into_inner();
        let kind = request.kind.try_into().expect("kind is invalid");

        let configs = store
            .list(kind)
            .await
            .expect("error querying db")
            .iter()
            .map(|c| proto::ConfigEntry {
                kind: c.kind.clone() as i32,
                name: c.name.clone(),
                metadata: c.meta.clone(),
                created_at: c.created_at.timestamp_millis() as u64,
                updated_at: c.updated_at.timestamp_millis() as u64,
            })
            .collect::<Vec<_>>();

        Ok(Response::new(proto::ListConfigsResponse {
            configs,
            next_page_token: "".to_string(),
        }))
    }

    async fn get_config(
        &self,
        request: Request<proto::GetConfigRequest>,
    ) -> Result<Response<proto::ConfigEntry>, Status> {
        let mut store = ConfigEntryStore::new(new_session().await);
        let request = request.into_inner();
        let kind = request.kind.try_into().expect("kind is invalid");

        let config = store
            .get(kind, &request.name)
            .await
            .expect("error querying db");

        if let Some(config) = config {
            let response = Response::new(proto::ConfigEntry {
                kind: config.kind as i32,
                name: config.name,
                metadata: config.meta,
                created_at: config.created_at.timestamp_millis() as u64,
                updated_at: config.updated_at.timestamp_millis() as u64,
            });

            Ok(response)
        } else {
            Err(Status::not_found("unable to find config"))
        }
    }

    async fn create_config(
        &self,
        request: Request<proto::CreateConfigRequest>,
    ) -> Result<Response<proto::ConfigEntry>, Status> {
        let mut store = ConfigEntryStore::new(new_session().await);
        let request = request.into_inner();
        let kind = request.kind.try_into().expect("kind is invalid");
        let config = ConfigEntry::new(kind, request.name, request.metadata).unwrap();
        let result = store.insert(config.clone()).await;

        if result.is_ok() {
            let response = Response::new(proto::ConfigEntry {
                kind: config.kind as i32,
                name: config.name,
                metadata: config.meta,
                created_at: config.created_at.timestamp_millis() as u64,
                updated_at: config.updated_at.timestamp_millis() as u64,
            });

            Ok(response)
        } else {
            Err(Status::failed_precondition("error occurred during create"))
        }
    }

    async fn update_config(
        &self,
        request: Request<proto::UpdateConfigRequest>,
    ) -> Result<Response<proto::ConfigEntry>, Status> {
        let mut store = ConfigEntryStore::new(new_session().await);
        let request = request.into_inner();
        let kind = request.kind.try_into().expect("kind is invalid");

        let config = store
            .get(kind, &request.name)
            .await
            .expect("error querying db");

        if config.is_none() {
            return Err(Status::not_found("unable to find config"));
        }

        let mut config = config.unwrap();
        config.meta = request.metadata;

        let result = store.update(config.clone()).await;
        if result.is_err() {
            return Err(Status::failed_precondition("error occurred during create"));
        }

        let response = Response::new(proto::ConfigEntry {
            kind: config.kind as i32,
            name: config.name,
            metadata: config.meta,
            created_at: config.created_at.timestamp_millis() as u64,
            updated_at: config.updated_at.timestamp_millis() as u64,
        });

        Ok(response)
    }

    async fn delete_config(
        &self,
        request: Request<proto::DeleteConfigRequest>,
    ) -> Result<Response<()>, Status> {
        let mut store = ConfigEntryStore::new(new_session().await);
        let request = request.into_inner();
        let kind = request.kind.try_into().expect("kind is invalid");

        store
            .remove(kind, &request.name)
            .await
            .expect("error deleting config");

        Ok(Response::new(()))
    }
}

fn make_configuration_service() -> ConfigurationServer<ConfigurationEndpoints> {
    ConfigurationServer::new(ConfigurationEndpoints::default())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Registry Server listening on {}", addr);

    Server::builder()
        .add_service(make_configuration_service())
        .serve(addr)
        .await?;

    Ok(())
}
