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
fn test_execute_create_poll_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    // instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreatePoll {
        poll_id: "p1".to_string(),
        question: "favorite chain".to_string(),
        options: vec!["ATOM".to_string(), "LUNA".to_string()],
    };
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "execute_create_poll"), attr("poll_id", "p1")]
    )
}

#[test]
fn test_execute_create_poll_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    // instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreatePoll {
        poll_id: "p1".to_string(),
        question: "favorite chain".to_string(),
        options: vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "11".to_string(),
        ],
    };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
