use crate::app::App;

pub mod app;
pub mod event;
pub mod ui; 

#[tokio::main]
async fn main() ->Result<(),Box<dyn std::error::Error>> {
    let terminal = ratatui::init(); 
    
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
    }
