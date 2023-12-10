# Soroban chat

Soroban Chat is a messaging and donation Dapp built with Soroban and Outsystems. With it you can start a chat from your wallet to any other wallet and it has the ability to donate some Tokens to that wallet.
To use it you have to connect to it using 

## Prerequisites

- **Rust Programming Language**: Ensure that you have Rust installed on your development machine.
- **Soroban SDK**: The smart contract relies on the Soroban SDK for blockchain interactions. Make sure to include the `soroban_sdk` crate in your project dependencies.
- **Soroban Cli** - You will need the `soroban_cli` in order to build and deploy the contract
- **Outsystems Service Studio**: The web app is built with Outsystems, so you will need to download service studio in order to edit it
- **Freighter Wallet** - Wallet extension for interact with the app.



## Smart Contract (Soroban)

The smart contract defines a messaging system and implements various methods to initialize the contract, update asset addresses, send donations, send messages, retrieve messages, and get contacts.

### Prepare and deploy the contract

1. Clone the repository:
```
git clone https://github.com/zingzongzung/soroban-chat.git
```

2. Build the contract:
```
soroban contract build
```

3. Config the network and an account:
```
soroban config network add --rpc-url https://rpc-futurenet.stellar.org:443 --network-passphrase "Test SDF Future Network ; October 2022" futurenet
soroban config identity generate admin
soroban config identity fund admin --network futurenet
```

3. Deploy the contract:
```
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/soroban_chat.wasm --source user_b --network futurenet
```

4. Get the native XLM address on the given network and save it for later:
```
soroban lab token id --asset native --network futurenet
```

5. If you want to use the contract directly you can generate multiple extra accounts:
```
soroban config identity generate user_a
soroban config identity fund user_a --network futurenet
```

### Contract Initialization

#### `initialize`

This method initializes the smart contract with the provided administrator and asset addresses. It checks if the contract has already been initialized and panics if it has.

```
soroban contract invoke --id $CONTRACT_ID --source admin --network futurenet -- initialize --admin $(soroban config identity address admin)  --asset_address $ASSET_ADDRESS
```

### Update Asset Address

#### `update_asset_address`

This method allows the administrator to update the asset address. It verifies that the contract is initialized and that the caller is the administrator.
This allows the admin to make changes to the token address.

```
soroban contract invoke --id $CONTRACT_ID --source admin --network futurenet -- update_asset_address --admin $(soroban config identity address admin)  --asset_address $NEW_ASSET_ADDRESS
```

### Sending Donations and Messages

#### `send_donation`

This method facilitates the transfer of tokens (`donation_amount`) from one address (`from`) to another address (`to`). 
Message must be an hexadecimal value, you can use an [online tool](https://codebeautify.org/string-hex-converter)  to convert your text to the required value

```
soroban contract invoke --id $CONTRACT_ID --source user_a --network futurenet -- send_donation --from $(soroban config identity address user_a) --to $(soroban config identity address user_b)  --donation_amount 123
```

#### `send_message`

This method sends a text message from one address (`from`) to another address (`to`).
Message must be an hexadecimal value, you can use an [online tool](https://codebeautify.org/string-hex-converter)  to convert your text to the required value

```
soroban contract invoke --id $CONTRACT_ID --source user_b --network futurenet -- send_message --from $(soroban config identity address user_b) --to $(soroban config identity address user_a)  --message 48656c6c6f207468657265
```

### Retrieving Messages and Contacts

#### `get_messages`

This method retrieves messages between the specified `origin` and `target` addresses.

```
soroban contract invoke --id $CONTRACT_ID --source user_b --network futurenet -- get_messages --origin $(soroban config identity address user_b) --target $(soroban config identity address user_a)   
```

#### `get_contacts`

This method retrieves the contact list for a given user.

```
soroban contract invoke --id $CONTRACT_ID --source user_b --network futurenet -- get_contacts --user_address $(soroban config identity address user_a) 
```

## Internal Functions

The smart contract also defines several internal functions used to manage messages and contacts. These functions include:

- `get_storage_messages`: Retrieves stored messages between two addresses.
- `get_storage_contacts`: Retrieves the contact list for a given user.
- `add_contact`: Adds a contact to the user's contact list.
- `add_message`: Adds a message to the storage.



## Outsystems

Outsystems is a low code platform that allows you to create mobile and web applications. Soroban chat is a web based application build with outsystems.

With this implementation we show an alternative way to build web applications that integrate with Soroban. 

At this moment all interactions are calling some actions on the Soroban Chat Outsystems module.


The web app integrates with the freighter wallet via [JS](https://docs.freighter.app/docs/)

For comunicating with Soroban Network it uses [js-stellar-sdk](https://github.com/stellar/js-stellar-sdk)

### Prepare the web app

1. Create an [Outsystems account](https://www.outsystems.com/Platform/Signup)
2. Download [Service Studio](https://www.outsystems.com/downloads/)
3. Get [source code](https://www.outsystems.com/forge/component-overview/8431/gtree)
4. Publish the code

### Configure Outsystems properties

1. Configure network and contract

Go to Outsystems service center, find the module and configure the following site properties:

`RPCUrl` Set it to the RPC URL of the network you deployed the contract to
`NetworkPassPhrase` Set it to the corresponding Network PassPhrase
`ContractId` Set it to the contract id that you got in the deploy

2. In order to get chat realtime messages the app is using Firebase Realtime Database to send/receive events, for that reason you need to create and configure a Firebase Developer account. You can follow steps at this [guide](https://www.outsystems.com/forge/component-documentation/1406/firebase/0) to help you through that process.
After that you can configure the following site properties:


