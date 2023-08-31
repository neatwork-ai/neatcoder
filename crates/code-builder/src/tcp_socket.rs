use crate::prelude::*;
use byteorder::{BigEndian, ByteOrder};
use code_builder::models::messages::outer::{ClientMsg, ServerMsg};
use serde_json;
use std::io::{self};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn try_read(
    socket: &mut TcpStream,
    buf: &mut Vec<u8>,
    length_buf: &mut [u8; 4],
) -> Result<Option<ClientMsg>> {
    const DELIMITER: &[u8] = b"MSG:"; // Define a suitable delimiter

    // TODO: we only need last `DELIMITER.len()` bytes
    let mut temp_buf = Vec::new();

    // Search for the delimiter
    while temp_buf.ends_with(DELIMITER) == false {
        let mut byte = [0u8; 1];

        match socket.try_read(&mut byte) {
            // socket.readable was a false positive
            Ok(0) => return Ok(None),
            // again, socket.readable was a false positive
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                return Ok(None)
            }
            Err(e) => {
                return Err(anyhow!(
                    "Got an error while searching for delimiter: {e}"
                ));
            }
            Ok(_) => {
                // remember the byte
                temp_buf.extend_from_slice(&byte);
            }
        }
    }
    // Once delimiter is found, read the LENGTH prefix
    socket.read_exact(length_buf).await?;
    let message_length = u32::from_be_bytes(*length_buf);

    // Ensure our buffer is big enough to hold the incoming message
    if buf.len() < message_length as usize {
        buf.resize(message_length as usize, 0);
    }

    // Read the actual message
    let n = socket
        .read_exact(&mut buf[0..message_length as usize])
        .await?;
    if n == 0 {
        warn!("Received an empty message!");
        return Ok(None);
    }

    match serde_json::from_slice::<ClientMsg>(&buf[..n]) {
        Ok(command) => Ok(Some(command)),
        Err(e) => {
            warn!(
                "Invalid message received: {e}\n\n{msg}",
                msg = String::from_utf8_lossy(&buf[..n])
            );
            Ok(None)
        }
    }
}

pub async fn write(socket: &mut TcpStream, msg: ServerMsg) -> Result<()> {
    socket.writable().await?;

    // Define the delimiter
    const DELIMITER: &[u8] = b"MSG:";

    // Serialize the message using serde
    let msg_string = serde_json::to_string(&msg)?;
    let msg_buffer = msg_string.as_bytes().to_vec();

    // Calculate the length and convert it to a buffer of 4 bytes
    let mut length_buffer = [0u8; 4];
    BigEndian::write_u32(&mut length_buffer, msg_buffer.len() as u32);

    // Combine DELIMITER, length_buffer, and message_buffer
    let mut complete_buffer = Vec::new();
    complete_buffer.extend_from_slice(DELIMITER);
    complete_buffer.extend_from_slice(&length_buffer);
    complete_buffer.extend_from_slice(&msg_buffer);

    // Write the complete buffer to the socket
    socket.write_all(&complete_buffer).await?;

    Ok(())
}
