use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider},
    signers::LocalWallet,
    types::{Address, H160, U256},
};
use eyre::Result;
use std::{str::FromStr, sync::Arc};

abigen!(Space, "src/abi/space.json");

pub struct SpaceCreation {
    to: Address,
    name: String,
    image: String,
    description: String,
}
const ADDRESS_SPACE: &str = "0xc1d86DfeD6ceb43d83c7fFEf264D8C9d764751eB";

impl SpaceCreation {
    async fn excute(
        &self,
        client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    ) -> Result<()> {
        let contract_address = H160::from_str(ADDRESS_SPACE).unwrap();
        let space = Space::new(contract_address, client);
        space
            .create_gallery_to(
                self.to,
                H160::from_str("0").unwrap(),
                U256::from_str("0").unwrap(),
                "space".to_string(),
                self.name.to_string(),
                self.image.to_string(),
                false,
                vec![],
                vec![],
                vec![],
            )
            .send()
            .await?
            .await?;
        Ok(())
    }
}
