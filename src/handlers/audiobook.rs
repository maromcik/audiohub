use crate::database::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbUpdate};

use crate::database::models::audiobook::{
    AudiobookCreate, AudiobookDelete, AudiobookDisplay, AudiobookGetById, AudiobookGetByIdJoin,
    AudiobookRecommenderDisplay, AudiobookUpdate,
};
use crate::database::models::genre::{GenreGetById, GenreSearch};

use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::database::repositories::user::repository::UserRepository;

use crate::error::{AppError, AppErrorKind};
use crate::forms::audiobook::{
    AudiobookCreateForm, AudiobookEditForm, AudiobookQuickSearchQuery, AudiobookThumbnailEditForm,
    AudiobookUploadForm,
};
use crate::handlers::utilities::{
    authorized_to_modify, authorized_to_modify_join, get_metadata_from_session,
    get_user_from_identity, parse_user_id, remove_file, save_file, validate_file,
    AudiobookCreateSessionKeys,
};
use crate::templates::audiobook::{
    AudiobookCoverUpload, AudiobookCreateContentTemplate, AudiobookCreatePageTemplate,
    AudiobookDetailContentTemplate, AudiobookDetailPageTemplate, AudiobookEditContentTemplate,
    AudiobookEditPageTemplate, AudiobookRecommendationTemplate, AudiobookUploadFormTemplate,
    NewReleasesContentTemplate, NewReleasesPageTemplate, PlayerTemplate, QuickSearchResults,
};
use crate::templates::audiobook::{
    AudiobookDetailAuthorContentTemplate, AudiobookDetailAuthorPageTemplate, DetailLikesTemplate,
};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{delete, get, patch, post, put, web, HttpRequest, HttpResponse, Responder};

use askama::Template;
use lofty::AudioFile;
use log::{info, warn};
use serde::Deserialize;
use sqlx::query;

use crate::{authorized, RECOMMEND_BOOKS_CNT};
use crate::database::models::active_audiobook::SetActiveAudiobook;
use crate::database::models::bookmark::BookmarkOperation;

use crate::handlers::helpers::{
    get_audiobook_detail_base, get_audiobook_edit, get_chapters_by_book, get_releases,
};
use uuid::Uuid;


