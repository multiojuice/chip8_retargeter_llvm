use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputDriver {
    event_pump: sdl2::EventPump,
}

impl InputDriver {
    pub fn new(context: &sdl2::Sdl) -> Self {
        InputDriver { 
          event_pump: context.event_pump().unwrap() 
        }
    }

    pub fn get_input(&mut self) -> Result<[bool; 16], ()> {
        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            };
        }

        // From SDL demo
        let keys_pressed: Vec<Keycode> = self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut input = [false; 16];

        for key in keys_pressed {
            // Chip8 spec
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            match index {
                Some(i) => input[i] = true,
                None => ()
            }
        }

        Ok(input)
    }
}