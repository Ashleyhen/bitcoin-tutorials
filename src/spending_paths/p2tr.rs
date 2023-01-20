use std::{str::FromStr, collections::BTreeMap};

use bitcoin::{
    blockdata::{opcodes::all, script::Builder},
    psbt::{Input, PartiallySignedTransaction, Prevouts},
    secp256k1::{Message, Scalar, Secp256k1, SecretKey, All},
    util::{sighash::{ SighashCache}},
    Address, Network, OutPoint, PackedLockTime, Script, Transaction, TxIn, TxOut,
    Witness, KeyPair, schnorr::TapTweak, SchnorrSig,
};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use miniscript::{psbt::PsbtExt};


pub fn p2tr(secret_string: Option<&str>) {
    let secp = Secp256k1::new();
    let scalar = Scalar::random();
    let secret = match secret_string {
        Some(sec_str) => SecretKey::from_str(&sec_str).unwrap(),
        None => {
            let secret_key = SecretKey::from_slice(&scalar.to_be_bytes()).unwrap();
            println!("secret_key: {}", secret_key.display_secret());
            secret_key
        }
    };
	
	let key_pair=KeyPair::from_secret_key(&secp, &secret);

	let (x_only,_)=key_pair.x_only_public_key();

	let address=Address::p2tr(&secp, x_only, None, Network::Regtest);
	
    println!("address {}", address.to_string());

	if(secret_string.is_none()){
		return;
	}

    let client = Client::new(
        "http://127.0.0.1:18443",
        Auth::UserPass(
            "foo".to_owned(),
            "qDDZdeQ5vw9XXFeVnXT4PZ--tGN2xNjjR4nrtyszZx0=".to_owned(),
        ),
    )
    .unwrap();

    let unspent = client
        .list_unspent(None, None, Some(&vec![&address]), None, None)
        .unwrap()[..3].to_vec();

    let tx_in_list = unspent
        .iter()
        .map(|entry| 
            TxIn {
                previous_output: OutPoint::new(entry.txid, entry.vout),
                script_sig: Script::new(),
                sequence: bitcoin::Sequence(0xFFFFFFFF),
                witness: Witness::default(),
        })
        .collect::<Vec<TxIn>>();

    let transaction_list = tx_in_list
        .iter()
        .map(|tx_in| {
            client
                .get_transaction(&tx_in.previous_output.txid, Some(true))
                .unwrap()
                .transaction()
                .unwrap()
        })
        .collect::<Vec<Transaction>>();

 	let prevouts =transaction_list.iter()
	.flat_map(|tx|tx.output.clone())
	.filter(|p|address.script_pubkey().eq(&p.script_pubkey))
	.collect::<Vec<TxOut>>();

	let total:u64 =prevouts.iter().map(|tx_out|tx_out.value).sum();

   let out_put = vec![TxOut {
        value: total-100000,
        script_pubkey: Address::from_str(
            "bcrt1prnpxwf9tpjm4jll4ts72s2xscq66qxep6w9hf6sqnvwe9t4gvqasklfhyj",
        )
        .unwrap()
        .script_pubkey(),
    }];


	let unsigned_tx = Transaction {
        version: 2,
        lock_time: PackedLockTime(0),
        input: tx_in_list,
        output: out_put,
    };
	
    let mut psbt = PartiallySignedTransaction::from_unsigned_tx(unsigned_tx.clone()).unwrap();

	psbt.inputs=sign_all_unsigned_tx(&secp,&prevouts,  &unsigned_tx,&key_pair );

	let tx=psbt.finalize(&secp).unwrap().extract_tx();

	client.send_raw_transaction(&tx).map(|tx|println!("transaction send transaction id is: {}",tx)).unwrap();


}

fn sign_all_unsigned_tx(
	secp: &Secp256k1<All>,
	prevouts: &Vec<TxOut>, 
	unsigned_tx: &Transaction, 
	key_pair: &KeyPair 
) -> Vec<Input> {
		return prevouts.iter().enumerate().map(|(index,tx_out)| sign_tx(secp,index,unsigned_tx, &prevouts, key_pair,  tx_out).clone()).collect(); 
}

fn sign_tx(secp: &Secp256k1<All>,index:usize,unsigned_tx: &Transaction,  prevouts: &Vec<TxOut>, key_pair: &KeyPair, tx_out: &TxOut) -> Input {

    let sighash=SighashCache::new(&mut unsigned_tx.clone())
			    .taproot_key_spend_signature_hash(
			    index, 
			    &Prevouts::All(&prevouts),
			    bitcoin::SchnorrSighashType::AllPlusAnyoneCanPay)
			    .unwrap();

    let message =Message::from_slice(&sighash).unwrap();

    let tweaked_key_pair =key_pair.tap_tweak(&secp, None);

    let sig=secp.sign_schnorr(&message, &tweaked_key_pair.to_inner());

	secp.verify_schnorr(&sig, &message, &tweaked_key_pair.to_inner().x_only_public_key().0).unwrap();

    let schnorr_sig =SchnorrSig{
				    sig,
				    hash_ty: bitcoin::SchnorrSighashType::AllPlusAnyoneCanPay,
			    };

    let mut input = Input::default();

    input.witness_script = Some(tx_out.script_pubkey.clone());

    input.tap_key_sig=Some(schnorr_sig);

    input.witness_utxo = Some(tx_out.clone());


    return input;
}
