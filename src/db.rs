use crate::Result;
use mongodb::{options::ClientOptions, options::ResolverConfig, Client, Collection};
const DB_NAME: &str = "RestaurantMgmt";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse_with_resolver_config(
            "mongodb+srv://TestUser:VZ1LKgzOdQOb6IDf@cluster0.edore.mongodb.net/RestaurantMgmt?retryWrites=true&w=majority",
            ResolverConfig::cloudflare(),
        )
        .await?;
        client_options.app_name = Some(DB_NAME.to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub fn get_collection(&self, coll: &str) -> Collection {
        self.client.database(DB_NAME).collection(coll)
    }
}
// pub enum Collections {
//     Counter,
//     Item,
//     OrderItem,
//     Order,
//     OrderTransaction,
//     User,
// }

// impl Collections {
//     pub fn getName(&self) -> &str {
//         match self {
//             Self::Counter => "counters",
//             Self::Item => "Item",
//             Self::OrderItem => "OrderItem",
//             Self::Order => "Orders",
//             Self::OrderTransaction => "OrderTransaction",
//             Self::User => "User",
//         }
//     }
// }
