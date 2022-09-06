use bitcoincore_rpc::{Client, RpcApi};
use electrum_client::ElectrumApi;
use setup::setup;

mod setup;

fn main() {
  setup();
}
