use bitcoincore_rpc::{Client, RpcApi};
use spending_paths::{p2wpkh::p2wpkh, p2tr::p2tr};

mod spending_paths;
fn main() {
let seed="1d454c6ab705f999d97e6465300a79a9595fb5ae1186ae20e33e12bea606c094";
    p2tr(Some( seed ));
    // p2wpkh(Some( seed ));
}
