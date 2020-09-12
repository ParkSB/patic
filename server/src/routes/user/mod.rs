use actix_session::Session;
use actix_web::{delete, patch, post, web, Responder};

use crate::models::error::*;
use crate::services::user::UserService;
use crate::utils::{http_util, session_util};

mod models;
use models::*;

/// User route
pub struct UserRoute {}

impl UserRoute {
    /// Creates a new user
    ///
    /// # Request
    ///
    /// ```text
    /// POST /users
    /// ```
    ///
    /// ## Parameters
    ///
    /// * user_public_key - A user's public key
    /// * token_key - A key for token search
    /// * token_pin - A pin for verifying
    ///
    /// ```json
    /// {
    ///     "user_public_key": "d63ee429"
    ///     "token_key": "71I3Qz9u",
    ///     "token_pin": "P9d82Jc5"
    /// }
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": true,
    ///     "error": null
    /// }
    /// ```
    pub fn create_user(args: web::Json<CreateArgs>) -> impl Responder {
        let CreateArgs {
            user_public_key,
            token_key,
            token_pin,
        } = args.into_inner();
        let response = UserService::new().create(&user_public_key, &token_key, &token_pin);
        http_util::get_response::<bool>(response)
    }

    /// Deletes a user
    ///
    /// # Request
    ///
    /// ```text
    /// DELETE /users/:id
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": true,
    ///     "error": null
    /// }
    /// ```
    pub fn delete_user(session: Session, id: web::Path<u64>) -> impl Responder {
        let response = if let Some(user_session) = session_util::get_session(&session) {
            let id_in_path = id.into_inner();
            if id_in_path != user_session.user_id {
                Err(ServiceError::Unauthorized)
            } else {
                UserService::new().delete(id_in_path)
            }
        } else {
            Err(ServiceError::Unauthorized)
        };

        http_util::get_response::<bool>(response)
    }

    /// Updates a user
    ///
    /// # Request
    ///
    /// ```text
    /// PATCH /users/:id
    /// ```
    ///
    /// ## Parameters
    ///
    /// * name - A name of the user.
    /// * password - A password of the user.
    /// * avatar_url - An avatar image url of the user.
    ///
    /// ```json
    /// {
    ///     "name": "park",
    ///     "password": "Ir5c7y8dS3",
    ///     "avatar_url": "avatar.jpg"
    /// }
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": true,
    ///     "error": null
    /// }
    /// ```
    pub fn update_user(
        session: Session,
        id: web::Path<u64>,
        args: web::Json<UpdateArgs>,
    ) -> impl Responder {
        let response = if let Some(user_session) = session_util::get_session(&session) {
            let id_in_path = id.into_inner();
            if id_in_path != user_session.user_id {
                Err(ServiceError::Unauthorized)
            } else {
                let UpdateArgs {
                    name,
                    password,
                    avatar_url,
                } = args.into_inner();
                UserService::new().update(id_in_path, &name, &password, &avatar_url)
            }
        } else {
            Err(ServiceError::Unauthorized)
        };

        http_util::get_response::<bool>(response)
    }

    /// Resets the password.
    ///
    /// # Request
    ///
    /// ```text
    /// POST /users/password
    /// ```
    ///
    /// ## Parameters
    ///
    /// * email - An email of the user.
    /// * token_id - A password token ID.
    /// * temporary_password - A temporary password.
    /// * new_password - A new password.
    ///
    /// ```json
    /// {
    ///     "email": "park@email.com",
    ///     "token_id": "d63ee429",
    ///     "temporary_password": "P9d82Jc5",
    ///     "new_password": "71I3Qz9u"
    /// }
    /// ```
    ///
    /// # Response
    ///
    /// ```json
    /// {
    ///     "data": true,
    ///     "error": null
    /// }
    /// ```
    pub fn reset_password(args: web::Json<ResetPasswordArgs>) -> impl Responder {
        let ResetPasswordArgs {
            email,
            token_id,
            temporary_password,
            new_password,
        } = args.into_inner();
        let response = UserService::new().reset_password(
            &email,
            &token_id,
            &temporary_password,
            &new_password,
        );
        http_util::get_response::<bool>(response)
    }
}

#[post("/users")]
pub async fn create_user_route(args: web::Json<CreateArgs>) -> impl Responder {
    UserRoute::create_user(args)
}

#[delete("/users/{id}")]
pub async fn delete_user_route(session: Session, id: web::Path<u64>) -> impl Responder {
    UserRoute::delete_user(session, id)
}

#[patch("/users/{id}")]
pub async fn update_user_route(
    session: Session,
    id: web::Path<u64>,
    user: web::Json<UpdateArgs>,
) -> impl Responder {
    UserRoute::update_user(session, id, user)
}

#[post("/users/password")]
pub async fn reset_password_route(args: web::Json<ResetPasswordArgs>) -> impl Responder {
    UserRoute::reset_password(args)
}

/// Initializes the user routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user_route);
    cfg.service(delete_user_route);
    cfg.service(update_user_route);
    cfg.service(reset_password_route);
}
