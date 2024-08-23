use log::debug;
use russh::{server::Handle, ChannelId};

pub struct Pty {
    buffer: Vec<u8>,
    channel_id: ChannelId,
    handle: Handle,
}

impl Pty {
    pub fn new(channel_id: ChannelId, handle: Handle) -> Self {
        Pty {
            channel_id: channel_id,
            handle: handle,
            buffer: Vec::new(),
        }
    }
}

impl std::io::Write for Pty {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        futures::executor::block_on(async {
            // TODO error handling
            debug!("okay, writing now");
            let _ = self
                .handle
                .data(self.channel_id, self.buffer.clone().into())
                .await;
            debug!("done writing");
        });
        self.buffer.clear();
        Ok(())
    }
}
