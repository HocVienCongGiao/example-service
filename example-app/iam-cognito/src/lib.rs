use rusoto_cognito_idp::{
    CognitoIdentityProvider, CognitoIdentityProviderClient, ListUsersRequest,
};
use rusoto_core::{Client, Region};

async fn cognito() {
    // let dispatcher = HttpClient::new().expect("failed to create request dispatcher");
    // let default_provider_result = ProfileProvider::new();
    // let mut default_provider = default_provider_result.unwrap();
    // default_provider.set_profile("hvcg");
    // let aws_client = Client::new_with(default_provider, dispatcher);
    let aws_client = Client::shared();

    let rusoto_cognito_idp_client =
        CognitoIdentityProviderClient::new_with_client(aws_client.clone(), Region::ApSoutheast1);
    // let create_user_request = AdminCreateUserRequest {
    //     desired_delivery_mediums: None,
    //     force_alias_creation: None,
    //     message_action: None,
    //     temporary_password: None,
    //     user_attributes: None,
    //     user_pool_id: "".to_string(),
    //     username: "".to_string(),
    //     validation_data: None
    // };
    let list_user_request = ListUsersRequest {
        attributes_to_get: None,
        filter: None,
        limit: None,
        pagination_token: None,
        user_pool_id: "ap-southeast-1_vmFHg7JIC".to_string(),
    };
    match rusoto_cognito_idp_client
        .list_users(list_user_request)
        .sync()
    {
        Ok(response) => match response.users {
            Some(user_types) => {
                println!("User Type here");
                for user_type in user_types {
                    println!("{}", user_type.username.unwrap())
                }
            }
            None => println!("No buckets in region!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn it_works() {
        crate::cognito().await;
        assert_eq!(2 + 2, 4);
    }
}
