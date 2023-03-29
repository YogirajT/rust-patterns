use std::io;
use std::str;

use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn read_file() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);

    let s = match str::from_utf8(&buffer[..n]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };

    println!("The string: {:?}", &s);

    Ok(())
}

#[cfg(test)]
mod async_tests {

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
        let _err = read_file().await;
    }
}
