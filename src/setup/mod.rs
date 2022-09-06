use bitcoincore_rpc::{Client, RpcApi};
use electrum_client::ElectrumApi;

pub fn setup(){
	  println!("Hello, world!");
    let client= Client::new("http://127.0.0.1:18443",
        bitcoincore_rpc::Auth::UserPass("polaruser".to_owned(),"polarpass".to_owned())).unwrap();

    let electrum =electrum_client::Client::new("ssl://electrum.blockstream.info:60002").unwrap();

    println!("local connection block count: {}", client.get_block_count().unwrap());
    println!("electrum connection fee: {}", electrum.relay_fee().unwrap());
}