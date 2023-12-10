use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Echo,
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let msg = Message::Text("hello".into());
        let mut buf = Vec::new();
        let mut ser = rmp_serde::Serializer::new(&mut buf);
        msg.serialize(&mut ser).unwrap();

        eprintln!("msg = {:?}", msg);
        eprintln!("buf = {:?}", buf);

        let mut de = rmp_serde::Deserializer::new(&buf[..]);
        let msg: Message = Deserialize::deserialize(&mut de).unwrap();

        eprintln!("msg = {:?}", msg);
    }
}

// impl From<&Message> for u8 {
//     fn from(req: &Message) -> Self {
//         match req {
//             Message::Echo => 1,
//             Message::Text(_) => 2,
//         }
//     }
// }