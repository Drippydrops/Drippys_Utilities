//Functions that define the basic outline of the application
fn title() {
    bash_cmd("clear");
    println!(
        r"
    ██████╗ ██████╗ ██╗██████╗ ██████╗ ██╗   ██╗███████╗    ██╗   ██╗████████╗██╗██╗     ██╗████████╗██╗███████╗███████╗
    ██╔══██╗██╔══██╗██║██╔══██╗██╔══██╗╚██╗ ██╔╝██╔════╝    ██║   ██║╚══██╔══╝██║██║     ██║╚══██╔══╝██║██╔════╝██╔════╝
    ██║  ██║██████╔╝██║██████╔╝██████╔╝ ╚████╔╝ ███████╗    ██║   ██║   ██║   ██║██║     ██║   ██║   ██║█████╗  ███████╗
    ██║  ██║██╔══██╗██║██╔═══╝ ██╔═══╝   ╚██╔╝  ╚════██║    ██║   ██║   ██║   ██║██║     ██║   ██║   ██║██╔══╝  ╚════██║
    ██████╔╝██║  ██║██║██║     ██║        ██║   ███████║    ╚██████╔╝   ██║   ██║███████╗██║   ██║   ██║███████╗███████║
    ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝     ╚═╝        ╚═╝   ╚══════╝     ╚═════╝    ╚═╝   ╚═╝╚══════╝╚═╝   ╚═╝   ╚═╝╚══════╝╚══════╝

    "
    );
}
fn menu() {
    loop {
        title();
        println!("1. Toggle the CRT monitor");
        println!("2. Launch MonkeyType");
        println!("3. Toggle lamps");
        println!("\n0. Exit");
        option();
        let mut menu_selection: String = String::new();
        std::io::stdin()
            .read_line(&mut menu_selection)
            .expect("Is this ever reached?");
        match menu_selection.trim() {
            "1" => {
                toggle_crt();
            }
            "2" => {
                launch_monkeytype();
            }
            "3" => {
                toggle_lamps();
            }
            "0" => {
                bash_cmd("clear");
                std::process::exit(0);
            }
            _ => {
                println!("\nInvalid option\n");
            }
        }
        //asdf
    }
}
fn bash_cmd(x: &str) {
    std::process::Command::new("bash")
        .arg("-c")
        .arg(x)
        .status()
        .expect("Failed to execute command");
}

//Misc enhancements
fn option() {
    print!("\nOption: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
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
fn toggle_crt() {
    let mut crt_monitor_state: MonitorState = MonitorState::On;

    loop {
        bash_cmd("clear");
        println!(
            "Toggle the monitor\
        \n1: OFF \
        \n2: ON
        \n\n\0. Back"
        );
        option();

        let mut state: String = String::new();
        std::io::stdin()
            .read_line(&mut state)
            .expect("Issue reading stdin");

        //This match statement is going to utilize enums only because I just learned about them.
        //It is not the cleanest way to do this, or the best.
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
                println!("\nInvalid option\n");
            },
        }
    }
}
fn launch_monkeytype() {
    bash_cmd("nohup xdg-open https://www.monkeytype.com &");
    bash_cmd("clear"); //Gets rid of some stdout messages that hold up main() from looping correctly
}
fn toggle_lamps() {
    bash_cmd("kasa --host 192.168.0.254 toggle");
    bash_cmd("kasa --host 192.168.0.253 toggle");
}
//Main application and the order that the functions are called
fn main() {
    menu();
}