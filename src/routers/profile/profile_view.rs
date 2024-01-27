use diesel::prelude::*;
use actix_web::web;

use crate::{error::RouterError, routers::user::FullUserProfile, DbPool, models::{Account, User, Email, UserName}};

pub async fn profile_view(
    user_id: web::ReqData<u32>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<FullUserProfile>, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as account_id};
    use crate::schema::app_user_names::dsl::primary_name;

    let user_id = user_id.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the account from user_id
        // which is unwraped from token
        let account: Account = app_accounts
            .filter(account_id.eq(user_id as i32))
            .get_result(&mut conn)?;

        let user: User = User::belonging_to(&account).get_result(&mut conn)?;

        let email = Email::belonging_to(&account).first::<Email>(&mut conn)?;

        // Now get the user names
        let names = UserName::belonging_to(&account)
            .filter(primary_name.eq(true))
            .load::<UserName>(&mut conn)?;

        // Is user have any names ?
        let names = if names.is_empty() { None } else { Some(names) };

        let profile = match names {
            Some(names) => {
                // Its must be always > 1 element
                let name: &UserName = names.get(0).unwrap();

                FullUserProfile {
                    uuid: account.uuid.to_string(),
                    email: email.email,
                    username: account.username.to_owned(),
                    first_name: Some(name.first_name.to_owned()),
                    last_name: Some(name.last_name.to_owned()),
                    birthday: user.clone().birthday,
                    profile_image: user.clone().profile_image,
                }
            }

            None => FullUserProfile {
                uuid: account.uuid.to_string(),
                email: email.email,
                username: account.username.to_owned(),
                first_name: None,
                last_name: None,
                birthday: user.clone().birthday,
                profile_image: user.clone().profile_image,
            },
        };

        Ok(web::Json(profile))
    })
    .await
    .unwrap()
}