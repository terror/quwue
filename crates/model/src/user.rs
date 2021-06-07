use crate::common::*;

#[derive(Debug, Eq, PartialEq)]
pub struct User {
  pub id:                u64,
  pub discord_id:        UserId,
  pub prompt_message:    Option<PromptMessage>,
  pub welcomed:          bool,
  pub bio:               Option<String>,
  pub profile_image_url: Option<Url>,
}

impl User {
  pub fn update(&self, response: &Response) -> Update {
    let prompt = if let Some(prompt_message) = self.prompt_message {
      prompt_message.prompt
    } else {
      return Update {
        prompt: Prompt::Welcome,
        action: None,
      };
    };

    let action = match response {
      Response::Message(content) => Self::action_for_message(prompt, content),
      Response::Reaction(emoji) => Self::action_for_reaction(prompt, *emoji),
      Response::Image(url) => Self::action_for_image(prompt, url.clone()),
      Response::UnrecognizedReaction(..) | Response::Custom(..) => None,
    };

    let action = if let Some(action) = action {
      action
    } else {
      return Update {
        action: None,
        prompt,
      };
    };

    Update {
      prompt: self.next_prompt(&action),
      action: Some(action),
    }
  }

  fn action_for_message(prompt: Prompt, content: &str) -> Option<Action> {
    use Prompt::*;

    let content = content.trim();

    match prompt {
      Welcome =>
        if content.to_lowercase() == "ok" {
          return Some(Action::Welcome);
        },
      Bio =>
        return Some(Action::SetBio {
          text: content.to_owned(),
        }),
      Candidate { id } => match content.to_lowercase().as_str() {
        "yes" | "y" => return Some(Action::AcceptCandidate { id }),
        "no" | "n" => return Some(Action::RejectCandidate { id }),
        _ => {},
      },
      Quiescent | ProfileImage | Match { .. } => {},
    }

    None
  }

  fn action_for_reaction(prompt: Prompt, emoji: Emoji) -> Option<Action> {
    use Emoji::*;
    use Prompt::*;

    match prompt {
      Welcome =>
        if emoji == ThumbsUp {
          Some(Action::Welcome)
        } else {
          None
        },
      Candidate { id } => match emoji {
        ThumbsUp => Some(Action::AcceptCandidate { id }),
        ThumbsDown => Some(Action::RejectCandidate { id }),
      },
      Quiescent | Bio | ProfileImage | Match { .. } => None,
    }
  }

  fn action_for_image(prompt: Prompt, url: Url) -> Option<Action> {
    use Prompt::*;

    match prompt {
      ProfileImage => Some(Action::SetProfileImage { url }),
      Welcome | Quiescent | Candidate { .. } | Bio | Match { .. } => None,
    }
  }

  fn next_prompt(&self, action: &Action) -> Prompt {
    if !(self.welcomed || *action == Action::Welcome) {
      return Prompt::Welcome;
    }

    if self.bio.is_none() {
      if let Action::SetBio { .. } = action {
      } else {
        return Prompt::Bio;
      }
    }

    if self.profile_image_url.is_none() {
      if let Action::SetProfileImage { .. } = action {
      } else {
        return Prompt::ProfileImage;
      }
    }

    Prompt::Quiescent
  }
}