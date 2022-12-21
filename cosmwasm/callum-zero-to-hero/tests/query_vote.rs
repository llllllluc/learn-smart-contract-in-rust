use callum_zero_to_hero::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, VoteResponse},
};
use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info},
};

pub const ADDR1: &str = "addr1";
pub const ADDR2: &str = "addr2";

#[test]
fn test_query_vote() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // create a vote
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id_1".to_string(),
        vote: "Juno".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // test exist
    let msg = QueryMsg::Vote {
        poll_id: "some_id_1".to_string(),
        address: ADDR1.to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
    assert!(res.vote.is_some());

    // test poll_id not exist
    let msg = QueryMsg::Vote {
        poll_id: "some_id_2".to_string(),
        address: ADDR1.to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
    assert!(res.vote.is_none());

    // test address not exist
    let msg = QueryMsg::Vote {
        poll_id: "some_id_1".to_string(),
        address: ADDR2.to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
    assert!(res.vote.is_none());
}
