//Functions that define the basic outline of the application
fn title() {
    bash_cmd("clear");
    println!(
        "

    ██████╗ ██████╗    ██╗   ██╗████████╗██╗██╗     
    ██╔══██╗██╔══██╗   ██║   ██║╚══██╔══╝██║██║     
    ██║  ██║██║  ██║   ██║   ██║   ██║   ██║██║     
    ██║  ██║██║  ██║   ██║   ██║   ██║   ██║██║     
    ██████╔╝██████╔╝██╗╚██████╔╝   ██║   ██║███████╗
    ╚═════╝ ╚═════╝ ╚═╝ ╚═════╝    ╚═╝   ╚═╝╚══════╝                                       
"
    );
}
fn menu() {
    loop {
        /* Temp fix - Defining index number here to later be passed to menu_number, otherwise
        numbers go TO THE MOON!! */
        let mut index: u8 = 0;
        title();
        println!(" {}. Toggle the CRT monitor", menu_number(&mut index));
        println!(" {}. Launch MonkeyType", menu_number(&mut index));
        println!(" {}. Toggle lamps", menu_number(&mut index));
        println!("\n\n\n 9. Shutdown");
        println!(" 0. Exit");
        user_menu_selection();
    }
}

//Misc menu enhancements
fn menu_number(index_input: &mut u8) -> u8 {
    *index_input += 1;
    *index_input
}

//Struct and Enum declarations
enum MonitorState {
    Off,
    On,
}

impl MonitorState {
    fn turn_monitor_off(&self) {
        bash_cmd("xrandr --output DP-0 --off");
    }
    fn turn_monitor_on(&self) {
        bash_cmd("xrandr --output DP-0 --mode 800x600 --pos 5120x840");
    }
}

//Main functions that are called within the application

fn user_menu_selection() {
    print!("\n Option: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
    let mut menu_selection: String = String::new();
    std::io::stdin()
        .read_line(&mut menu_selection)
        .expect("Is this ever reached?");
    match menu_selection.trim() {
        "1" => toggle_crt(),
        "2" => launch_monkeytype(),
        "3" => toggle_lamps(),
        "9" => shutdown(),
        "0" => exit(),
        _ => println!("\nInvalid option\n"),
    }
    //Clears the menu_selection string as ".read_line()" continuously appends to the string
    menu_selection.clear();
}
fn bash_cmd(x: &str) {
    std::process::Command::new("bash")
        .arg("-c")
        .arg(x)
        .status()
        .expect("Failed to execute command");
}
fn option() {
    print!("\n Option: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
}
fn toggle_crt() {
    let mut crt_monitor_state: MonitorState = MonitorState::On;

    loop {
        bash_cmd("clear");
        title();
        println!(
            " Toggle the monitor\
        \n 1: OFF \
        \n 2: ON
        \n\n\n\n 0. Back"
        );

        option();
        let mut state: String = String::new();
        std::io::stdin()
            .read_line(&mut state)
            .expect("Issue reading stdin");


        //This match statement is going to utilize enums only because I just learned about them.
        //It is not the cleanest way to do this... or the best.
        match state.trim() {
            "1" => match crt_monitor_state {
                MonitorState::Off => {
                    println!("The CRT monitor is already off. Hit Enter to continue.");
                    crt_monitor_state = MonitorState::Off;
                    std::io::stdin()
                        .read_line(&mut "".to_string())
                        .expect("Issue reading stdin");
                }
                MonitorState::On => {
                    crt_monitor_state.turn_monitor_off();
                    crt_monitor_state = MonitorState::Off;
                }
            },
            "2" => match crt_monitor_state {
                MonitorState::On => {
                    println!("The CRT monitor is already on. Hit Enter to continue.");
                    crt_monitor_state = MonitorState::On;
                    std::io::stdin()
                        .read_line(&mut "".to_string())
                        .expect("Issue reading stdin");
                }
                MonitorState::Off => {
                    crt_monitor_state.turn_monitor_on();
                    crt_monitor_state = MonitorState::On;
                }
            },
            "0" => {
                break;
            }
            _ => {
                println!("\nInvalid option. Hit enter to continue.\n");
                std::io::stdin()
                    .read_line(&mut "".to_string())
                    .expect("Issue reading stdin");
            }
        }
    }
}
fn launch_monkeytype() {
    bash_cmd("nohup xdg-open https://www.monkeytype.com &");
    bash_cmd("clear"); //Gets rid of some stdout messages that hold up main() from looping correctly
}
/*fn toggle_lamps() {
!!Depreciated after tplink_smartplug.py integration FU KASA!!! !!
   bash_cmd("kasa --host 192.168.0.254 toggle");
   bash_cmd("kasa --host 192.168.0.253 toggle");
}*/
fn toggle_lamps() {
    let desk_lamp_state = std::process::Command::new("bash")
        .arg("-c")
        .arg("/home/drippy/Documents/rust/Drippys_Utilities/tplink_smartplug.py -t 192.168.0.254 -c info")
        .output()
        .expect("Failed to execute command");
    let desk_lamp_stdout = String::from_utf8_lossy(&desk_lamp_state.stdout);

    if desk_lamp_stdout.contains("\"relay_state\":0") {
        bash_cmd("nohup /home/drippy/Documents/rust/Drippys_Utilities/tplink_smartplug.py -t 192.168.0.254 -c on &");
        bash_cmd("clear;")
    } else {
        bash_cmd("nohup /home/drippy/Documents/rust/Drippys_Utilities/tplink_smartplug.py -t 192.168.0.254 -c off &");
        bash_cmd("clear;")
    }

    let floor_lamp_state = std::process::Command::new("bash")
        .arg("-c")
        .arg("/home/drippy/Documents/rust/Drippys_Utilities/tplink_smartplug.py -t 192.168.0.253 -c info")
        .output()
        .expect("Failed to execute command");
    let floor_lamp_stdout = String::from_utf8_lossy(&floor_lamp_state.stdout);

    if floor_lamp_stdout.contains("\"relay_state\":0") {
        bash_cmd("nohup /home/drippy/Documents/rust/Drippys_Utilities/tplink_smartplug.py -t 192.168.0.253 -c on &");
        bash_cmd("clear;")
    } else {
        bash_cmd("nohup /home/drippy/Documents/rust/Drippys_Utilities/tplink_smartplug.py -t 192.168.0.253 -c off &");
        bash_cmd("clear;")
    }
}
fn shutdown() {
    bash_cmd("clear");
    title();
    println!(
        " Are you sure?
    \n 1. Shutdown\
    \n\n\n\n\n 0. Back"
    );
    
    option();
    let mut menu_selection: String = String::new();
    std::io::stdin()
        .read_line(&mut menu_selection)
        .expect("Is this ever reached?");
    match menu_selection.trim() {
        "1" => {
            title();
            bash_cmd("shutdown now");
        }
        "0" => {
            menu();
        }
        _ => {
            println!("\nInvalid option. Hit enter to continue.\n");
            std::io::stdin()
                .read_line(&mut "".to_string())
                .expect("Issue reading stdin");
        }
    }
}
fn exit() {
    title();
    println!(
        " Are you sure?
                    \n 1. Exit \
                    \n\n\n\n\n 0. Back"
    );
    option();
    let mut menu_selection: String = String::new();
    std::io::stdin()
        .read_line(&mut menu_selection)
        .expect("Is this ever reached?");
    match menu_selection.trim() {
        "1" => {
            std::process::exit(0);
        }
        "0" => {
            menu();
        }
        _ => {
            println!("\nInvalid option\n");
        }
    }
}
//Main application and the order that the functions are called
fn main() {
    menu();
}
