
pub mod users {
    use sqlx::MySqlPool;

    use crate::models::auth::UserModel;

    pub async fn insert(user: UserModel, pool: &MySqlPool) -> anyhow::Result<()> {
        let result = sqlx::query("insert into users (firstname, lastname,password,email) values (?, ?, ?, ?) returning id ")
            .bind(user.firstname)
            .bind(user.lastname)
            .bind(user.password)
            .bind(user.email)
            .fetch_one(pool)
            .await?;
        Ok(())
    }

    pub async fn get(id: u64, pool: &MySqlPool) -> anyhow::Result<UserModel> {
        let result = sqlx::query_as_unchecked!(UserModel,
                "select * from users where id = ?",
            id)
            .fetch_one(pool)
            .await?;
        return Ok(result);
    }

    pub async fn delete(id: i64, pool: &MySqlPool) -> anyhow::Result<()> {
        let result = sqlx::query("delete from users where id =?")
            .bind(id)
            .execute(pool)
            .await?;
        return Ok(());
    }

    pub async fn get_all(pool: &MySqlPool) -> anyhow::Result<Vec<UserModel>> {
        let result = sqlx::query_as_unchecked!(UserModel,"select * from users")
            .fetch_all(pool)
            .await?;

        return Ok(result);
    }

    pub fn update(user: UserModel, pool: &MySqlPool) {

    }
}

pub mod todos {
    pub fn insert() {

    }

    pub fn get(id: String) {

    }

    pub fn delete() {

    }

    pub fn get_all() {

    }

    pub fn update() {
        
    }
}