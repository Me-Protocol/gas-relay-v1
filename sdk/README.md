# Gas Relay TypeScript SDK

This TypeScript SDK is designed to facilitate interaction with a Gas Relay server, allowing you to send gasless transactions on Ethereum-compatible networks via meta-transactions. This SDK enables developers to easily integrate gasless transaction functionality into their decentralized applications (dApps).

Features
- Gasless Transactions: Send transactions on behalf of users without them needing to pay gas fees, leveraging the meta-transaction mechanism.
- Batch Requests: Send multiple relay requests in one transaction, optimizing the number of calls to the relay server.
- Easy Integration: This SDK integrates with the ERC-2771 Trusted Forwarder standard, simplifying setup.
- Transaction Signing: Sign transactions using ethers.js and prepare them for relay through a trusted forwarder.
- Gas Estimation: Estimate the required gas for a transaction relayed through a trusted forwarder.

### Installation

Install the SDK via your preferred package manager:
- npm:
```sh
npm install @developeruche/gas-relay-sdk
```

- yarn:
```sh
yarn add  @developeruche/gas-relay-sdk
```


- pnpm:
```sh
pnpm add  @developeruche/gas-relay-sdk
```


### Usage

#### Import the SDK

```typescript 
import { prepRequest, gaslessRelay, batchedGaslessRelay, estimateGas } from '@developeruche/gas-relay-sdk';
import { ethers } from 'ethers';
```

#### 1. Prepare a Relay Request

The `prepRequest` function prepares a gasless transaction request by signing the transaction with the user’s Ethereum signer and estimating the required gas.
```typescript
const signer = new ethers.Wallet('your-private-key', provider);

const relayRequest = await prepRequest(
  signer,
  '0xRecipientAddress',
  BigInt(0),  // value in wei
  '0xData',   // transaction data
  '0xTrustedForwarderAddress',
  'ERC2771Forwarder',  // Name of the trusted forwarder
  BigInt(1),  // Chain ID
  'your-access-key'
);

console.log('Relay Request:', relayRequest);
```

#### 2. Send a Gasless Relay Request

Once you have prepared a relay request, you can send it to the gas relay server using the gaslessRelay function.
```typescript
const response = await gaslessRelay(relayRequest, 'http://relay-server-url/relay');
console.log('Relay Response:', response);
```


#### 3. Send Multiple Relay Requests (Batching)

If you need to send multiple relay requests, you can batch them using the batchedGaslessRelay function.

```typescript
    let re1 = await prepRequest(
      signer,
      '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512',
      BigInt(0),
      '0x3e6fec04',
      '0x5FbDB2315678afecb367f032d93F642f64180aa3',
      'ERC2771Forwarder',
      BigInt(31337)
    );
    let re2 = await prepRequest(
      signer,
      '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512',
      BigInt(0),
      '0x3e6fec04',
      '0x5FbDB2315678afecb367f032d93F642f64180aa3',
      'ERC2771Forwarder',
      BigInt(31337)
    );
    let payableAccount = '0x5FbDB2315678afecb367f032d93F642f64180aa3';
    let res1 = await batchedGaslessRelay([re1, re2], payableAccount, 'http://127.0.0.1:8010/batch-relay');
      console.log(res1);
```

#### 4. Estimate Gas for a Transaction
You can estimate the gas required for a transaction by using the estimateGas function.

```typescript
const estimatedGas = await estimateGas(
  '0xRecipientAddress',
  '0xData',
  '0xSenderAddress',
  BigInt(0),
  '0xTrustedForwarderAddress',
  signer
);

console.log('Estimated Gas:', estimatedGas.toString());
```



## Types

This SDK uses the following types:
- `RelayRequest`: A complete relay request, including signature and transaction details.
- `RelayRequestNoSignature`: A relay request without a signature, used to prepare the request before signing.
- `RelayRequestSerialized`: A serialized version of the relay request for transmission to the relay server.

Example Workflow

Here’s an example workflow that demonstrates how to use this SDK to send a gasless transaction:
1.	Create a signer: Use ethers.js to create a signer (e.g., a wallet).
2.	Prepare the request: Use prepRequest to prepare a relay request with the necessary parameters.
3.	Send the request: Use gaslessRelay to send the prepared request to the gas relay server.
4.	Handle the response: Check the response from the relay server to ensure that the transaction was successfully relayed.

```typescript
import { ethers } from 'ethers';
import { prepRequest, gaslessRelay } from '@developeruche/gas-relay-sdk';

// Initialize provider and signer
const provider = new ethers.JsonRpcProvider('http://localhost:8545');
const signer = new ethers.Wallet('your-private-key', provider);

// Prepare the relay request
const relayRequest = await prepRequest(
  signer,
  '0xRecipientAddress',
  BigInt(0),
  '0xTransactionData',
  '0xTrustedForwarderAddress',
  'ERC2771Forwarder',
  BigInt(1),
  'your-access-key'
);

// Send the relay request to the gas relay server
const relayResponse = await gaslessRelay(relayRequest, 'http://localhost:8010/relay');

console.log('Relay Response:', relayResponse);
```


### Development

To contribute to the SDK or make improvements, clone this repository and run the following commands:
1.	Clone the repository:
```sh
git clone https://github.com/developeruche/gas-relay.git
cd gas-relay/sdk/ts
```


2.	Install dependencies:
```sh
yarn
```


License

This project is licensed under the MIT License. See the LICENSE file for more details.

For any questions or support, feel free to reach out via GitHub Issues.

This README provides a basic overview of how to interact with the gas relay server using the SDK, along with installation, usage, and contribution guidelines.