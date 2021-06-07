use std::convert::TryInto;
use solana_program::program_error::ProgramError;
use arrayref::{array_ref, array_refs};

use crate::error::RevenueSharingError::InvalidInstruction;

pub enum RevenueSharingInstruction {

    /// Initializes the revenue sharing by: 
    /// * Creating and populating a revenue sharing state account
    /// * Transferring ownership of the shared account to the PDA
    ///
    /// Accounts expected:
    /// 0. `[signer]` 
    ///    * The account of the initializer
    ///    * Transfering ownership of shared account requires signature of initializer
    ///
    /// 1. `[writable]` 
    ///    * Shared account: token account that holds tokens to be shared between members
    ///    * Should be created prior to this instruction and owned by the initializer
    ///    * Should be writable because its ownership will be transfered to the PDA
    ///
    /// 2. `[writable]` 
    ///    * State account
    ///    * Stores data about the revenue sharing: member public keys, member shares and the amount each member has already withdrawn
    ///
    /// 3. `[]` The rent sysvar
    ///
    /// 4. `[]` The token program account
    ///
    /// 5. `[]` Main account of member 1
    /// 
    /// 6. `[]` Main account of member 2
    /// 
    /// NOTES: This is a proof of concept that supports only 2 members
    /// 
    InitRevenueSharing {
        member_1_shares: u16,
        member_2_shares: u16,
    },

    /// Withdraw instruction
    /// Allow members to withdraw their shares from the shared account
    /// 
    /// Accounts expected:
    /// 0. `[signer]`
    ///    * Account of the member executing the withdraw
    /// 
    /// 1. `[writable]`
    ///    * State account
    ///    * Stores data about the revenue sharing: member public keys, member shares and the amount each member has already withdrawn
    /// 
    /// 2. `[writable]`
    ///    * Shared account: token account that holds tokens to be shared between members
    /// 
    /// 3. `[]` 
    ///    * Destination account of withdraw
    /// 
    /// 4. `[]` The token program account
    /// 
    /// 5. `[]` The PDA account
    Withdraw {
        amount: u64,
    }
}

impl RevenueSharingInstruction {
    
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::unpack_revenue_sharing(rest),
            1 => Self::Withdraw { amount: Self::unpack_amount(rest)? },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    /*
     * Data has the following structure:
     * Member 1 shares: u16 (2 bytes)
     * Member 2 shares: u16 (2 bytes)
     * Total length: 4 bytes
     */
    fn unpack_revenue_sharing(data: &[u8]) -> Self {
        let data = array_ref![data, 0, 4];
        let ( 
            member_1_shares_slice,
            member_2_shares_slice,
        ) = array_refs![data, 2, 2];
        let member_1_shares = u16::from_le_bytes(*member_1_shares_slice);
        let member_2_shares = u16::from_le_bytes(*member_2_shares_slice);
        Self::InitRevenueSharing { member_1_shares, member_2_shares }
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
