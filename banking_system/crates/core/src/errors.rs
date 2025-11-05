use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Repo error: {0}")]
    Repo(#[from] RepoError),
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("deposit error: {0}")]
    ClosedAccount(String),
    #[error("deposit error: {0}")]
    NegativeAmount(String),
    #[error("deposit error: {0}")]
    FrozenAccount(String),
    // #[error("Libp2p noise error: {0}")]
    // Libp2pNoise(#[from] libp2p::noise::Error),
    // #[error("Libp2p swarm builder error: {0}")]
    // Libp2pSwarmBuilder(String),
    // #[error("Parsing Libp2p multiaddress error: {0}")]
    // Libp2pMultiAddrParse(#[from] multiaddr::Error),
    // #[error("Libp2p Transport error: {0}")]
    // Libp2pTransport(#[from] TransportError<io::Error>),
    // #[error("Libp2p gossipsub subscription error: {0}")]
    // Libp2pGossipsubSubscription(#[from] SubscriptionError),
}

#[derive(Debug, Error)]
pub enum RepoError {}
