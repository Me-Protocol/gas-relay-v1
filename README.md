# Gas Relay

**Gas Relay for a Secure Protocol for Native Meta Transactions**

## Overview
A secure and efficient Gas Relay implementation based on the ERC-2771 specification. This software enables meta-transactions by receiving signed requests from Transaction Signers, covering gas costs, and relaying transactions to a Trusted Forwarder.


## Motivation
Meta-transactions provide an elegant solution to this problem by allowing third parties to sponsor transaction costs on behalf of users. This shifts the gas payment burden from users to the service providers, enabling a more user-friendly and accessible dApp experience.

Despite the promise of meta-transactions, implementing them securely and efficiently remains a challenge for developers. Without a standardized approach, the risk of fragmented solutions, duplicated efforts, and potential security vulnerabilities increases.


## Chain Support
Gas relay is built to interface with any EVM compatiable chain, it is all about a change in the config of the `Relay` server. It is important to not that all trusted forwarder across chain are deployed to that same address using `create2`.
These contract ar **not upgradeable**.

## Architecture
**Components**
1.	Gas Relay
    - Handles incoming signed requests.
    - Verifies signatures and validates nonces.
    - Submits transactions via the Trusted Forwarder.
2.	Trusted Forwarder
    - Forwards the transaction to the Recipient contract.
    - Ensures the integrity of msg.sender for meta-transactions.
3.	Recipient Contract
    - Implements the ERC-2771 interface to accept meta-transactions.



## Installation **Server**
### Prerequisites
- Rust: Ensure you have Rust installed.
- Foundry: Required for running tests locally.
- Docker (optional): For containerized deployment.

### Clone repo
```
git clone https://github.com/developeruche/gas-relay.git  
cd gas-relay/server
```

### Build Relay server
```
cargo build --release
```

### Introduce Config
create a `.config.toml` file, adding these configs.

```toml 
[server]
db_url = "host=localhost user=postgres"
# db_url ="host=xxxx.amazonaws.com port=5432 user=postgres password=xxxx dbname=postgres sslmode=disable" production-sample
server_url = "127.0.0.1:8010"
access_key = "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356"

[[chains]]
name = "anvil"
rpc_url = "http://127.0.0.1:8545/"
chain_id = 1
accounts_private_keys = ["0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356", "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356", "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356", "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356", "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356", "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356", "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356", "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356"]
trusted_forwarder = "0x5FbDB2315678afecb367f032d93F642f64180aa3"
```

### Run Server
```sh 
./target/release/relayer --config-path ./.config.toml  
```


## Installation **TypeScript SDK**
### Prerequisites
- Node: Ensure the node runtime engine is installed.

### Install the SDK via your preferred package manager:
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

## Installation **Solidity smart Contract**
### Prerequisites
- Foundry: Required for running tests locally.

### Install the SDK via your preferred package manager:
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

### Install from GitHub for Foundry
- forge:
```sh
forge install OpenZeppelin/openzeppelin-contracts
```

### Implement ERC2771Context on your contract
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/metatx/ERC2771Context.sol";

contract StorageContract is ERC2771Context {
    mapping(string => string) private storageMap;

    constructor(address trustedForwarder) ERC2771Context(address(trustedForwarder)) {}

    function setValue(string memory key, string memory value) public {
        storageMap[key] = value;
    }

    function getValue(string memory key) public view returns (string memory) {
        return storageMap[key];
    }
}
```


## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).




## License
The Gas Relay library (i.e. all code outside of the `bin` directory) is licensed under the GNU General Public License v3.0.

The Gas Relay binaries (i.e. all code inside of the `bin` directory) is licensed under the GNU General Public License v3.0.

The Gas Relay SDKs is licensed under the GNU General Public License v3.0.
