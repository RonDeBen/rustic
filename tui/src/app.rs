use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use shared_lib::api_client::ApiClient;
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::{
    action::{
        Action,
        UIAct::{self, *},
    },
    api_client::ApiClientExt,
    components::{home::Home, Component},
    config::Config,
    mode::Mode,
    tui,
};

pub struct App {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub last_tick_key_events: Vec<KeyEvent>,
    pub api_client: ApiClient,
}

impl App {
    pub async fn new(tick_rate: f64, frame_rate: f64, api_client: &ApiClient) -> Result<Self> {
        let starting_state = api_client.get_full_state().await?;
        let config = Config::new()?;
        let home = Home::new(starting_state, &config);
        let mode = Mode::Crud;
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![Box::new(home)],
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            last_tick_key_events: Vec::new(),
            api_client: api_client.clone(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        // tui.mouse(true);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(Action::UI(Quit))?,
                    tui::Event::Tick => action_tx.send(Action::UI(Tick))?,
                    tui::Event::Render => action_tx.send(Action::UI(Render))?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::UI(Resize(x, y)))?,
                    tui::Event::Key(key) => {
                        if let Some(keymap) = self.config.keybindings.get(&self.mode) {
                            if let Some(action) = keymap.get(&vec![key]) {
                                log::info!("Got action: {action:?}");
                                action_tx.send(action.clone())?;
                            } else {
                                // If the key was not handled as a single key action,
                                // then consider it for multi-key combinations.
                                self.last_tick_key_events.push(key);

                                // Check for multi-key combinations
                                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                                    log::info!("Got action: {action:?}");
                                    action_tx.send(action.clone())?;
                                }
                            }
                        };
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                if action != Action::UI(Tick) && action != Action::UI(Render) {
                    log::debug!("{action:?}");
                }
                match &action {
                    Action::UI(ui_act) => self.process_ui_action(ui_act, &mut tui, &action_tx)?,
                    Action::TT(_tt_act) => {
                        // processing TT actions, should not happen in this loop
                    }
                    Action::Api(api_act) => {
                        self.api_client
                            .process_api_action(api_act, &action_tx)
                            .await
                    }
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }
            }
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::UI(Resume))?;
                tui = tui::Tui::new()?
                    .tick_rate(self.tick_rate)
                    .frame_rate(self.frame_rate);
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    fn process_ui_action(
        &mut self,
        action: &UIAct,
        tui: &mut tui::Tui,
        action_tx: &UnboundedSender<Action>,
    ) -> Result<()> {
        match action {
            UIAct::Tick => {
                self.last_tick_key_events.drain(..);
            }
            UIAct::Quit => self.should_quit = true,
            UIAct::Suspend => self.should_suspend = true,
            UIAct::Resume => self.should_suspend = false,
            UIAct::Resize(w, h) => {
                tui.resize(Rect::new(0, 0, *w, *h))?;
                tui.draw(|f| {
                    for component in self.components.iter_mut() {
                        let r = component.draw(f, f.size());
                        if let Err(e) = r {
                            action_tx
                                .send(Action::UI(Error(format!("Failed to draw: {:?}", e))))
                                .unwrap();
                        }
                    }
                })?;
            }
            UIAct::Render => {
                tui.draw(|f| {
                    for component in self.components.iter_mut() {
                        let r = component.draw(f, f.size());
                        if let Err(e) = r {
                            action_tx
                                .send(Action::UI(Error(format!("Failed to draw: {:?}", e))))
                                .unwrap();
                        }
                    }
                })?;
            }
            _ => {}
        }
        Ok(())
    }
}
