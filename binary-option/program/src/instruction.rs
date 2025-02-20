use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InitializeBinaryOptionArgs {
    pub decimals: u8,
    pub expiry: u64,
    pub strike: u64,
    pub strike_exponent: i64,
   // pub underlying_asset_address: String,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct TradeArgs {
    pub size: u64,
    pub buy_price: u64,
    pub sell_price: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum BinaryOptionInstruction {
    // TODO: Add comments here
    InitializeBinaryOption(InitializeBinaryOptionArgs),

    Trade(TradeArgs),

    SettleOracle,

    Collect,
}

/// Creates an InitializeBinaryOption instruction
#[allow(clippy::too_many_arguments)]
pub fn initialize_binary_option(

    program_id: Pubkey, // Address of deployed program
    pool_account: Pubkey, // Address of Upgrade Authority
    escrow_mint: Pubkey, // TBD
    escrow_account: Pubkey, // Unique per option [ generated by front-end SPL library ]
    long_token_mint: Pubkey, // Unique per option [ generated by front-end SPL library ]
    short_token_mint: Pubkey, // Unique per option [ generated by front-end SPL library ]
    mint_authority: Pubkey, // Fixed address [ generated/chosen by us ]
    update_authority: Pubkey, // Address of Upgrade Authority
    decimals: u8, // 2 = 100, 3 = 1000 [ only using 2 for now ]
    expiry:  u64, // Unix timestamp
    strike: u64, // strike_price
    strike_exponent: i64, // strike_price
    //underlying_asset_address: Pubkey, // Address of underlying asset [ from Pyth for now ]

) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(pool_account, true),
            AccountMeta::new_readonly(escrow_mint, false),
            AccountMeta::new(escrow_account, true),
            AccountMeta::new_readonly(long_token_mint, true),
            AccountMeta::new_readonly(short_token_mint, true),
            AccountMeta::new_readonly(mint_authority, true),
            AccountMeta::new_readonly(update_authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: BinaryOptionInstruction::InitializeBinaryOption(InitializeBinaryOptionArgs {
            decimals,
            expiry,
            strike,
            strike_exponent,
        })
        .try_to_vec()
        .unwrap(),
    }
}

/// Creates a Trade instruction
#[allow(clippy::too_many_arguments)]
pub fn trade(
    program_id: Pubkey,
    pool_account: Pubkey,
    escrow_account: Pubkey,
    long_token_mint: Pubkey,
    short_token_mint: Pubkey,
    buyer: Pubkey,
    seller: Pubkey,
    buyer_account: Pubkey,
    seller_account: Pubkey,
    buyer_long_token_account: Pubkey,
    buyer_short_token_account: Pubkey,
    seller_long_token_account: Pubkey,
    seller_short_token_account: Pubkey,
    escrow_authority: Pubkey,
    size: u64,
    buy_price: u64,
    sell_price: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(pool_account, false),
            AccountMeta::new(escrow_account, false),
            AccountMeta::new(long_token_mint, false),
            AccountMeta::new(short_token_mint, false),
            AccountMeta::new_readonly(buyer, true),
            AccountMeta::new_readonly(seller, true),
            AccountMeta::new(buyer_account, false),
            AccountMeta::new(seller_account, false),
            AccountMeta::new(buyer_long_token_account, false),
            AccountMeta::new(buyer_short_token_account, false),
            AccountMeta::new(seller_long_token_account, false),
            AccountMeta::new(seller_short_token_account, false),
            AccountMeta::new_readonly(escrow_authority, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: BinaryOptionInstruction::Trade(TradeArgs {
            size,
            buy_price,
            sell_price,
        })
        .try_to_vec()
        .unwrap(),
    }
}

/// Creates a Settle instruction
 /*
pub fn settle(
    program_id: Pubkey,
    pool_account: Pubkey,
    winning_mint: Pubkey,
    pool_authority: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(pool_account, false),
            AccountMeta::new_readonly(winning_mint, false),
            AccountMeta::new_readonly(pool_authority, true),
        ],
        data: BinaryOptionInstruction::Settle.try_to_vec().unwrap(),
    }
}
*/

/// Creates a Settle instruction using oracle
pub fn settle_oracle(
    program_id: Pubkey,
    pool_account: Pubkey,
    long_token_mint: Pubkey,
    short_token_mint: Pubkey,
    pool_authority: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(pool_account, false),
            AccountMeta::new_readonly(long_token_mint, false),
            AccountMeta::new_readonly(short_token_mint, false),
            AccountMeta::new_readonly(pool_authority, true),
        ],
        data: BinaryOptionInstruction::SettleOracle.try_to_vec().unwrap(),
    }
}

/// Create a Collect instruction
#[allow(clippy::too_many_arguments)]
pub fn collect(
    program_id: Pubkey,
    pool_account: Pubkey,
    collector_account: Pubkey,
    collector_long_token_account: Pubkey,
    collector_short_token_account: Pubkey,
    collector_collateral_account: Pubkey,
    long_token_mint_account: Pubkey,
    short_token_mint_account: Pubkey,
    escrow_account: Pubkey,
    escrow_authority_account: Pubkey,
    fee_payer_account: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(pool_account, false),
            AccountMeta::new_readonly(collector_account, false),
            AccountMeta::new(collector_long_token_account, false),
            AccountMeta::new(collector_short_token_account, false),
            AccountMeta::new(collector_collateral_account, false),
            AccountMeta::new(long_token_mint_account, false),
            AccountMeta::new(short_token_mint_account, false),
            AccountMeta::new(escrow_account, false),
            AccountMeta::new(escrow_authority_account, false),
            AccountMeta::new_readonly(fee_payer_account, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: BinaryOptionInstruction::Collect.try_to_vec().unwrap(),
    }
}
