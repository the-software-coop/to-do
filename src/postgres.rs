use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
 
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
 
pub fn get_pool() -> PostgresPool {
   let url = "postgres://postgres:password@localhost:5432";
   let migr = ConnectionManager::<PgConnection>::new(url);
   r2d2::Pool::builder()
       .build(migr)
       .expect("could not build connection pool")
}