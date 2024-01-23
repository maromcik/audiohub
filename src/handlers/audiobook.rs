use crate::database::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::audiobook::{AudiobookCreate, AudiobookDelete, AudiobookDisplay, AudiobookGetByIdJoin, AudiobookUpdate};
use crate::database::models::genre::{GenreGetById, GenreSearch};

use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use crate::forms::audiobook::{
    AudiobookCreateForm, AudiobookQuickSearchQuery, AudiobookUploadForm, AudiobookEditForm, AudiobookThumbnailEditForm
};
use crate::handlers::utilities::{
    get_metadata_from_session, get_user_from_identity, parse_user_id, remove_file, save_file,
    validate_file, AudiobookCreateSessionKeys,
};
use crate::templates::audiobook::{AudiobookCreateContentTemplate, AudiobookCreatePageTemplate, AudiobookDetailContentTemplate, AudiobookDetailPageTemplate, AudiobookUploadFormTemplate, NewReleasesContentTemplate, NewReleasesPageTemplate, PlayerTemplate, QuickSearchResults, AudiobookEditContentTemplate, AudiobookEditPageTemplate};
use crate::templates::audiobook::{AudiobookDetailAuthorContentTemplate, AudiobookDetailAuthorPageTemplate, DetailLikesTemplate};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{get, patch, post, put, web, HttpResponse, delete, Responder};
use actix_web::http::StatusCode;
use actix_web::web::Redirect;

use askama::Template;
use lofty::AudioFile;
use serde::Deserialize;

use crate::authorized;
use crate::database::common::error::{BackendError, BackendErrorKind};
use crate::database::models::active_audiobook::SetActiveAudiobook;
use crate::database::models::bookmark::BookmarkOperation;

use uuid::Uuid;
use crate::handlers::helpers::{get_audiobook_detail_base, get_releases};

#[get("/create")]
pub async fn create_audiobook_page(
    identity: Option<Identity>,
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;
    let template = AudiobookCreatePageTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/create-content")]
pub async fn create_audiobook_content(
    identity: Option<Identity>,
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;
    let template = AudiobookCreateContentTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/edit")]
pub async fn edit_audiobook_page(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    user_repo: web::Data<UserRepository>,
    genre_repo: web::Data<GenreRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user = get_user_from_identity(identity, &user_repo).await?;
    let book_id = path.into_inner().0;
    let audiobook = AudiobookDisplay::from(audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(user.id, book_id))
        .await?);

    if user.id != audiobook.author_id {
        return Err(AppError::from(BackendError::new(
            BackendErrorKind::UnauthorizedOperation,
        )));
    }

    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;
    let template = AudiobookEditPageTemplate { genres: genres, audiobook: audiobook };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/edit-content")]
pub async fn edit_audiobook_content(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    user_repo: web::Data<UserRepository>,
    genre_repo: web::Data<GenreRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user = get_user_from_identity(identity, &user_repo).await?;
    let book_id = path.into_inner().0;
    let audiobook = AudiobookDisplay::from(audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(user.id, book_id))
        .await?);

    if user.id != audiobook.author_id {
        return Err(AppError::from(BackendError::new(
            BackendErrorKind::UnauthorizedOperation,
        )));
    }

    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;
    let template = AudiobookEditContentTemplate { genres: genres, audiobook: audiobook };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}


#[post("/edit")]
pub async fn edit_audiobook(
    identity: Option<Identity>,
    session: Session,
    genre_repo: web::Data<GenreRepository>,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    form: web::Form<AudiobookEditForm>,
) -> Result<impl Responder, AppError> {
    let identity = authorized!(identity);
    let user = get_user_from_identity(identity, &user_repo).await?;
    let book_id = form.audiobook_id;
    let audiobook = AudiobookDisplay::from(audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(user.id, book_id))
        .await?);

    if user.id != audiobook.author_id {
        return Err(AppError::from(BackendError::new(
            BackendErrorKind::UnauthorizedOperation,
        )));
    }

    let book_update = AudiobookUpdate::new(
        &book_id, Some(&form.name), None,
        Some(&form.genre_id), None, None,
        None, None, None, None,
        Some(&form.description));
    let book = audiobook_repo.update(&book_update).await?;

    let handler = format!("/audiobook/{}/manage-content", book_id);
    Ok(HttpResponse::Found().header("Location", handler).finish())
}

