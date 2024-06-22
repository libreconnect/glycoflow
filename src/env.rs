use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Env {
    #[clap(env)]
    pub rabbitmq_url: String,
    #[clap(env)]
    pub rabbitmq_password: String,
    #[clap(env)]
    pub rabbitmq_user: String,
    #[clap(env)]
    pub rabbitmq_port: u16,
}
