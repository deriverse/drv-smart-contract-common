use crate::program::processor::{
    airdrop::airdrop, change_ref_program::change_ref_program, deposit::deposit,
    dividends_allocation::dividends_allocation, dividends_claim::dividends_claim,
    fees_deposit::fees_deposit, fees_withdraw::fees_withdraw,
    move_spot_avail_funds::move_spot_avail_funds, new_holder_account::new_holder_account,
    new_instrument::new_instrument, new_operator::new_operator, new_perp_order::new_perp_order,
    new_ref_link::new_ref_link, new_root_account::new_root_account, new_spot_order::new_spot_order,
    new_token::new_token, next_voting::next_voting, perp_change_leverage::perp_change_leverage,
    perp_deposit::perp_deposit, perp_forced_close::perp_forced_close,
    perp_mass_cancel::perp_mass_cancel, perp_order_cancel::perp_order_cancel,
    perp_quotes_replace::perp_quotes_replace, perp_statistics_reset::perp_statistics_reset,
    perp_withdraw::perp_withdraw, set_instr_oracle_feed::set_instr_oracle_feed,
    set_instr_ready_for_perp_upgrade::set_instr_ready_for_perp_upgrade, spot_lp::spot_lp,
    spot_mass_cancel::spot_mass_cancel, spot_order_cancel::spot_order_cancel,
    spot_quotes_replace::spot_quotes_replace, swap::swap, upgrade_to_perp::upgrade_to_perp,
    voting::voting, withdraw::withdraw,
};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

pub mod program;
pub mod state;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    match _instruction_data[0] {
        0 => new_holder_account(program_id, accounts)?,
        1 => new_operator(program_id, accounts, _instruction_data)?,
        2 => new_root_account(program_id, accounts, _instruction_data)?,
        3 => perp_withdraw(program_id, accounts, _instruction_data)?,
        4 => new_token(program_id, accounts, _instruction_data)?,
        5 => fees_deposit(program_id, accounts, _instruction_data)?,
        7 => deposit(program_id, accounts, _instruction_data)?,
        8 => withdraw(program_id, accounts, _instruction_data)?,
        9 => new_instrument(program_id, accounts, _instruction_data)?,
        10 => upgrade_to_perp(program_id, accounts, _instruction_data)?,
        11 => perp_deposit(program_id, accounts, _instruction_data)?,
        12 => new_spot_order(program_id, accounts, _instruction_data)?,
        13 => spot_order_cancel(program_id, accounts, _instruction_data)?,
        14 => spot_lp(program_id, accounts, _instruction_data)?,
        15 => spot_mass_cancel(program_id, accounts, _instruction_data)?,
        16 => next_voting(program_id, accounts)?,
        19 => new_perp_order(program_id, accounts, _instruction_data)?,
        //23 => run_test(program_id, accounts, _instruction_data)?,
        25 => dividends_allocation(program_id, accounts)?,
        26 => swap(program_id, accounts, _instruction_data)?,
        27 => airdrop(program_id, accounts, _instruction_data)?,
        28 => dividends_claim(program_id, accounts)?,
        30 => perp_order_cancel(program_id, accounts, _instruction_data)?,
        32 => voting(program_id, accounts, _instruction_data)?,
        34 => spot_quotes_replace(program_id, accounts, _instruction_data)?,
        36 => perp_mass_cancel(program_id, accounts, _instruction_data)?,
        37 => perp_change_leverage(program_id, accounts, _instruction_data)?,
        38 => perp_forced_close(program_id, accounts, _instruction_data)?,
        39 => fees_withdraw(program_id, accounts, _instruction_data)?,
        40 => set_instr_oracle_feed(program_id, accounts, _instruction_data)?,
        41 => set_instr_ready_for_perp_upgrade(program_id, accounts, _instruction_data)?,
        42 => perp_quotes_replace(program_id, accounts, _instruction_data)?,
        43 => move_spot_avail_funds(program_id, accounts, _instruction_data)?,
        44 => change_ref_program(program_id, accounts, _instruction_data)?,
        45 => new_ref_link(program_id, accounts)?,
        46 => perp_statistics_reset(program_id, accounts, _instruction_data)?,
        47 => buy_market_seat(program_id, accounts, _instruction_data)?,
        48 => sell_market_seat(program_id, accounts, _instruction_data)?,
        49 => new_private_client(program_id, accounts, _instruction_data)?,
        50 => terminate_private_mode(program_id, accounts)?,
        51 => change_points_program_exipratin(program_id, accounts, _instruction_data)?,
        _ => {}
    }
    Ok(())
}
