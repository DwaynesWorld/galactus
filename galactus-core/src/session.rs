use cdrs_tokio::cluster::session::{Session, SessionBuilder, TcpSessionBuilder};
use cdrs_tokio::cluster::{NodeTcpConfigBuilder, TcpConnectionManager};
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::transport::TransportTcp;

pub type TcpSession = Session<
    TransportTcp,
    TcpConnectionManager,
    RoundRobinLoadBalancingStrategy<TransportTcp, TcpConnectionManager>,
>;

pub async fn create_session() -> TcpSession {
    let config = NodeTcpConfigBuilder::new()
        .with_contact_point("localhost:9042".into())
        .build()
        .await
        .unwrap();

    TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), config).build()
}