#[get("/upload")]
pub async fn upload_audiobook_form(identity: Option<Identity>) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let template = AudiobookUploadFormTemplate {
        message: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_audiobook(
    identity: Option<Identity>,
    session: Session,
    genre_repo: web::Data<GenreRepository>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<AudiobookCreateForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = AudiobookCreateSessionKeys::new(user.id);
    let genre = genre_repo
        .read_one(&GenreGetById::new(&form.genre_id))
        .await?;

    session.insert(session_keys.name.as_str(), &form.name)?;
    session.insert(session_keys.genre_id.as_str(), genre.id)?;
    session.insert(session_keys.description.as_str(), &form.description)?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/audiobook/upload"))
        .finish())
}

#[post("/upload")]
pub async fn upload_audiobook(
    identity: Option<Identity>,
    session: Session,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    MultipartForm(mut form): MultipartForm<AudiobookUploadForm>,
) -> Result<HttpResponse, AppError> {
    let uuid = Uuid::new_v4();
    let u = authorized!(identity);
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = AudiobookCreateSessionKeys::new(user.id);

    let audiobook_path = validate_file(
        &form.audio_file,
        uuid,
        "audio",
        "audiobook",
        AppErrorKind::AudiobookUploadError,
    )?;
    let thumbnail_path = match &form.thumbnail {
        None => None,
        Some(thumb) => Some(validate_file(
            thumb,
            uuid,
            "image",
            "audiobook",
            AppErrorKind::AudiobookUploadError,
        )?),
    };
    let metadata = get_metadata_from_session(&session, &session_keys)?;

    let audio_file = form.audio_file.file.as_file_mut();
    let lofty_audio_file = lofty::read_from(audio_file)?;
    let properties = lofty_audio_file.properties();
    let length = properties.duration().as_secs_f64();
    if let (Some(thumb_path), Some(thumbnail)) = (&thumbnail_path, form.thumbnail) {
        save_file(thumbnail, thumb_path, AppErrorKind::AudiobookUploadError)?;
    }
    let book_crate = AudiobookCreate::new(
        &metadata.name,
        &user.id,
        &metadata.genre_id,
        &audiobook_path,
        &length,
        thumbnail_path.clone(),
        &metadata.description,
    );
    let book = audiobook_repo.create(&book_crate).await?;

    save_file(
        form.audio_file,
        &audiobook_path,
        AppErrorKind::AudiobookUploadError,
    )?;

    session.remove(session_keys.name.as_str());
    session.remove(session_keys.description.as_str());
    session.remove(session_keys.genre_id.as_str());

    let handler = format!("/audiobook/{}/manage-content", book.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, handler))
        .finish())
}

#[get("/{id}/manage")]
pub async fn manage_audiobook(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    user_repo: web::Data<UserRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user = get_user_from_identity(identity, &user_repo).await?;
    let book_id = path.into_inner().0;
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(user.id, book_id))
        .await?;

    if user.id != audiobook.author_id {
        return Err(AppError::from(BackendError::new(
            BackendErrorKind::UnauthorizedOperation,
        )));
    }

    let base = get_audiobook_detail_base(audiobook_repo, chapter_repo, user.id, book_id).await?;
    let body = AudiobookDetailAuthorPageTemplate {
        audiobook: base.audiobook,
        chapters: base.chapters,
        is_liked: audiobook.is_liked,
    }
    .render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/manage-content")]
pub async fn manage_audiobook_content(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    user_repo: web::Data<UserRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user = get_user_from_identity(identity, &user_repo).await?;
    let book_id = path.into_inner().0;
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(user.id, book_id))
        .await?;

    if user.id != audiobook.author_id {
        return Err(AppError::from(BackendError::new(
            BackendErrorKind::UnauthorizedOperation,
        )));
    }

    let base = get_audiobook_detail_base(audiobook_repo, chapter_repo, user.id, book_id).await?;
    let body = AudiobookDetailAuthorContentTemplate {
        audiobook: base.audiobook,
        chapters: base.chapters,
        is_liked: audiobook.is_liked,
    }
    .render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/detail")]
