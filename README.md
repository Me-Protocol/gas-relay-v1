# Gas Relay

**Gas Relay for a Secure Protocol for Native Meta Transactions**

## Overview
A secure and efficient Gas Relay implementation based on the ERC-2771 specification. This software enables meta-transactions by receiving signed requests from Transaction Signers, covering gas costs, and relaying transactions to a Trusted Forwarder.


### Motivation
Meta-transactions provide an elegant solution to this problem by allowing third parties to sponsor transaction costs on behalf of users. This shifts the gas payment burden from users to the service providers, enabling a more user-friendly and accessible dApp experience.

Despite the promise of meta-transactions, implementing them securely and efficiently remains a challenge for developers. Without a standardized approach, the risk of fragmented solutions, duplicated efforts, and potential security vulnerabilities increases.


### Chain Support
Gas relay is built to interface with any EVM compatiable chain, it is all about a change in the config of the `Relay` server. It is important to not that all trusted forwarder across chain are deployed to that same address using `create2`.
These contract ar **not upgradeable**.

### Architecture
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



## Installation
### Prerequisites
- Rust: Ensure you have Rust installed.
- Node.js: Required for Ethereum utilities (e.g., ethers.js).
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


### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).




## License
The Gas Relay library (i.e. all code outside of the `bin` directory) is licensed under the GNU General Public License v3.0.

The Gas Relay binaries (i.e. all code inside of the `bin` directory) is licensed under the GNU General Public License v3.0.

The Gas Relay SDKs is licensed under the GNU General Public License v3.0.
