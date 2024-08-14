use iced::{Alignment, Element, Length, Subscription, Command, Application, time, Settings, Theme, executor, widget::{Row, Column, Button, Container, Text}};
use std::time::{Duration, Instant};

fn main() -> iced::Result {
    Pomodoro::run(Settings {
        window: iced::window::Settings {
            size: iced::Size { width: 640.0, height: 360.0 },
            resizable: false,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

struct Pomodoro {
    state: State,
    mode: Mode,
    timer: Duration,
    last_tick: Instant,
}

enum State {
    Idle,
    Paused,
    Running
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Work,
    LongBreak,
    ShortBreak,
}

#[derive(Debug, Clone, Copy)]
enum PomodoroMessage {
    Start,
    Pause,
    Resume,
    SwitchMode(Mode),
    Reset,
    Tick
}

impl Application for Pomodoro {
    type Message = PomodoroMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Pomodoro, iced::Command<PomodoroMessage>) {
        (Pomodoro { state: State::Idle, mode: Mode::Work, timer: Duration::from_secs(25 * 60), last_tick: Instant::now() }, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Rusty Pomodoro")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        const WORK: Duration = Duration::from_secs(25 * 60);
        const SHORT_BREAK: Duration = Duration::from_secs(5 * 60);
        const LONG_BREAK: Duration = Duration::from_secs(15 * 60);
        match message {
            PomodoroMessage::Start => {
                self.timer = match self.mode {
                    Mode::Work => WORK,
                    Mode::ShortBreak => SHORT_BREAK,
                    Mode::LongBreak => LONG_BREAK,
                };
                self.last_tick = Instant::now();
                self.state = State::Running;
                Command::none()
            }
            PomodoroMessage::Resume => {
                self.last_tick = Instant::now();
                self.state = State::Running;
                Command::none()
            }
            PomodoroMessage::SwitchMode(mode) => {
                self.state = State::Idle;
                self.mode = mode;
                self.timer = match self.mode {
                    Mode::Work => WORK,
                    Mode::ShortBreak => SHORT_BREAK,
                    Mode::LongBreak => LONG_BREAK,
                };
                Command::none()
            }
            PomodoroMessage::Pause => {
                self.state = State::Paused;
                Command::none()
            }
            PomodoroMessage::Reset => {
                self.state = State::Idle;
                self.timer = match self.mode {
                    Mode::Work => WORK,
                    Mode::ShortBreak => SHORT_BREAK,
                    Mode::LongBreak => LONG_BREAK,
                };
                Command::none()
            }
            PomodoroMessage::Tick => {
                if let State::Running = self.state {
                    let now = Instant::now();
                    let delta = now - self.last_tick;
                    self.last_tick = now;
                    self.timer = self.timer.checked_sub(delta).unwrap_or_default();
                    if self.timer.as_secs() == 0 {
                        self.state = State::Idle;
                    }
                }
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Paused => Subscription::none(),
            State::Running { .. } => {
                time::every(Duration::from_millis(10)).map(|_| Self::Message::Tick)
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        const HOUR: u64 = 60 * 60;
        const MINUTE: u64 = 60;
        let seconds = self.timer.as_secs();
        let timer = Text::new(format!("{:0>2}:{:0>2}", (seconds % HOUR) / MINUTE, seconds % MINUTE)).size(120);
        let timer_container = Container::new(timer).width(Length::Fill).center_x().center_y();
        let work_button = Button::new("Work").width(Length::FillPortion(1)).height(Length::Fill).on_press(PomodoroMessage::SwitchMode(Mode::Work));
        let short_break_button = Button::new("Short break").width(Length::FillPortion(1)).height(Length::Fill).on_press(PomodoroMessage::SwitchMode(Mode::ShortBreak));
        let long_break_button = Button::new("Long Break").width(Length::FillPortion(1)).height(Length::Fill).on_press(PomodoroMessage::SwitchMode(Mode::LongBreak));
        let start_or_pause_or_resume_button = match self.state {
            State::Idle => Button::new("Start").width(Length::FillPortion(1)).height(Length::Fill).on_press(PomodoroMessage::Start),
            State::Paused => Button::new("Resume").width(Length::FillPortion(1)).height(Length::Fill).on_press(PomodoroMessage::Resume),
            State::Running => Button::new("Pause").width(Length::FillPortion(1)).height(Length::Fill).on_press(PomodoroMessage::Pause),
        };
        let reset_button = Button::new("Reset").width(Length::FillPortion(1)).height(Length::Fill).on_press(PomodoroMessage::Reset);
        let upper_row = Row::new().width(Length::Fill).height(Length::FillPortion(1)).spacing(2).push(work_button).push(short_break_button).push(long_break_button);
        let middle_row = Row::new().width(Length::Fill).height(Length::FillPortion(3)).align_items(Alignment::Center).push(timer_container);
        let lower_row = Row::new().width(Length::Fill).height(Length::FillPortion(1)).spacing(2).push(start_or_pause_or_resume_button).push(reset_button);
        let col = Column::new().push(upper_row).push(middle_row).push(lower_row);
        Container::new(col).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
    }

    fn theme(&self) -> Theme {
        iced::Theme::Dark
    }
}