use exchange_sign_hook::SignClosure;

use crate::client::ApiCred;
use crate::client::Config;
use crate::client::KeyClosure;
use crate::client::RestClient;

mod certificates;
mod close_order;
mod create_order;
mod query_order;
mod transfer_fund;
mod webhook;

pub mod prelude {
    pub use super::certificates::Certificate;
    pub use super::certificates::CertificateRequest;
    pub use super::certificates::CertificateResponse;
    pub use super::close_order::CloseOrderRequest;
    pub use super::close_order::CloseOrderResponse;
    pub use super::create_order::CreateOrderRequest;
    pub use super::create_order::CreateOrderResponse;
    pub use super::create_order::OrderResult;
    pub use super::query_order::PayerInfo;
    pub use super::query_order::QueryOrderRequest;
    pub use super::query_order::QueryOrderResponse;
    pub use super::query_order::QueryOrderResult;
    pub use super::transfer_fund::TransferFundRequest;
    pub use super::transfer_fund::TransferFundResponse;
    pub use super::transfer_fund::TransferResult;
    pub use super::transfer_fund::TransferStatus;
    pub use super::transfer_fund::TransferType;
    pub use super::webhook::BinancePayWebHookRequest;
    pub use super::webhook::BinancePayWebHookResponse;
    pub use super::webhook::BizStatus;
    pub use super::webhook::Notification;
    pub use super::webhook::ReturnCode;
}

#[derive(Clone, Default)]
pub struct Api {
    pub client: RestClient,
}

impl Api {
    pub fn new() -> Self {
        Api::default()
    }

    pub fn from_env() -> Self {
        Api::with_config(Config::from_env())
    }

    pub fn with_cred(cred: ApiCred) -> Self {
        let signer = cred.into();
        Api::with_config(Config {
            signer,
            ..Config::default()
        })
    }

    pub fn with_closure(api_key: String, closure: SignClosure) -> Self {
        let signer = KeyClosure::new(api_key, closure).into();
        Api::with_config(Config {
            signer,
            ..Config::default()
        })
    }

    pub fn with_config(config: Config) -> Self {
        let client = RestClient::with_config(config);
        Api { client }
    }

    pub fn merchant_id(&self) -> u64 {
        self.client.merchant_id()
    }
}

pub mod json_string {
    use serde::de::{self, Deserialize, DeserializeOwned, Deserializer};
    use serde::ser::{self, Serialize, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        let j = serde_json::to_string(value).map_err(ser::Error::custom)?;
        j.serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>,
    {
        let j = String::deserialize(deserializer)?;
        serde_json::from_str(&j).map_err(de::Error::custom)
    }
}

pub mod uuid_simple {
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(value: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.to_simple().to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        Uuid::deserialize(deserializer)
    }
}

pub mod opt_uuid_simple {
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(value: &Option<Uuid>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(uuid) => uuid.to_simple().to_string().serialize(serializer),
            None => Option::<String>::None.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<Uuid>::deserialize(deserializer)
    }
}
