
/// 加密
pub fn hash_pwd<P: AsRef<[u8]>>(pwd: P) -> String {
    return bcrypt::hash(pwd, bcrypt::DEFAULT_COST).unwrap();
}

/// 密码是否匹配
pub fn is_match_pwd<P: AsRef<[u8]>>(raw: P, target: String) -> bool {
    bcrypt::verify(raw, target.as_str()).unwrap_or(false)
}