use crate::{HubService, HubServiceError};
use objs::{
  impl_error_from, Alias, AppError, ChatTemplate, ChatTemplateError, ChatTemplateType, HubFile,
  ObjValidationError, TOKENIZER_CONFIG_JSON,
};
use std::sync::Arc;
use validator::Validate;

#[derive(Debug, thiserror::Error, errmeta_derive::ErrorMeta)]
#[error_meta(trait_to_impl = AppError)]
pub enum ObjExtsError {
  #[error(transparent)]
  HubService(#[from] HubServiceError),
  #[error(transparent)]
  ChatTemplate(#[from] ChatTemplateError),
  #[error(transparent)]
  ObjValidationError(#[from] ObjValidationError),
}

impl_error_from!(
  ::validator::ValidationErrors,
  ObjExtsError::ObjValidationError,
  objs::ObjValidationError
);

pub trait IntoChatTemplate {
  #[allow(clippy::wrong_self_convention)]
  fn into_chat_template(
    &self,
    hub_service: Arc<dyn HubService>,
    alias: &Alias,
  ) -> Result<ChatTemplate, ObjExtsError>;
}

impl IntoChatTemplate for ChatTemplateType {
  fn into_chat_template(
    &self,
    hub_service: Arc<dyn HubService>,
    alias: &Alias,
  ) -> Result<ChatTemplate, ObjExtsError> {
    let chat_template = match self {
      ChatTemplateType::Id(id) => {
        let repo = (*id).into();
        let file = hub_service.find_local_file(&repo, TOKENIZER_CONFIG_JSON, None)?;
        let chat_template: ChatTemplate = ChatTemplate::try_from(file)?;
        chat_template.validate()?;
        chat_template
      }
      ChatTemplateType::Repo(repo) => {
        let repo = repo.clone();
        let file = hub_service.find_local_file(&repo, TOKENIZER_CONFIG_JSON, None)?;
        let chat_template: ChatTemplate = ChatTemplate::try_from(file)?;
        chat_template.validate()?;
        chat_template
      }
      ChatTemplateType::Embedded => hub_service.model_chat_template(alias)?,
    };
    Ok(chat_template)
  }
}

pub trait HubDownloadable {
  fn download(&self, hub_service: Arc<dyn HubService>) -> Result<Option<HubFile>, ObjExtsError>;
}

impl HubDownloadable for ChatTemplateType {
  fn download(&self, hub_service: Arc<dyn HubService>) -> Result<Option<HubFile>, ObjExtsError> {
    let repo = match self {
      ChatTemplateType::Id(id) => Some((*id).into()),
      ChatTemplateType::Repo(repo) => Some(repo.clone()),
      ChatTemplateType::Embedded => None,
    };
    if let Some(repo) = repo {
      let hub_file = hub_service.download(&repo, TOKENIZER_CONFIG_JSON, None)?;
      Ok(Some(hub_file))
    } else {
      Ok(None)
    }
  }
}
