use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey, };

use crate::error::BinaryOptionError;
use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct BinaryOption {
    pub decimals: u8,
    pub expiry: u64,
    pub strike: u64,
    pub strike_exponent: i64,
    pub circulation: u64,
    pub settled: bool,
    pub escrow_mint_account_pubkey: Pubkey,
    pub escrow_account_pubkey: Pubkey,
    pub long_mint_account_pubkey: Pubkey,
    pub short_mint_account_pubkey: Pubkey,
    pub owner: Pubkey,
    pub winning_side_pubkey: Pubkey,
    //pub underlying_asset_address: String,
    //let clock = Clock::get();
    // clock.unix_timestamp; 
    // note pub unix_timestamp: UnixTimestamp,
    // pub type UnixTimestamp = i64;

}

impl BinaryOption {
    pub const LEN: usize = 226;
    // u8 = 1 
    // u64 = 8
    // pubkey 32

    pub fn from_account_info(a: &AccountInfo) -> Result<BinaryOption, ProgramError> {
        let binary_option = BinaryOption::try_from_slice(&a.data.borrow_mut())?;
        Ok(binary_option)
    }

    pub fn increment_supply(&mut self, n: u64) {
        self.circulation += n;
    }

    pub fn decrement_supply(&mut self, n: u64) -> ProgramResult {
        if self.circulation < n {
            return Err(BinaryOptionError::InvalidSupply.into());
        }
        self.circulation -= n;
        Ok(())
    }
}
