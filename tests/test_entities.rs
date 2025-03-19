use usermodel::{core::FieldControl, entities::User};

#[test]
fn new_user() {
    let mut user = User::new();
    assert_eq!(user.username().get(), None);
    assert_eq!(user.email().get(), None);

    let _ = user.username().set(Some("username".to_string()));
    let _ = user.email().set(Some("name@domain".to_string()));
    assert_eq!(user.username().get(), Some("username".to_string()));
    assert_eq!(user.email().get(), Some("name@domain".to_string()));
}

#[test]
fn test_validation() {
    let mut user = User::new();
    let r = user.username().set(Some("username".to_string())).validate();
    assert!(r.is_ok());
    let r = user.email().set(Some("name@domain".to_string())).validate();
    assert!(r.is_ok());

    let r = user.username().set(Some("a".to_string())).validate();
    assert!(r.is_err());
    let r = user.username().set(Some("@".to_string())).validate();
    assert!(r.is_err());
    let r = user.email().set(Some("namedomain".to_string())).validate();
    assert!(r.is_err());
    let r = user
        .email()
        .set(Some("name@domain@".to_string()))
        .validate();
    assert!(r.is_err());
}
