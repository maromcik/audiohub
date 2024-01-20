pub fn get_default_profile_picture(profile_picture: &Option<String>) -> String {
    profile_picture.clone().unwrap_or_else(|| "/static/images/profile_picture.png".to_string())
}

pub fn get_default_thumbnail(thumbnail: &Option<String>) -> String {
    thumbnail.clone().unwrap_or_else(|| "/static/images/thumbnail.png".to_string())
}