#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec, Address, Bytes , token, panic_with_error };

mod types;
use crate::types::*;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    pub fn initialize(env: Env, admin: Address, asset_address: Address) {
        admin.require_auth();
        if env.storage().instance().has(&ContractProperties::Admin) {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        env.storage().instance().set(&ContractProperties::Admin, &admin);
        env.storage().instance().set(&ContractProperties::AssetAddress, &asset_address);
    }

    pub fn update_asset_address(env: Env, admin: Address, asset_address: Address){
        admin.require_auth();
        if !env.storage().instance().has(&ContractProperties::Admin) {
            panic_with_error!(&env, Error::NotInitialized);
        }

        let stored_admin : Address = env.storage().instance().get(&ContractProperties::Admin).unwrap();

        if stored_admin != admin {
            panic_with_error!(&env, Error::NotInitialized);
        }

        env.storage().instance().set(&ContractProperties::AssetAddress, &asset_address);

    }
 
    pub fn send_donation(env: Env, from: Address, to: Address, donation_amount: i128) -> bool { 
        from.require_auth();  
        if !env.storage().instance().has(&ContractProperties::Admin) {
            panic_with_error!(&env, Error::NotInitialized);
        }
  
        let asset_address : Address = env.storage().instance().get(&ContractProperties::AssetAddress).unwrap();
        token::Client::new(&env, &asset_address).transfer(&from, &to, &donation_amount);
 
        let text = Bytes::new(&env);
        send_message(env, from, to,text  ,donation_amount , MessageType::Donation)
    }

    pub fn send_message(env: Env, from: Address, to: Address, message: Bytes) -> bool { 
        from.require_auth();

        send_message(env, from, to, message, 0,MessageType::Text)
    }

    pub fn get_messages(env: Env,  origin: Address, target: Address) -> Vec<ChatMessage>  {
        origin.require_auth();
        let mut result: Vec<ChatMessage> = Vec::new(&env);
  
        let sent_messages = get_storage_messages(&env, origin.clone(), target.clone());  
        let received_messages = get_storage_messages(&env, target.clone(), origin.clone());
        
        received_messages.iter().for_each(|message| result.push_back(ChatMessage { message , is_sent: false}));
        sent_messages.iter().for_each(|message| result.push_back(ChatMessage { message , is_sent: true}));
        

        result
    }

    pub fn get_contacts(env: Env, user_address: Address) -> Vec<Address>{
        let contacts: Vec<Address> = get_storage_contacts(&env, user_address.clone());

        contacts
    }
 
    

   

}

fn get_storage_messages(env: &Env, from: Address, to: Address)  -> Vec<Message>{
    let messages: Vec<Message>;
    if env.storage().instance().has(&Key::Messsages(from.clone(), to.clone())) {
        messages = env.storage().instance().get(&Key::Messsages(from.clone(), to.clone())).unwrap();
    } else {
        messages = Vec::new(&env);
    } 

    messages
}

fn get_storage_contacts(env: &Env, user_address: Address) -> Vec<Address>{
    let contacts: Vec<Address>;
    if env.storage().instance().has(&Key::Contacts(user_address.clone())) {
        contacts = env.storage().instance().get(&Key::Contacts(user_address.clone())).unwrap();
    } else {
        contacts = Vec::new(&env);
    }

    contacts
}

fn add_contact(env: &Env, user_a: Address, user_b: Address) {
    let mut contacts: Vec<Address> = get_storage_contacts(&env, user_a.clone());
    let found = contacts.iter().any(|contact| contact == user_b);

    if !found {
        contacts.push_back(user_b.clone());

        env.storage().instance().set(&Key::Contacts(user_a.clone()), &contacts);
    }
    
}

fn add_message(env: &Env, from: Address, to:Address, message: Bytes,donation_amount: i128, message_type: MessageType){
    

    let mut messages: Vec<Message>;
    if env.storage().instance().has(&Key::Messsages(from.clone(), to.clone())) {
        messages = env.storage().instance().get(&Key::Messsages(from.clone(), to.clone())).unwrap();
    } else {
        messages = Vec::new(&env);
    }

    let message_struct = Message {
         timestamp: env.ledger().timestamp(),message, donation_amount, message_type
    };

    messages.push_back(message_struct);

    env.storage().instance().set(&Key::Messsages(from.clone(), to.clone()), &messages);
}
 
pub fn send_message(env: Env, from: Address, to: Address, message: Bytes , donation_amount: i128,  message_type: MessageType) -> bool {  

    add_contact(&env, from.clone(), to.clone());
    add_contact(&env, to.clone(), from.clone());

    add_message(&env, from, to, message, donation_amount, message_type);

    true
}