use crate::recommender::recommandation_system::{delete_book_from_recommandation, recommend_books};
use crate::recommender::recommender::{add_book_recommender};
#[get("/create")]
pub async fn create_audiobook_page(
    request: HttpRequest,
    identity: Option<Identity>,
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;
    let template = AudiobookCreatePageTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/create-content")]
pub async fn create_audiobook_content(
    request: HttpRequest,
    identity: Option<Identity>,
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;
    let template = AudiobookCreateContentTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/upload")]
pub async fn upload_audiobook_form(
    request: HttpRequest,
    identity: Option<Identity>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let template = AudiobookUploadFormTemplate {
        message: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/cover/{id}/upload")]
pub async fn upload_book_cover(
    request: HttpRequest,
    audiobook_repo: web::Data<AudiobookRepository>,
    identity: Option<Identity>,
    path: web::Path<(Id,)>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user_id = parse_user_id(u)?;
    let audiobook =
        authorized_to_modify_join(&audiobook_repo, user_id, path.into_inner().0).await?;

    let template = AudiobookCoverUpload {
        message: "".to_string(),
        audiobook: AudiobookDisplay::from(audiobook),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/cover/upload")]
pub async fn upload_book_cover_post(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    MultipartForm(form): MultipartForm<AudiobookThumbnailEditForm>,
) -> Result<HttpResponse, AppError> {
    let uuid = Uuid::new_v4();
    let u = authorized!(identity, request.path());
    let audiobook_id = form.audiobook_id.into_inner();
    authorized_to_modify(&audiobook_repo, parse_user_id(u)?, audiobook_id).await?;

    let thumbnail_path = validate_file(&form.thumbnail, uuid, "image", "audiobook")?;
    let book_update = AudiobookUpdate::new(
        &audiobook_id,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(thumbnail_path.clone()),
        None,
    );
    audiobook_repo.update(&book_update).await?;
    save_file(form.thumbnail, &thumbnail_path)?;

    let handler = format!("/audiobook/{}/manage-content", audiobook_id);
    return Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, handler))
        .finish());
}

#[post("/create")]
pub async fn create_audiobook(
    request: HttpRequest,
    identity: Option<Identity>,
    session: Session,
    genre_repo: web::Data<GenreRepository>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<AudiobookCreateForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
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
    request: HttpRequest,
    identity: Option<Identity>,
    session: Session,
    user_repo: web::Data<UserRepository>,
    genre_repo: web::Data<GenreRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    MultipartForm(mut form): MultipartForm<AudiobookUploadForm>,
) -> Result<HttpResponse, AppError> {
    let uuid = Uuid::new_v4();
    let u = authorized!(identity, request.path());
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = AudiobookCreateSessionKeys::new(user.id);

    let audiobook_path = validate_file(&form.audio_file, uuid, "audio", "audiobook")?;
    let thumbnail_path = match &form.thumbnail {
        None => None,
        Some(thumb) => Some(validate_file(thumb, uuid, "image", "audiobook")?),
    };
    let metadata = get_metadata_from_session(&session, &session_keys)?;

    let audio_file = form.audio_file.file.as_file_mut();
    let lofty_audio_file = match lofty::read_from(audio_file) {
        Ok(f) => f,
        Err(e) => {
            let template = AudiobookUploadFormTemplate {
                message: e.to_string(),
            }
            .render()?;
            return Ok(HttpResponse::Ok().content_type("text/html").body(template));
        }
    };
    let properties = lofty_audio_file.properties();
    let length = properties.duration().as_secs_f64();
    if let (Some(thumb_path), Some(thumbnail)) = (&thumbnail_path, form.thumbnail) {
        save_file(thumbnail, thumb_path)?;
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

    let genre = genre_repo
        .read_one(&GenreGetById::new(&book.genre_id))
        .await?;

    if let Err(err) = add_book_recommender(&book, &*genre.name).await {
        warn!("failed add book too grpc recommender system, check if server is running: {err}");
    } else {
        info!("book added to the grpc repository!");
    };

    save_file(form.audio_file, &audiobook_path)?;

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
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let user_id = parse_user_id(u)?;
    let audiobook =
        authorized_to_modify_join(&audiobook_repo, user_id, path.into_inner().0).await?;
    let body = AudiobookDetailAuthorPageTemplate {
        chapters: get_chapters_by_book(chapter_repo, audiobook.id).await?,
        is_liked: audiobook.is_liked,
        audiobook: AudiobookDisplay::from(audiobook),
    }
    .render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/manage-content")]
pub async fn manage_audiobook_content(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let user_id = parse_user_id(u)?;
    let audiobook =
        authorized_to_modify_join(&audiobook_repo, user_id, path.into_inner().0).await?;
    let body = AudiobookDetailAuthorContentTemplate {
        chapters: get_chapters_by_book(chapter_repo, audiobook.id).await?,
        is_liked: audiobook.is_liked,
        audiobook: AudiobookDisplay::from(audiobook),
    }
    .render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/edit")]
pub async fn edit_audiobook_page(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    genre_repo: web::Data<GenreRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let base = get_audiobook_edit(u, audiobook_repo, genre_repo, path.into_inner().0).await?;
    let template = AudiobookEditPageTemplate::from(base);
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/edit-content")]
pub async fn edit_audiobook_content(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    genre_repo: web::Data<GenreRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let base = get_audiobook_edit(u, audiobook_repo, genre_repo, path.into_inner().0).await?;
    let template = AudiobookEditContentTemplate::from(base);
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/edit")]
pub async fn edit_audiobook(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    form: web::Form<AudiobookEditForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    authorized_to_modify(&audiobook_repo, parse_user_id(u)?, form.audiobook_id).await?;
    let book_update = AudiobookUpdate::new(
        &form.audiobook_id,
        Some(&form.name),
        None,
        Some(&form.genre_id),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&form.description),
    );
    audiobook_repo.update(&book_update).await?;

    let path = format!("/audiobook/{}/manage-content", form.audiobook_id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish())
}

#[get("/{id}/detail")]
pub async fn get_audiobook(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity, request.path());
    let base = get_audiobook_detail_base(
        audiobook_repo,
        chapter_repo,
        parse_user_id(identity)?,
        path.into_inner().0,
    )
    .await?;

    let body = AudiobookDetailPageTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/similar")]
pub async fn recommend_audiobooks(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    genre_repo: web::Data<GenreRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity, request.path());
    let book = audiobook_repo.read_one(&AudiobookGetById { id: path.into_inner().0, fetch_deleted: false, })
        .await?;

    let genre = genre_repo
        .read_one(&GenreGetById::new(&book.genre_id))
        .await?;

    let recommendations = match recommend_books(&book.description, book.id, &genre.name, RECOMMEND_BOOKS_CNT).await{
        Ok(books) => {books}
        Err(e) => {
            warn!("Book recommendation failed! {e}");
            vec![]
        }
    };
    let audiobooks = audiobook_repo.get_books_by_ids(recommendations).await?;

    let template = AudiobookRecommendationTemplate {
        books: audiobooks.iter().map(|b| AudiobookRecommenderDisplay::from(b.clone())).collect(),
    };

    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/detail-content")]
pub async fn get_audiobook_detail_content(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity, request.path());
    let base = get_audiobook_detail_base(
        audiobook_repo,
        chapter_repo,
        parse_user_id(identity)?,
        path.into_inner().0,
    )
    .await?;

    let body = AudiobookDetailContentTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/releases")]
async fn releases_page(
    request: HttpRequest,
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let template = NewReleasesPageTemplate {
        audiobooks: get_releases(u, book_repo).await?,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/releases-content")]
async fn releases_content(
    request: HttpRequest,
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let template = NewReleasesContentTemplate {
        audiobooks: get_releases(u, book_repo).await?,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[delete("/{id}/delete")]
pub async fn remove_audiobook(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let audiobook =
        authorized_to_modify(&audiobook_repo, parse_user_id(u)?, path.into_inner().0).await?;
    audiobook_repo
        .delete(&AudiobookDelete::new(&audiobook.id))
        .await?;

    if let Err(e) = delete_book_from_recommandation(audiobook.id).await {
        warn!("Recommender was not able to delete book: {e}")
    } else {
        info!("Book was deleted from recommender system")
    }

    let path = format!("/audiobook/{}/manage-content", audiobook.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish())
}

#[delete("/{id}/hard-delete")]
pub async fn hard_remove_audiobook(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let audiobook =
        authorized_to_modify(&audiobook_repo, parse_user_id(u)?, path.into_inner().0).await?;
    remove_file(&audiobook.file_path)?;
    if let Some(thumbnail) = &audiobook.thumbnail {
        remove_file(thumbnail)?;
    }
    audiobook_repo
        .hard_delete(&AudiobookDelete::new(&audiobook.id))
        .await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/studio-content"))
        .finish())
}

#[put("/{id}/restore")]
pub async fn restore_audiobook(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let audiobook =
        authorized_to_modify(&audiobook_repo, parse_user_id(u)?, path.into_inner().0).await?;
    audiobook_repo
        .restore(&AudiobookGetById::new(&audiobook.id, true))
        .await?;
    let path = format!("/audiobook/{}/manage-content", audiobook.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish())
}

#[patch("{id}/likes")]
pub async fn change_like(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity, request.path());

    let user = get_user_from_identity(identity, &user_repo).await?;
    let audiobook_id = path.into_inner().0;

    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(user.id, audiobook_id, false))
        .await?;

    let bookmark = BookmarkOperation::new(user.id, audiobook_id);
    let likes = match audiobook.is_liked {
        false => {
            user_repo.bookmark(&bookmark).await?;
            audiobook.like_count + 1
        }
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
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    user_repo: web::Data<UserRepository>,
    query: web::Query<AudiobookQuickSearchQuery>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let query_string = query.query.as_str();

    let template = match query.search_type.as_str() {
        "book" => {
            let results = audiobook_repo.quick_search(query_string).await?;
            Ok(QuickSearchResults {
                results,
                root_path: String::from("audiobook"),
                end_path: String::from("/detail-content"),
                end_push_url: String::from("/detail")
            })
        },
        "author" => {
            let results = user_repo.quick_search(query_string).await?;
            Ok(QuickSearchResults {
                results,
                root_path: String::from("user"),
                end_path: String::from("/author-content"),
                end_push_url: String::from("")
            })
        },
        _ => Err(AppError{app_error_kind: AppErrorKind::BadRequest, message: String::from("No other quicksearch types supported")})
    }?;

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
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    query: web::Query<Position>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity, request.path());

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
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity, request.path());
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
    position: Option<f64>,
}

/// Returns audio player for book with id defined in path.
#[get("/{id}/player")]
pub async fn get_audiobook_player(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    position_query: web::Query<PositionQuery>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity, request.path());
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
