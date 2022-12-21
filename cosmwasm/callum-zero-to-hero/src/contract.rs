#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    AllPollsResponse, ConfigResponse, ExecuteMsg, InstantiateMsg, PollResponse, QueryMsg,
    VoteResponse,
};
use crate::state::{Ballot, Config, Poll, BALLOTS, CONFIG, POLLS};

const CONTRACT_NAME: &str = "crates.io:callum-zero-to-hero";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or_else(|| info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll {
            poll_id,
            question,
            options,
        } => execute_create_poll(deps, info, poll_id, question, options),
        ExecuteMsg::Vote { poll_id, vote } => execute_vote(deps, info, poll_id, vote),
        ExecuteMsg::DeletePoll { poll_id } => todo!(), // execute_delete_poll(deps, info, msg, poll_id),
        ExecuteMsg::RevokeVote { poll_id } => todo!(), // execute_revoke_vote(deps, poll_id),
    }
}

fn execute_create_poll(
    deps: DepsMut,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }

    let mut opts: Vec<(String, u64)> = vec![];
    for option in options {
        opts.push((option, 0));
    }

    let poll = Poll {
        // why don't we validate sender here like validate admin in init
        // ah because sender is already addr type, unlike admin in init comes from msg, which is string type
        creator: info.sender,
        question,
        options: opts,
    };
    POLLS.save(deps.storage, poll_id.clone(), &poll)?;

    Ok(Response::new()
        .add_attribute("action", "execute_create_poll")
        .add_attribute("poll_id", poll_id))
}

fn execute_vote(
    deps: DepsMut,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    let poll = POLLS.may_load(deps.storage, poll_id.clone())?;
    match poll {
        Some(mut poll) => {
            // if not exist just create new, else reduce old vote count by 1
            BALLOTS.update(
                deps.storage,
                (info.sender, poll_id.clone()),
                |ballot| -> StdResult<Ballot> {
                    match ballot {
                        Some(ballot) => {
                            let position_of_old_vote = poll
                                .options
                                .iter()
                                .position(|option| option.0 == ballot.option)
                                .unwrap();
                            poll.options[position_of_old_vote].1 -= 1;
                            Ok(Ballot {
                                option: vote.clone(),
                            })
                        }
                        None => Ok(Ballot {
                            option: vote.clone(),
                        }),
                    }
                },
            )?;
            let position = poll.options.iter().position(|option| option.0 == vote);
            if position.is_none() {
                return Err(ContractError::PollNotExist {});
            }
            let position = position.unwrap();
            poll.options[position].1 += 1;

            POLLS.save(deps.storage, poll_id, &poll)?;
            Ok(Response::new()
                .add_attribute("action", "execute_vote")
                .add_attribute("vote", vote))
        }
        None => Err(ContractError::PollNotExist {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPolls {} => query_all_polls(deps),
        QueryMsg::Poll { poll_id } => query_poll(deps, poll_id),
        QueryMsg::Vote { poll_id, address } => query_vote(deps, poll_id, address),
        QueryMsg::Config {} => query_config(deps),
        QueryMsg::AllVotes { address } => todo!(), //query_all_votes(_deps, address),
    }
}

fn query_all_polls(deps: Deps) -> StdResult<Binary> {
    let polls = POLLS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;
    to_binary(&AllPollsResponse { polls })
}

fn query_poll(deps: Deps, poll_id: String) -> StdResult<Binary> {
    let poll = POLLS.may_load(deps.storage, poll_id)?;
    to_binary(&PollResponse { poll })
}

fn query_vote(deps: Deps, poll_id: String, address: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address).unwrap();
    let vote = BALLOTS.may_load(deps.storage, (validated_address, poll_id))?;
    to_binary(&VoteResponse { vote })
}

fn query_config(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&ConfigResponse { config })
}
