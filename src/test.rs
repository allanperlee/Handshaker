#[cfg(test)]
mod tests {
    use super::*;
    use std::net::ToSocketAddrs;

    #[tokio::test]
    async fn test_handshake() -> Result<(), Error> {
        let socket_addr = format!("{}:{}", TO_ADDR, PORT)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();

        let mut peer = BitcoinPeer::connect(socket_addr).await?;
        peer.send_version().await?;
        let mut message = peer.read_message().await?;
        match message {
            NetworkMessage::Version(version_message) => {
                println!("Version Message Returned: {:?}", version_message);

                let verack_message = NetworkMessage::Verack;
                peer.send_message(verack_message).await?;

                message = peer.read_message().await?;
                assert_eq!(message, NetworkMessage::Verack);

                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Expected Version message",
            )),
        }
    }

    #[tokio::test]
    async fn test_handshake_and_getaddr() -> Result<(), Error> {
        let socket_addr = format!("{}:{}", TO_ADDR, PORT)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();

        let mut peer = BitcoinPeer::connect(socket_addr).await?;
        peer.send_version().await?;
        let mut message = peer.read_message().await?;
        match message {
            NetworkMessage::Version(version_message) => {
                println!("Version Message Returned: {:?}", version_message);

                let verack_message = NetworkMessage::Verack;
                peer.send_message(verack_message).await?;

                message = peer.read_message().await?;
                assert_eq!(message, NetworkMessage::Verack);

                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Expected Version message",
            )),
        }
    
        
    }

    #[tokio::test]
    #[should_panic]
    async fn test_handshake_fail() -> Result<(), Error> {
        let socket_addr = format!("{}:{}", TO_ADDR, PORT)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();

        let mut peer = BitcoinPeer::connect(socket_addr).await?;
        peer.send_version().await?;
        let mut message = peer.read_message().await?;
        match message {
            NetworkMessage::Version(version_message) => {
                println!("Version Message Returned: {:?}", version_message);

                let wrong_message = NetworkMessage::GetBlocks;
                peer.send_message(wrong_message).await?;

                message = peer.read_message().await?;
                assert_eq!(message, NetworkMessage::Verack);

                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Expected Version message",
            )),
        }
    
        
    }
}

