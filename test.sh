# soroban config identity generate admin
# zingzongzung@zingzonzungsMBP message % soroban config identity fund admin --network testnet
ASSET_ADDRESS=$(soroban lab token id --asset native --network testnet)
echo "Stellar address " $ASSET_ADDRESS
soroban contract build
CONTRACT_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/skirmish11.wasm --source rpciege --network testnet )
echo $CONTRACT_ID
 
 
teste_messages(){ 
	echo "Running all commands" 

	soroban contract invoke --id $CONTRACT_ID --source admin --network testnet -- initialize --admin $(soroban config identity address admin)  --asset_address $ASSET_ADDRESS
	
	
	soroban contract invoke --id $CONTRACT_ID --source rpciege --network testnet -- send_message --from $(soroban config identity address rpciege) --to $(soroban config identity address user_a)  --message 48656c6c6f207468657265
	soroban contract invoke --id $CONTRACT_ID --source user_a --network testnet -- send_message --from $(soroban config identity address user_a) --to $(soroban config identity address rpciege)  --message 48656c6c6f207468657265
	soroban contract invoke --id $CONTRACT_ID --source user_a --network testnet -- send_donation --from $(soroban config identity address user_a) --to $(soroban config identity address rpciege)  --donation_amount 123

	soroban contract invoke --id $CONTRACT_ID --source rpciege --network testnet -- get_messages --origin $(soroban config identity address rpciege) --target $(soroban config identity address user_a)   
	soroban contract invoke --id $CONTRACT_ID --source rpciege --network testnet -- get_contacts --user_address $(soroban config identity address user_a) 
	
}

init(){ 
	echo "Init" 

	soroban contract invoke --id $CONTRACT_ID --source admin --network testnet -- initialize --admin $(soroban config identity address admin)  --asset_address $ASSET_ADDRESS	
}


 init
