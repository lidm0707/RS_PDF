use self::repo::schema::*;
use crate::*;
use anyhow;
use diesel::dsl::sum;
use diesel::prelude::*;

use entity::entity_credit::GroupBySumCredit;
use repo::db_connect::connect_database;
use service::date::date_format::format_period;
use service::date::now::thai_now_string;

pub fn union_installment_credit(
    start: &str,
    end: &str,
) -> Result<Vec<GroupBySumCredit>, anyhow::Error> {
    let mut conn: SqliteConnection = connect_database();
    let preriod_now = format_period(&thai_now_string());

    let credit_table = credits::table
        .filter(credits::period.ge(start).and(credits::period.le(end)))
        .group_by((credits::label_id, credits::period))
        .select((credits::period, credits::label_id, sum(credits::amount)));

    let installment_table = installment_items::table
        .inner_join(installment::table)
        .filter(
            (installment_items::bank_id.eq(1).and(
                installment_items::period
                    .ge(start)
                    .and(installment_items::period.le(end)),
            ))
            .or(installment_items::bank_id
                .eq(2)
                .and(installment_items::period.le(preriod_now))),
        )
        .group_by((installment_items::period, installment::label_id))
        .select((
            installment_items::period,
            installment::label_id,
            sum(installment_items::amount), // ใช้ฟังก์ชัน sum
        ));

    let results = installment_table
        .union(credit_table)
        .load::<GroupBySumCredit>(&mut conn)?;

    Ok(results)
}

pub fn union_credit_installment_label(
    start: &str,
    end: &str,
) -> Result<Vec<(i32, Option<f64>)>, anyhow::Error> {
    let mut conn: SqliteConnection = connect_database();
    let preriod_now = format_period(&thai_now_string());

    let credit_table = credits::table
        .filter(credits::period.ge(start).and(credits::period.le(end)))
        .group_by((credits::label_id))
        .select((credits::label_id, sum(credits::amount)));

    let installment_table = installment_items::table
        .inner_join(installment::table)
        .filter(
            (installment_items::bank_id.eq(1).and(
                installment_items::period
                    .ge(start)
                    .and(installment_items::period.le(end)),
            ))
            .or(installment_items::bank_id
                .eq(2)
                .and(installment_items::period.le(preriod_now))),
        )
        .group_by((installment::label_id))
        .select((
            installment::label_id,
            sum(installment_items::amount), // ใช้ฟังก์ชัน sum
        ));

    let results = installment_table
        .union(credit_table)
        .load::<(i32, Option<f64>)>(&mut conn)?;

    Ok(results)
}
