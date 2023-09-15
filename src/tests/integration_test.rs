#[cfg(test)]
mod integration_tests {
    use actix_web::test;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use rustychain::api::dto::block::BlockDTO;
    use rustychain::api::dto::fungible_token::FungibleTokenDTO;
    use rustychain::api::dto::transaction::TransactionDTO;
    use rustychain::api::dto::wallet::WalletDTO;
    use rustychain::create_app::create_app;
    use rustychain::domain::constants::POSTGRESQL_DB_URI;
    use rustychain::domain::models::transaction::TransactionStatus;
    use rustychain::domain::repositories::repository::ResultPaging;
    use rustychain::infrastructure::databases::postgresql::db_pool;
    use serde_json::json;
    use serde_json::{self, Value};
    use std::env;
    use std::sync::Arc;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    pub const PG_CONNECTION_STRING: &str = "postgresql://rusty:rusty@localhost:5432/rustychain";

    // Test data
    pub const APPLE_TOKEN_NAME: &str = "Apple";
    pub const APPLE_TOKEN_SYMBOL: &str = "APPLE";
    pub const APPLE_TOKEN_DECIMALS: i32 = 0;
    pub const APPLE_TOKEN_INITIAL_SUPPLY: i64 = 100;
    pub const SONY_TOKEN_NAME: &str = "Sony";
    pub const SONY_TOKEN_SYMBOL: &str = "SONY";
    pub const SONY_TOKEN_DECIMALS: i32 = 0;
    pub const SONY_TOKEN_INITIAL_SUPPLY: i64 = 200;
    pub const OWNER_ADDRESS: &str = "0x00000000000000000000000000000000000ARPIT";
    pub const SYSTEM_CONTRACT_ADDRESS: &str = "0x00000000000000000000000000SYSTEMCONTRACT";
    pub const USER1_ADDRESS: &str = "0x00000000000000000000000000000000000SCOTT";
    pub const USER2_ADDRESS: &str = "0x0000000000000000000000000000000000ZENITH";
    pub const BLOCK_MINER_ADDRESS: &str = "0x00000000000000000000000000000000000MINER";
    pub const NON_EXISTENT_FUNGIBLE_TOKEN_ADDRESS: &str = "0x00000000000000000000000000000000NONEXIST";

    // API Paths
    pub const API_TRANSACTION_PATH: &str = "/api/transactions";
    pub const API_BLOCKS_PATH: &str = "/api/blocks";
    pub const API_WALLET_PATH: &str = "/api/wallets";
    pub const API_FUNGIBLE_TOKENS_PATH: &str = "/api/fts";

    /*
     * Returns a TestRequest for post API request
     */
    fn post_request(path: &str, body: &Value) -> test::TestRequest {
        test::TestRequest::post().uri(path).set_json(body)
    }

    /*
     * Returns a TestRequest for listing a transaction by it's hash
     */
    fn get_request(path: &str) -> test::TestRequest {
        test::TestRequest::get().uri(path)
    }

    /*
     * Returns a TestRequest for listing fungible tokens with given limit and offset
     */
    fn list_request(path: &str, limit: i32, offset: i32) -> test::TestRequest {
        test::TestRequest::get().uri(path).param("limit", limit.to_string()).param("offset", offset.to_string())
    }

