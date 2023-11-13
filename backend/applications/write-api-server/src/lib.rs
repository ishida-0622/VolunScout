use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use command_infrastructure::controllers;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::create_group_account,
        controllers::update_group_account,
        controllers::delete_group_account,
        controllers::create_participant_account,
        controllers::update_participant_account,
        controllers::delete_participant_account,
    ),
    components(schemas(
        controllers::WriteApiResponseSuccessBody,
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
