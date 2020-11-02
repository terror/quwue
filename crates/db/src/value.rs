use crate::common::*;

pub(crate) trait Value: Sized {
  type Err;
  type Storage;

  fn store(self) -> Self::Storage;

  fn load(storage: Self::Storage) -> Result<Self, Self::Err>;
}

impl Value for u64 {
  type Err = Infallible;
  type Storage = i64;

  fn store(self) -> Self::Storage {
    i64::from_le_bytes(self.to_le_bytes())
  }

  fn load(storage: Self::Storage) -> Result<Self, Self::Err> {
    Ok(Self::from_le_bytes(storage.to_le_bytes()))
  }
}

impl Value for Prompt {
  type Err = Error;
  type Storage = i64;

  fn store(self) -> Self::Storage {
    let n: u64 = self.into();
    i64::from_le_bytes(n.to_le_bytes())
  }

  fn load(storage: Self::Storage) -> Result<Self, Self::Err> {
    let value = u64::from_le_bytes(storage.to_le_bytes());
    Self::try_from(value).context(error::PromptLoad)
  }
}

impl Value for UserId {
  type Err = Infallible;
  type Storage = i64;

  fn store(self) -> Self::Storage {
    self.0.store()
  }

  fn load(storage: Self::Storage) -> Result<Self, Self::Err> {
    Ok(UserId(u64::load(storage).unwrap_infallible()))
  }
}

impl Value for MessageId {
  type Err = Infallible;
  type Storage = i64;

  fn store(self) -> Self::Storage {
    self.0.store()
  }

  fn load(storage: Self::Storage) -> Result<Self, Self::Err> {
    Ok(MessageId(u64::load(storage).unwrap_infallible()))
  }
}
