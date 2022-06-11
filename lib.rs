use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use std::mem::size_of;

declare_id!("5Q1KEwxpDC3oBQHM6uSPCaMfY9DRKBsiXYNrBkzurx1v");

#[program]
pub mod crowd_funding {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        Image: String,
        Description: String,
        Title: String,
        Doniation_Goal: u64,
    ) -> ProgramResult {
        let campaign = &mut ctx.accounts.crowd_funding_platform;
        campaign.Image = Image;
        campaign.Description = Description;
        campaign.Title = Title;
        campaign.Amount_Donated = 0;
        campaign.Donation_Goal = Doniation_Goal;
        Ok(())
    }

    pub fn Donation(ctx: Context<Donation>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.crowd_funding_platform;

        let donation_transaction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &campaign.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &donation_transaction,
            &[
                ctx.accounts.authority.to_account_info(),
                campaign.to_account_info(),
                ctx.accounts.system_program.to_account_info()
            ],
        );

        campaign.Amount_Donated += amount;
        Ok(())
    }

    pub fn Withdraw(ctx: Context<Withdraw>) -> ProgramResult {
        let campaign = &mut ctx.accounts.crowd_funding_platform;

        let amount = campaign.Amount_Donated;
        let goal = campaign.Donation_Goal;

        if ctx.accounts.authority.key() == campaign.key() {
            if amount >= goal {
                let withdrawal_transaction = anchor_lang::solana_program::system_instruction::transfer(
                    &ctx.accounts.authority.key(),
                    &campaign.key(),
                    amount,
                );
                anchor_lang::solana_program::program::invoke(
                    &withdrawal_transaction,
                    &[
                        ctx.accounts.authority.to_account_info(),
                        campaign.to_account_info(),
                        ctx.accounts.system_program.to_account_info()
                    ],
                );
            }

            campaign.Amount_Donated = 0;

        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
    init, 
    payer = authority, 
    space = size_of::<CrowdFundingPlatform>() + 8,
    seeds = [b"crowd_funding_platform".as_ref()],
    bump
    )]
    pub crowd_funding_platform: Account<'info, CrowdFundingPlatform>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donation<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
    init,
    payer = authority, 
    space = size_of::<CrowdFundingPlatform>() + 8,
    seeds = [b"crowd_funding_platform".as_ref()],
    bump
    )]
    pub crowd_funding_platform: Account<'info, CrowdFundingPlatform>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
    init,
    payer = authority, 
    space = size_of::<CrowdFundingPlatform>() + 8,
    seeds = [b"crowd_funding_platform".as_ref(), authority.key().as_ref()],
    bump
    )]
    pub crowd_funding_platform: Account<'info, CrowdFundingPlatform>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CrowdFundingPlatform {
    pub Image: String,
    pub Description: String,
    pub Title: String,
    pub Amount_Donated: u64,
    pub Donation_Goal: u64,
}
