use async_graphql::{
    dynamic::Schema,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_axum::GraphQLRequest;
use loco_rs::prelude::*;
use seaography::async_graphql;

async fn graphql_playground() -> Result<Response> {
    // Setup GraphQL playground web and specify the endpoint for GraphQL resolver
    let config =
        GraphQLPlaygroundConfig::new("/api/graphql").with_header("Authorization", "AUTO_TOKEN");

    let res = playground_source(config).replace(
        r#""Authorization":"AUTO_TOKEN""#,
        r#""Authorization":`Bearer ${localStorage.getItem('auth_token')}`"#,
    );

    Ok(Response::new(res.into()))
}

async fn graphql_handler(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    gql_req: GraphQLRequest,
) -> Result<async_graphql_axum::GraphQLResponse, (axum::http::StatusCode, &'static str)> {
    let user = crate::models::user::Entity::find()
        .filter(crate::models::user::Column::Email.eq(auth.claims.pid))
        .one(&ctx.db)
        .await
        .map_err(|_| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            )
        })?
        .ok_or((axum::http::StatusCode::UNAUTHORIZED, "User not found"))?;

    let mut gql_req = gql_req.into_inner();
    gql_req = gql_req.data(seaography::UserContext { user_id: user.id });

    let schema: Schema = ctx.shared_store.get().ok_or((
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "GraphQL not setup",
    ))?;
    let res = schema.execute(gql_req).await.into();

    Ok(res)
}

pub fn routes() -> Routes {
    Routes::new()
        // GraphQL route prefix
        .prefix("graphql")
        // Serving the GraphQL playground web
        .add("/", get(graphql_playground))
        // Handling GraphQL request
        .add("/", post(graphql_handler))
}
