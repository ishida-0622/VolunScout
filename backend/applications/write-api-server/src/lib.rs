use utoipa::OpenApi;

use command_infrastructure::controllers;
use command_infrastructure::controllers::participant as controllers_participant;
use command_infrastructure::controllers::group as controllers_group;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers_group::create_group_account,
        controllers_group::update_group_account,
        controllers_group::delete_group_account,
        controllers_participant::create_participant_account,
        controllers_participant::update_participant_account,
        controllers_participant::delete_participant_account,
    ),
    components(schemas(
        controllers::WriteApiResponseSuccessBody,
        controllers::WriteApiResponseFailureBody,
        controllers_group::CreateGroupAccountRequestBody,
        controllers_group::UpdateGroupAccountRequestBody,
        controllers_group::DeleteGroupAccountRequestBody,
        controllers_participant::CreateParticipantAccountRequestBody,
        controllers_participant::UpdateParticipantAccountRequestBody,
        controllers_participant::DeleteParticipantAccountRequestBody,
    )),
    tags(
        (name = "write-api-server", description = "Write API Server")
    )
)]
pub struct ApiDoc;
