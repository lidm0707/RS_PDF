#[derive(Debug,Clone)]
pub struct ModelLabelsName {
    pub id: i32,
    pub label: String,
    pub ord:i32,
    pub show_able:bool,
}

#[derive(Debug,Clone)]
pub struct ModelLabels {
    pub id: i32,
    pub id_label: i32,
    pub abb_ctx: String,
}