# Telurean Chain

Telurean Chain is a blockchain based on the Substrate Node Template. Its main purpose is to encode on-chain a set of super-rules that will make Telurean agnostic to the specific role-playing game it is applied to.

## Key Functions
- Human-Validated Consensus (PoA) consensus algorithm.
- Advanced NFT system based on the following hierarchy:

```plaintext
  Entity
    ┗━ Complex
        ┣━ Creature
        ┃   ┣━ Owner
        ┃   ┃   ┣━ Character
        ┃   ┗━ Carrier
        ┃       ┗━ Beast
        ┃           ┗━ Mount
        ┃
        ┣━ Container
        ┃   ┗━ Vehicle
        ┃
        ┗━ Compound
            ┗━ Place
                ┣━ Map
                ┗━ POI
  Badge
    ┣━ Honor
    ┣━ Rank
    ┗━ State