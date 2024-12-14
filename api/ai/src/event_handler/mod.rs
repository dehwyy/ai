use serenity::{
    all::{Context, EventHandler as EventHandlerTrait, Ready},
    async_trait,
};

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventHandlerTrait for EventHandler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("{} is connected!", ctx.cache.current_user().name);
    }
}
