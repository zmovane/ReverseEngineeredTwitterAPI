use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider},
    signers::LocalWallet,
    types::{Address, H160, U256},
};
use eyre::Result;
use log::error;
use std::{str::FromStr, sync::Arc};

abigen!(Collective, "src/abi/collective.json");

const ADDRESS_SINGLE_COLLECTIVE: &str = "0x9ED826624d295a8B276947d567a5438Be83aaACC";

pub struct NFTMinting {
    pub to: Address,
    pub name: String,
    pub image: String,
    pub description: String,
}

impl NFTMinting {
    pub async fn excute(
        &self,
        client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    ) -> Result<()> {
        let collection = "".to_string();
        let contract_address = H160::from_str(ADDRESS_SINGLE_COLLECTIVE).unwrap();
        let collective = Collective::new(contract_address, client);
        let result = collective
            .mint_to(collection, U256::from(1), self.image.to_string(), self.to)
            .send()
            .await?
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("failed to mint NFT: {}", e);
                Ok(())
            }
        }
    }
}
