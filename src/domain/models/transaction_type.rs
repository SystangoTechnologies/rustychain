use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub enum TransactionType {
    InitFt,
    MintFt,
    BurnFt,
    TransferFt,
    InitNft,
    MintNft,
    BurnNft,
    TransferNft,
    None,
}

impl TransactionType {
    pub fn as_str(&self) -> &str {
        match self {
            TransactionType::InitFt => "INIT_FT",
            TransactionType::MintFt => "MINT_FT",
            TransactionType::BurnFt => "BURN_FT",
            TransactionType::TransferFt => "TRANSFER_FT",
            TransactionType::InitNft => "INIT_NFT",
            TransactionType::MintNft => "MINT_NFT",
            TransactionType::BurnNft => "BURN_NFT",
            TransactionType::TransferNft => "TRANSFER_NFT",
            TransactionType::None => "",
        }
    }

    pub fn from_str(s: &str) -> TransactionType {
        match s {
            "INIT_FT" => TransactionType::InitFt,
            "MINT_FT" => TransactionType::MintFt,
            "BURN_FT" => TransactionType::BurnFt,
            "TRANSFER_FT" => TransactionType::TransferFt,
            "INIT_NFT" => TransactionType::InitNft,
            "MINT_NFT" => TransactionType::MintNft,
            "BURN_NFT" => TransactionType::BurnNft,
            "TRANSFER_NFT" => TransactionType::TransferNft,
            _ => TransactionType::None,
        }
    }
}

pub struct InitFt {
    pub symbol: String,
    pub name: String,
    pub decimals: i32,
}

impl Into<InitFt> for Value {
    fn into(self) -> InitFt {
        let symbol = self.get("symbol").and_then(|s| s.as_str()).unwrap_or_default();
        let name = self.get("name").and_then(|s| s.as_str()).unwrap_or_default();
        let decimals = self.get("decimals").and_then(|d| d.as_i64()).unwrap_or(0) as i32;

        InitFt {
            symbol: symbol.to_string(),
            name: name.to_string(),
            decimals,
        }
    }
}

pub struct TransferFt {
    pub token_address: String,
}

impl Into<TransferFt> for Value {
    fn into(self) -> TransferFt {
        let token_address = self.get("token_address").and_then(|s| s.as_str()).unwrap_or_default();

        TransferFt {
            token_address: token_address.to_string(),
        }
    }
}

pub struct MintFt {
    pub token_address: String,
}

impl Into<MintFt> for Value {
    fn into(self) -> MintFt {
        let token_address = self.get("token_address").and_then(|s| s.as_str()).unwrap_or_default();

        MintFt {
            token_address: token_address.to_string(),
        }
    }
}

pub struct BurnFt {
    pub token_address: String,
}

impl Into<BurnFt> for Value {
    fn into(self) -> BurnFt {
        let token_address = self.get("token_address").and_then(|s| s.as_str()).unwrap_or_default();

        BurnFt {
            token_address: token_address.to_string(),
        }
    }
}
