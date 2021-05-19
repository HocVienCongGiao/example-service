use async_trait::async_trait;

pub trait Test1SimpleMutationInputBoundary {
    fn create_test1(&self, request: Test1SimpleMutationRequest) -> Test1SimpleMutationResponse;
}

pub struct Test1SimpleMutationRequest {
    pub name: String,
}

pub struct Test1SimpleMutationResponse {}

pub trait MutationOutputBoundary {}

#[async_trait]
pub trait Test1DbGateway {
    async fn exists_by_name(&self, name: String) -> bool;
}

// CommonUser
// CommonUserFactory
// JpaUser
// JpaUserRepository
// User
// UserDataMapper
// UserDsRequestModel
// UserFactory
// UserInputBoundary
// UserPresenter
// UserRegisterController
// UserRegisterDsGateway
// UserRegisterInteractor
// UserRequestModel
// UserResponseFormatter
// UserResponseModel
