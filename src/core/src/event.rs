use sys::{control, render};
use ::game;

#[derive(Debug)]
pub struct GameEventHub {
    pub control_channel: Option<::sys::control::Channel>,
    pub render_channel: Option<::sys::render::Channel>,
    pub game_channel: Option<::game::Channel>,
}

impl GameEventHub {
    pub fn new(
        control_channel: ::sys::control::Channel,
        render_channel: ::sys::render::Channel,
        game_channel: ::game::Channel,
    ) -> GameEventHub {
        GameEventHub {
            control_channel: Some(control_channel),
            render_channel: Some(render_channel),
            game_channel: Some(game_channel),
        }
    }
}

#[derive(Debug)]
pub struct DevEventHub {
    send_to_control: ::std::sync::mpsc::Sender<::sys::control::RecvEvent>,
    recv_from_control: ::std::sync::mpsc::Receiver<::sys::control::SendEvent>,
    send_to_render: ::std::sync::mpsc::Sender<::sys::render::RecvEvent>,
    recv_from_render: ::std::sync::mpsc::Receiver<::sys::render::SendEvent>,
    send_to_game: ::std::sync::mpsc::Sender<::game::RecvEvent>,
    recv_from_game: ::std::sync::mpsc::Receiver<::game::SendEvent>,
}

impl DevEventHub{
    pub fn new() -> (DevEventHub, GameEventHub) {

        let (send_to_control, recv_to_control) = ::std::sync::mpsc::channel();
        let (send_from_control, recv_from_control) = ::std::sync::mpsc::channel();
        let (send_to_render, recv_to_render) = ::std::sync::mpsc::channel();
        let (send_from_render, recv_from_render) = ::std::sync::mpsc::channel();
        let (send_to_game, recv_to_game) = ::std::sync::mpsc::channel();
        let (send_from_game, recv_from_game) = ::std::sync::mpsc::channel();

        (
            DevEventHub::new_internal(
                send_to_control, recv_from_control,
                send_to_render, recv_from_render,
                send_to_game, recv_from_game,
            ),
            GameEventHub::new((send_from_control, recv_to_control), (send_from_render, recv_to_render), (send_from_game, recv_to_game))
        )
    }

    fn new_internal(
        send_to_control: ::std::sync::mpsc::Sender<::sys::control::RecvEvent>,
        recv_from_control: ::std::sync::mpsc::Receiver<::sys::control::SendEvent>,
        send_to_render: ::std::sync::mpsc::Sender<::sys::render::RecvEvent>,
        recv_from_render: ::std::sync::mpsc::Receiver<::sys::render::SendEvent>,
        send_to_game: ::std::sync::mpsc::Sender<::game::RecvEvent>,
        recv_from_game: ::std::sync::mpsc::Receiver<::game::SendEvent>,
    ) -> DevEventHub
    {
        DevEventHub {
            send_to_control: send_to_control,
            recv_from_control: recv_from_control,
            send_to_render: send_to_render,
            recv_from_render: recv_from_render,
            send_to_game: send_to_game,
            recv_from_game: recv_from_game,

        }
    }

    pub fn send_to_control(&mut self, event: ::sys::control::RecvEvent) {
        match self.send_to_control.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to control error: {}", err),
        }
    }

    pub fn recv_from_control(&mut self) -> control::SendEvent {
        match self.recv_from_control.recv() {
            Ok(event) => event,
            Err(err) => panic!("recv from control error: {}", err),
        }
    }

    pub fn try_recv_from_control(&mut self) -> Option<control::SendEvent> {
        match self.recv_from_control.try_recv() {
            Ok(event) => Some(event),
            Err(err) => match err {
                ::std::sync::mpsc::TryRecvError::Empty => None,
                ::std::sync::mpsc::TryRecvError::Disconnected => panic!("try recv from control was disconnected"),
            }
        }
    }

    pub fn send_to_render(&mut self, event: ::sys::render::RecvEvent) {
        match self.send_to_render.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to render error: {}", err),
        }
    }

    pub fn recv_from_render(&mut self) -> render::SendEvent {
        match self.recv_from_render.recv() {
            Ok(event) => event,
            Err(err) => panic!("recv from render err: {}", err),
        }
    }

    pub fn send_to_game(&mut self, event: ::game::RecvEvent) {
        match self.send_to_game.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to game error: {}", err),
        }
    }

    pub fn recv_from_game(&mut self) -> game::SendEvent {
        match self.recv_from_game.recv() {
            Ok(event) => event,
            Err(err) => panic!("recv from game err: {}", err),
        }
    }

    pub fn try_recv_from_game(&mut self) -> Option<game::SendEvent> {
        match self.recv_from_game.try_recv() {
            Ok(event) => Some(event),
            Err(err) => match err {
                ::std::sync::mpsc::TryRecvError::Empty => None,
                ::std::sync::mpsc::TryRecvError::Disconnected => panic!("try recv from game was disconnected"),
            },
        }
    }

    pub fn process_glutin(&mut self, event: ::glutin::Event) {
        match event {
            ::glutin::Event::MouseMoved(x, y) => self.send_to_control(::sys::control::RecvEvent::MouseMoved(x as u32, y as u32)),
            ::glutin::Event::MouseInput(state, button) => self.send_to_control(::sys::control::RecvEvent::MouseInput(match state {
                ::glutin::ElementState::Pressed => true,
                ::glutin::ElementState::Released => false,
            },
            button)),
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::D)) |
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::Right)) => match state {
                ::glutin::ElementState::Pressed => self.send_to_control(::sys::control::RecvEvent::Right(true)),
                ::glutin::ElementState::Released => self.send_to_control(::sys::control::RecvEvent::Right(false)),
            },
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::A)) |
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::Left)) => match state {
                ::glutin::ElementState::Pressed => self.send_to_control(::sys::control::RecvEvent::Left(true)),
                ::glutin::ElementState::Released => self.send_to_control(::sys::control::RecvEvent::Left(false)),
            },
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::W)) |
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::Up)) => match state {
                ::glutin::ElementState::Pressed => self.send_to_control(::sys::control::RecvEvent::Up(true)),
                ::glutin::ElementState::Released => self.send_to_control(::sys::control::RecvEvent::Up(false)),
            },
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::S)) |
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::Down)) => match state {
                ::glutin::ElementState::Pressed => self.send_to_control(::sys::control::RecvEvent::Down(true)),
                ::glutin::ElementState::Released => self.send_to_control(::sys::control::RecvEvent::Down(false)),
            },
            ::glutin::Event::Resized(width, height) => self.send_to_control(::sys::control::RecvEvent::Resize(width, height)),
            _ => (),
        }
    }
}