pub async fn get_audiobook(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let base = get_audiobook_detail_base(
        audiobook_repo,
        chapter_repo,
        parse_user_id(identity)?,
        path.into_inner().0
    )
    .await?;

    let body = AudiobookDetailPageTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// TODO: refactor content methods, so it does not duplicate code
#[get("/{id}/detail-content")]
pub async fn get_audiobook_detail_content(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let base = get_audiobook_detail_base(
        audiobook_repo,
        chapter_repo,
        parse_user_id(identity)?,
        path.into_inner().0
    )
    .await?;

    let body = AudiobookDetailContentTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}


#[get("/releases")]
async fn releases_page(
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let template = NewReleasesPageTemplate { audiobooks: get_releases(u, book_repo).await? };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/releases-content")]
async fn releases_content(
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    //add functionality for ordering audiobooks
    let u = authorized!(identity);
    let template = NewReleasesContentTemplate { audiobooks: get_releases(u, book_repo).await? };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}


#[delete("/{id}/delete")]
pub async fn remove_audiobook(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user = get_user_from_identity(identity, &user_repo).await?;
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(user.id, path.into_inner().0))
        .await?;

    if user.id != audiobook.author_id {
        return Err(AppError::from(BackendError::new(
            BackendErrorKind::UnauthorizedOperation,
        )));
    }
    remove_file(&audiobook.file_path)?;

    if let Some(thumbnail) = &audiobook.thumbnail {
        remove_file(thumbnail)?;
    }
    audiobook_repo
        .delete(&AudiobookDelete::new(&audiobook.id))
        .await?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/studio-content"))
        .finish())
}

#[patch("{id}/likes")]
pub async fn change_like(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);

    let user = get_user_from_identity(identity, &user_repo).await?;
    let audiobook_id = path.into_inner().0;

    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(user.id, audiobook_id))
        .await?;

    let bookmark = BookmarkOperation::new(user.id, audiobook_id);
    let likes = match audiobook.is_liked {
        false => {
            user_repo.bookmark(&bookmark).await?;
            audiobook.like_count + 1
        },
        true => {
            user_repo.unbookmark(&bookmark).await?;
            audiobook.like_count - 1
        }
    };

    let update = AudiobookUpdate::update_likes(audiobook_id, likes);

    audiobook_repo.update(&update).await?;

    let template = DetailLikesTemplate {
        is_liked: !audiobook.is_liked,
        likes,
    };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render()?))
}

#[get("/search")]
pub async fn search(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    query: web::Query<AudiobookQuickSearchQuery>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let quicksearch = audiobook_repo.quick_search(&query.query).await?;
    let template = QuickSearchResults {
        results: quicksearch,
    };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render()?))
}

#[derive(Deserialize)]
struct Position {
    position: f64,
}

#[put("/{id}/active")]
pub async fn set_active_audiobook(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    query: web::Query<Position>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);

    audiobook_repo
        .set_active_audiobook(&SetActiveAudiobook::new(
            parse_user_id(identity)?,
            path.into_inner().0,
            query.position,
        ))
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/last-played")]
pub async fn get_last_active_audiobook(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let id = parse_user_id(identity)?;
    let latest = audiobook_repo.get_latest_active_audiobook(&id).await?;

    if latest.is_none() {
        // return empty container
        return Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body("<div id='player-container'></div>"));
    }

    let template = PlayerTemplate {
        played_book: latest.unwrap(),
    };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render()?))
}
#[derive(Deserialize)]
pub struct PositionQuery {
    position: Option<f64>
}

#[get("/{id}/player")]
pub async fn get_audiobook_player(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    position_query: web::Query<PositionQuery>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user_id = parse_user_id(identity)?;
    let mut played = audiobook_repo
        .get_or_create_active_audiobook(&user_id, &path.into_inner().0)
        .await?;

    if position_query.position.is_some() {
        played.playback_position = position_query.position.unwrap();
    }

    let template = PlayerTemplate {
        played_book: played,
    };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render()?))
}
