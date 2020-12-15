#![allow(dead_code)]

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(rename_all = "lowercase")]
enum Role {
    User,
    #[ts(rename = "administrator")]
    Admin,
}

#[derive(Serialize, TS)]
// when 'serde-compat' is enabled, ts-rs tries to use supported serde attributes.
#[serde(rename_all = "UPPERCASE")]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, TS)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
    role: Role,
    #[ts(inline)]
    gender: Gender,
}

#[cfg(test)]
mod export_ts {
    use crate::{Role, User};
    use ts_rs::TS;

    #[test]
    fn export_ts() {
        let _ = std::fs::remove_file("bindings.ts");
        Role::dump("bindings.ts").unwrap();
        User::dump("bindings.ts").unwrap();
    }
}
