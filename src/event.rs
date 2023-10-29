use crate::app::AppResult;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent, KeyEventKind};
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};
use std::time::{Duration, Instant};

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
    /// Nothing, dont do anything
    Nothing,
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: UnboundedReceiver<Event>,
    /// Event handler thread.
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let handler = {
            let sender = sender.clone();
            tokio::spawn(async move {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(key) => {
                                if key.kind == KeyEventKind::Press {
                                  let _ = sender.send(Event::Key(key));
                                };
                            },
                            CrosstermEvent::Mouse(e) => {let _ = sender.send(Event::Mouse(e));},
                            CrosstermEvent::Resize(w, h) => {let _ = sender.send(Event::Resize(w, h));},
                            _ => {},
                        }
                        // .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&mut self) -> AppResult<Event> {
        match self.receiver.try_recv() {
            Ok(app_result) => AppResult::Ok(app_result),
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => AppResult::Ok(Event::Nothing),
            Err(_) => AppResult::Err("".into()),
        }
    }
}
