use soroban_sdk::{contracttype,  Address, Bytes , contracterror };

#[contracttype]
#[derive(Clone)]
pub struct Message {
    pub timestamp: u64,
    pub message: Bytes,
    pub donation_amount: i128, 
    pub message_type: MessageType
}

#[contracttype]
#[derive(Clone)]
pub struct ChatMessage {
    pub message: Message,
    pub is_sent: bool
}

#[contracttype]
pub enum Key {
    Contacts(Address),
    Messsages(Address,Address),
    MessagesCount(Address,Address)
}

#[derive(Clone)]
#[contracttype]
pub enum MessageType {
    Donation,
    Text
}

#[derive(Clone)]
#[contracttype]
pub enum ContractProperties {
    Admin,
    AssetAddress
}

#[contracterror]
#[derive(Copy, Clone, Debug)]
pub enum Error {
    NotInitialized = 0,
    AlreadyInitialized = 1,
    NotAdmin = 2
}
 