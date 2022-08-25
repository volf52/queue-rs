#![deny(clippy::all)]

use std::collections::VecDeque;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi(js_name = "Queue")]
#[derive(Debug, Default)]
pub struct Queue {
  q: VecDeque<i64>,
}

#[napi]
impl Queue {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self { q: VecDeque::new() }
  }

  #[napi]
  pub fn push(&mut self, val: i64) {
    self.q.push_back(val);
  }

  #[napi]
  pub fn pop(&mut self) -> Option<i64> {
    self.q.pop_front()
  }

  #[napi]
  pub fn peek(&mut self) -> Option<i64> {
    let q_len = self.q.len();

    match q_len {
      0 => None,
      _ => Some(self.q[q_len - 1]),
    }
  }

  #[napi]
  pub fn len(&self) -> i64 {
    self.q.len() as i64
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.q.is_empty()
  }
}
