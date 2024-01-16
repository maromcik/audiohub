use crate::authorized;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use crate::templates::user::{
    LoginTemplate, RegistrationTemplate, UserManagePasswordTemplate,
    UserManageProfilePictureFormTemplate, UserManageProfileTemplate,
};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::web::Redirect;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use askama::Template;
use hmac::Mac;
use uuid::Uuid;

use crate::database::common::{DbCreate, DbReadOne, DbUpdate};

use crate::database::models::user::{
    UserCreate, UserGetById, UserLogin, UserUpdate, UserUpdatePassword,
};
use crate::forms::user::{
    ProfilePictureUploadForm, UserCreateForm, UserUpdateForm, UserUpdatePasswordForm,
};

use crate::handlers::utilities::{
    get_user_from_identity, parse_user_id, remove_file, save_file, validate_file,
};

#[get("/register")]
pub async fn register() -> Result<HttpResponse, AppError> {
    let template = RegistrationTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/login")]
pub async fn login(identity: Option<Identity>) -> Result<HttpResponse, AppError> {
    if identity.is_some() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish());
    }
    let template = LoginTemplate {
        message: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/register")]
pub async fn register_user(
    form: web::Form<UserCreateForm>,
    user_repo: web::Data<UserRepository>,
) -> Result<impl Responder, AppError> {
    let new_user = UserCreate {
        username: form.username.to_string(),
        email: form.email.to_string(),
        name: form.name.to_string(),
        surname: form.surname.to_string(),
        bio: String::new(),
        profile_picture: String::new(),
        password: form.password.clone(),
    };

    user_repo.create(&new_user).await?;
    Ok(Redirect::to("/user/login").using_status_code(StatusCode::FOUND))
}

#[post("/login")]
pub async fn login_user(
    request: HttpRequest,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserLogin>,
) -> Result<impl Responder, AppError> {
    match user_repo
        .read_one(&UserLogin::new(&form.email_or_username, &form.password))
        .await
    {
        Ok(user) => {
            Identity::login(&request.extensions(), user.id.to_string())?;
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish())
        }
        Err(db_error) => {
            let Some(backend_error) = db_error.get_backend_error() else {
                return Err(AppError::from(db_error));
            };

            if backend_error.is_login_error() {
                let template = LoginTemplate {
                    message: backend_error.to_string(),
                };
                let body = template.render()?;
                return Ok(HttpResponse::Ok().content_type("text/html").body(body));
            }

            Err(AppError::from(db_error))
        }
    }
}

#[get("/logout")]
pub async fn logout_user(identity: Option<Identity>) -> Result<impl Responder, AppError> {
    if let Some(u) = identity {
        u.logout();
    }
    Ok(Redirect::to("/user/login").using_status_code(StatusCode::FOUND))
}

#[get("/manage")]
pub async fn user_manage_form(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity);
    let user = user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?;
    let template = UserManageProfileTemplate { user };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage/password")]
pub async fn user_manage_password_form(
    identity: Option<Identity>,
) -> Result<impl Responder, AppError> {
    authorized!(identity);
    let template = UserManagePasswordTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage/picture")]
pub async fn user_manage_picture_form(
    identity: Option<Identity>,
) -> Result<impl Responder, AppError> {
    authorized!(identity);
    let template = UserManageProfilePictureFormTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/manage")]
pub async fn user_manage(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserUpdateForm>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity);
    let user_update = UserUpdate::new(
        &parse_user_id(u)?,
        Some(&form.username),
        Some(&form.email),
        Some(&form.name),
        Some(&form.surname),
        Some(&form.bio),
        None,
        None,
    );
    user_repo.update(&user_update).await?;
    // TEMPORARY SOLUTION
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}

#[post("/manage/password")]
pub async fn user_manage_password(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserUpdatePasswordForm>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity);
    user_repo
        .update_password(&UserUpdatePassword::new(
            &parse_user_id(u)?,
            &form.old_password,
            &form.new_password,
        ))
        .await?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}

#[post("/manage/picture")]
pub async fn user_manage_picture(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    MultipartForm(form): MultipartForm<ProfilePictureUploadForm>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity);
    let path = validate_file(&form.picture, Uuid::new_v4(), "image", "user")?;
    let user = get_user_from_identity(u, user_repo.clone()).await?;
    remove_file(&user.profile_picture)?;
    let user_update = UserUpdate::new(
        &user.id,
        None,
        None,
        None,
        None,
        None,
        Some(path.as_str()),
        None,
    );

    user_repo.update(&user_update).await?;
    save_file(form.picture, path)?;
    // Ok(HttpResponse::Ok()
    //     .insert_header(("HX-Redirect", "/user/manage"))
    //     .finish())
    // TEMPORARY SOLUTION
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}
