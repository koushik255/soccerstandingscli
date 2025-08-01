use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
        fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("july32")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text =  "".to_string();
    // so i have to render the objects in order of their position
    // that really does not seem to diffucult
    //
            
            let teams_stand = self.get_standings(); 

        

        let rankings = format!( 
        " Current Standings.\n\
                {}\n ",
            teams_stand,);

        let paragraph = Paragraph::new(text)
            .block(block.clone())
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        let standings = Paragraph::new(rankings)
            
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

       
        paragraph.render(area, buf);
         standings.render(area,buf);
        
    }
}
