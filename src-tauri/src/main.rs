use smb::{Client, Config, TreeConnect, Session};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // SMBサーバーの設定
        let config = Config::default();
        let client = Client::new(config);

        // サーバー情報
        let server = "192.168.1.10";
        let share = "shared";
        let username = "guest";
        let password = "";

        // サーバーに接続
        match client
            .connect(server, share, username, password)
            .await
        {
            Ok(session) => {
                // セッションが確立された場合の処理
                println!("Successfully connected to SMB server.");

                // ここでファイルの読み取りや書き込みなどの操作を行います
            }
            Err(e) => {
                // エラーが発生した場合の処理
                eprintln!("Failed to connect to SMB server: {:?}", e);
            }
        }
    });
}
