use super::*;

#[test]
fn user_is_user() {
    assert_eq!(UserType::User.is_user(), true);
}

#[test]
fn user_is_not_admin() {
    assert_eq!(UserType::User.is_admin(), false);
}

#[test]
fn admin_is_admin() {
    assert_eq!(UserType::Admin.is_admin(), true);
}

#[test]
fn admin_is_not_user() {
    assert_eq!(UserType::Admin.is_user(), false);
}

#[test]
fn from_1() {
    assert_matches!(UserType::try_from(1), Ok(UserType::User));
}

#[test]
fn from_2() {
    assert_matches!(UserType::try_from(2), Ok(UserType::Admin));
}

#[test]
fn from_3() {
    assert_matches!(UserType::try_from(3), Err(_));
}

#[test]
fn user_to_num() {
    assert_eq!(Into::<u16>::into(UserType::User), 1);
}

#[test]
fn admin_to_num() {
    assert_eq!(Into::<u16>::into(UserType::Admin), 2);
}
