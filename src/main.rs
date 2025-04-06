use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

fn main() {
    // Create a new RPC client connected to the Solana devnet
    let rpc_url = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new(rpc_url.to_string());

    // Define the public key of the account
    let pubkey_str = "FXvW9kSD5qbUkEuZuZ8Pz2PEhzusmpTrj7HumyYpdTLy";
    let pubkey = Pubkey::from_str(pubkey_str).unwrap();

    // Fetch the account balance in lamports
    match rpc_client.get_balance(&pubkey) {
        Ok(balance) => {
            println!("Balance: {} lamports", balance);
        }
        Err(err) => {
            eprintln!("Error fetching balance: {}", err);
        }
    }

    // Fetch the minimum balance for rent exemption
    let data_length = 0; // Adjust the data length as needed
    match rpc_client.get_minimum_balance_for_rent_exemption(data_length) {
        Ok(min_balance) => {
            println!("Minimum balance for rent exemption: {} lamports", min_balance);
        }
        Err(err) => {
            eprintln!("Error fetching minimum balance for rent exemption: {}", err);
        }
    }
}
