Project Overview
----------------

Uprock IDO Protocol facilitates Initial DEX Offerings (IDOs) with advanced features for managing token distribution, whitelisting, and fundraising. 
The project leverages the Anchor framework for Solana smart contracts and includes functionalities such as creating and updating fundraising pools, managing whitelisted users, and handling token distributions. 
The outlined commands and functions provide a comprehensive guide to deploying, testing, and interacting with the IDO protocol.

Smart Contract
---------------

The project is organized into several modules:

-   `access_controls`: Contains modules for enforcing access control policies, ensuring secure and authorized interactions with smart contract functions. 
-   `anchor_metaplex`: Integrates with the Metaplex library, providing additional features and capabilities for handling NFTs and tokens. 
-   `errors`: Defines custom error types used throughout the project. 
-   `instructions_ido_v1`: Implements the core instructions and logic for the IDO protocol, including pool creation, user whitelisting, token deposits, claiming, and more. 
-   `state`: Defines the data structures representing the state of the smart contracts, including pool information, whitelisted user details, and various statistics.


Devnet Keys
------------
Following private keys (they should be in `.keyparis` folders) are being used in the smart contract code but not included in the repo. They will be used to keep the devnet program stable

| Key                                              | Description                  |
|--------------------------------------------------|------------------------------|
| devKH8tdPeByT13FWMVcX48B43v3wB1Qsa2xfGvpttm.json | Developer (admin) key        |
| funJG6uooFXjJpALB6ExhnBefsPFJkamzy6Gfv7zN98.json | Fundraiser (ido owner) key   |
| ido5zRdfphtyeov8grJqBszfWquvTCAvRzGycSfPXuX.json | Program (smart contract) key |
| poo4b5Lsyg5fMdrbmXkf6apxTNKQrAx4booXkbGVNNW.json | Pool (IDO ID) key            |
| TicpC2VhBZbknfc4RhYkdPty6SE4HjBozZS9umAMX1d.json | Pre Required Token (TICKET)  |
| tokBWvxCaa1AJnVUUbgeNHeVDJYbu1DpYXS9yTuvmjY.json | Selling Token Mint           |
| usdA7bUXh1kNAwhCmabJf7QmWsaTk4Mymk26aEsjAeB.json | Quote Token (USDC) Mint      |
| usrsq7DVNrLgaZPuuQcrNeFBLQe4i1ZrFGNofvC3Pfw.json | User (IDO Participant) Key   |

Smart Contract Commands
-----------------------

The project includes several commands to deploy, test, and manage the IDO protocol. Ensure you have the necessary environment variables and keypairs configured as specified in the Project Setup section.

### Airdrop

Get airdrop to the developer wallet (devnet & testnet only):
`make airdrop`

### Build
Build the Solana program:
`make build`

### Deploy
Deploy the Solana program:
`make deploy`

### Deploy with Resume
Deploy the Solana program with resume functionality (using a buffer):
`make deploy-resume buffer=<buffer_value>`

### Test
Run the tests:
`make test`

### Test (Skip Build and Deploy)
Run the tests without building and deploying:
`make test-skip`

### Create Pool Example
Create a pool using the provided TypeScript example:
`make create-pool`

Smart Contract Functions
------------------------
The smart contract functions represent the core functionalities of the IDO protocol. These include:

- `create_pool`: Admin-only function to create a fundraising pool with specified parameters. 
- `update_pool`: Admin-only function to update the parameters of an existing pool. 
- `whitelist_nft` and `whitelist_ticket`: Whitelist users by locking their NFTs or TICKET tokens during specific phases. 
- `deposit_nft` and `deposit_ticket`: Allow users to deposit NFTs or TICKET tokens during designated phases.  
- `boost`: A mechanism to boost participant allocations during specific phases.  
- `unlock_nft` and `unlock_ticket`: Unlock NFTs or TICKET tokens for users during the distribution phase.  
- `raise`: Raise funds by transferring base tokens from users to the fundraising pool.  
- `claim`: Allow users to claim their allocated tokens during the distribution phase.  
- `force_claim`: Admin-only function to forcefully claim tokens on behalf of users.  
- `refund`: Refund users if the raise date has not occurred yet.  
- `close_pool`: Admin-only function to close a fundraising pool.  
- `close_whitelist_account`: Admin-only function to close a whitelist account.  
- `recover_nft`, `recover_ticket`, `recover_usdc`: Admin-only functions to recover specific tokens.  
- `migrate_ownership`: Admin-only function to migrate ownership of the smart contract.  
- `just_close_whitelist_account`: Admin-only function to close a whitelist account without additional actions.  
- `update_pool_rate`: Admin-only function to update the rate of tokens for the fundraising pool.  
- `force_raise`: Admin-only function to forcefully raise funds.  
- `update_whitelist_account`: Admin-only function to update whitelist account details.  
- 
Refer to the source code for detailed information about each function's implementation and usage.

Note
----

Ensure that the Solana CLI is installed, dependencies are satisfied, and the environment is correctly configured before running any commands. 
Adjust environment variables, paths, and parameters as needed for your specific setup.

Developer Grant Program: Earn UPT Tokens
-------
We're offering UPT tokens to developers who contribute valuable code to the IDO protocol. Here's how you can earn tokens:
#### Contribute to Key Features:
- Add [WNS](https://www.jupresear.ch/t/wen-new-standard-wns-0-0) support.
- Add [Token2022](https://spl.solana.com/token-2022) support.
- Upgrade [Anchor](https://www.anchor-lang.com/) version.
- Implement [a fair launch](https://www.coingecko.com/learn/what-is-a-fair-launch-in-crypto) feature.
- Permissionless pools
  - A frontend for permissionless IDO pool creation & management
#### Reward Criteria:
We'll review pull requests and reward contributions based on complexity, utility, and overall impact on the community.
##### Why Contribute?
**Earn Rewards:** Get UPT tokens for your contributions.  
**Make an Impact:** Help us enhance our project and benefit the ecosystem.

Submit your pull requests, and let’s innovate together!

License
-------
MIT