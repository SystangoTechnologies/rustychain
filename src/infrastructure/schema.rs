// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (block_number) {
        block_number -> Int4,
        #[max_length = 66]
        block_hash -> Varchar,
        #[max_length = 66]
        parent_hash -> Varchar,
        timestamp -> Nullable<Timestamp>,
        #[max_length = 42]
        miner_address -> Varchar,
        transaction_count -> Int4,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        block_number -> Nullable<Int4>,
        #[max_length = 66]
        transaction_hash -> Varchar,
        value -> Int4,
        timestamp -> Nullable<Timestamp>,
        data -> Nullable<Text>,
    }
}

diesel::table! {
    fungible_tokens (address) {
        #[max_length = 42]
        address -> Varchar,
        #[max_length = 10]
        symbol -> Varchar,
        #[max_length = 66]
        name -> Varchar,
        #[max_length = 42]
        owner_address -> Varchar,
        decimals -> Int4,
        total_supply -> Int8,
        block_number -> Int4,
        #[max_length = 66]
        transaction_hash -> Varchar,
    }
}

diesel::table! {
    service_contexts (id) {
        id -> Int4,
        maintenance -> Bool,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        block_number -> Nullable<Int4>,
        #[max_length = 66]
        transaction_hash -> Varchar,
        #[max_length = 42]
        from_address -> Varchar,
        #[max_length = 42]
        to_address -> Varchar,
        #[max_length = 42]
        transaction_type -> Varchar,
        value -> Int8,
        timestamp -> Nullable<Timestamp>,
        data -> Nullable<Json>,
        is_mined -> Nullable<Bool>,
        #[max_length = 42]
        status -> Varchar,
    }
}

diesel::table! {
    wallets (address, token_address) {
        #[max_length = 42]
        address -> Varchar,
        #[max_length = 42]
        token_address -> Varchar,
        balance -> Int8,
        block_number -> Int4,
        #[max_length = 66]
        transaction_hash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blocks,
    events,
    fungible_tokens,
    service_contexts,
    transactions,
    wallets,
);
