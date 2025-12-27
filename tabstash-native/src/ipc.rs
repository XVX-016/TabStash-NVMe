use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub async fn read_message() -> io::Result<Vec<u8>> {
    let mut stdin = io::stdin();
    let mut len_buf = [0u8; 4];
    stdin.read_exact(&mut len_buf).await?;
    let len = u32::from_le_bytes(len_buf) as usize;

    let mut buffer = vec![0u8; len];
    stdin.read_exact(&mut buffer).await?;
    Ok(buffer)
}

pub async fn write_message(data: &[u8]) -> io::Result<()> {
    let mut stdout = io::stdout();
    let len = data.len() as u32;
    stdout.write_all(&len.to_le_bytes()).await?;
    stdout.write_all(data).await?;
    stdout.flush().await?;
    Ok(())
}

