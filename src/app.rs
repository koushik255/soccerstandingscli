

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
            "Arsenal".to_string(),
            Team {
                name: "arsenal".to_string(),
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

    /// now i just need a way to get the live positions, i could just use a api then i would just
    /// have to make 20 different calls to set the team position for each team
    /// also if i were to use a api i could get all the other data alwell for the team, 
    /// of course i could start with the soccer api but then later i could just migrate to my own 
    /// because i would just have to webscrape but been there done that doesnt really work well cloudflare
    pub fn change(&mut self) {
        let standings = self.standings.clone();
        if let Some(arsenal_team) = self.teams.get_mut("Arsenal"){
           let _new_name:Vec<String> = standings.lines()
        .filter(|line| line.contains("Arsenal"))
        .map(|line| line.to_string())
        .collect();
              
            let mut  tp = self.teampoint.clone();
            let team_point = tp.drain();

        

           
            for (t,v) in team_point{
                if t =="Arsenal"{
                    arsenal_team.name = t;
                } 

                if v.wins == *"0".to_string(){
                    arsenal_team.position = 4;
                    
                }
                // im trying to set the team variable to the result of the team_point hash
                
                arsenal_team.points = v.points;
                arsenal_team.wins = v.wins;
                arsenal_team.losses = v.losses;
                
                

            }
           
           // arsenal_team.name = new_name.first().unwrap().to_string();
            // arsenal_team.position = 3;
        }
        if let Some(mancity_team) = self.teams.get_mut("ManCity"){
            mancity_team.position = 1;

            let mut tp = self.teampoint.clone();
            let team_point = tp.drain();

            for (t,_v) in team_point{
                if t =="Manchester City"{
                    mancity_team.name = t;
                }
            }
        }

        if let Some(manutd_team) = self.teams.get_mut("ManUtd") {
            manutd_team.position= 2;
        }
       
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
    

    pub fn get_standings(&self) -> String {
        let mut teams: Vec<&Team> = self.teams.values().collect();

        teams.sort_by_key(|team| team.position);
       
        let mut standings_string = String::from("");
        for team in teams.iter(){
            standings_string.push_str(&format!("{}. {} {} {} {}\n",team.position,team.name,team.wins,team.losses,team.points)
             );
        }
        standings_string
    }


    pub fn scrape_main() -> Result<(),Box<dyn std::error::Error>> {
        let url = "https://onefootball.com/en/competition/premier-league-9/table";
        let response = reqwest::blocking::get(url)?;

        println!("Response Status : {}", response.status());

        let body = response.text()?;
        println!("REsponse length: {}",body.len());

        Ok(())
    }

   
// i honestyl really think that putting this into a text file then just using filesystem and
// assigned each position to each team would be better than this
//
// or i just return it in a String, ok well self.standings is already a string i can just clone the
// results of that?
// ok if i parse the string with the name of the team then 
// -- i could make this function consitant for all the teams because they follow the same format
// i would be able to connect the points with the acuatly team struct 
// Finding another way (it must exist)
// what if i made a function which took everything then printed the values into a hashmap while 
// assigning each value to the proper team, althorugh i dont think this would work because
// how would i , maybe it would read the string and then print out just the team and points
// then i would be able to easially assign these, but then that makes for more functin calls which
// is more api calls so more waiting, it would be much better do beable to run just this function
// below for scraping, so parseing the string would be the best method for that but im still not
// convinced, wait if im getting returend the team and points i should easilly be able to match
// them up

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
            self.teampoint.insert(team.clone(),TeamStats{
                points: points.clone(),
                wins: wins.clone(),
                losses : losses.clone()
            
            });
            
            standings.push_str(&format!("| {:<8} | {:<16} | {:<6} | {:<4} | {:<5} | {:<6} | {:<14} | {:<6} |\n", 
                     position, team, played, wins, draws, losses, goal_diff, points));
           
        }
    }
    
    standings.push_str("+----------+------------------+--------+------+-------+--------+----------------+--------+\n");
    self.standings = standings;
    self.change();
    
    Ok(())
}

   

}
