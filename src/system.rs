use solana_program::{
    account_info::AccountInfo, 
    // program_error::ProgramError,
};

pub fn transfer_sol(from: &AccountInfo, to: &AccountInfo, amount: u64) {
// -> Result<std::cell::RefMut<&mut u64>, ProgramError> {
    **from.try_borrow_mut_lamports().unwrap() -= amount; // todo use "?" op.
    **to.try_borrow_mut_lamports().unwrap() += amount;
}