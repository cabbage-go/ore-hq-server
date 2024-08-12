// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct TxnsTypeEnum;
}

diesel::table! {
    challenges (id) {
        id -> Integer,
        pool_id -> Integer,
        submission_id -> Nullable<Integer>,
        #[max_length = 32]
        challenge -> Binary,
        rewards_earned -> Nullable<Bigint>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    claims (id) {
        id -> Integer,
        miner_id -> Integer,
        pool_id -> Integer,
        txn_id -> Integer,
        amount -> Tinyint,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    miners (id) {
        id -> Integer,
        #[max_length = 44]
        pubkey -> Varchar,
        enabled -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pools (id) {
        id -> Integer,
        #[max_length = 44]
        proof_pubkey -> Varchar,
        total_rewards -> Nullable<Bigint>,
        claimed_rewards -> Nullable<Bigint>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    submissions (id) {
        id -> Integer,
        miner_id -> Integer,
        challenge_id -> Integer,
        difficulty -> Tinyint,
        nonce -> Bigint,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TxnsTypeEnum;

    txns (id) {
        id -> Integer,
        #[sql_name = "type"]
        #[max_length = 5]
        type_ -> Nullable<TxnsTypeEnum>,
        #[max_length = 200]
        signature -> Varchar,
        priority_fee -> Nullable<Unsigned<Integer>>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    challenges,
    claims,
    miners,
    pools,
    submissions,
    txns,
);
