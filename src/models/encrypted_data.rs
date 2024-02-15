use std::str::FromStr;

use bincode::{deserialize, serialize};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use snarkvm::prelude::{Address, Ciphertext, Group, Network};
use uuid::Uuid;

use crate::errors::{AvError, AvailResult};

use super::traits::encryptable::EncryptedStruct;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptedDataTypeCommon {
    Record,
    TransactionMessage,
    Transition,
    Transaction,
    Deployment,
}

impl EncryptedDataTypeCommon {
    pub fn to_str(&self) -> &'static str {
        match self {
            EncryptedDataTypeCommon::Record => "record",
            EncryptedDataTypeCommon::TransactionMessage => "transaction_message",
            EncryptedDataTypeCommon::Transaction => "transaction",
            EncryptedDataTypeCommon::Transition => "transition",
            EncryptedDataTypeCommon::Deployment => "deployment",
        }
    }
}

impl From<&str> for EncryptedDataTypeCommon {
    fn from(s: &str) -> Self {
        match s {
            "record" => EncryptedDataTypeCommon::Record,
            "transaction_message" => EncryptedDataTypeCommon::TransactionMessage,
            "transaction" => EncryptedDataTypeCommon::Transaction,
            "transition" => EncryptedDataTypeCommon::Transition,
            "deployment" => EncryptedDataTypeCommon::Deployment,
            _ => EncryptedDataTypeCommon::Record,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncryptedDataRecord {
    pub id: Option<Uuid>,
    pub owner: String,
    pub ciphertext: String,
    pub nonce: String,
    pub flavour: EncryptedDataTypeCommon,
    pub network: String,
    pub synced_on: Option<DateTime<Utc>>,
}

impl EncryptedDataRecord {
    pub fn new(
        id: Option<Uuid>,
        owner: String,
        ciphertext: String,
        nonce: String,
        flavour: EncryptedDataTypeCommon,
        network: String,
        synced_on: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            owner,
            ciphertext,
            nonce,
            flavour,
            network,
            synced_on,
        }
    }

    pub fn to_enrypted_struct<N: Network>(&self) -> AvailResult<EncryptedStruct<N>> {
        let ciphertext = Ciphertext::<N>::from_str(&self.ciphertext)?;
        let nonce = Group::<N>::from_str(&self.nonce)?;
        let encrypted_for = Address::<N>::from_str(&self.owner)?;

        let encrypted_struct = EncryptedStruct::new(ciphertext, encrypted_for, nonce);

        Ok(encrypted_struct)
    }
}

impl From<EncryptedData> for EncryptedDataRecord {
    fn from(data: EncryptedData) -> Self {
        Self {
            id: data.id,
            owner: data.owner,
            ciphertext: data.ciphertext,
            nonce: data.nonce,
            flavour: data.flavour,
            network: data.network,
            synced_on: data.synced_on,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub id: Option<Uuid>,
    pub owner: String,
    pub ciphertext: String,
    pub nonce: String,
    pub flavour: EncryptedDataTypeCommon,
    pub record_type: Option<RecordTypeCommon>,

    //Json string of program ids array
    pub program_ids: Option<String>,
    //Json string of function ids array
    pub function_ids: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub synced_on: Option<DateTime<Utc>>,
    pub network: String,
    pub record_name: Option<String>,
    pub spent: Option<bool>,
    pub event_type: Option<EventTypeCommon>,
    pub record_nonce: Option<String>,
    pub transaction_state: Option<TransactionState>,
}

impl EncryptedData {
    pub fn new(
        id: Option<Uuid>,
        owner: String,
        ciphertext: String,
        nonce: String,
        flavour: EncryptedDataTypeCommon,
        record_type: Option<RecordTypeCommon>,
        program_ids: Option<String>,
        function_ids: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
        synced_on: Option<DateTime<Utc>>,
        network: String,
        record_name: Option<String>,
        spent: Option<bool>,
        event_type: Option<EventTypeCommon>,
        record_nonce: Option<String>,
        transaction_state: Option<TransactionState>,
    ) -> Self {
        Self {
            id,
            owner,
            ciphertext,
            nonce,
            flavour,
            record_type,
            program_ids,
            function_ids,
            created_at,
            updated_at,
            synced_on,
            network,
            record_name,
            spent,
            event_type,
            record_nonce,
            transaction_state,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, AvError> {
        let bytes = serialize(&self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, AvError> {
        let data = deserialize(&bytes)?;
        Ok(data)
    }

    pub fn to_enrypted_struct<N: Network>(&self) -> AvailResult<EncryptedStruct<N>> {
        let ciphertext = Ciphertext::<N>::from_str(&self.ciphertext)?;
        let nonce = Group::<N>::from_str(&self.nonce)?;
        let encrypted_for = Address::<N>::from_str(&self.owner)?;

        let encrypted_struct = EncryptedStruct::new(ciphertext, encrypted_for, nonce);

        Ok(encrypted_struct)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedDataUpdateRequest {
    pub id: Uuid,
    pub ciphertext: String,
    pub nonce: String,
}

impl EncryptedDataUpdateRequest {
    pub fn new(id: Uuid, ciphertext: String, nonce: String) -> Self {
        Self {
            id,
            ciphertext,
            nonce,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, AvError> {
        let bytes = serialize(&self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, AvError> {
        let data = deserialize(&bytes)?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedDataSyncRequest {
    pub owner: String,
    pub last_sync: i64,
}

impl EncryptedDataSyncRequest {
    pub fn new(owner: String, last_sync: i64) -> Self {
        Self { owner, last_sync }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, AvError> {
        let bytes = serialize(&self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, AvError> {
        let data = deserialize(&bytes)?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecordTypeCommon {
    AleoCredits,
    NFT,
    Contracts,
    Tokens,
    None,
}

impl RecordTypeCommon {
    pub fn to_str(&self) -> &'static str {
        match self {
            RecordTypeCommon::AleoCredits => "aleo_credits",
            RecordTypeCommon::NFT => "nft",
            RecordTypeCommon::Contracts => "contracts",
            RecordTypeCommon::Tokens => "tokens",
            RecordTypeCommon::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "aleo_credits" => Some(RecordTypeCommon::AleoCredits),
            "nft" => Some(RecordTypeCommon::NFT),
            "contracts" => Some(RecordTypeCommon::Contracts),
            "tokens" => Some(RecordTypeCommon::Tokens),
            "none" => Some(RecordTypeCommon::None),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventTypeCommon {
    Deploy,
    Execute,
    Send,
    Receive,
    Join,
    Split,
    Shield,
    Unshield,
}

impl EventTypeCommon {
    pub fn to_str(&self) -> &'static str {
        match self {
            EventTypeCommon::Deploy => "Deploy",
            EventTypeCommon::Execute => "Execute",
            EventTypeCommon::Send => "Send",
            EventTypeCommon::Receive => "Receive",
            EventTypeCommon::Join => "Join",
            EventTypeCommon::Split => "Split",
            EventTypeCommon::Shield => "Shield",
            EventTypeCommon::Unshield => "Unshield",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Deploy" => Some(EventTypeCommon::Deploy),
            "Execute" => Some(EventTypeCommon::Execute),
            "Send" => Some(EventTypeCommon::Send),
            "Receive" => Some(EventTypeCommon::Receive),
            "Join" => Some(EventTypeCommon::Join),
            "Split" => Some(EventTypeCommon::Split),
            "Shield" => Some(EventTypeCommon::Shield),
            "Unshield" => Some(EventTypeCommon::Unshield),
            _ => None,
        }
    }
}

// counts for things done in app from our services
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EventStatus {
    Creating,
    Pending,
    Settled,
    Failed,
}

impl EventStatus {
    pub fn to_transaction_state(&self) -> TransactionState {
        match self {
            EventStatus::Creating => TransactionState::Processing,
            EventStatus::Pending => TransactionState::Pending,
            EventStatus::Settled => TransactionState::Confirmed,
            EventStatus::Failed => TransactionState::Failed,
        }
    }
}

/// The state of an Aleo Transaction
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TransactionState {
    Processing,
    Pending,
    Confirmed,
    Failed,
    Rejected,
    Aborted,
    Cancelled,
}

impl TransactionState {
    pub fn to_event_status(&self) -> EventStatus {
        match self {
            TransactionState::Processing => EventStatus::Creating,
            TransactionState::Pending => EventStatus::Pending,
            TransactionState::Confirmed => EventStatus::Settled,
            TransactionState::Failed => EventStatus::Failed,
            TransactionState::Rejected => EventStatus::Failed,
            TransactionState::Aborted => EventStatus::Failed,
            TransactionState::Cancelled => EventStatus::Failed,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            TransactionState::Processing => "Processing".to_string(),
            TransactionState::Pending => "Pending".to_string(),
            TransactionState::Confirmed => "Confirmed".to_string(),
            TransactionState::Failed => "Failed".to_string(),
            TransactionState::Rejected => "Rejected".to_string(),
            TransactionState::Aborted => "Aborted".to_string(),
            TransactionState::Cancelled => "Cancelled".to_string(),
        }
    }

    pub fn from_str(state: &str) -> Option<Self> {
        match state {
            "Processing" => Some(TransactionState::Processing),
            "Pending" => Some(TransactionState::Pending),
            "Confirmed" => Some(TransactionState::Confirmed),
            "Failed" => Some(TransactionState::Failed),
            "Rejected" => Some(TransactionState::Rejected),
            "Aborted" => Some(TransactionState::Aborted),
            "Cancelled" => Some(TransactionState::Cancelled),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub record_pointers: Vec<EncryptedDataRecord>,
    pub transactions: Vec<EncryptedDataRecord>,
    pub transitions: Vec<EncryptedDataRecord>,
    pub deployments: Vec<EncryptedDataRecord>,
}

impl Data {
    pub fn new(
        record_pointers: Vec<EncryptedDataRecord>,
        transactions: Vec<EncryptedDataRecord>,
        transitions: Vec<EncryptedDataRecord>,
        deployments: Vec<EncryptedDataRecord>,
    ) -> Self {
        Self {
            record_pointers,
            transactions,
            transitions,
            deployments,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, AvError> {
        let bytes = serialize(&self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, AvError> {
        let data = deserialize(&bytes)?;
        Ok(data)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DataRequest {
    pub address: String,
    pub data: Data,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageRequest {
    pub page: i64
}