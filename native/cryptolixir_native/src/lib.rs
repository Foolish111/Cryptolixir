//#[rustler::nif]
//fn add(a: i64, b: i64) -> i64 {
//    a + b
//}

//rustler::init!("Elixir.Cryptolixir.Native");

use sha2::{Digest, Sha256}
use ruslter::{Encoder, Env, Term};

// NIF initalization
rustler::init!("Elixir.Cryptolixir.Native",[sha256]);

// Rust function called from Elixir
fn sha256
