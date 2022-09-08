use crate::utils::{
    abi_calls::{
        approve, balance_of, constructor, mint, owner_of, set_approval_for_all, transfer_from,
    },
    test_helpers::setup,
    Identity,
};
use fuels::signers::Signer;
// use fuel_crypto::{Message, PublicKey, SecretKey, Signature, Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn transfers_by_permit() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        let wallet = launch_provider_and_get_wallet().await;
        let message = "my message";
        // message.as_bytes() ?
        let signature = wallet.sign_message(message).await;
        let admin = Identity::Address(wallet.address());

        /*
        let message = "my message";
        let signature = owner1.sign_message(message).await?;
        println!("signature {}", signature)
        */
        assert(true)
/* 
        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address());
        let to = Identity::Address(owner2.wallet.address());
        // let approved_identity = Option::Some(to.clone());
        let approved_identity = to.clone();

        mint(1, &owner1.contract, &minter).await;

        approve(&approved_identity, &owner1.contract, 0).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
        assert_eq!(balance_of(&owner2.contract, &to).await, 0);

        transfer_from(&owner2.contract, &minter, &to, 0).await;

        assert_eq!(owner_of(&owner1.contract, 0).await, approved_identity);
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        assert_eq!(balance_of(&owner2.contract, &to).await, 1);*/
    }
/* 
    #[tokio::test]
    async fn transfers_multiple() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address());
        constructor(true, &deploy_wallet.contract, &admin, 4).await;

        let minter = Identity::Address(owner1.wallet.address());
        let to = Identity::Address(owner2.wallet.address());

        mint(4, &owner1.contract, &minter).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 4);
        assert_eq!(balance_of(&owner2.contract, &to).await, 0);

        transfer_from(&owner1.contract, &minter, &to, 0).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 3);
        assert_eq!(balance_of(&owner2.contract, &to).await, 1);

        transfer_from(&owner1.contract, &minter, &to, 1).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 2);
        assert_eq!(balance_of(&owner2.contract, &to).await, 2);

        transfer_from(&owner1.contract, &minter, &to, 2).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
        assert_eq!(balance_of(&owner2.contract, &to).await, 3);

        transfer_from(&owner1.contract, &minter, &to, 3).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        assert_eq!(balance_of(&owner2.contract, &to).await, 4);
    }*/
}
/*
mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_token_does_not_exist() {
        let (_deploy_wallet, owner1, owner2) = setup().await;

        let from = Identity::Address(owner1.wallet.address());
        let to = Identity::Address(owner2.wallet.address());
        transfer_from(&owner1.contract, &from, &to, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_signature_is_invalid() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address());
        mint(1, &owner1.contract, &minter).await;

        let to = Identity::Address(owner2.wallet.address());
        transfer_from(&owner2.contract, &minter, &to, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_domain_separator_is_invalid() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address());
        mint(1, &owner1.contract, &minter).await;

        let to = Identity::Address(owner2.wallet.address());
        transfer_from(&owner2.contract, &minter, &to, 0).await;
    }

}
*/