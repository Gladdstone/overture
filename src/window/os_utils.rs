pub fn is_linux() -> bool {
    if std::env::consts::OS == "linux" {
        return true;
    }
    return false;
}

pub fn is_mac() -> bool {
    if std::env::consts::OS == "macos" {
        return true;
    }
    return false;
}

