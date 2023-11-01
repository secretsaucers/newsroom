# Newsroom
[![Rust](https://github.com/secretsaucers/newsroom/actions/workflows/rust.yml/badge.svg)](https://github.com/secretsaucers/newsroom/actions/workflows/rust.yml)

The goal: Instantly get a personalized summary of headline articles from your favourite RSS sources

## Layout

Newsroom uses Crossterm and Ratatui to render the application.
Most of the codebase is asynchronous using Tokio. Two long running threads are used, the event thread, and the main thread. 

### Event thread
The event thread is responsible for awaiting inputs from the user, such as keypresses, terminal changes, such as resizing, and a minimal 'tick' event which occurs at a regular interval.
These events are relayed to the main thread over a channel. This allows us to capture input even while the application is busy.

### Main thread
The main thread has two main responsibilities:
- Render the UI
- Act on application state transitions

The UI rendering portion is relatively straightforward and is mostly a pure function based on the App struct. 
The key thing about the main thread's responsibility to act on app transtions is that the main thread alone owns the App struct, and is the only thread that can modify it. As such we use a pump system, where any thread can send transition requests to the App struct over a channel, which the App struct can act on one at a time by calling `app.poll_and_run_action().await`.

As an example lets say we want to fetch some rss articles, process them, and change the app state to have it contain a vector of news articles. The fetching takes the most time and we cannot block the main thread to wait for it (or else we cannot draw our UI!). So instead we spin up a green thread to await the fetching of the news artcles, and pass it a transmitter to the app transition channel. The main thread is now free, and once the fetch thread is completed, it will send a `NewsroomTransition::fetched_data(vec[Articles])` message over the channel. The main thread will call `app.poll_and_run_action().await` in due time, where the app state can be changed with the new data.

## Authors
In order of squash ability:
1) [@spencer-dale](https://github.com/spencer-dale)
2) [@jackwilliamson](https://github.com/jackwilliamson)
3) [@15jgme](https://github.com/15jgme)
