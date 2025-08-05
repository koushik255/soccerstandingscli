use ratatui::{
    buffer::Buffer, layout::{Alignment, Rect}, style::Styled, widgets::{Block, BorderType, Paragraph, Widget,}
    
};
use ratatui::text::{Line,Span,Text};
use ratatui::style::{Color,Style};

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

       
    // so i have to render the objects in order of their position
    // that really does not seem to diffucult
    //
            
           // let teams_stand = self.get_standings(); 
            //let teams_stand_wins = self.get_standings_wins();  


            let current_standings = self.standings.clone();

        //let teams_and_points = self.teampoint.clone();

        

        let rankings = format!( 
        " Current Standings.\n {}\n ",current_standings,);

            let standings = Paragraph::new(Text::from(rankings))
            .block(block.clone())
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap{trim :false});
        //
        // let scraped = Paragraph::new(scraped_standings)
        //     .fg(Color::Cyan)
        //     .bg(Color::Black)
        //     .centered();
            //
      //  let team_and_point_list: Vec<ListItem> = teams_and_points
       //     .iter()
      //      .map(|(team,points)| {
      //          ListItem::new(format!("{:?}: {:?}",team,points))
      //      })
     //   .collect();

       // let list = List::new(team_and_point_list)
      //      .block(block.clone())
        //    .fg(Color::Cyan)
        //    .bg(Color::Black);


      
         standings.render(area,buf);
         // scraped.render(area,buf);
        // list.render(area,buf);
        
    }
}
