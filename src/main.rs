// #![feature(i128_type)]

// extern crate telegram;
// extern crate serde;

// #[macro_use]
// extern crate serde_derive;

// use std::mem::size_of;
// use std::io::Cursor;
// use serde::{Serialize, Deserialize, Deserializer};
// use serde::ser::{SerializeStruct, SerializeTuple};

// // Exports `Serialize` and `Deserialize` procedural derive macros
// // telegram_derive

// // telegram_codegen

extern crate telegram_codegen;

fn main() {
    telegram_codegen::translate("schema.json", "schema.rs").unwrap();
}

// // Path: /api
// // [..] they are made into a payload which is followed by a POST request to the URL/api [..]
// // https://core.telegram.org/mtproto#http-transport

// // #[derive(Serialize, Deserialize, Debug, Clone)]
// // #[id(85337187)]
// struct ResPQ {
//     nonce: i128,
//     server_nonce: i128,
//     pq: Vec<u8>,
//     server_public_key_fingerprints: Vec<i64>,
// }

// pub fn req_pq(nonce: i128) -> Result<()> {
//     // #[derive(Serialize)]
//     // #[id(1615239032)]
//     struct req_pq {
//         nonce: i128,
//     }

//     let data = req_pq { nonce: nonce };

//     // A message is ::
//     //  auth_key_id: u64
//     //  message_id: u64
//     //  message_length: u32
//     //  data: ...

//     let mut body = Vec::<u8>::new();

//     // Key Identifier
//     // [auth_key_id: u64]
//     0u64.serialize(&mut body);

//     // Compute message identifier
//     let now_d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
//     let now_s = now_d.as_secs();
//     let message_id = ((now_s as u64) << 32) + (now_d.subsec_nanos() as u64);

//     // Message Identifier
//     // [message_id: u64]
//     message_id.serialize(&mut body);

//     // Compute message length
//     // <identifier> (4) + <data size>
//     let message_data_length = 4 + (size_of::<req_pq>()) as u32;

//     // Message Length
//     message_data_length.serialize(&mut body);

//     // Message Data

//     // Identifier
//     1615239032i32.serialize(&mut body);

//     // ***
//     data.nonce.serialize(&mut body);

//     let client = Client::new();
//     let mut res = client.post("http://149.154.167.50:443/api")
//         .header(Connection::keep_alive())
//         .header(ContentLength(body.len() as u64))
//         .body(Body::BufBody(&body, body.len()))
//         .send()?;

//     println!("status: {}", res.status);
//     println!("headers: {}", res.headers);
//     println!("\n-----\n{:?}", res);

//     let mut res_body = Vec::new();
//     res.read_to_end(&mut res_body)?;

//     let mut f = File::create("foo.txt")?;
//     f.write_all(&res_body)?;

//     Ok(())
// }
