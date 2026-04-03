use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use teloxide::types::Message;
use tokio::sync::RwLock;

pub type AllowedUsers = Arc<RwLock<HashSet<i64>>>;

pub fn load_allowed_users<P: AsRef<Path>>(path: P) -> Result<HashSet<i64>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;

    let users: HashSet<i64> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && ! line.starts_with('#'))
        .filter_map(| line | line.parse::<i64>().ok())
        .collect();
    Ok(users)
}

pub async fn check_access(msg: Message, allowed_users: &AllowedUsers) -> bool {
    let user_id = match msg.from {
        Some(user) => user.id.0 as i64,
        None => return false,
    };

    let allowed = allowed_users.read().await;
    allowed.contains(&user_id)
}

pub fn check_access_filter(
    allowed_users: AllowedUsers,
) -> impl Fn(Message) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + Send>> {
    move |msg: Message| {
        let allowed = allowed_users.clone();
        Box::pin(async move {check_access(msg, &allowed).await})

    }
}