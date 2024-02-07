use base64::prelude::*;

pub fn game_config_url() -> String{
    return crate::tools::vec_to_string(BASE64_STANDARD.decode(b"aHR0cHM6Ly9yaXpsaW5lYXNzZXRzdG9yZS5waWdlb25nYW1lcy5jbi9jb25maWdzL2dhbWVfY29uZmlnLmpzb24=").unwrap()).unwrap();
}

pub fn game_apkpure_download_url() -> String{
    return crate::tools::vec_to_string(BASE64_STANDARD.decode(b"aHR0cHM6Ly9kLmFwa3B1cmUubmV0L2IvQVBLL2NvbS5QaWdlb25HYW1lcy5SaXpsaW5lP3ZlcnNpb249bGF0ZXN0").unwrap()).unwrap();
}

pub fn game_apkcombo_download_page() -> String{
    return crate::tools::vec_to_string(BASE64_STANDARD.decode(b"aHR0cHM6Ly9hcGtjb21iby5jb20vdHcvcml6bGluZS9jb20uUGlnZW9uR2FtZXMuUml6bGluZS9kb3dubG9hZC9hcGs=").unwrap()).unwrap();
}