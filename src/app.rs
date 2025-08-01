

use std::collections::HashMap;

use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

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
}


impl Default for App {
    fn default() -> Self {
        let mut teams = HashMap::new();
        teams.insert(
            "Arsenal".to_string(),
            Team {
                name: "Arsenal".to_string(),
                position: 1,
            },
        );
        teams.insert(
            "ManCity".to_string(),
            Team {
                name: "ManCity".to_string(),
                position: 2,
            }, 
        );
        teams.insert(
            "ManUtd".to_string(),
            Team {
                name: "ManUtd".to_string(),
                position: 3,
            },
        );

        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            teams,
           
        }
    }
}
#[derive(Debug,Default)]
pub struct Team {
        name: String,
        position: i64,
    }

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
   

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
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
    /// because i would just have to webscrape but been there done that doesnt really work well
    /// cloudflare
    pub fn change(&mut self) {
        if let Some(arsenal_team) = self.teams.get_mut("Arsenal"){
            arsenal_team.position = 3;
        }
        if let Some(mancity_team) = self.teams.get_mut("ManCity"){
            mancity_team.position = 1;
        }

        if let Some(manutd_team) = self.teams.get_mut("ManUtd") {
            manutd_team.position= 2;
        }
       
    }
    // maybe i can append them to a list in order? 
    

    pub fn get_standings(&self) -> String {
        let mut teams: Vec<&Team> = self.teams.values().collect();

        teams.sort_by_key(|team| team.position);

        let mut standings_string = String::from("");
        for team in teams.iter(){
            standings_string.push_str(&format!("{}. {}\n",team.position,team.name,)
                );
        }
        standings_string
    }


}
