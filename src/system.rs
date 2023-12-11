use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
};

pub fn transfer_sol(
    from: &AccountInfo, to: &AccountInfo, amount: u64
) -> ProgramResult {
    **from.try_borrow_mut_lamports()? -= amount;
    **to.try_borrow_mut_lamports()? += amount;
    Ok(())
}