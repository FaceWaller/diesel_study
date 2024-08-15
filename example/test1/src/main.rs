use std::io;

use diesel::Connection;
use diesel_study::diesel::{
    prelude::table, query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl,
};
use diesel_study::*;
use user_info::dsl;

table! {
    user_info(user_id) {
        user_id -> BigInt,
        name -> Text,
        icon -> Text,
        age -> Integer
    }
}

#[derive(Insertable, Queryable, AsChangeset, Clone, Debug, Default)]
#[primary_key(user_id)]
#[diesel(table_name = user_info)]
pub struct UserInfo {
    user_id: i64,
    name: String,
    icon: String,
    age: i32,
}

#[derive(AsChangeset, Clone, Debug, Default)]
#[diesel(table_name = user_info)]
pub struct UserInfoChangest {
    user_id: Option<i64>,
    name: Option<String>,
    icon: Option<String>,
    age: Option<i32>,
}

fn main() {
    let conn = &mut *get_conn().unwrap();
    let user1 = UserInfo {
        user_id: 1,
        name: "完达山一号".to_string(),
        icon: "icon1.png".to_string(),
        age: 3,
    };
    let user2 = UserInfo {
        user_id: 2,
        name: "花花".to_string(),
        icon: "icon2.png".to_string(),
        age: 5,
    };
    // 增
    let _ = diesel::insert_into(user_info::table)
        .values(vec![user1, user2])
        .execute(conn);

    // 删
    let filter = dsl::user_info.filter(dsl::name.eq("花花"));
    let _ = diesel::delete(filter).execute(conn);
    // let _ = diesel::delete(dsl::user_info).execute(conn); // 删除所有

    // 改
    let update_user1 = UserInfo {
        name: "完达山二号".to_string(),
        ..Default::default()
    };
    let filter = dsl::user_info.filter(dsl::name.eq("完达山一号"));
    let _ = diesel::update(filter).set(update_user1).execute(conn);

    // 查
    let users: Vec<UserInfo> = dsl::user_info.load(conn).unwrap();
    println!("users : {:?}", users);


    // 事务
    let _ = conn.transaction::<(), diesel::result::Error, _>(move |conn_trans| {
        let user3 = UserInfo {
            user_id: 3,
            name: "事务x".to_string(),
            icon: "icon1.png".to_string(),
            age: 3,
        };
        let user4 = UserInfo {
            user_id: 4,
            name: "事务y".to_string(),
            icon: "icon1.png".to_string(),
            age: 3,
        };
        let _ = diesel::insert_into(user_info::table)
            .values(user3)
            .execute(conn_trans);
        let _ = diesel::insert_into(user_info::table)
            .values(user4)
            .execute(conn_trans);
        Ok(())
    });
}
