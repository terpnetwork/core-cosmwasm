# Terp Residual Registry

The Terp Residual Registry contract is a CosmWasm smart contract deployed on the Terp chain. It allows NFT collection admins to define the residuals that should be paid to them when their NFTs are sold on the Terp chain. The residual registry logic is applied as described below.

## Residual Registry Logic

- Only the collection admin can register a residual for a collection. The collection admin is defined to be the admin on the NFT collection contract. If that contract admin does not exist, then the collection admin is the contract creator.
- The collection admin can set a default residual percentage for the collection. This default residual percentage is applied when there is no specific protocol residual percentage set for a given protocol.
- The collection admin can set a protocol residual percentage for a given protocol. This protocol residual percentage is applied when the protocol itself is calculating a residual for the NFT sale.
- Any residual percentage set by a given collection owner can only be changed by a the maximum amount of config parameter `max_share_delta` per invocation. After changing a residual percentage, the collection owner must wait `update_wait_period` to update the percentage again.

## Additional Notes

- The shares percentages set in the residual registry are represented as [cosmwasm_std::Decimal]. The max residual share is 1.0, which is equivalent to 100%. Consumers of the residual registry should be aware of this when calculating the residual amount to be paid, and can set a cap on the amount of residuals to be paid if the percentage is too high.
