#![allow(
    clippy::too_many_arguments,
    clippy::new_without_default,
    non_camel_case_types,
    unused_imports
)]
/*
 * Mayastor RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

use crate::apis::IntoVec;

/// VolumeState : Runtime state of the volume

/// Runtime state of the volume
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VolumeState {
    /// array of children nexuses
    #[serde(rename = "children")]
    pub children: Vec<crate::models::Nexus>,
    #[serde(rename = "protocol")]
    pub protocol: crate::models::Protocol,
    /// size of the volume in bytes
    #[serde(rename = "size")]
    pub size: u64,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::VolumeStatus>,
    /// name of the volume
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl VolumeState {
    /// VolumeState using only the required fields
    pub fn new(
        children: impl IntoVec<crate::models::Nexus>,
        protocol: impl Into<crate::models::Protocol>,
        size: impl Into<u64>,
        uuid: impl Into<uuid::Uuid>,
    ) -> VolumeState {
        VolumeState {
            children: children.into_vec(),
            protocol: protocol.into(),
            size: size.into(),
            status: None,
            uuid: uuid.into(),
        }
    }
    /// VolumeState using all fields
    pub fn new_all(
        children: impl IntoVec<crate::models::Nexus>,
        protocol: impl Into<crate::models::Protocol>,
        size: impl Into<u64>,
        status: impl Into<Option<crate::models::VolumeStatus>>,
        uuid: impl Into<uuid::Uuid>,
    ) -> VolumeState {
        VolumeState {
            children: children.into_vec(),
            protocol: protocol.into(),
            size: size.into(),
            status: status.into(),
            uuid: uuid.into(),
        }
    }
}