    #[actix_web::test]
    async fn test_1_init_ft() {
        env::set_var("RUST_LOG", "info");
        env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();
        env::set_var(POSTGRESQL_DB_URI, PG_CONNECTION_STRING);

        let pool = Arc::new(db_pool());
        pool.get().unwrap().run_pending_migrations(MIGRATIONS).unwrap();
        let app = test::init_service(create_app()).await;

        // Test-1 : It should create a INIT_FT transaction
        let request_body = json!({
            "from_address": OWNER_ADDRESS,
            "to_address": SYSTEM_CONTRACT_ADDRESS,
            "transaction_type": "INIT_FT",
            "value": APPLE_TOKEN_INITIAL_SUPPLY,
            "data": {
                "symbol": APPLE_TOKEN_SYMBOL,
                "name": APPLE_TOKEN_NAME,
                "decimals": APPLE_TOKEN_DECIMALS
            }
        });
        let resp = post_request(API_TRANSACTION_PATH, &request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let txn: TransactionDTO = test::read_body_json(resp).await;
        let result = validate_transaction(&txn, OWNER_ADDRESS, SYSTEM_CONTRACT_ADDRESS, "INIT_FT", APPLE_TOKEN_INITIAL_SUPPLY, TransactionStatus::RAW).await;
        println!("test_init_ft : TEST-1 : PASS = {}", result);

        // Test-2 : It should create a new block
        let block: BlockDTO = mine_block().await;
        let txn: TransactionDTO = get_transaction_by_hash(&txn.transaction_hash).await;
        let result = validate_block(&block, &txn, 1, BLOCK_MINER_ADDRESS, 1, TransactionStatus::SUCCESS).await;
        println!("test_init_ft : TEST-2 : PASS = {}", result);

        // Test-3 : It should create the fungible token for owner
        let fungible_token: FungibleTokenDTO = get_ft_by_index(0).await;
        let result = validate_fungible_token(
            &fungible_token,
            &txn,
            APPLE_TOKEN_NAME,
            APPLE_TOKEN_SYMBOL,
            APPLE_TOKEN_DECIMALS,
            APPLE_TOKEN_INITIAL_SUPPLY,
            OWNER_ADDRESS,
            1,
        )
        .await;
        println!("test_init_ft : TEST-3 : PASS = {}", result);

        // Test-4 : It should create a wallet for owner and credit the initial supply to it
        let owner_wallet_details: WalletDTO = get_wallet_by_address(OWNER_ADDRESS, &fungible_token.address).await;
        let result = validate_wallet(&owner_wallet_details, &txn, &fungible_token, OWNER_ADDRESS, APPLE_TOKEN_INITIAL_SUPPLY, 1).await;
        println!("test_init_ft : TEST-4 : PASS = {}", result);
    }

    #[actix_web::test]
    async fn test_2_mint_ft() {
        let app = test::init_service(create_app()).await;

        // Test-1 : It should create a INIT_FT transaction
        let request_body = json!({
            "from_address": OWNER_ADDRESS,
            "to_address": SYSTEM_CONTRACT_ADDRESS,
            "transaction_type": "INIT_FT",
            "value": SONY_TOKEN_INITIAL_SUPPLY,
            "data": {
                "symbol": SONY_TOKEN_SYMBOL,
                "name": SONY_TOKEN_NAME,
                "decimals": SONY_TOKEN_DECIMALS
            }
        });
        let resp = post_request(API_TRANSACTION_PATH, &request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let txn: TransactionDTO = test::read_body_json(resp).await;
        let result = validate_transaction(&txn, OWNER_ADDRESS, SYSTEM_CONTRACT_ADDRESS, "INIT_FT", SONY_TOKEN_INITIAL_SUPPLY, TransactionStatus::RAW).await;
        println!("test_mint_ft : TEST-1 : PASS = {}", result);

        // Test-2 : It should create a new block
        let block: BlockDTO = mine_block().await;
        let txn: TransactionDTO = get_transaction_by_hash(&txn.transaction_hash).await;
        let result = validate_block(&block, &txn, 2, BLOCK_MINER_ADDRESS, 1, TransactionStatus::SUCCESS).await;
        println!("test_mint_ft : TEST-2 : PASS = {}", result);

        // Test-3 : It should create the fungible token for owner
        let fungible_token: FungibleTokenDTO = get_ft_by_index(1).await;
        let result = validate_fungible_token(
            &fungible_token,
            &txn,
            SONY_TOKEN_NAME,
            SONY_TOKEN_SYMBOL,
            SONY_TOKEN_DECIMALS,
            SONY_TOKEN_INITIAL_SUPPLY,
            OWNER_ADDRESS,
            2,
        )
        .await;
        println!("test_mint_ft : TEST-3 : PASS = {}", result);

        // Test-4 : It should create a MINT_FT transaction
        let request_body = json!({
            "from_address": OWNER_ADDRESS,
            "to_address": USER1_ADDRESS,
            "transaction_type": "MINT_FT",
            "value": 100,
            "data": {
                "token_address": fungible_token.address
            }
        });
        let resp = post_request(API_TRANSACTION_PATH, &request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let txn_2: TransactionDTO = test::read_body_json(resp).await;
        let result = validate_transaction(&txn_2, OWNER_ADDRESS, USER1_ADDRESS, "MINT_FT", 100, TransactionStatus::RAW).await;
        println!("test_mint_ft : TEST-4 : PASS = {}", result);

        // Test-5 : It should create a new block
        let block: BlockDTO = mine_block().await;
        let txn_2: TransactionDTO = get_transaction_by_hash(&txn_2.transaction_hash).await;
        let result = validate_block(&block, &txn, 3, BLOCK_MINER_ADDRESS, 1, TransactionStatus::SUCCESS).await;
        println!("test_mint_ft : TEST-5 : PASS = {}", result);

        // Test-6 : It should create a wallet for user_1 and credit the supply of 100 tokens to it
        let user1_wallet_details: WalletDTO = get_wallet_by_address(USER1_ADDRESS, &fungible_token.address).await;
        let fungible_token: FungibleTokenDTO = get_ft_by_index(1).await;
        let result = validate_wallet(&user1_wallet_details, &txn_2, &fungible_token, USER1_ADDRESS, 100, 3).await;
        assert_eq!(fungible_token.total_supply, SONY_TOKEN_INITIAL_SUPPLY + 100);
        println!("test_mint_ft : TEST-6 : PASS = {}", result);
    }

    #[actix_web::test]
    async fn test_3_transfer_ft() {
        let app = test::init_service(create_app()).await;

        // Fetch SONY token details
        let fungible_token: FungibleTokenDTO = get_ft_by_index(1).await;

        // Test-1 : It should transfer SONY fungible tokens from user_1 to user_2
        let request_body = json!({
            "from_address": USER1_ADDRESS,
            "to_address": USER2_ADDRESS,
            "transaction_type": "TRANSFER_FT",
            "value": 30,
            "data": {
                "token_address": fungible_token.address
            }
        });
        let resp = post_request(API_TRANSACTION_PATH, &request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let txn: TransactionDTO = test::read_body_json(resp).await;
        let result = validate_transaction(&txn, USER1_ADDRESS, USER2_ADDRESS, "TRANSFER_FT", 30, TransactionStatus::RAW).await;
        println!("test_transfer_ft : TEST-1 : PASS = {}", result);

        // Test-2 : It should create a new block
        let block: BlockDTO = mine_block().await;
        let txn: TransactionDTO = get_transaction_by_hash(&txn.transaction_hash).await;
        let result = validate_block(&block, &txn, 4, BLOCK_MINER_ADDRESS, 1, TransactionStatus::SUCCESS).await;
        println!("test_transfer_ft : TEST-2 : PASS = {}", result);

        // Test-3 : It should result in successful balance update in user_1 and user_2 wallets
        let user1_wallet: WalletDTO = get_wallet_by_address(USER1_ADDRESS, &fungible_token.address).await;
        let user2_wallet: WalletDTO = get_wallet_by_address(USER2_ADDRESS, &fungible_token.address).await;
        let result_1 = validate_wallet(&user1_wallet, &txn, &fungible_token, USER1_ADDRESS, 70, 4).await;
        let result_2 = validate_wallet(&user2_wallet, &txn, &fungible_token, USER2_ADDRESS, 30, 4).await;
        println!("test_transfer_ft : TEST-3 : PASS = {}", result_1 & result_2);
    }

    #[actix_web::test]
    async fn test_4_burn_ft() {
        let app = test::init_service(create_app()).await;

        // Fetch SONY token details
        let fungible_token: FungibleTokenDTO = get_ft_by_index(1).await;

        // Test-1 : It should create a BURN_FT transaction
        let request_body = json!({
            "from_address": USER1_ADDRESS,
            "to_address": "",
            "transaction_type": "BURN_FT",
            "value": 50,
            "data": {
                "token_address": fungible_token.address
            }
        });
        let resp = post_request(API_TRANSACTION_PATH, &request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let txn: TransactionDTO = test::read_body_json(resp).await;
        let result = validate_transaction(&txn, USER1_ADDRESS, "", "BURN_FT", 50, TransactionStatus::RAW).await;
        println!("test_burn_ft : TEST-1 : PASS = {}", result);

        // Test-2 : It should create a new block
        let block: BlockDTO = mine_block().await;
        let txn: TransactionDTO = get_transaction_by_hash(&txn.transaction_hash).await;
        let result = validate_block(&block, &txn, 5, BLOCK_MINER_ADDRESS, 1, TransactionStatus::SUCCESS).await;
        println!("test_burn_ft : TEST-2 : PASS = {}", result);

        // Fetch SONY token details
        let fungible_token: FungibleTokenDTO = get_ft_by_index(1).await;

        // Test-3 : It should burn tokens from user_1 and update total_supply of the fungible token
        let user1_wallet: WalletDTO = get_wallet_by_address(USER1_ADDRESS, &fungible_token.address).await;
        let result = validate_wallet(&user1_wallet, &txn, &fungible_token, USER1_ADDRESS, 20, 5).await;
        assert_eq!(fungible_token.total_supply, SONY_TOKEN_INITIAL_SUPPLY + 100 - 50);
        println!("test_burn_ft : TEST-3 : PASS = {}", result);
    }

    #[actix_web::test]
    async fn test_5_fail_transfer_non_existent_ft() {
        let app = test::init_service(create_app()).await;

        // Test-16 : It should fail to transfer a non existent token
        let request_body = json!({
            "from_address": USER1_ADDRESS,
            "to_address": USER2_ADDRESS,
            "transaction_type": "TRANSFER_FT",
            "value": 30,
            "data": {
                "token_address": NON_EXISTENT_FUNGIBLE_TOKEN_ADDRESS
            }
        });
        let resp = post_request(API_TRANSACTION_PATH, &request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let txn: TransactionDTO = test::read_body_json(resp).await;
        let result = validate_transaction(&txn, USER1_ADDRESS, USER2_ADDRESS, "TRANSFER_FT", 30, TransactionStatus::RAW).await;
        println!("test_fail_transfer_non_existent_ft : TEST-1 : PASS = {}", result);

        // Test-2 : It should create a new block with failed transaction
        let block: BlockDTO = mine_block().await;
        let txn: TransactionDTO = get_transaction_by_hash(&txn.transaction_hash).await;
        let result = validate_block(&block, &txn, 6, BLOCK_MINER_ADDRESS, 1, TransactionStatus::FAIL).await;
        println!("test_fail_transfer_non_existent_ft : TEST-2 : PASS = {}", result);
    }

    #[actix_web::test]
    async fn test_6_fail_transfer_excess_ft() {
        let app = test::init_service(create_app()).await;

        // Fetch SONY token details
        let fungible_token: FungibleTokenDTO = get_ft_by_index(1).await;

        // Test-1 : It should not transfer SONY fungible tokens from user_1 to user_2 due to insufficient balance
        let request_body = json!({
            "from_address": USER1_ADDRESS,
            "to_address": USER2_ADDRESS,
            "transaction_type": "TRANSFER_FT",
            "value": 100,
            "data": {
                "token_address": fungible_token.address
            }
        });
        let resp = post_request(API_TRANSACTION_PATH, &request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let txn: TransactionDTO = test::read_body_json(resp).await;
        let result = validate_transaction(&txn, USER1_ADDRESS, USER2_ADDRESS, "TRANSFER_FT", 100, TransactionStatus::RAW).await;
        println!("test_fail_transfer_excess_ft : TEST-1 : PASS = {}", result);

        // Test-2 : It should create a new block
        let block: BlockDTO = mine_block().await;
        let txn: TransactionDTO = get_transaction_by_hash(&txn.transaction_hash).await;
        let result = validate_block(&block, &txn, 7, BLOCK_MINER_ADDRESS, 1, TransactionStatus::FAIL).await;
        println!("test_fail_transfer_excess_ft : TEST-2 : PASS = {}", result);
    }

    /*
     * Mines and returns the newly created block
     */
    async fn mine_block() -> BlockDTO {
        let app = test::init_service(create_app()).await;
        let request_body = json!({
            "miner_address": BLOCK_MINER_ADDRESS,
        });
        let resp = post_request(API_BLOCKS_PATH, &request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let block: BlockDTO = test::read_body_json(resp).await;
        block
    }

    /*
     * Returns the transaction details for the given transaction hash
     */
    async fn get_transaction_by_hash(txn_hash: &str) -> TransactionDTO {
        let app = test::init_service(create_app()).await;
        let resp = get_request(&format!("{}/{}", API_TRANSACTION_PATH, txn_hash)).send_request(&app).await;
        assert!(resp.status().is_success());
        let transaction: TransactionDTO = test::read_body_json(resp).await;
        transaction
    }

    /*
     * Returns the wallet details for the given wallet address and token address
     */
    async fn get_wallet_by_address(wallet_address: &str, token_address: &str) -> WalletDTO {
        let app = test::init_service(create_app()).await;
        let resp = get_request(&format!("{}/{}/{}", API_WALLET_PATH, wallet_address, token_address)).send_request(&app).await;
        assert!(resp.status().is_success());
        let wallet: WalletDTO = test::read_body_json(resp).await;
        wallet
    }

    /*
     * Returns the fungible token details for the given index
     */
    async fn get_ft_by_index(index: usize) -> FungibleTokenDTO {
        let app = test::init_service(create_app()).await;
        let resp = list_request(API_FUNGIBLE_TOKENS_PATH, 2, 0).send_request(&app).await;
        assert!(resp.status().is_success());
        let fungible_token_result: ResultPaging<FungibleTokenDTO> = test::read_body_json(resp).await;
        let fungible_token: FungibleTokenDTO = fungible_token_result.items[index].clone();
        fungible_token
    }

    /*
     *  Validates the transaction details
     */
    async fn validate_transaction(txn: &TransactionDTO, from_address: &str, to_address: &str, transaction_type: &str, value: i64, transaction_status: TransactionStatus) -> bool {
        assert_eq!(txn.from_address, from_address);
        assert_eq!(txn.to_address, to_address);
        assert_eq!(txn.transaction_type.as_str(), transaction_type);
        assert_eq!(txn.value, value);
        assert!(!txn.transaction_hash.is_empty());
        assert_eq!(txn.status, transaction_status.to_string());
        true
    }

    /*
     * Validates the block details
     */
    async fn validate_block(block: &BlockDTO, txn: &TransactionDTO, block_number: i32, miner_address: &str, transaction_count: i32, transaction_status: TransactionStatus) -> bool {
        assert!(block.block_number == block_number);
        assert!(!block.block_hash.is_empty());
        assert!(!block.parent_hash.is_empty());
        assert_eq!(block.miner_address, miner_address);
        assert!(block.transaction_count == transaction_count);
        assert_eq!(txn.status, transaction_status.to_string());
        true
    }

    /*
     * Validates the fungible token details
     */
    async fn validate_fungible_token(
        token: &FungibleTokenDTO,
        txn: &TransactionDTO,
        token_name: &str,
        token_symbol: &str,
        token_decimals: i32,
        token_supply: i64,
        owner_address: &str,
        block_number: i32,
    ) -> bool {
        assert!(!token.address.is_empty());
        assert_eq!(token.name, token_name);
        assert_eq!(token.symbol, token_symbol);
        assert_eq!(token.decimals, token_decimals);
        assert_eq!(token.total_supply, token_supply);
        assert_eq!(token.owner_address, owner_address);
        assert!(token.block_number == block_number);
        assert_eq!(token.transaction_hash, txn.transaction_hash);
        true
    }

    /*
     *  Validates the wallet details
     */
    async fn validate_wallet(wallet: &WalletDTO, txn: &TransactionDTO, fungible_token: &FungibleTokenDTO, address: &str, balance: i64, block_number: i32) -> bool {
        assert!(!wallet.wallet_address.is_empty());
        assert_eq!(wallet.wallet_address, address);
        assert_eq!(wallet.token_address, fungible_token.address);
        assert_eq!(wallet.balance, balance);
        assert_eq!(wallet.updated_at_block_number, block_number);
        assert_eq!(wallet.updated_by_transaction_hash, txn.transaction_hash);
        true
    }
}
