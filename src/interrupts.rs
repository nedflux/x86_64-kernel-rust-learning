use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use pic8259::ChainedPics;
use spin;
use lazy_static::lazy_static;

use crate::print;
use crate::println;
use crate::gdt;
use crate::format;
use crate::hlt_loop;

use crate::vga_buffer::PROMPT;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});
pub const MAX_LEN: usize = 20;

lazy_static!{
    static ref TEXT: spin::Mutex<[u8; MAX_LEN]> = spin::Mutex::new([b' '; MAX_LEN]);
}

lazy_static!{
    static ref LEN: spin::Mutex<usize> = spin::Mutex::new(0);
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        
        idt
    };
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame
)
{
    // print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode
)
{
    use x86_64::registers::control::Cr2;

    println!("EXEPTION: Page Fault\nAccessed Address: {:?}\nError Code: {:?}\n{:#?}", Cr2::read(), error_code, stack_frame);
    hlt_loop();
}

// fn execute(arg: [u8; MAX_LEN]) {
//     match arg {
//         [b'c', b'o', b'w', b's', b'a', b'y', b's', ..] => println!("
    // (MOO)
    //  \\   ^__^
    //   \\  (oo)\\_______
    //      (__)\\       )\\/\\
    //          ||----w |
    //          ||     ||

//             "),
//         _ => {
//             println!();
//             print!("Command \"");
//             for c in arg {
//                 if c == b' ' {
//                     break;
//                 }
//                 print!("{}", c);
//             }
//             print!("\" is not found");
//             println!();
//         }
//     }    
// }

extern "x86-interrupt" fn keyboard_interrupt_handler( _stack_frame: InterruptStackFrame ) {
    use x86_64::instructions::port::Port;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore)
        );
    }
    let mut keyboard = KEYBOARD.lock();
    
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };

    // let key = match scancode {
    //     0x02 => Some('1'),
    //     0x03 => Some('2'),
    //     0x04 => Some('3'),
    //     0x05 => Some('4'),
    //     0x06 => Some('5'),
    //     0x07 => Some('6'),
    //     0x08 => Some('7'),
    //     0x09 => Some('8'),
    //     0x0a => Some('9'),
    //     0x0b => Some('0'),
    //     _ => None,
    // };

    // if let Some(character) = key {
        // print!("{}", character);
    // }
    // unsafe {
        // let mut locked = TEXT.lock();
        // *locked = "helloooo";
        // print!("{}",*locked);
    // }
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            use pc_keyboard::KeyCode;
            match key {
                 DecodedKey::Unicode(character) => {

                    if character == '\u{0008}'{

                        let mut text = TEXT.lock();
                        let mut len = LEN.lock();

                        if *len > 0 {
                            text[*len - 1] = b' ';
                            *len -= 1;

                            let mut writer = crate::vga_buffer::WRITER.lock();
                            writer.remove_byte();
                        }
                    }

                    else if *LEN.lock() < MAX_LEN {
                        
                        if character == '\n' {
                            let mut text = TEXT.lock();
                            let mut args = [0; MAX_LEN + 1];
                            
                            args[1..1 + text.len()].copy_from_slice(&text[..]);
                            
                            let mut len = LEN.lock();

                            *text = [b' '; MAX_LEN];
                            *len = 0;

                            println!();
                            
                            crate::shell::execute(args);
                            println!();
                        
                            print!("{}", PROMPT);
                        }
                        else {
                            unsafe {
                                let mut text = TEXT.lock();
                                let mut len = LEN.lock();
                                
                                text[*len] = character as u8;

                                *len += 1;
                            }
                            print!("{}", character);
                        }            
                    }               
                }
                // DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => {
                    match key {
                        KeyCode::Backspace | KeyCode::Delete => {
                    
                            let mut text = TEXT.lock();
                            let mut len = LEN.lock();

                            if *len > 0 {
                                text[*len - 1] = b' ';
                                *len -= 1;
                            }
                        }
                        
                        _ => (),
                    }
                    
                } // print!("{:?}", key),
            }
        }
    }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64
) -> !
{
    panic!("DOUBLE FAULT ERROR!!!!\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
)
{
    println!("BREAKPOINT EXEPTION\n{:#?}", stack_frame);
}

