

use ratatui::widgets::ListState;
use crate::managers::{PackageManager, Package};


#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Search,
    Executing,
    Done,
}

pub struct App {
    pub state: ListState,                  
    pub packages: Vec<Package>,            
    pub search_query: String,              
    pub input_mode: InputMode,             
    pub manager: Box<dyn PackageManager>,  
    pub selected_for_removal: Vec<String>, 
    pub logs: Vec<String>,                 
}

impl App {
    
    pub fn new(manager: Box<dyn PackageManager>) -> Self {
        let mut app = Self {
            state: ListState::default(),
            packages: Vec::new(),
            search_query: String::new(),
            input_mode: InputMode::Normal,
            manager,
            selected_for_removal: Vec::new(),
            logs: Vec::new(),
        };
        app.refresh_packages();
        app
    }

    
    pub fn refresh_packages(&mut self) {
        self.packages = self.manager.list_packages().unwrap_or_else(|_| Vec::new());
        if !self.packages.is_empty() {
            self.state.select(Some(0)); 
        } else {
            self.state.select(None);
        }
    }

    
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.packages.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.packages.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}