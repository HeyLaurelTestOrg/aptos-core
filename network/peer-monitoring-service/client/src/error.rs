// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_network::protocols::network::RpcError;
use aptos_peer_monitoring_service_types::{PeerMonitoringServiceError, UnexpectedResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Error from remote monitoring service: {0}")]
    PeerMonitoringServiceError(#[from] PeerMonitoringServiceError),

    #[error("Aptos network rpc error: {0}")]
    RpcError(#[from] RpcError),

    #[error("Unexpected error encountered: {0}")]
    UnexpectedError(String),
}

impl Error {
    /// Returns a summary label for the error
    pub fn get_label(&self) -> &'static str {
        match self {
            Self::NetworkError(_) => "network_error",
            Self::PeerMonitoringServiceError(_) => "peer_monitoring_service_error",
            Self::RpcError(_) => "rpc_error",
            Self::UnexpectedError(_) => "unexpected_error",
        }
    }
}

impl From<aptos_network::application::error::Error> for Error {
    fn from(error: aptos_network::application::error::Error) -> Self {
        Error::NetworkError(error.to_string())
    }
}

impl From<UnexpectedResponseError> for Error {
    fn from(error: UnexpectedResponseError) -> Self {
        Error::UnexpectedError(error.to_string())
    }
}
