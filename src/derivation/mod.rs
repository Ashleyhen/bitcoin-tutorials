
pub fn bip32(){


/*
An HDW is organized as several 'accounts'. Accounts are numbered, the default account ("") being number 0. 
Clients are not required to support more than one account - if not, they only use the default account.

Each account is composed of two keypair chains: an internal and an external one. 

The external keychain is used to generate new public addresses, 

Internal keychain is used for all other operations (change addresses, generation addresses, ..., anything that doesn't need to be communicated). 
Clients that do not support separate keychains for these should use the external one for everything.

m/iH/0/k corresponds to the k'th keypair of the external chain of account number i of the HDW derived from master m.
m/iH/1/k corresponds to the k'th keypair of the internal chain of account number i of the HDW derived from master m.
*/

/* 
	The next step is cascading several CKD constructions to build a tree. 
	We start with one root, the master extended key m. By evaluating CKDpriv(m,i) 
	for several values of i, we get a number of level-1 derived nodes. As each of 
	these is again an extended key, CKDpriv can be applied to those as well.

	To shorten notation, we will write CKDpriv(CKDpriv(CKDpriv(m,3H),2),5) as m/3H/2/5. 
	Equivalently for public keys, we write CKDpub(CKDpub(CKDpub(M,3),2),5) as M/3/2/5. 
	This results in the following identities:

	N(m/a/b/c) = N(m/a/b)/c = N(m/a)/b/c = N(m)/a/b/c = M/a/b/c.
	N(m/aH/b/c) = N(m/aH/b)/c = N(m/aH)/b/c.
*/

/* 	Full wallet sharing: m
	In cases where two systems need to access a single shared wallet, 
	and both need to be able to perform spendings, one needs to share the master private extended key. 
	Nodes can keep a pool of N look-ahead keys cached for external chains, to watch for incoming payments. 
	The look-ahead for internal chains can be very small, as no gaps are to be expected here. 
	An extra look-ahead could be active for the first unused account's chains - 
	triggering the creation of a new account when used. Note that the name of the account will still need to 
	be entered manually and cannot be synchronized via the block chain.
*/

/*
	Audits: N(m/ *)
	In case an auditor needs full access to the list of incoming and outgoing payments, one can share all account public extended keys. 
	This will allow the auditor to see all transactions from and to the wallet, in all accounts, but not a single secret key.
*/

/*
	Per-office balances: m/iH
	When a business has several independent offices, they can all use wallets derived from a single master. 
	This will allow the headquarters to maintain a super-wallet that sees all incoming and outgoing transactions of all offices, 
	and even permit moving money between the offices.
 */

/*  
	Recurrent business-to-business transactions: N(m/iH/0)
	In case two business partners often transfer money, one can use the extended public key for the external chain of a specific account (M/i h/0) as a sort of "super address", allowing frequent transactions that cannot (easily) be associated, but without needing to request a new address for each payment. Such a mechanism could also be used by mining pool operators as variable payout address.
*/

/*
	Unsecure money receiver: N(m/iH/0)
	When an unsecure webserver is used to run an e-commerce site, it needs to know public addresses that are used to receive payments. The webserver only needs to know the public extended key of the external chain of a single account. This means someone illegally obtaining access to the webserver can at most see all incoming payments but will not be able to steal the money, will not (trivially) be able to distinguish outgoing transactions, nor be able to see payments received by other webservers if there are several.
 */

}

pub fn bip43(){
	// We propose the first level of BIP32 tree structure to be used as "purpose". This purpose determines the further structure beneath this node.
	// Example: Scheme described in BIP44 should use 44' (or 0x8000002C) as purpose.
	// Note that m / 0' / * is already taken by BIP32 (default account), which preceded this BIP.
}
pub fn bip44(){

	/* 
	 We define the following 5 levels in BIP32 path: 
	 m / purpose' / coin_type' / account' / change / address_index
	 example bank, donations, etc...
	 */

}

pub fn bip44(){

	/* 
		change purpose to 84 for P2WPKH
	 */
	 
}

pub fn bip44(){
	//  P2WPKH-nested-in-P2SH purpose = 49
}