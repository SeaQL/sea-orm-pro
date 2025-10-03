use async_graphql::{
    dynamic::*,
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextParseQuery},
    parser::types::{ExecutableDocument, OperationType},
    ServerError, ServerResult, Variables,
};
use sea_orm::DatabaseConnection;
use seaography::{
    async_graphql, lazy_static, Builder, BuilderContext, LifecycleHooks, MultiLifecycleHooks,
};
use std::{env, sync::Arc};

lazy_static::lazy_static! {
    static ref CONTEXT: BuilderContext = {
        BuilderContext {
            hooks: LifecycleHooks::new( //
                MultiLifecycleHooks::default()
            ),
            ..Default::default()
        }
    };
    // Start: Scaffold //
    static ref DEMO_SITE: bool = env::var_os("DEMO_SITE").unwrap_or_default() == "true";
    // End: Scaffold //
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    // Construct GraphQL schema
    let builder = Builder::new(&CONTEXT, database.clone());
    let builder = crate::models::register_entity_modules(builder);
    builder
        // Maximum depth of the constructed query
        .set_depth_limit(depth)
        // Maximum complexity of the constructed query
        .set_complexity_limit(complexity)
        .schema_builder()
        // GraphQL schema with database connection
        .data(database)
        // Start: Scaffold //
        .extension(Readonly)
        // End: Scaffold //
        .finish()
}

// Start: Scaffold //
pub struct Readonly;

impl ExtensionFactory for Readonly {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(ReadonlyExtension)
    }
}

struct ReadonlyExtension;

#[async_trait::async_trait]
impl Extension for ReadonlyExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let document = next.run(ctx, query, variables).await?;
        if *DEMO_SITE {
            let is_mutation = document
                .operations
                .iter()
                .any(|(_, operation)| matches!(operation.node.ty, OperationType::Mutation));
            if is_mutation {
                return Err(ServerError::new("Demo Site is Readonly, to test the full CRUD please run the demo on your local machine", None));
            }
        }
        Ok(document)
    }
}
// End: Scaffold //
