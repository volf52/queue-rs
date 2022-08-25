#![deny(clippy::all)]

use std::collections::VecDeque;

#[macro_use]
extern crate napi_derive;

#[napi(js_name = "Queue")]
#[derive(Debug, Default)]
pub struct Queue(VecDeque<i64>);

#[napi]
impl Queue {
  #[napi(constructor)]
  pub fn new() -> Self {
    let q = VecDeque::new();

    Self(q)
  }

  #[napi]
  pub fn push(&mut self, val: i64) {
    self.0.push_back(val);
  }

  #[napi]
  pub fn pop(&mut self) -> Option<i64> {
    self.0.pop_front()
  }

  #[napi]
  pub fn peek(&mut self) -> Option<i64> {
    let q_len = self.0.len();

    match q_len {
      0 => None,
      _ => Some(self.0[q_len - 1]),
    }
  }

  #[napi]
  pub fn len(&self) -> i64 {
    self.0.len() as i64
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}
