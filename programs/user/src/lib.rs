use anchor_lang::prelude::*;
use document::cpi::accounts::*;
use document::program::Document;
use document::{self, DocumentData, DOCUMENT_LEN, DOCUMENT_PDA_SEED};


declare_id!("joS6yzC1bz5VRXg6wxqRmMDJkGnXWXF6Kw4yehZeVo6");

pub const USER_PDA_SEED: &[u8] = b"users";

#[program]
pub mod user {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        if name.as_bytes().len() > 64 {
            return err!(ErrorCode::UserNameTooLong);
        }
        let user_data = &mut ctx.accounts.user_data;
        user_data.name = name;
        user_data.user = ctx.accounts.user.key();

        Ok(())
    }

    pub fn create_user_document(ctx: Context<CreateUserDocument>, ref_id: String, name: String) -> Result<()>{
        let context = ctx.accounts.create_address_ctx();
        let ref_id_cloned = ref_id.clone();
        let (_pda, bump_seed) = Pubkey::find_program_address(
            &[
                DOCUMENT_PDA_SEED,
                ctx.accounts.user.to_account_info().key.as_ref(),
                ref_id_cloned.as_bytes().as_ref(),
            ],
            ctx.accounts.document_program.key,
        );
        let seeds = &[
            DOCUMENT_PDA_SEED,
            ctx.accounts.user.to_account_info().key.as_ref(),
            ref_id_cloned.as_bytes().as_ref(),
            &[bump_seed],
        ]; 

        document::cpi::create_document(context.with_signer(&[&seeds[..]]), ref_id, name)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user,
        space = USER_DATA_LEN,
        seeds = [USER_PDA_SEED.as_ref(), user.key().as_ref()], 
        bump,
    )]
    pub user_data: Account<'info, UserData>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(ref_id: String)]
pub struct CreateUserDocument<'info> {
    #[account(mut)]
    pub user: Signer<'info>, 
    #[account(
        init, 
        payer = user, 
        space = DOCUMENT_LEN,
        seeds = [DOCUMENT_PDA_SEED.as_ref(), user.key().as_ref(), ref_id.as_bytes().as_ref()], 
        bump,
    )]
    pub document: Account<'info, DocumentData>,
    pub document_program: Program<'info, Document>,
    pub system_program: Program<'info, System>,
}


impl<'info> CreateUserDocument<'info> {
    pub fn create_address_ctx(&self) -> CpiContext<'_, '_, '_, 'info, CreateDocument<'info>> {
        let cpi_program = self.document_program.to_account_info();
        let cpi_accounts = CreateDocument {
            document: self.document.to_account_info().clone(),
            system_program: self.system_program.to_account_info().clone(),
            user: self.user.to_account_info().clone(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub const USER_DATA_LEN: usize = 32 + (64 + 4) + 8;
#[account]
pub struct UserData {
    pub user: Pubkey,
    pub name: String, 
}

#[error_code]
pub enum ErrorCode {
    #[msg("User name can only be 64 chars long.")]
    UserNameTooLong,
}