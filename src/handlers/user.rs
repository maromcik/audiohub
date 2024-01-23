use crate::authorized;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use crate::templates::user::{
    LoginTemplate, RegistrationTemplate, UserManagePasswordTemplate,
    UserManageProfileContentTemplate, UserManageProfilePageTemplate,
    UserManageProfilePictureFormTemplate, UserManageProfilePictureTemplate,
    UserManageProfileUserFormTemplate,
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

use crate::database::models::user::{User, UserCreate, UserDisplay, UserGetById, UserLogin, UserUpdate, UserUpdatePassword};
use crate::error::AppErrorKind::InternalServerError;
use crate::forms::user::{UserLoginReturnURL, ProfilePictureUploadForm, UserCreateForm, UserUpdateForm, UserUpdatePasswordForm, UserLoginForm};

use crate::handlers::utilities::{
    get_user_from_identity, parse_user_id, remove_file, save_file, validate_file,
};

#[get("/register")]
pub async fn register() -> Result<HttpResponse, AppError> {
    let template = RegistrationTemplate {
        message: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/login")]
pub async fn login(identity: Option<Identity>,
                   query: web::Query<UserLoginReturnURL>) -> Result<HttpResponse, AppError> {
    let return_url = query.ret.clone().unwrap_or("/".to_string());
    if identity.is_some() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, return_url))
            .finish());
    }
    let template = LoginTemplate {
        message: "".to_string(),
        return_url
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/register")]
pub async fn register_user(
    form: web::Form<UserCreateForm>,
    user_repo: web::Data<UserRepository>,
) -> Result<impl Responder, AppError> {

    if form.password != form.confirm_password {
        let template = RegistrationTemplate {
            message: "Passwords do not match".to_string(),
        };
        let body = template.render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    let new_user = UserCreate {
        username: form.username.to_string(),
        email: form.email.to_string(),
        name: form.name.to_string(),
        surname: form.surname.to_string(),
        bio: String::new(),
        profile_picture: None,
        password: form.password.clone(),
    };

    user_repo.create(&new_user).await?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/user/login"))
        .finish())

}

#[post("/login")]
pub async fn login_user(
    request: HttpRequest,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserLoginForm>,
) -> Result<impl Responder, AppError> {
    match user_repo
        .read_one(&UserLogin::new(&form.email_or_username, &form.password))
        .await
    {
        Ok(user) => {
            Identity::login(&request.extensions(), user.id.to_string())?;
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, form.return_url.clone()))
                .finish())
        }
        Err(db_error) => {
            let Some(backend_error) = db_error.get_backend_error() else {
                return Err(AppError::from(db_error));
            };

            if backend_error.is_login_error() {
                let template = LoginTemplate {
                    message: backend_error.to_string(),
                    return_url: form.return_url.clone()
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
pub async fn user_manage_form_page(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user = user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?;
    let template = UserManageProfilePageTemplate {
        user: UserDisplay::from(user),
        message: "".to_string(),
        message_success: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage-content")]
pub async fn user_manage_form_content(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user = user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?;
    let template = UserManageProfileContentTemplate {
        user: UserDisplay::from(user),
        message: "".to_string(),
        message_success: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage/password")]
pub async fn user_manage_password_form(
    request: HttpRequest,
    identity: Option<Identity>,
) -> Result<impl Responder, AppError> {
    authorized!(identity, request.path());
    let template = UserManagePasswordTemplate {
        message: "".to_string(),
        message_success: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage/picture")]
pub async fn user_manage_picture_form(
    request: HttpRequest,
    identity: Option<Identity>,
) -> Result<impl Responder, AppError> {
    authorized!(identity, request.path());
    let template = UserManageProfilePictureFormTemplate {
        message: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage/profile")]
pub async fn user_manage_profile_form(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user = user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?;
    let template = UserManageProfileUserFormTemplate {
        user: UserDisplay::from(user),
        message: "".to_string(),
        message_success: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/manage")]
pub async fn user_manage(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserUpdateForm>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
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
    let user = user_repo.update(&user_update).await?;

    if let Some(user_valid) = user.get(0) {
        let template = UserManageProfileUserFormTemplate {
            user: UserDisplay::from(user_valid.clone()),
            message: "".to_string(),
            message_success: "Profile update successful".to_string(),
        };
        let body = template.render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    Err(AppError::new(InternalServerError, "Update of user profile failed"))
}

#[post("/manage/password")]
pub async fn user_manage_password(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserUpdatePasswordForm>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());

    if form.new_password != form.confirm_password {
        let template = UserManagePasswordTemplate {
            message: "Passwords do not match".to_string(),
            message_success: "".to_string(),
        };
        let body = template.render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    let update_status = user_repo
        .update_password(&UserUpdatePassword::new(
            &parse_user_id(u)?,
            &form.old_password,
            &form.new_password,
        ))
        .await;

    if update_status.is_err() {
        let template = UserManagePasswordTemplate {
            message: "Old password incorrect".to_string(),
            message_success: "".to_string(),
        };
        let body = template.render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    let template = UserManagePasswordTemplate {
        message: "".to_string(),
        message_success: "Password update successful".to_string(),
    };
    let body = template.render()?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(body));
}

#[post("/manage/picture")]
pub async fn user_manage_picture(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    MultipartForm(form): MultipartForm<ProfilePictureUploadForm>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let path = validate_file(
        &form.picture,
        Uuid::new_v4(),
        "image",
        "user",
        AppErrorKind::ProfilePictureUploadError,
    )?;
    let user = get_user_from_identity(u, &user_repo).await?;
    if let Some(pic) = &user.profile_picture {
        remove_file(pic)?;
    }
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

    let users = user_repo.update(&user_update).await?;
    save_file(form.picture, &path, AppErrorKind::ProfilePictureUploadError)?;
    // // Ok(HttpResponse::Ok()
    // //     .insert_header(("HX-Redirect", "/user/manage"))
    // //     .finish())

    if let Some(user) = users.into_iter().next() {
        let template = UserManageProfilePictureTemplate {
            user: UserDisplay::from(user),
        };
        let body = template.render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }
    // TEMPORARY SOLUTION
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}
