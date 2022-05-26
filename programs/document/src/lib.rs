use anchor_lang::prelude::*;

declare_id!("HmKR4hznDDJ4TtHztNry5FGjcXCGL2AqqRAtgFNjVDkY");

pub const DOCUMENT_PDA_SEED: &[u8] = b"user-documents";

#[program]
pub mod document {
    use super::*;

    pub fn create_document(ctx: Context<CreateDocument>, ref_id: String, name: String) -> Result<()> {
        if name.as_bytes().len() > 200 {
            return err!(ErrorCode::DocumentNameTooLong);
        }
        if ref_id.as_bytes().len() > 32 {
            return err!(ErrorCode::RefIdTooLong);
        }
        let document = &mut ctx.accounts.document;
        document.name = name;
        document.ref_id = ref_id;
        document.created =  Clock::get()?.unix_timestamp;

        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(ref_id: String)]
pub struct CreateDocument<'info> {   
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
    pub system_program: Program<'info, System>,
}

pub const DOCUMENT_LEN: usize = (4 + 200) + 32 + (32 + 4) + 8 + 8;
#[account]
pub struct DocumentData {
    pub name: String,
    pub user: Pubkey,
    pub ref_id: String,
    pub created: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Document name can only be 200 chars long.")]
    DocumentNameTooLong,
    #[msg("Document referenceId can only be 32 characters")]
    RefIdTooLong,
}