# Rust + GraphQL + PostgreSQL

### Welcome!

## Get started

This is a handy template for the stack above. 
To make it work, you have to put the following in your `.env` file in the root of the project. 
```
DATABASE_URL=postgres://username@localhost:port/dbname
```


To launch it, simply write 
```bash  
cargo install
```
and
```powershell
cargo run
```

## Instruments

- I use [SeaORM](https://www.sea-ql.org/SeaORM/) as ORM for PostgresSQL. It is relatively new, and i like it.
- I use [Juniper](https://docs.rs/juniper/latest/juniper/) as GrahpQL wrapper. The amount of work it encapsulates is just awesome.
- I use [Actix-web](https://actix.rs/) as a backend framework.

