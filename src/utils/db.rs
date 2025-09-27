
pub mod users {
    use sqlx::MySqlPool;

    use crate::models::auth::UserModel;

    pub async fn insert(user: UserModel, pool: &MySqlPool, id: &str) -> anyhow::Result<()> {
        let result = sqlx::query("insert into users (id,firstname, lastname,password,email) values (?,?, ?, ?, ?) returning id ")
            .bind(id)
            .bind(user.firstname)
            .bind(user.lastname)
            .bind(user.password)
            .bind(user.email)
            .fetch_one(pool)
            .await?;
        Ok(())
    }

    pub async fn get(id: String, pool: &MySqlPool) -> anyhow::Result<UserModel> {
        let result = sqlx::query_as(
                "select * from users where id = ? limit 1")
                .bind(id)
            .fetch_one(pool)
            .await?;
        return Ok(result);
    }

    pub async fn get_by_email(email: String, pool: &MySqlPool) -> anyhow::Result<UserModel> {
        let result = sqlx::query_as(
                "select * from users where email = ? limit 1")
                .bind(email)
            .fetch_one(pool)
            .await?;
        return Ok(result);
    }

    pub async fn delete(id: String, pool: &MySqlPool) -> anyhow::Result<()> {
        let result = sqlx::query("delete from users where id =?")
            .bind(id)
            .execute(pool)
            .await?;
        return Ok(());
    }

    pub async fn get_all(pool: &MySqlPool) -> anyhow::Result<Vec<UserModel>> {
        let result = sqlx::query_as("select * from users order by id ")
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