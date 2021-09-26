import "regenerator-runtime/runtime.js";
import * as nearAPI from "near-api-js";
import getConfig from "./claimerConfig.js";
import { functionCall, signTransaction } from "near-api-js/lib/transaction.js";
import { baseDecode } from 'borsh';
const nearConfig = getConfig("development");

export default class ClaimerContract {
  // near
  // wallet_connection
  // contract
  // status
  // account
  // keyPair
  // nonce
  // provider
  async init(private_key, nonce) {
    this.nonce = nonce
    const keyStore = new nearAPI.keyStores.InMemoryKeyStore();
    const PRIVATE_KEY = private_key
    this.keyPair = nearAPI.KeyPair.fromString(PRIVATE_KEY);
    await keyStore.setKey("testnet", nearConfig.contractName, this.keyPair);

    nearConfig.keyStore = keyStore
    this.near = await nearAPI.connect(nearConfig);

    this.account = await this.near.account(nearConfig.contractName);
    
    this.contract = await new nearAPI.Contract(this.account, nearConfig.contractName, {
      // View methods are read-only – they don't modify the state, but usually return some value
      viewMethods: ['get_info'],
      // Change methods can modify the state, but you don't receive the returned value when called
      changeMethods: ['claim'],
      // Sender is the account ID to initialize transactions.
      // getAccountId() will return empty string if user is still unauthorized
      sender: this.account
    });
    this.provider = await new nearAPI.providers.JsonRpcProvider(nearConfig.nodeUrl);
  }

  async claim(receiver) {
    const block = await this.account.connection.provider.block({ finality: 'final' });
    const blockHash = block.header.hash;

    const nonce = this.nonce
    let actions = [functionCall("claim", {receiver: receiver}, 3000000000000, 0)]
    let [txHash, signedTx] = await signTransaction(
      nearConfig.contractName, nonce, actions, baseDecode(blockHash), this.account.connection.signer, this.account.accountId, this.account.connection.networkId
    );
    console.log(txHash)
    await this.account.connection.provider.sendTransaction(signedTx);
  }

  async get_info() {
    return await this.contract.get_info()
  }
}


