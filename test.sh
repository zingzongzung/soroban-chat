# soroban config identity generate admin
# zingzongzung@zingzonzungsMBP message % soroban config identity fund admin --network testnet
ASSET_ADDRESS=$(soroban lab token id --asset native --network futurenet)
echo "Stellar address " $ASSET_ADDRESS
soroban contract build
CONTRACT_ID=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/soroban_chat.wasm --source user_b --network futurenet )
echo $CONTRACT_ID

soroban contract invoke --id CBGFLK7ENPKPKNW7UQVWZFZHCVZVCY37VXVARVEFZ52V55BPFJ3ZHU7D --source user_b --network futurenet -- send_message --from $(soroban config identity address user_b) --to GCBELBPTCGSGFT3D2Q53CAQTEA25RDTNOX4I6OCNVMLGYZ4B4TM3VZZH  --message 48656c6c6f207468657265
	
 
 
teste_messages(){ 
	echo "Running all commands" 

	soroban contract invoke --id $CONTRACT_ID --source admin --network futurenet -- initialize --admin $(soroban config identity address admin)  --asset_address $ASSET_ADDRESS
	
	
	soroban contract invoke --id $CONTRACT_ID --source user_b --network futurenet -- send_message --from $(soroban config identity address user_b) --to $(soroban config identity address user_a)  --message 48656c6c6f207468657265
	soroban contract invoke --id $CONTRACT_ID --source user_a --network futurenet -- send_message --from $(soroban config identity address user_a) --to $(soroban config identity address user_b)  --message 48656c6c6f207468657265
	soroban contract invoke --id $CONTRACT_ID --source user_a --network futurenet -- send_donation --from $(soroban config identity address user_a) --to $(soroban config identity address user_b)  --donation_amount 123

	soroban contract invoke --id $CONTRACT_ID --source user_b --network futurenet -- get_messages --origin $(soroban config identity address user_b) --target $(soroban config identity address user_a)   
	soroban contract invoke --id $CONTRACT_ID --source user_b --network futurenet -- get_contacts --user_address $(soroban config identity address user_a) 
	
}

init(){ 
	echo "Init" 

	soroban contract invoke --id $CONTRACT_ID --source admin --network futurenet -- initialize --admin $(soroban config identity address admin)  --asset_address $ASSET_ADDRESS	
}

	soroban contract invoke --id CBGFLK7ENPKPKNW7UQVWZFZHCVZVCY37VXVARVEFZ52V55BPFJ3ZHU7D --source user_b --network futurenet -- get_messages --origin $(soroban config identity address user_b) --target GCBELBPTCGSGFT3D2Q53CAQTEA25RDTNOX4I6OCNVMLGYZ4B4TM3VZZH

 init
