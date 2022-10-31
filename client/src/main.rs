mod consts;
mod structs;
mod transactions;

use crate::transactions::add_bot::add_bot;
use crate::transactions::add_supported_token::add_supported_token;
use crate::transactions::close_game::close_game;
use crate::transactions::forced_close::forced_close;
use crate::transactions::init::init;
use crate::transactions::join_game::join_game;
use crate::transactions::manually_close::manually_close;
use crate::transactions::new_game::new_game;
use crate::transactions::registration::registration;
use crate::transactions::setters::{
    lock_bets, new_delay, new_manager, set_admin_fee, set_global_fee, set_transaction_fee,
    set_type_price, set_winner_fee, unlock_bets,
};
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};

fn main() {
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("manager")
                        .short("m")
                        .long("manager")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("s_token")
                        .short("t")
                        .long("s_token")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("change_close_delay")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("new_delay")
                        .short("d")
                        .long("new_delay")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("lock_bets")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("unlock_bets")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("new_manager")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("manager")
                        .short("m")
                        .long("manager")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set_global_fee")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("fee")
                        .short("f")
                        .long("fee")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set_admin_fee")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("fee")
                        .short("f")
                        .long("fee")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set_winner_fee")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("fee")
                        .short("f")
                        .long("fee")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set_transaction_fee")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("fee")
                        .short("f")
                        .long("fee")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set_type_price")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("price")
                        .short("p")
                        .long("price")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("add_supported_token")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("token")
                        .short("t")
                        .long("token")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("registration")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("referrer")
                        .short("r")
                        .long("referrer")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("add_bot")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("bot")
                        .short("b")
                        .long("bot")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("new_game")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("value")
                        .short("v")
                        .long("value")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("forced_close")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("user")
                        .short("u")
                        .long("user")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("manually_close")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("join_game")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("master")
                        .short("m")
                        .long("master")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("value")
                        .short("v")
                        .long("value")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("close_game")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("user")
                        .short("u")
                        .long("user")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("winner")
                        .short("w")
                        .long("winner")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        init(matches);
    }

    if let Some(matches) = matches.subcommand_matches("change_close_delay") {
        new_delay(matches);
    }

    if let Some(matches) = matches.subcommand_matches("lock_bets") {
        lock_bets(matches);
    }

    if let Some(matches) = matches.subcommand_matches("unlock_bets") {
        unlock_bets(matches);
    }

    if let Some(matches) = matches.subcommand_matches("new_manager") {
        new_manager(matches);
    }

    if let Some(matches) = matches.subcommand_matches("set_global_fee") {
        set_global_fee(matches);
    }

    if let Some(matches) = matches.subcommand_matches("set_admin_fee") {
        set_admin_fee(matches);
    }

    if let Some(matches) = matches.subcommand_matches("set_winner_fee") {
        set_winner_fee(matches);
    }

    if let Some(matches) = matches.subcommand_matches("set_transaction_fee") {
        set_transaction_fee(matches);
    }

    if let Some(matches) = matches.subcommand_matches("set_type_price") {
        set_type_price(matches);
    }

    if let Some(matches) = matches.subcommand_matches("add_supported_token") {
        add_supported_token(matches);
    }

    if let Some(matches) = matches.subcommand_matches("registration") {
        registration(matches);
    }

    if let Some(matches) = matches.subcommand_matches("add_bot") {
        add_bot(matches);
    }

    if let Some(matches) = matches.subcommand_matches("new_game") {
        new_game(matches);
    }

    if let Some(matches) = matches.subcommand_matches("forced_close") {
        forced_close(matches);
    }

    if let Some(matches) = matches.subcommand_matches("manually_close") {
        manually_close(matches);
    }

    if let Some(matches) = matches.subcommand_matches("join_game") {
        join_game(matches);
    }

    if let Some(matches) = matches.subcommand_matches("close_game") {
        close_game(matches);
    }
}
