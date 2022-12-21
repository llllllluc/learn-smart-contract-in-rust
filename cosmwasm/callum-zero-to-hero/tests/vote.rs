use callum_zero_to_hero::{
    contract::{execute, instantiate},
    msg::{ExecuteMsg, InstantiateMsg},
};
use cosmwasm_std::{
    attr,
    testing::{mock_dependencies, mock_env, mock_info},
};

pub const ADDR1: &str = "addr1";

#[test]
fn test_execute_vote() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    // instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // create the poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "p1".to_string(),
        question: "favorite chain".to_string(),
        options: vec!["ATOM".to_string(), "LUNA".to_string()],
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // vote new poll
    let msg = ExecuteMsg::Vote {
        poll_id: "p1".to_string(),
        vote: "ATOM".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "execute_vote"), attr("vote", "ATOM")]
    );

    // re-vote existing poll
    let msg = ExecuteMsg::Vote {
        poll_id: "p1".to_string(),
        vote: "LUNA".to_string(),
    };
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "execute_vote"), attr("vote", "LUNA")]
    )
}

#[test]
fn test_execute_vote_poll_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    // instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    // create the poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "p1".to_string(),
        question: "favorite chain".to_string(),
        options: vec!["ATOM".to_string(), "LUNA".to_string()],
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // poll not exist
    let msg = ExecuteMsg::Vote {
        poll_id: "p2".to_string(),
        vote: "LUNA".to_string(),
    };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

    // option not exist
    let msg = ExecuteMsg::Vote {
        poll_id: "p2".to_string(),
        vote: "OSMO".to_string(),
    };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
