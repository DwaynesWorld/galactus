use std::collections::HashMap;
use tonic::{transport::Server, Request, Response, Status};

use config::configuration_server::{Configuration, ConfigurationServer};
use config::{
    ConfigEntry, CreateConfigRequest, DeleteConfigRequest, EntryKind, GetConfigRequest,
    ListConfigsRequest, ListConfigsResponse, UpdateConfigRequest,
};

pub mod config {
    tonic::include_proto!("config.v1");
}

#[derive(Debug, Default)]
pub struct ConfigurationHandlers {}

#[tonic::async_trait]
impl Configuration for ConfigurationHandlers {
    async fn list_configs(
        &self,
        request: Request<ListConfigsRequest>,
    ) -> Result<Response<ListConfigsResponse>, Status> {
        let _ = request.into_inner();
        todo!()
    }

    async fn get_config(
        &self,
        request: Request<GetConfigRequest>,
    ) -> Result<Response<ConfigEntry>, Status> {
        let request = request.into_inner();

        let response = Response::new(ConfigEntry {
            kind: EntryKind::Kafka.into(),
            name: request.name,
            create_time: 1,
            modify_time: 1,
            metadata: HashMap::new(),
        });

        Ok(response)
    }

    async fn create_config(
        &self,
        request: Request<CreateConfigRequest>,
    ) -> Result<Response<ConfigEntry>, Status> {
        let request = request.into_inner();

        let response = Response::new(ConfigEntry {
            kind: request.kind,
            name: request.name,
            create_time: 1,
            modify_time: 1,
            metadata: request.metadata,
        });

        Ok(response)
    }

    async fn update_config(
        &self,
        request: Request<UpdateConfigRequest>,
    ) -> Result<Response<ConfigEntry>, Status> {
        let request = request.into_inner();

        let response = Response::new(ConfigEntry {
            kind: request.kind,
            name: request.name,
            create_time: 1,
            modify_time: 1,
            metadata: HashMap::new(),
        });

        Ok(response)
    }

    async fn delete_config(
        &self,
        request: Request<DeleteConfigRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();

        println!(
            "Deleting config entry with kind '{}' and name '{}'",
            request.kind, request.name
        );

        Ok(Response::new(()))
    }
}

fn make_configuration_service() -> ConfigurationServer<ConfigurationHandlers> {
    ConfigurationServer::new(ConfigurationHandlers::default())
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
