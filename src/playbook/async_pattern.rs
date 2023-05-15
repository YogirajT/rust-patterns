#![allow(dead_code)]
use std::str;

use tokio::fs::File;
use tokio::io::AsyncReadExt;

use std::io::{self, Read, Seek, SeekFrom};

struct Chunks<R> {
    read: R,
    size: usize,
    hint: (usize, Option<usize>),
}

impl<R> Chunks<R> {
    pub fn new(read: R, size: usize) -> Self {
        Self {
            read,
            size,
            hint: (0, None),
        }
    }

    pub fn from_seek(mut read: R, size: usize) -> io::Result<Self>
    where
        R: Seek,
    {
        let old_pos = read.stream_position()?;
        let len = read.seek(SeekFrom::End(0))?;

        let rest = (len - old_pos) as usize;
        if rest != 0 {
            read.seek(SeekFrom::Start(old_pos))?;
        }

        let min = (rest / size) + (rest % size != 0) as usize;

        Ok(Self {
            read,
            size,
            hint: (min, None),
        })
    }

}

impl<R> Iterator for Chunks<R>
where
    R: Read,
{
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.size);
        match self
            .read
            .by_ref()
            .take(chunk.capacity() as u64)
            .read_to_end(&mut chunk)
        {
            Ok(n) => {
                if n != 0 {
                    Some(Ok(chunk))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.hint
    }
}

pub async fn read_file() -> io::Result<String> {
    let mut f = File::open("foo").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    let s = match str::from_utf8(&buffer[..n]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };

    Ok(s.to_string())
}

#[cfg(test)]
mod async_tests {

    use crate::playbook::async_pattern::Chunks;

    use super::read_file;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn async_test() {
        assert!(true);
    }

    #[test]
    fn async_test2() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                assert!(true);
            })
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_test3() {
        let string = read_file().await.unwrap();
        assert_eq!(string, "111111111\n");
    }

    #[test]
    fn sync_read_test4() {
        let file = std::fs::File::open("foo").unwrap();
        let iter = Chunks::from_seek(file, 0x8).unwrap();
        //can throw error
        let chunks = iter.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks.capacity(), 4);
    }
}
