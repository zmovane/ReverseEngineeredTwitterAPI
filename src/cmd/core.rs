use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider},
    signers::LocalWallet,
    types::{Address, H160, U256},
};
use eyre::Result;
use log::error;
use std::{str::FromStr, sync::Arc};

abigen!(Space, "src/abi/space.json");
abigen!(Collective, "src/abi/collective.json");

const ADDRESS_SPACE: &str = "0xc1d86DfeD6ceb43d83c7fFEf264D8C9d764751eB";
const ADDRESS_COLLECTIVE: &str = "0x9ED826624d295a8B276947d567a5438Be83aaACC";

pub struct Cmd {
    pub client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub collective: Collective<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub space: Space<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

pub struct NFTArgs {
    pub to: Address,
    pub name: String,
    pub image: String,
    pub description: String,
}

pub struct SpaceArgs {
    pub to: Address,
    pub name: String,
    pub image: String,
    pub description: String,
}

impl Cmd {
    pub fn new(client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>) -> Cmd {
        let collective_address = H160::from_str(ADDRESS_COLLECTIVE).unwrap();
        let collective = Collective::new(collective_address, client.to_owned());
        let space_address = H160::from_str(ADDRESS_SPACE).unwrap();
        let space = Space::new(space_address, client.to_owned());
        Cmd {
            client,
            collective,
            space,
        }
    }

    pub async fn mint_nft(&self, args: NFTArgs) -> Result<()> {
        let collection = String::from("");
        let result = self
            .collective
            .mint_to(collection, U256::from(1), args.image.to_string(), args.to)
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

    async fn create_space(&self, args: SpaceArgs) -> Result<()> {
        let result = self
            .space
            .create_gallery_to(
                args.to,
                H160::from_str("0").unwrap(),
                U256::from_str("0").unwrap(),
                "space".to_string(),
                args.name.to_string(),
                args.image.to_string(),
                false,
                vec![],
                vec![],
                vec![],
            )
            .send()
            .await?
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("failed to create Space: {}", e);
                Ok(())
            }
        }
    }
}
