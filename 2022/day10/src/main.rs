fn main() {
    find_signal_strengths();
}

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl From<String> for Instruction {
    fn from(line: String) -> Self {
        match &line[0..4] {
            "noop" => Instruction::Noop,
            "addx" => {
                let amount = line[5..].parse::<i32>().unwrap();
                Instruction::AddX(amount)
            }
            _ => unreachable!("Invalid instruction"),
        }
    }
}

struct Cpu {
    x: i32,
    add_x_after_cycle: i32,
    cycles: u64,
    current_instruction_cycles: u64,
    instruction: Option<Instruction>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            x: 1,
            add_x_after_cycle: 0,
            cycles: 0,
            current_instruction_cycles: 0,
            instruction: None,
        }
    }

    fn clear_instruction(&mut self) {
        self.current_instruction_cycles = 0;
        self.instruction = None;
    }

    /// returns `true` if it's ready to fetch a new instruction
    pub fn tick(&mut self) {
        self.cycles += 1;

        let instruction = self.instruction.expect("Ticked without fetching");
        match instruction {
            Instruction::Noop => {
                self.clear_instruction();
            }
            Instruction::AddX(x) => {
                self.current_instruction_cycles += 1;

                if self.current_instruction_cycles == 2 {
                    self.add_x_after_cycle = x;
                    self.clear_instruction();
                }
            }
        }
    }

    // Things that happen ""after" the cycle.."
    pub fn apply_mutations(&mut self) {
        self.x += self.add_x_after_cycle;
        self.add_x_after_cycle = 0;
    }

    pub fn should_fetch(&self) -> bool {
        self.instruction.is_none()
    }

    pub fn fetch(&mut self, instruction: Instruction) {
        self.instruction = Some(instruction);
    }

    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    pub fn x(&self) -> i32 {
        self.x
    }
}

pub fn find_signal_strengths() {
    let mut lines = common::get_lines!();
    let mut cpu = Cpu::new();
    let mut crt_x = 0;

    let mut total_signal_strength = 0;

    'draw_crt: loop {
        if cpu.should_fetch() {
            let Some(line) = lines.next() else {
                break 'draw_crt;
            };
            let instruction: Instruction = line.unwrap().into();
            cpu.fetch(instruction);
        }

        cpu.apply_mutations();
        cpu.tick();

        let cycles = cpu.cycles() as i32;
        let sprite_x = cpu.x();

        // draw crt
        if sprite_x.abs_diff(crt_x) <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        crt_x += 1;
        if crt_x == 40 {
            crt_x = 0;
            println!();
        }

        // handle signal strength
        match cycles {
            20 | 60 | 100 | 140 | 180 | 220 => {
                total_signal_strength += cycles * sprite_x;
            }
            _ => {}
        }
    }

    println!();
    println!("Total signal strength: {}", total_signal_strength);
}
