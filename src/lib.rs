use scrypto::prelude::*;

#[blueprint]
mod escrow {
    struct Escrow {
        requested_resource: ResourceSpecifier,
        offered_resource: Vault,
        requested_resource_vault: Vault,
        escrow_nft: ResourceAddress,
    }

    impl Escrow {

        pub fn instantiate_escrow(
            requested_resource: ResourceSpecifier,
            offered_resource: Bucket
        ) -> (Global<Escrow>, NonFungibleBucket) {
            
            // Creating an empty vault for the requested resource
            let requested_resource_vault = Vault::new(requested_resource.get_resource_address());

            // Minting the EscrowBadge NFT which will be used to manage the escrow.
            let escrow_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", "Scrypto 101 Escrow Badge")
                .mintable(rule!(require(RADIX_TOKEN)), LOCKED)
                .burnable(rule!(require(RADIX_TOKEN)), LOCKED)
                .updateable_non_fungible_data(rule!(require(RADIX_TOKEN)), LOCKED)
                .no_initial_supply();

            // Creating a unique badge ID and mint the badge with the offered resource information.
            let badge_id = NonFungibleLocalId::random();
            let badge = escrow_badge.mint_non_fungible(&badge_id, EscrowBadge {
                offered_resource: offered_resource.resource_address(),
            });

            // Instntianting the Escrow component with the initial state
            let component = Self {
                requested_resource,
                offered_resource: Vault::with_bucket(offered_resource),
                requested_resource_vault,
                escrow_nft: escrow_badge,
            }
            .instantiate();

            // We have to return the instantiated component and the minted badge
            (component, badge)

        }

        pub fn exchange(&mut self, bucket_of_resource: Bucket) -> Bucket {
            match &self.requested_resource {
                ResourceSpecifier::Fungible { resource_address, amount } => {
                    // Provided resource need to match the requested resource address and amount
                    assert_eq!(bucket_of_resource.resource_address(), *resource_address, "Oooops wrong resource address");
                    assert!(bucket_of_resource.amount() >= *amount, "Insufficient amount of resource");

                    // Transfer the requested amount to the requested resource vault.
                    self.requested_resource_vault.put(bucket_of_resource.take(*amount));
                },
                ResourceSpecifier::NonFungible { resource_address, non_fungible_local_id } => {
                    // Provided resource matches the requested resource address and ID
                    assert_eq!(bucket_of_resource.resource_address(), *resource_address, "Oooops wrong resource address");
                    assert!(bucket_of_resource.contains_non_fungible(*non_fungible_local_id), "Non-fungible ID not found");

                    // Transfer the requested non-fungible token to the requested resource vault.
                    self.requested_resource_vault.put(bucket_of_resource.take_non_fungible(*non_fungible_local_id));
                },
            }
            // Returns offered resource to the other party.
            self.offered_resource.take_all()
        }

        pub fn withdraw_resource(&mut self, escrow_nft: NonFungibleBucket) -> Bucket {

            todo!();

        }

        pub fn cancel_escrow(&mut self, escrow_nft: NonFungibleBucket) -> Bucket {

            todo!();

        }
    }
}



// Types //

#[derive(ScryptoSbor, Clone)]
pub enum ResourceSpecifier {
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal
    },
    NonFungible {
        resource_address: ResourceAddress,
        non_fungible_local_id: NonFungibleLocalId
    }
}

impl ResourceSpecifier {

    pub fn get_resource_address(&self) -> ResourceAddress {
        match self {
            Self::Fungible {
                resource_address, ..
            }
            | Self::NonFungible {
                resource_address, ..
            } => *resource_address,
        }
    }
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct EscrowBadge {
    offered_resource: ResourceAddress
}