use anchor_lang::prelude::*;
use std::str::FromStr;
declare_id!("rand1Bj6Hftpmmr3KdjZxMpryEUBd19B9dofzXXpQWX");

pub const PREFIX: &str = "random";
pub const RAND_SOURCE: &str = "DhYCi6pvfhJkPRpt5RjYwsE1hZw84iu6twbRt9B6dYLV";

#[program]
pub mod true_random_oracle {
    use super::*;

    pub fn request_result(
        ctx: Context<RequestResult>,
        num_results: u8,
        min: u8,
        max: u8,
    ) -> Result<()> {
        ctx.accounts.result.bump = *ctx.bumps.get("result").unwrap();
        emit!(RequestEvent {
            result_account: ctx.accounts.result.key(),
            result_count: num_results,
            result_min: min,
            result_max: max,
        });
        Ok(())
    }

    pub fn post_result(ctx: Context<PostResult>, data: Vec<u8>) -> Result<()> {
        ctx.accounts.result.numbers = data;
        emit!(ResultEvent {
            result_account: ctx.accounts.result.key()
        });
        Ok(())
    }

    pub fn delete_result(_ctx: Context<DeleteResult>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(num_results: u8)]
pub struct RequestResult<'info> {
    #[account(init, space = 8 + 4 + (num_results as usize) + 1, payer = payer, seeds = [PREFIX.as_bytes(), payer.key().as_ref()], bump)]
    pub result: Account<'info, RandomResult>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostResult<'info> {
    pub result: Account<'info, RandomResult>,
    #[account(mut, address = Pubkey::from_str(RAND_SOURCE).unwrap())]
    pub poster: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteResult<'info> {
    #[account(mut, close = poster)]
    pub result: Account<'info, RandomResult>,
    #[account(mut, address = Pubkey::from_str(RAND_SOURCE).unwrap())]
    pub poster: Signer<'info>,
}

#[account]
pub struct RandomResult {
    pub numbers: Vec<u8>,
    pub bump: u8,
}

#[event]
pub struct RequestEvent {
    pub result_account: Pubkey,
    pub result_count: u8,
    pub result_min: u8,
    pub result_max: u8,
}

#[event]
pub struct ResultEvent {
    pub result_account: Pubkey,
}
