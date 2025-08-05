

use std::collections::HashMap;

use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

use scraper::{Html,Selector};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    pub teams : HashMap<String,Team>,
    pub standings: String,
    pub teampoint: HashMap<String,TeamStats>,
}


impl Default for App {
    fn default() -> Self {
        let mut teams = HashMap::new();
        teams.insert(
            "AFC".to_string(),
            Team {
                name: "AFC".to_string(),
                position: 1,
                points:"".to_string(),
                wins: "".to_string(),
                losses: "".to_string(),
               
            },
        );
        teams.insert(
            "ManCity".to_string(),
            Team {
                name: "ManCity".to_string(),
                position: 2,
                points:"".to_string(),
                wins: "".to_string(),
                losses: "".to_string(),

                
        
            }, 
        );
        teams.insert(
            "ManUtd".to_string(),
            Team {
                name: "ManUtd".to_string(),
                position: 3,
                points:"".to_string(),
                wins: "".to_string(),
                losses: "".to_string(),
                
            },
        );
        teams.insert(
            "Lvpl".to_string(),
            Team{
            name: "Lvpl".to_string(),
            position:4,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
        teams.insert(
            "Sund".to_string(),
            Team{
            name: "Sund".to_string(),
            position:4,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
         teams.insert(
            "Evr".to_string(),
            Team{
            name: "Evr".to_string(),
            position:4,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
        teams.insert(
            "Crys".to_string(),
            Team{
            name: "Crys".to_string(),
            position:4,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
         teams.insert(
            "Leed".to_string(),
            Team{
            name: "Leed".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
        teams.insert(
            "Tott".to_string(),
            Team{
            name: "Tott".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
         teams.insert(
            "Bren".to_string(),
            Team{
            name: "Bren".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
         teams.insert(
            "West".to_string(),
            Team{
            name: "West".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
         teams.insert(
            "Fulh".to_string(),
            Team{
            name: "Fulh".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
          teams.insert(
            "BHA".to_string(),
            Team{
            name: "BHA".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
           teams.insert(
            "Asvi".to_string(),
            Team{
            name: "Asvi".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
            teams.insert(
            "Chel".to_string(),
            Team{
            name: "Chel".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
         teams.insert(
            "AFCB".to_string(),
            Team{
            name: "AFCB".to_string(),
            position:4,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
          teams.insert(
            "Burn".to_string(),
            Team{
            name: "Burn".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
         teams.insert(
            "NUFC".to_string(),
            Team{
            name: "NUFC".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
          teams.insert(
            "NOTT".to_string(),
            Team{
            name: "NOTT".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            },
        );
         teams.insert(
            "wolf".to_string(),
            Team{
            name: "wolf".to_string(),
            position:0,
            points: "".to_string(),
            wins: "".to_string(),
            losses: "".to_string(),
            
            },
        );
        

          
    


        let  teampoint = HashMap::new();

        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            teams,
            standings: String::new(),
            teampoint,
           
        }
    }
}
#[derive(Debug,Default,Clone)]
pub struct Team {
        name: String,
        position: i64,
        points: String,
        wins: String,
        losses: String,
       
    }

#[derive(Debug,Default,Clone)]
pub struct TeamStats{
    points:String,
    wins:String,
    losses:String,
    stand: i64,
   
}
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
   

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<(),Box <dyn std::error::Error>> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => if let crossterm::event::Event::Key(key_event) = 
                    event{self.handle_key_events(key_event)?},
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::Double => self.double(),
                    AppEvent::Change=> self.change(),
                    AppEvent::Swap=> self.blud(),
                    AppEvent::ShowStand =>self.scrape_standings().await?,
                    AppEvent::SortWin => self.get_standings_wins(),
                
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            KeyCode::Up => self.events.send(AppEvent::Double),
            KeyCode::Char('a')=> self.events.send(AppEvent::Change),
            KeyCode::Char('b') => self.events.send(AppEvent::Swap),
            KeyCode::Char('u') => self.events.send(AppEvent::ShowStand),
            KeyCode::Char('o') => self.events.send(AppEvent::SortWin),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn double(&mut self) {

        self.counter = self.counter.saturating_add(2)   ;
    }

    
    pub fn change(&mut self) {
               
            self.set_team_vars("Arsenal","AFC");
            self.set_team_vars("Manchester United","ManUtd");
            self.set_team_vars("Manchester City", "ManCity");
            self.set_team_vars("Liverpool FC","Lvpl");
            self.set_team_vars("Sunderland","Sund");
            self.set_team_vars("Everton","Evr");
            self.set_team_vars("Crystal Palace","Crys");
            self.set_team_vars("Leeds United","Leed");
            self.set_team_vars("Tottenham Hotspur","Tott");
            self.set_team_vars("Brentford", "Bren");
            self.set_team_vars("West Ham United", "West");
            self.set_team_vars("Fulham","Fulh");
            self.set_team_vars("Brighton & Hove Albion", "BHA");
            self.set_team_vars("Aston Villa","Asvi");
            self.set_team_vars("Chelsea","Chel");
            self.set_team_vars("AFC Bournemouth","AFCB");
            self.set_team_vars("Burnley","Burn");
            self.set_team_vars("Newcastle United","NUFC");
            self.set_team_vars("Nottingham Forest","NOTT");
            self.set_team_vars("Wolverhampton Wanderers","wolf");


        






    }
    // maybe i can append them to a list in order? 
    //

    pub fn blud(&mut self) {
        if let Some(manutd_team) = self.teams.get_mut("ManUtd") {
            manutd_team.position = 1;
        }
        if let Some(mancity_team) = self.teams.get_mut("ManCity") {
            mancity_team.position = 2;
        }

    }
    

    pub fn get_standings(&mut self) -> String {
        let mut teams: Vec<&Team> = self.teams.values().collect();

        teams.sort_by_key(|team| team.position);
       
        let mut standings= String::from("");
        for team in teams.iter(){
            standings.push_str(&format!("{}. {} {} {} {}\n",team.position,team.name,team.wins,team.losses,team.points)
             );
        }
        self.standings= standings.clone();
        standings
    }

    // what if i made the standings string a state so then i could just update the state?
    // the current standings is the html standings,nvm it does nothing
    // if i just 
    //

    pub fn get_standings_wins(&mut self){
    
        let mut teams: Vec<&Team> = self.teams.values().collect();
        teams.sort_by_key(|team| team.wins.clone());

        let mut standings = String::new();

        for team in teams.iter(){
            standings.push_str(&format!("{}. {} {} {} {}\n",team.position,team.name,team.wins,team.losses,team.points));
        
        }
        self.standings= standings.clone();

       

    }




    pub fn scrape_main() -> Result<(),Box<dyn std::error::Error>> {
        let url = "https://onefootball.com/en/competition/premier-league-9/table";
        let response = reqwest::blocking::get(url)?;

        println!("Response Status : {}", response.status());

        let body = response.text()?;
        println!("REsponse length: {}",body.len());

        Ok(())
    }

pub async fn scrape_standings(&mut self)  -> Result<(), Box<dyn std::error::Error>> {
    let link = "https://onefootball.com/en/competition/premier-league-9/table";
    
    let response = reqwest::get(link).await?.text().await?;
    let document = Html::parse_document(&response);
    
    let row_selector = Selector::parse("li.Standing_standings__row__5sdZG").unwrap();
    let position_selector = Selector::parse("div.Standing_standings__cell__5Kd0W").unwrap();
    let team_name_selector = Selector::parse("p.Standing_standings__teamName__psv61").unwrap();

       
    let mut standings = String::new();
    standings.push_str("+----------+------------------+--------+------+-------+--------+----------------+--------+\n");
    standings.push_str("| Position | Team             | Played | Wins | Draws | Losses | Goal Difference| Points |\n");
    standings.push_str("+----------+------------------+--------+------+-------+--------+----------------+--------+\n");

    for row in document.select(&row_selector) {
        let position = row.select(&position_selector).next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
            
        let team = row.select(&team_name_selector).next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        
        let stats: Vec<_> = row.select(&position_selector).collect();
        
        if stats.len() >= 8 {
            let played = stats[2].text().collect::<String>().trim().to_string();
            let wins = stats[3].text().collect::<String>().trim().to_string();
            let draws = stats[4].text().collect::<String>().trim().to_string();
            let losses = stats[5].text().collect::<String>().trim().to_string();
            let goal_diff = stats[6].text().collect::<String>().trim().to_string();
            let points = stats[7].text().collect::<String>().trim().to_string();
            // i need a way to just said certain information, or i need to parse the text 
            // parseing the text would be easier i think and i could run that format for all of the
            // teams since they all have the same format
            let stand_val = position.clone().parse::<i64>().unwrap();
            self.teampoint.insert(team.clone(),TeamStats{
                points: points.clone(),
                wins: wins.clone(),
                losses : losses.clone(),
                stand : stand_val,
            });
            
            standings.push_str(&format!("| {:<8} | {:<16} | {:<6} | {:<4} | {:<5} | {:<6} | {:<14} | {:<6} |\n", 
                     position, team, played, wins, draws, losses, goal_diff, points));
           
        }
    }
    
    standings.push_str("+----------+------------------+--------+------+-------+--------+----------------+--------+\n");
    //self.standings = standings;

    self.change();
    self.get_standings();   
    Ok(())
}


    pub fn set_team_vars(&mut self,team: &str,teamnik: &str) {
        if let Some(this_team) = self.teams.get_mut(teamnik){
        let mut tp = self.teampoint.clone();
        let team_point = tp.drain();

        for (t,v) in team_point {
            
            
            if t == team {
            this_team.name = t;
            this_team.wins = v.wins;
            this_team.losses = v.losses;
            this_team.points = v.points;
            this_team.position = v.stand;
            
            
          }
        }
     }
  }

// next i should add well maybe the badges first,
// or acually i want to add the badges at a later points because i dont resally
// care about them right now anyways 
// i want to add sorting methods so that you can chose like the way the 
// teams are listed depending on the different attributes,
// make arsenals text golden because best team ofc
// 

   

}
