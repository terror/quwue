use crate::common::*;

#[macro_export]
macro_rules! test_bot {
  () => {{
    crate::test_bot::TestBot::new(crate::test_name!())
  }};
}

use futures::future::{Map, Shared};
use tokio::{
  sync::oneshot::{error::RecvError, Receiver},
  task::JoinError,
};

type MapResult = fn(Result<JoinError, RecvError>) -> Arc<Result<JoinError, RecvError>>;

pub(crate) type ErrorReceiver = Shared<Map<Receiver<JoinError>, MapResult>>;

pub(crate) struct TestBot {
  error:            ErrorReceiver,
  test_name:        String,
  next_user_number: u64,
  bot:              Bot,
  #[allow(unused)]
  tmpdir:           TempDir,
}

impl TestBot {
  pub(crate) async fn new(test_name: String) -> Self {
    let test_run = TestDispatcher::get_instance().await.test_run_id();

    let test_id = TestId::new(test_run, test_name.clone());

    let tmpdir = tempfile::tempdir().unwrap();

    let db_path = tmpdir.path().join("db.sqlite");

    let bot = Bot::new_test_instance(&db_path, test_id)
      .await
      .expect("Failed to construct quwue instance");

    let clone = bot.clone();
    let handle = tokio::spawn(async move {
      clone.run().await.expect("quwue failed");
    });

    let (tx, rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
      if let Err(err) = handle.await {
        if err.is_panic() {
          tx.send(err).unwrap();
        }
      }
    });

    Self {
      error: rx.map(Arc::new as MapResult).shared(),
      next_user_number: 0,
      bot,
      test_name,
      tmpdir,
    }
  }

  pub(crate) async fn new_user(&mut self) -> TestUser {
    let next_user_number = self.next_user_number;
    self.next_user_number += 1;
    let test_user_id = TestUserId::new(self.test_name.clone(), next_user_number);
    TestUser::new(self.bot.clone(), self.error.clone(), test_user_id).await
  }

  pub(crate) fn db(&self) -> &Db {
    self.bot.db()
  }
}
