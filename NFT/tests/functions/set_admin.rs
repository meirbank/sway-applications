use crate::utils::{
    abi_calls::{balance_of, constructor, mint, set_admin},
    test_helpers::setup,
    Identity,
};
use fuels::signers::Signer;

mod success {

    use super::*;

    #[tokio::test]
    async fn changes_admin() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // let admin = Option::Some(Identity::Address(owner1.wallet.address()));
        let admin = Identity::Address(owner1.wallet.address());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner2.wallet.address());
        // let new_admin = Option::Some(minter.clone());
        let new_admin = minter.clone();
        set_admin(&owner1.contract, &new_admin).await;

        assert_eq!(balance_of(&owner2.contract, &minter).await, 0);

        mint(1, &owner2.contract, &minter).await;

        assert_eq!(balance_of(&owner2.contract, &minter).await, 1);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_admin_not_set() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        // let admin = Option::Some(Identity::Address(owner1.wallet.address()));
        let admin = Identity::Address(owner1.wallet.address());
        set_admin(&owner1.contract, &admin).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_not_admin_identity() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // let admin = Option::Some(Identity::Address(owner1.wallet.address()));
        let admin = Identity::Address(owner1.wallet.address());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        // let new_admin = Option::Some(Identity::Address(owner2.wallet.address()));
        let new_admin = Identity::Address(owner2.wallet.address());
        set_admin(&owner2.contract, &new_admin).await;
    }
}
