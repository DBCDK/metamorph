// SPDX-FileCopyrightText: 2024 Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

const CONFIRMATION_YES: &str = "y";
const CONFIRMATION_NO: &str = "n";

/// Pauses execution, awaiting user confirmation
pub async fn get_confirmation() -> bool {
  use futures::StreamExt;
  use tokio::io::stdin;
  use tokio_util::codec::{FramedRead, LinesCodec};

  let mut reader = FramedRead::new(stdin(), LinesCodec::new());

  println!("Continue deploying (Y/n)");

  let mut input_buffer = reader.next().await.transpose().unwrap().unwrap();

  // Normalize input
  input_buffer = input_buffer.to_lowercase();

  // Remove newline/return from input
  if let Some('\n') = input_buffer.chars().next_back() {
    input_buffer.pop();
  }
  if let Some('\r') = input_buffer.chars().next_back() {
    input_buffer.pop();
  }

  match input_buffer.as_str() {
    CONFIRMATION_YES => true,
    CONFIRMATION_NO => false,
    _ => {
      println!("Please answer before continuing");
      Box::pin(get_confirmation()).await
    }
  }
}
