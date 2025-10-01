#[cfg(test)]
mod tests {
use solana_system_interface::instruction::transfer;
use solana_sdk::{
 hash::hash,
 message::Message,
 pubkey::Pubkey,
 signature::{Keypair, Signer, read_keypair_file},
 transaction::Transaction,
};

use std::str::FromStr;

 use solana_sdk;
 use bs58;
 use std::io::{self, BufRead};
 use solana_client::rpc_client::RpcClient;

 #[test]
 fn keygen() {
    let kp = Keypair::new();
    println!("You've generated Solana Private Key: {}\n",kp.pubkey());
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
 }

 #[test]
 fn base58_to_wallet() {
    println!("Input your private key as a base58 string:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file format is:");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
    println!("Input your private key as a JSON byte array (e.g.[12,34,...]):");
    let stdin = io::stdin();
    let wallet = stdin
    .lock()
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .trim_start_matches('[')
    .trim_end_matches(']')
    .split(',')
    .map(|s| s.trim().parse::<u8>().unwrap())
    .collect::<Vec<u8>>();
    println!("Your Base58-encoded private key is:");
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}



 
 #[test]
 fn claim_airdrop() {
    const RPC_URL: &str = "https://api.devnet.solana.com";
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    // we'll establish a connection to Solana devnet using the const we defined above
    let client = RpcClient::new(RPC_URL);
    // We're going to claim 2 devnet SOL tokens (2 billion lamports)
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
    Ok(sig) => {
    println!("Success! Check your TX here:");
    println!("https://explorer.solana.com/tx/{}?cluster=devnet",sig);
    }
    Err(err) => {
    println!("Airdrop failed: {}", err);
    }
    }

 }
 #[test]
 fn transfer_sol() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let pubkey = keypair.pubkey();
    let message_bytes = b"I verify my Solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());
    match sig.verify(&keypair.pubkey().as_ref(), message_bytes) {
        true => println!("Signature verified"),
        false => println!("Verification failed"),
    }
    let to_pubkey = Pubkey::from_str("7Q5iC1cj2ap3uxg2okKDU75xNSEKjh8fq4yvWY8faD9n").unwrap();
    const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
    let rpc_client = RpcClient::new(RPC_URL);
    //balances
    let balance = rpc_client
    .get_balance(&keypair.pubkey())
    .expect("Failed to get balance");
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    let fee = rpc_client
    .get_fee_for_message(&message)
    .expect("Failed to get fee calculator");


    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send final transaction");
        println!("Success! Entire balance transferred:https://explorer.solana.com/tx/{}/?cluster=devnet",signature);



 }
}
