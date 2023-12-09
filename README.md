# Soroban Rust Smart Contract

This Rust smart contract is designed to provide messaging, donations, and contact management functionalities on the Soroban blockchain. The contract is implemented using the Soroban SDK.

## Prerequisites

- **Rust Programming Language**: Ensure that you have Rust installed on your development machine.
- **Soroban SDK**: The smart contract relies on the Soroban SDK for blockchain interactions. Make sure to include the `soroban_sdk` crate in your project dependencies.

## Overview

The smart contract defines a `Contract` struct and implements various methods to initialize the contract, update asset addresses, send donations, send messages, retrieve messages, and manage contacts.

### Contract Initialization

#### `initialize`

This method initializes the smart contract with the provided administrator and asset addresses. It checks if the contract has already been initialized and panics if it has.

### Update Asset Address

#### `update_asset_address`

This method allows the administrator to update the asset address. It verifies that the contract is initialized and that the caller is the administrator.

### Sending Donations and Messages

#### `send_donation`

This method facilitates the transfer of tokens (`donation_amount`) from one address (`from`) to another address (`to`). It also sends a donation message.

#### `send_message`

This method sends a text message from one address (`from`) to another address (`to`).

### Retrieving Messages and Contacts

#### `get_messages`

This method retrieves messages between the specified `origin` and `target` addresses.

#### `get_contacts`

This method retrieves the contact list for a given user.

## Internal Functions

The smart contract also defines several internal functions used to manage messages and contacts. These functions include:

- `get_storage_messages`: Retrieves stored messages between two addresses.
- `get_storage_contacts`: Retrieves the contact list for a given user.
- `add_contact`: Adds a contact to the user's contact list.
- `add_message`: Adds a message to the storage.

## Usage Example

Here's a brief example of how to use the smart contract:

```rust
// Initialize the contract
Contract::initialize(env, admin_address, asset_address);

// Update asset address
Contract::update_asset_address(env, admin_address, new_asset_address);

// Send a donation
Contract::send_donation(env, from_address, to_address, donation_amount);

// Send a text message
Contract::send_message(env, from_address, to_address, text_message);

// Retrieve messages
let messages = Contract::get_messages(env, user1_address, user2_address);

// Retrieve contacts
let contacts = Contract::get_contacts(env, user_address);
```