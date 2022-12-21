use callum_zero_to_hero::{contract::instantiate, msg::InstantiateMsg};
use cosmwasm_std::{
    attr,
    testing::{mock_dependencies, mock_env, mock_info},
};

pub const ADDR1: &str = "addr1";
pub const ADDR2: &str = "addr2";

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR1)]
    )
}

#[test]
fn test_instantiate_with_admin() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: Some(ADDR2.to_string()),
    };
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR2)]
    )
}
