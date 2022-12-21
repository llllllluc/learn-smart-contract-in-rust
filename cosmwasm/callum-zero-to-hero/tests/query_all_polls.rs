use callum_zero_to_hero::{
    contract::{execute, instantiate, query},
    msg::{AllPollsResponse, ExecuteMsg, InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info},
};

pub const ADDR1: &str = "addr1";

#[test]
fn test_query_all_polls() {
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

    // Create a second poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_2".to_string(),
        question: "What's your colour?".to_string(),
        options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let msg = QueryMsg::AllPolls {};
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: AllPollsResponse = from_binary(&bin).unwrap();
    assert_eq!(res.polls.len(), 2);
}
