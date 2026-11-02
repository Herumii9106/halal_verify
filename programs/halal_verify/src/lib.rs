use anchor_lang::prelude::*;

declare_id!("BkgYnVNVWNx3m9wcpUthg7wT4JzqgCnYdQaTJN3KnwV2");

#[program]
pub mod halal_verify {
    use super::*;

    pub fn register_product(
        ctx: Context<RegisterProduct>,
        product_id: String,
        product_name: String,
        halal_cert_number: String,
        origin_region: String,
    ) -> Result<()> {
        let product_record = &mut ctx.accounts.product_record;
        product_record.authority = ctx.accounts.authority.key();
        product_record.product_id = product_id;
        product_record.product_name = product_name;
        product_record.halal_cert_number = halal_cert_number;
        product_record.origin_region = origin_region;
        product_record.is_verified = true;
        
        msg!("Produk Halal Berhasil Didaftarkan di Blockchain Solana!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterProduct<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 8 + 32 + 64 + 64 + 64 + 64 + 1
    )]
    pub product_record: Account<'info, ProductRecord>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProductRecord {
    pub authority: Pubkey,
    pub product_id: String,
    pub product_name: String,
    pub halal_cert_number: String,
    pub origin_region: String,
    pub is_verified: bool,
}
