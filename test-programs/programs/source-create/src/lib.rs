use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create_wallet<'a>(mut new_wallet: Create<'a, Wallet<'a>>) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_wallet.key());
        new_wallet.create()
    }
    fn create_wallet_with_payer<'a>(
        mut new_wallet: Create<'a, Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_wallet.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_wallet.create_with_payer(rent_payer)
    }

    fn create_mint<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_mint.key());
        new_mint.create(decimals, mint_authority.clone(), Some(mint_authority))
    }
    fn create_mint_with_payer<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_mint.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_mint.create_with_payer(
            decimals,
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )
    }

    fn create_metadata<'a>(
        mut new_metadata: Create<'a, Metadata<'a>>,
        mint: Mint<'a>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_metadata.key());
        msg!("-- Mint Public Key: {}", &mint.key());
        new_metadata.create(
            title,
            symbol,
            uri,
            mint,
            mint_authority.clone(),
            mint_authority,
        )
    }
    fn create_metadata_with_payer<'a>(
        mut new_metadata: Create<'a, Metadata<'a>>,
        mint: Mint<'a>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_metadata.key());
        msg!("-- Mint Public Key: {}", &mint.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_metadata.create_with_payer(
            title,
            symbol,
            uri,
            mint,
            mint_authority.clone(),
            mint_authority,
            rent_payer,
        )
    }

    fn create_associated_token<'a>(
        mut new_associated_token: Create<'a, AssociatedTokenAccount<'a>>,
        mint: Mint<'a>,
        owner: Wallet<'a>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_associated_token.key());
        msg!("-- Mint Public Key: {}", &mint.key());
        msg!("-- Owner Public Key: {}", &owner.key());
        new_associated_token.create(mint, owner)
    }
    fn create_associated_token_with_payer<'a>(
        mut new_associated_token: Create<'a, AssociatedTokenAccount<'a>>,
        mint: Mint<'a>,
        owner: Wallet<'a>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_associated_token.key());
        msg!("-- Mint Public Key: {}", &mint.key());
        msg!("-- Owner Public Key: {}", &owner.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_associated_token.create_with_payer(mint, owner, rent_payer)
    }

    fn create_token<'a>(
        mut new_token: Create<'a, Token<'a>>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_token.key());
        new_token.create(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
        )
    }
    fn create_token_with_payer<'a>(
        mut new_token: Create<'a, Token<'a>>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_token.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_token.create_with_payer(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )
    }

    fn transfer_wallet<'a>(
        from: Signer<Wallet<'a>>,
        to: Mut<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        msg!(
            "Transferring {} From: {} to: {}",
            amount,
            from.key(),
            to.key()
        );
        from.transfer_lamports(to, amount)
    }
}
