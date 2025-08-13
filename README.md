# Telurean Chain

**Telurean Chain** is a specialized blockchain built on the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) using Substrate, designed to power advanced Non-Fungible Tokens (NFTs) for tabletop role-playing games (RPGs). It enables the creation, management, and tracking of dynamic NFTs that represent characters, achievements, world elements, and more, tailored to the immersive and narrative-driven needs of RPG communities. With a focus on flexibility and extensibility, Telurean Chain provides a foundation for game designers and players to bring their stories to life on-chain.

## Vision

Telurean Chain aims to revolutionize tabletop RPGs by leveraging blockchain technology to create a rich ecosystem of NFTs that evolve with the game. Key features include:
- **Dynamic Character NFTs**: Representing RPG characters with customizable attributes such as classes, races, and abilities, adaptable to specific game systems.
- **Adventure Tracking**: NFTs that capture achievements, honors, battles, and dramatic moments, preserving the narrative history of characters.
- **Character Status NFTs**: Representing dynamic states like wounds, mental trauma, social status improvements, or family relationships.
- **World-Building NFTs**: Representing in-game elements such as locations, maps, creatures, and mythical beasts, enabling immersive storytelling.
- **Hierarchical Relationships**: Support for different types of relationships between NFTs (ownership, composition, carrier, etc.)

The Minimum Viable Product (MVP) focuses on core NFT functionalities, with plans to expand into a full ecosystem for RPG-specific use cases.

## Features (MVP)

The current MVP of Telurean Chain includes the following core functionalities:
- **NFT Creation and Management**: Create, transfer, and burn NFTs representing characters, items, or other game elements using `pallet_uniques`.
- **Hierarchical Relationships**: Establish parent-child relationships (e.g., a character NFT owning an item NFT) with `pallet_nft_hierarchy`.
- **Dynamic Attributes**: Store and modify attributes for NFTs (e.g., character stats, status conditions) using `pallet_nft_attributes`.

### Planned Features
- Support for advanced game mechanics.
- Complex hierarchical relationships:

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
