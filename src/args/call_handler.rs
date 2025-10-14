use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CallHandlerArgs {
    pub escrow_index: u8,
    /// This is raw instruction data, it could include discriminator + args
    /// or can be in any other custom format
    pub instruction_data: Vec<u8>,
}
