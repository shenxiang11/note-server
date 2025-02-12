use crate::pb::note::{CreateOrUpdateRequest, CreateOrUpdateResponse};
use crate::NoteSrv;
use tonic::{Response, Status};

impl NoteSrv {
    pub async fn create_or_update(
        &self,
        req: CreateOrUpdateRequest,
    ) -> Result<Response<CreateOrUpdateResponse>, Status> {
        let note = self
            .note_repo
            .upsert(
                req.user_id,
                req.id,
                req.title,
                req.content,
                if req.images.is_none() {
                    None
                } else {
                    let list = req.images.unwrap_or_default();
                    Some(list.images)
                },
                req.video,
                if req.status.is_none() {
                    None
                } else {
                    let ret = req.status.unwrap_or_default().try_into();
                    match ret {
                        Ok(status) => Some(status),
                        Err(e) => return Err(Status::invalid_argument(e.to_string())),
                    }
                },
            )
            .await;

        match note {
            Ok(_) => Ok(Response::new(CreateOrUpdateResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
