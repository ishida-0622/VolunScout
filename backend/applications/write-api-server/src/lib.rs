use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use infra::controllers;

#[derive(OpenApi)]
#[openapi(
    paths(controllers::create_group_account,),
    components(schemas(
        controllers::WriteApiResponseFailureBody,
        controllers::CreateGroupAccountRequestBody,
        controllers::UpdateGroupAccountRequestBody,
        controllers::DeleteGroupAccountRequestBody,
        controllers::CreateParticipantAccountRequestBody,
        controllers::UpdateParticipantAccountRequestBody,
        controllers::DeleteParticipantAccountRequestBody,
    ))
)]
pub struct ApiDoc;
