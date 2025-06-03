use anchor_lang::prelude::*;

declare_id!("5MzvwUQpkdML29mNh1uLsaRjJkUmp2frttnV1z9zuAou");

#[program]
pub mod counter {
    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = ctx.accounts.owner.key();
        journal_entry.tital = title;
        journal_entry.message = message;
        Ok(())
    }

    pub fn update_journal_entry(ctx: Context<UpdateEntry>, _title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        // if journal_entry.owner != ctx.accounts.owner.key() {
        //     return Err(ErrorCode::Unauthorized.into());
        // }
        journal_entry.message = message;
        Ok(())
    }


}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        seeds = [tital.as_bytes(),owner.key().as_ref()],
        bump,
        payer= owner,
        space = 8 + JournalEntryState::INIT_SPACE,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(_title: String, message: String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut,
        seeds = [tital.as_bytes(),owner.key().as_ref()],
        bump,
        realloc = 8 + JournalEntryState::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>// use to realloc the space of the account
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState<'info> {
    pub owner: Pubkey,
    #[max_len(280)]
    pub tital: String,
    #[max_len(280)]
    pub message: String,
}
