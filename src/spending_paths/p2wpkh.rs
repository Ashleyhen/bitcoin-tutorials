use std::str::FromStr;

use bitcoin::{
    blockdata::{opcodes::all, script::Builder},
    psbt::{Input, PartiallySignedTransaction},
    secp256k1::{Message, Scalar, Secp256k1, SecretKey},
    util::sighash,
    Address, EcdsaSig, Network, OutPoint, PackedLockTime, Script, Transaction, TxIn, TxOut,
    Witness,
};
use bitcoin_hashes::{hash160, Hash};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use miniscript::{psbt::PsbtExt, ToPublicKey};


pub fn p2wpkh(secret_string: Option<&str>) {
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

    let pub_k = secret.public_key(&secp);

    let script = Builder::new()
        .push_int(0)
        .push_slice(&hash160::Hash::hash(&pub_k.to_public_key().to_bytes()))
        .into_script();

    let address = Address::from_script(&script, Network::Regtest).unwrap();

    println!("address {}", address.to_string());

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
        .unwrap()[..2].to_vec();

    let tx_in_list = unspent
        .iter()
        .map(|entry| {
            return TxIn {
                previous_output: OutPoint::new(entry.txid, entry.vout),
                script_sig: Script::new(),
                sequence: bitcoin::Sequence(0xFFFFFFFF),
                witness: Witness::default(),
            };
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

    let out_put = vec![TxOut {
        value: 1000000,
        script_pubkey: Address::from_str(
            "bcrt1prnpxwf9tpjm4jll4ts72s2xscq66qxep6w9hf6sqnvwe9t4gvqasklfhyj",
        )
        .unwrap()
        .script_pubkey(),
    }];

    let unsigned_tx = Transaction {
        version: 0,
        lock_time: PackedLockTime(0),
        input: tx_in_list,
        output: out_put,
    };

    let mut psbt = PartiallySignedTransaction::from_unsigned_tx(unsigned_tx.clone()).unwrap();

    let input_list = transaction_list
        .iter()
        .flat_map(|prev_tx| {
            let partial_sig_list = prev_tx
                .output
                .iter()
                .enumerate()
                .filter(|(_, tx_out)| tx_out.script_pubkey.eq(&address.script_pubkey()))
                .map(|(input_index, tx_out)| {
                    let script_pubkey = Builder::new()
                        .push_opcode(all::OP_DUP)
                        .push_opcode(all::OP_HASH160)
                        .push_slice(&tx_out.script_pubkey[2..])
                        .push_opcode(all::OP_EQUALVERIFY)
                        .push_opcode(all::OP_CHECKSIG)
                        .into_script();

                    let sig_hash = sighash::SighashCache::new(&mut unsigned_tx.clone())
                        .segwit_signature_hash(
                            input_index,
                            &script_pubkey,
                            tx_out.value,
                            bitcoin::EcdsaSighashType::All,
                        )
                        .unwrap();
                    let signature =
                        secp.sign_ecdsa(&Message::from_slice(&sig_hash).unwrap(), &secret);
                    let mut input = Input::default();
                    input.witness_script = Some(tx_out.script_pubkey.clone());
                    input.witness_utxo = Some(tx_out.clone());
                    input
                        .partial_sigs
                        .insert(pub_k.to_public_key(), EcdsaSig::sighash_all(signature));
                        dbg!(input.clone());
                    return input;
                })
                .collect::<Vec<Input>>();
            return partial_sig_list;
        })
        .collect::<Vec<Input>>();

    psbt.inputs=input_list;
        dbg!(psbt.clone());

    let extracted = psbt.finalize(&secp).unwrap().extract(&secp).unwrap();
}
