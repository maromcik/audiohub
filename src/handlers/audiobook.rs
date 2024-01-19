use crate::database::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::audiobook::{
    AudiobookCreate, AudiobookDelete, AudiobookGetByIdJoin, AudiobookSearch, AudiobookUpdate,
};
use crate::database::models::genre::{GenreGetById, GenreSearch};

use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::forms::audiobook::{
    AudiobookCreateForm, AudiobookSearchQuery, AudiobookSetActiveForm, AudiobookUploadForm,
};
use crate::handlers::utilities::{
    get_metadata_from_session, get_user_from_identity, parse_user_id, remove_file, save_file,
    validate_file, AudiobookCreateSessionKeys,
};
use crate::templates::audiobook::{AudiobookCreateContentTemplate, AudiobookCreatePageTemplate, AudiobookDetailBase, AudiobookDetailContentTemplate, AudiobookDetailPageTemplate, AudiobookUploadFormTemplate, NewReleasesContentTemplate, NewReleasesPageTemplate, PlayerTemplate};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{get, patch, post, web, HttpResponse, put};
use askama::Template;
use lofty::AudioFile;

use crate::authorized;
use crate::database::common::error::{BackendError, BackendErrorKind};
use crate::database::common::query_parameters::DbQueryParams;
use crate::database::models::active_audiobook::{PlayedAudiobook, SetActiveAudiobook};
use crate::database::models::bookmark::BookmarkOperation;
use crate::database::models::chapter::{ChapterDisplay, ChaptersGetByBookId};
use uuid::Uuid;

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

#[get("/upload")]
pub async fn upload_audiobook_form(identity: Option<Identity>) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let template = AudiobookUploadFormTemplate {};
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
    let thumbnail_path = validate_file(&form.thumbnail, uuid, "image", "audiobook")?;
    let audiobook_path = validate_file(&form.audio_file, uuid, "audio", "audiobook")?;

    let metadata = get_metadata_from_session(&session, &session_keys)?;

    let audio_file = form.audio_file.file.as_file_mut();
    let lofty_audio_file = lofty::read_from(audio_file)?;
    let properties = lofty_audio_file.properties();
    let length = properties.duration().as_secs_f64();

    let book_crate = AudiobookCreate::new(
        &metadata.name,
        &user.id,
        &metadata.genre_id,
        &audiobook_path,
        &length,
        &thumbnail_path,
        &metadata.description,
    );
    let book = audiobook_repo.create(&book_crate).await?;

    save_file(form.thumbnail, thumbnail_path)?;
    save_file(form.audio_file, audiobook_path)?;

    session.remove(session_keys.name.as_str());
    session.remove(session_keys.description.as_str());
    session.remove(session_keys.genre_id.as_str());

    let handler = format!("/audiobook/{}/detail", book.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, handler))
        .finish())
}

#[get("/{id}/detail")]
pub async fn get_audiobook(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let base = get_audiobook_detail_base(audiobook_repo, chapter_repo, path.into_inner().0).await?;
    let body = AudiobookDetailPageTemplate {
        audiobook: base.audiobook,
        chapters: base.chapters,
    }
        .render()?;
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
    authorized!(identity);
    let base = get_audiobook_detail_base(audiobook_repo, chapter_repo, path.into_inner().0).await?;
    let body = AudiobookDetailContentTemplate {
        audiobook: base.audiobook,
        chapters: base.chapters,
    }
        .render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}


async fn get_audiobook_detail_base (
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    book_id: Id
) -> Result<AudiobookDetailBase, AppError>  {
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(&book_id))
        .await?;

    let chapters = chapter_repo
        .read_many(&ChaptersGetByBookId::new(book_id))
        .await?;

    let displayed_chapters: Vec<ChapterDisplay> = chapters
        .into_iter()
        .enumerate()
        .map(|(order, ch)| ChapterDisplay {
            name: ch.name,
            order: order + 1,
            position: ch.position,
        })
        .collect();

    Ok(AudiobookDetailBase {
        audiobook,
        chapters: displayed_chapters
    })
}

#[get("/releases")]
async fn releases_page(
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    //add functionality for ordering audiobooks
    let u = authorized!(identity);
    let books = book_repo
        .read_many(&AudiobookSearch::with_params(DbQueryParams::limit(5, 0), parse_user_id(u)?))
        .await?;

    let template = NewReleasesPageTemplate { audiobooks: books };
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
    let books = book_repo
        .read_many(&AudiobookSearch::with_params(DbQueryParams::limit(5, 0), parse_user_id(u)?))
        .await?;

    let template = NewReleasesContentTemplate { audiobooks: books };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/delete")]
pub async fn remove_audiobook(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user = get_user_from_identity(identity, &user_repo).await?;
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(&path.into_inner().0))
        .await?;

    if user.id != audiobook.author_id {
        return Err(AppError::from(BackendError::new(
            BackendErrorKind::UnauthorizedOperation,
        )));
    }
    remove_file(&audiobook.file_path)?;
    remove_file(&audiobook.thumbnail)?;
    audiobook_repo
        .delete(&AudiobookDelete::new(&audiobook.id))
        .await?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
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
    let book_id = path.into_inner().0;

    let liked = user_repo.is_bookmarked(&user.id, &book_id).await?.is_some();

    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(&book_id))
        .await?;

    let bookmark = BookmarkOperation::new(user.id, book_id);
    let likes = match liked {
        true => {
            user_repo.unbookmark(&bookmark).await?;
            audiobook.like_count - 1
        }
        false => {
            user_repo.bookmark(&bookmark).await?;
            audiobook.like_count + 1
        }
    };

    let update = AudiobookUpdate::update_likes(book_id, likes);

    audiobook_repo.update(&update).await?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(likes.to_string()))
}

#[get("/search")]
pub async fn search(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    q: web::Query<AudiobookSearchQuery>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let _books = audiobook_repo.quick_search(&q.name).await?;
    Ok(HttpResponse::Ok().finish())
}

#[post("/active")]
pub async fn set_active_audiobook(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    form: web::Form<AudiobookSetActiveForm>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    audiobook_repo
        .set_active_audiobook(&SetActiveAudiobook::new(
            parse_user_id(identity)?,
            form.audiobook_id,
            form.position,
        ))
        .await?;
    todo!()
}

#[get("/last-played")]
pub async fn get_last_active_audiobook(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let id = parse_user_id(identity)?;
    let latest = audiobook_repo.get_latest_active_audiobook(&id)
        .await?;

    if latest.is_none() {
        // TODO: solve no listened book
        let template = PlayerTemplate {last_played: PlayedAudiobook{playback_position: 0 as f64,
            path: String::from(""), name: String::from("")}};

        return Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(template.render()?));
    }

    let template = PlayerTemplate {last_played: latest.unwrap()};
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render()?))
}

#[put("/{id}/play")]
pub async fn set_played_audiobook(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let id = parse_user_id(identity)?;
    let latest = audiobook_repo.get_latest_active_audiobook(&id)
        .await?;

    if latest.is_none() {

        return Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body("xd"));
    }

    let template = PlayerTemplate {last_played: latest.unwrap()};
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render()?))
}