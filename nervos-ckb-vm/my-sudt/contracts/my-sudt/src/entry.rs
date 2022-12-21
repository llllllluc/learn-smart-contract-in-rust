// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::vec::Vec;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    high_level::{load_cell_data, load_cell_lock_hash, load_script, QueryIter},
};

use crate::error::Error;

const UDT_LEN: usize = 16;

pub fn main() -> Result<(), Error> {
    // // remove below examples and write your code here

    // let script = load_script()?;
    // let args: Bytes = script.args().unpack();
    // debug!("script args is {:?}", args);

    // // return an error if args is invalid
    // if args.is_empty() {
    //     return Err(Error::MyError);
    // }

    // let tx_hash = load_tx_hash()?;
    // debug!("tx hash is {:?}", tx_hash);

    // let _buf: Vec<_> = vec![0u8; 32];

    // Ok(())
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    // let args: Vec<u8> = script.args().unpack();

    if check_owner_mode(&args)? {
        return Ok(());
    }

    let inputs_amount = collect_amount(true)?;
    let outputs_amount = collect_amount(false)?;
    if inputs_amount < outputs_amount {
        return Err(Error::InputAmountLessThanOutput);
    }

    Ok(())
}

fn check_owner_mode(args: &Bytes) -> Result<bool, Error> {
    // for i in 0.. {
    //     let lock_hash = match load_cell_lock_hash(i, Source::Input) {
    //         Ok(lock_hash) => lock_hash,
    //         Err(SysError::IndexOutOfBound) => return Ok(false),
    //         Err(err) => return Err(err.into()),
    //     };
    //     if args[..] == lock_hash[..] {
    //         return Ok(true);
    //     }
    // }
    // Ok(false)
    let is_owner_mode = QueryIter::new(load_cell_lock_hash, Source::Input)
        .find(|lock_hash| args[..] == lock_hash[..])
        .is_some();
    Ok(is_owner_mode)
}

fn collect_amount(is_input: bool) -> Result<u128, Error> {
    // let mut amount: u128 = 0;
    let mut buf = [0u8; UDT_LEN];

    let source: Source;
    if is_input {
        source = Source::GroupInput;
    } else {
        source = Source::GroupOutput;
    }

    // for i in 0.. {
    //     let data = match load_cell_data(i, source) {
    //         Ok(data) => data,
    //         Err(SysError::IndexOutOfBound) => break,
    //         Err(err) => return Err(err.into()),
    //     };

    //     if data.len() != UDT_LEN {
    //         return Err(Error::Encoding);
    //     }
    //     buf.copy_from_slice(&data);
    //     amount += u128::from_le_bytes(buf);
    // }

    let udt_list = QueryIter::new(load_cell_data, source)
        .map(|data| {
            if data.len() != UDT_LEN {
                Err(Error::Encoding)
            } else {
                buf.copy_from_slice(&data);
                Ok(u128::from_le_bytes(buf))
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(udt_list.into_iter().sum::<u128>())
}
