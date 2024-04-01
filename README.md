# Terp Network Core Cosmwasm Libraries

This repository contains the core contracts and libraries that are shared among all Terp Network protocols.

| NFT Contracts                                                     | Description                                                                                      |
|-------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| [Terp721 Base](./contracts/nft/collections/terp721-base/README.md)      | A base Terp NFT collection contract.                                                    |
| [Terp721 Metadata-Onchain](./contracts/nft/collections/terp721-metadata-onchain/README.md)      | A contract extending TERP721 NFT to store metadata on chain.                                         |
| [Terp721 Non-Transferable](./contracts/nft/collections/terp721-nt/README.md)      | A Terp Non-transferrable NFT collection contract.                                       |
| [Terp721 Updatable](./contracts/nft/collections/terp721-updatable/README.md)      |                                                     |
| [Earlybird](./contracts/nft/earlybirds/earlybird/README.md)      |                                                     |
| [Earlybird Flex](./contracts/nft/earlybirds/earlybird-flex/README.md)      |                                                     |
| [Earlybird Immutable](./contracts/nft/earlybirds/earlybird-immutable/README.md)      |                                                     |
| [Base Factory](./contracts/nft/factories/base-factory/README.md)      |                                                     |
| [Open Edition Factory](./contracts/nft/factories/open-edition-factory/README.md)      |                                                     |
| [Vending Factory](./contracts/nft/factories/vending-factory/README.md)      |                                                     |
| [Base Minter](./contracts/nft/minters/base-minter/README.md)      |                                                     |
| [Open Edition Minter](./contracts/nft/minters/open-edition-minter/README.md)      |                                                     |
| [Vending Minter](./contracts/nft/minters/vending-minter/README.md)      |                                                     |
| [Vending Minter Earlybird Flex](./contracts/nft/minters/vending-minter-eb-flex/README.md)      |                                                     |

| Revenue Contracts                                                     | Description                                                                                      |
|-------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| [Terp Fair Burn](./contracts/revenue/fair-burn/README.md)      |      
| [Terp Residual Registry](./contracts/revenue/residual-registry/README.md)      | Contract for fees and Developer Residual.       
| [Terp Splits](./contracts/revenue/splits/README.md)      |                                                     |

___
| Core Packages                                                     | Description                                                                                      |
|-------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| [Controllers](./packages/actions/controllers/README.md)      |      
| [Ethereum Signature Verification](./packages/actions/ethereum-verify/README.md)      |      
| [Mint Hooks](./packages/actions/mint-hooks/README.md)      |      
| [Terp Index Query](./packages/actions/terp-index-query/README.md)      |     
| [Factory Utils](./packages/nft/factory-utils/README.md)      |      
| [Minter Utils](./packages/nft/minter-utils/README.md)      |      
| [Terp Metadata](./packages/nft/terp-metadata/README.md)      |      
| [Terp721](./packages/nft/terp721/README.md)      |      
| [Terp Fee](./packages/revenue/terp-fee/README.md)      |      
| [Terp Multi Test](./packages/utils/terp-multi-test/README.md)      |      
| [Terp SDK](./packages/utils/terp-sdk/README.md)      |      
| [Test Suite](./packages/utils/test-suite/README.md)      |      
| [Unit Test](./packages/utils/unit-tests/README.md)      |      


> *Heavily modified fork of Stargaze [core](https://github.com/public-awesome/core) & [launchpad contract](https://github.com/public-awesome/launchpad). Massive respect to its contributors.*


```
39a83f7c258f3a6811d3e9733dd0fc4ec1992097b5b62a8b2be6e2eb0446ad71  auction.wasm
0b9f156b8b3a56061d1bcb438094c83dc81b94f6651250f2a2b8290ed0c80f6d  base_factory.wasm
f9005ba905cc68524bb48974e67a1dc66de40e48ebf35495703cc348eff20547  base_minter.wasm
98a6287a1c9e6c677aab726423c99672d470024414afc7c93fa947ab3793e839  earlybird.wasm
0dadbf1c426aa34c02287480432c9f6591497ce0d3bff874abe2fffa4817f7b3  earlybird_flex.wasm
fbc1713f2886f5c04b7df7eb3bfde7d28f6d0c627319f7ff0a293d162ee11cd1  earlybird_immutable.wasm
f78420913eddb55b927e8d091fc0256ef9561028978f1c2e9af9201a7ac9e8af  open_edition_factory.wasm
858bfc0c87915c4ae5b363b934a8de4ec37c317419c96465ff2663b8ccaa064d  open_edition_minter.wasm
f933ac3776fdf869e0141631ee27c85e54c952adcf72a37f91e026d6b184d690  terp721_base.wasm
b568fb2286f74b990b01180b022bbb4d620d58b3e06fe6246633f5979f88368a  terp721_metadata_onchain.wasm
1fee1d90d496d446a7e52e5d86fad7a820549dd808bb3335bb6402ed55099f6f  terp721_nt.wasm
dcdea63347ef884a51aa83be4598bdc91d06e921c18ce643636575f7152363f7  terp721_updatable.wasm
a4432cadfadb9277e9ee7d0a832078b63a084e4e5fd2f7e00df4367c9a7b2206  terp_fair_burn.wasm
f5ae2c98e63a58bcce634feeb6b9486aa9334d883587f2fbc0fbc785094815d4  terp_residual_registry.wasm
0c2165a7f83bc08fe86082b3f79b3ddbb2a572d01868e6088224c8ccc4e0b091  terp_splits.wasm
105144dd2b9c6cd1aff9c7c578478966b4a7e2371960fe621894c9538b063c0c  vending_factory.wasm
e6c3aeac7955bb297cf8df9a98a8921973f0eb920f44f6a29c7808555ad2c6a3  vending_minter.wasm
5a1b620880cb3160a3891cc9f1c404eaa2de9818427417a62eeb4f3892fb57f9  vending_minter_eb_flex.wasm
```