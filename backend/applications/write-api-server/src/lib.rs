use utoipa::OpenApi;

use command_infrastructure::controllers::{
    self,
    participant as controllers_participant,
    group as controllers_group,
    apply as controllers_apply,
    scout as controllers_scout,
    volunteer as controllers_volunteer,
    review as controllers_review
};

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers_group::create_group_account,
        controllers_group::update_group_account,
        controllers_group::delete_group_account,
        controllers_participant::create_participant_account,
        controllers_participant::update_participant_account,
        controllers_participant::delete_participant_account,
        controllers_volunteer::create_volunteer,
        controllers_volunteer::update_volunteer,
        controllers_volunteer::delete_volunteer,
        controllers_volunteer::register_favorite,
        controllers_volunteer::unregister_favorite,
        controllers_apply::create_apply,
        controllers_apply::update_apply_allowed_status,
        controllers_apply::update_apply_is_sent,
        controllers_scout::create_scout,
        controllers_scout::update_scout_is_sent,
        controllers_scout::update_scout_is_read,
        controllers_scout::update_scout_denied,
        controllers_review::review_to_volunteer,
        controllers_review::review_to_participant
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
        controllers_volunteer::CreateVolunteerRequestBody,
        controllers_volunteer::UpdateVolunteerRequestBody,
        controllers_volunteer::DeleteVolunteerRequestBody,
        controllers_volunteer::RegisterVolunteerFavoriteRequestBody,
        controllers_volunteer::UnregisterVolunteerFavoriteRequestBody,
        controllers_apply::CreateApplyRequestBody,
        controllers_apply::UpdateApplyAllowedStatusRequestBody,
        controllers_apply::UpdateApplyIsSentRequestBody,
        controllers_scout::CreateScoutRequestBody,
        controllers_scout::UpdateScoutIsSentRequestBody,
        controllers_scout::UpdateScoutIsReadRequestBody,
        controllers_scout::UpdateScoutDeniedRequestBody,
        controllers_review::ReviewToVolunteerRequestBody,
        controllers_review::ReviewToParticipantRequestBody
    )),
    tags(
        (name = "write-api-server", description = "Write API Server")
    )
)]
pub struct ApiDoc;
