use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Env {
    #[clap(env)]
    pub host: String,
    #[clap(env)]
    pub port: u16,

    #[clap(env)]
    pub rabbitmq_url: String,
    #[clap(env)]
    pub rabbitmq_password: String,
    #[clap(env)]
    pub rabbitmq_user: String,
    #[clap(env)]
    pub rabbitmq_port: u16,

    #[clap(env)]
    pub postgres_url: String,
    #[clap(env)]
    pub postgres_password: String,
    #[clap(env)]
    pub postgres_user: String,
    #[clap(env)]
    pub postgres_port: u16,
    #[clap(env)]
    pub postgres_db: String,
}
