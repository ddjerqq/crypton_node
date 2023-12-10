use std::io;
use std::io::{ErrorKind, Read, Write};
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};

pub fn send<TWriter: Write, TData: Serialize>(mut writer: TWriter, data: &TData) -> io::Result<()> {
    let mut ser = Serializer::new(&mut writer);

    match data.serialize(&mut ser) {
        Ok(_) => {
            writer.flush()?;
            Ok(())
        }

        Err(e) => {
            Err(io::Error::new(ErrorKind::Other, format!("Failed to serialize data: {:?}", e)))
        }
    }
}

pub fn recv<'de, TReader: Read, TData: Deserialize<'de>>(reader: &mut TReader) -> io::Result<TData> {
    let mut de = rmp_serde::Deserializer::new(reader);

    Deserialize::deserialize(&mut de)
        .map_err(|e| io::Error::new(ErrorKind::Other, format!("Failed to deserialize data: {:?}", e)))
}
