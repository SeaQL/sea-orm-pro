use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Admin panel config
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCfg {
    /// Site config
    pub site: SiteCfg,
    /// Dashboard config
    #[serde(default)]
    pub dashboard: DashboardCfg,
    /// Raw table config
    #[serde(default)]
    pub raw_tables: IndexMap<String, RawTableCfg>,
    /// Composite table config
    #[serde(default)]
    pub composite_tables: IndexMap<String, CompositeTableCfg>,
}

/// Site config
#[derive(Debug, Serialize, Deserialize)]
pub struct SiteCfg {
    /// Theme config
    pub theme: ThemeCfg,
    /// Navbar config
    #[serde(default)]
    pub navbar: NavbarCfg,
    /// Menu config
    #[serde(default)]
    pub menu: MenuCfg,
    /// RBAC config
    #[serde(default)]
    pub rbac: RbacCfg,
}

/// Theme config
#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeCfg {
    /// Title of admin panel
    pub title: String,
    /// File path of admin panel logo
    pub logo: String,
    /// File path of admin panel login banner
    pub login_banner: String,
}

/// Navbar config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NavbarCfg {
    /// Whether to show the GraphQL playground button
    pub graphql_playground: bool,
}

/// Menu config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MenuCfg {
    /// Dashboard
    pub dashboard: MenuItemCfg,
    /// Raw Table
    pub raw_table: MenuItemCfg,
    /// Composite Table
    pub composite_table: MenuItemCfg,
}

/// RBAC config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RbacCfg {
    /// Is RBAC enabled?
    pub enable: bool,
}

/// Menu item config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MenuItemCfg {
    /// Title of menu item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Hide in menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

/// Dashboard config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DashboardCfg {
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub info: Option<DashboardInfo>,
    #[serde(default)]
    pub row: Vec<DashboardRow>,
}

/// Dashboard info
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardInfo {
    pub card: Vec<DashboardInfoCard>,
}

/// Dashboard info card
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardInfoCard {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub link: String,
}

/// Dashboard panel row
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardRow {
    pub gutter: u32,
    pub col: Vec<DashboardRowCol>,
}

/// Dashboard panel column
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardRowCol {
    pub span: u32,
    pub title: String,
    pub chart: Option<DashboardChart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardChart {
    chart: String,
    kind: String,
    timescale: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    default_date_range: Option<String>,
    x_axis_title: Option<String>,
    y_axis_title: Option<String>,
}

/// Composite table config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompositeTableCfg {
    /// Parent table config
    pub parent: ParentTableCfg,
    /// Child tables config
    pub children: Vec<ChildTableCfg>,
}

/// Parent table config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParentTableCfg {
    /// Name of the parent table
    pub name: String,
    /// Parent table config
    #[serde(flatten)]
    pub parent_config: RawTableCfg,
}

/// Child tables config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChildTableCfg {
    /// Name of the SeaORM relation
    pub relation: String,
    /// Child tables config
    #[serde(flatten)]
    pub child_config: RawTableCfg,
}

/// Raw table config
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RawTableCfg {
    /// Table config
    pub table: TableCfg,
    /// filter config
    pub filter: FilterCfg,
    /// View config
    pub view: ViewCfg,
    /// Editor config
    pub editor: EditorCfg,
    /// Create config
    pub create: CreateCfg,
    /// Update config
    pub update: UpdateCfg,
    /// Delete config
    pub delete: DeleteCfg,
}

/// Table config
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TableCfg {
    /// Show all columns including column not mention in the `columns` config
    pub all_columns: bool,
    /// Column specific config
    pub columns: Vec<ColumnCfg>,
    /// List of columns that are hidden on the view table
    pub hidden_columns: Vec<String>,
    /// Sorter of the view table
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<ViewOrderByCfg>,
    /// Number of rows per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<usize>,
    /// Display density, options: large, middle, small
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size: Option<TableSize>,
    /// Rename table title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl Default for TableCfg {
    fn default() -> TableCfg {
        TableCfg {
            all_columns: true,
            columns: Vec::new(),
            hidden_columns: Vec::new(),
            order_by: None,
            page_size: None,
            table_size: None,
            title: None,
        }
    }
}

/// Filter config
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FilterCfg {
    /// List of columns that are hidden on the filter panel
    pub hidden_columns: Vec<String>,
}

/// Display density
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TableSize {
    Large,
    Middle,
    Small,
}

/// Column specific config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ColumnCfg {
    /// Display title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Name of the SQL column
    pub field: String,
    /// Name of the SeaORM relation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    /// Column width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<usize>,
    /// Data type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    // TODO
    // pub image_url_prefix: Option<String>,
    /// Clip long text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsis: Option<bool>,
}

/// Editor config
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EditorCfg {
    /// Use editor for this table?
    pub enable: bool,
    /// Display field value as title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_field: Option<String>,
    /// Field specific config
    pub fields: Vec<FieldCfg>,
}

/// Field specific config
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FieldCfg {
    /// Display title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Name of the SQL column
    pub field: String,
    /// Column span
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<usize>,
    /// Data type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    /// Row length of textarea
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<usize>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ViewCfg {}

/// Sorter of the view table
#[derive(Debug, Serialize, Deserialize)]
pub struct ViewOrderByCfg {
    /// Sort by which column
    pub field: String,
    /// Sort in ASC / DESC direction
    pub order: Order,
}

/// Sort in ASC / DESC direction
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Asc,
    Desc,
}

/// Create config
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateCfg {
    /// Is create allowed for this table?
    pub enable: bool,
    /// List of columns that are hidden on the create form
    pub hidden_columns: Vec<String>,
}

/// Update config
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateCfg {
    /// Is update allowed for this table?
    pub enable: bool,
    /// List of columns that are hidden on the update form
    pub hidden_columns: Vec<String>,
    /// List of columns that are readonly on the update form
    pub readonly_columns: Vec<String>,
}

/// Delete config
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DeleteCfg {
    /// Is delete allowed for this table?
    pub enable: bool,
}
