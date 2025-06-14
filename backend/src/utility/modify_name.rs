/// 文字列から半角/全角のスペースを削除
pub fn normalize_name(name: &str) -> String {
    name.chars().filter(|c| !c.is_whitespace()).collect()
}
