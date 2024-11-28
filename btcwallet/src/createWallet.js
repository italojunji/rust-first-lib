//importing dependencies
const bip32 = require('bip32')
const bip39 = require('bip39')
const bitcoin = require('bitcoinjs-lib')

//defining network
//bitcoin - main
//testnet - test
const network = bitcoin.networks.testnet

//wallets HD derivation
const path = `m/49'/1'/0'/0`

//creating mnemonic to seed (words for password)
let mnemonic = bip39.generateMnemonic()
const seed = bip39.mnemonicToSeedSync(mnemonic)

//creating root of wallet HD
let root = bip32.fromSeed(seed, network)

//creating account - pair of pvt-pub keys
let account = root.derivePath(path);
let node = account.derive(0). derive(0)

let btcAddress = bitcoin.payments.p2pkh({
    pubkey: node.publicKey,
    network: network,
}).address

console.log("Wallet was generated")
console.log("Address: ", btcAddress)
console.log("Private key: ", node.toWIF()) //Wallet Import Format
console.log("Seed: ", mnemonic)