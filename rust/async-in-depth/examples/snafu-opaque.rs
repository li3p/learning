use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub struct Error(InnerError);

// That's all it takes! The rest is demonstration of how to use it.

pub fn login(id: i32) -> Result<(), Error> {
    validate_user(id)?;
    is_user_locked(id)?;
    Ok(())
}

#[derive(Debug, Snafu)]
enum InnerError {
    #[snafu(display("User ID {user_id} is invalid"))]
    InvalidUser { user_id: i32 },
    #[snafu(display("User ID {user_id} is locked"))]
    UserLocked { user_id: i32 },
}

fn validate_user(user_id: i32) -> Result<(), InnerError> {
    InvalidUserSnafu { user_id }.fail()
}

fn is_user_locked(user_id: i32) -> Result<(), InnerError> {
    UserLockedSnafu { user_id }.fail()
}

fn main() {
    let res = login(5);
    println!("res: {:?}", res);
    println!("res: {}", res.unwrap_err());
}
