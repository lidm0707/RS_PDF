use self::backend::repo::schema::planing_credit::dsl::*;
use crate::backend::entity::entity_plan_credit::*;
use crate::*;
use backend::repo::db_connect::connect_database;
use diesel::prelude::*;

pub fn insert_plan_credit(
    period_value: String,
    label_id_value: i32,
    amount_value: f64,
) -> SelectPlanCredit {
    let mut conn = connect_database();

    let new_post = InsertPlanCredit {
        period: period_value,
        label_id: label_id_value,
        amount: amount_value,
    };

    diesel::insert_into(planing_credit)
        .values(new_post)
        .returning(SelectPlanCredit::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}
