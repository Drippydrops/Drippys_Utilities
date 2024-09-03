//Functions that define the basic outline of the application
fn title() {
    bash_cmd("clear");
    println!(r"
    ██████╗ ██████╗ ██╗██████╗ ██████╗ ██╗   ██╗███████╗    ██╗   ██╗████████╗██╗██╗     ██╗████████╗██╗███████╗███████╗
    ██╔══██╗██╔══██╗██║██╔══██╗██╔══██╗╚██╗ ██╔╝██╔════╝    ██║   ██║╚══██╔══╝██║██║     ██║╚══██╔══╝██║██╔════╝██╔════╝
    ██║  ██║██████╔╝██║██████╔╝██████╔╝ ╚████╔╝ ███████╗    ██║   ██║   ██║   ██║██║     ██║   ██║   ██║█████╗  ███████╗
    ██║  ██║██╔══██╗██║██╔═══╝ ██╔═══╝   ╚██╔╝  ╚════██║    ██║   ██║   ██║   ██║██║     ██║   ██║   ██║██╔══╝  ╚════██║
    ██████╔╝██║  ██║██║██║     ██║        ██║   ███████║    ╚██████╔╝   ██║   ██║███████╗██║   ██║   ██║███████╗███████║
    ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝     ╚═╝        ╚═╝   ╚══════╝     ╚═════╝    ╚═╝   ╚═╝╚══════╝╚═╝   ╚═╝   ╚═╝╚══════╝╚══════╝

    ");
}
fn menu() {
    loop {
        title();
        println!("1. Toggle the CRT monitor");
        println!("2. Launch MonkeyType");
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
            "0" => {
                std::process::exit(0);
            }
            _ => {
                println!("\nInvalid option\n");
            }
        }
    }
}
fn bash_cmd(x: &str, ) {
    std::process::Command::new("bash")
        .arg("-c")
        .arg(x)
        .status()
        .expect("Failed to execute command");
}
fn option() {
    print!("\nOption: ");
    std::io::Write::flush(&mut std::io::stdout())
        .expect("Failed to flush stdout");
}

//Main functions that are called within the application
fn toggle_crt() {
    loop {
        bash_cmd("clear");
        let mut state: String = String::new();
        println!("Toggle the monitor\
        \n1: OFF \
        \n2: ON\n\n\
        0. Back");
        option();

        std::io::stdin()
            .read_line(&mut state)
            .expect("Is this ever reached?");
        match state.trim() {
            "1" => {
                bash_cmd("xrandr --output DP-0 --off");
            }
            "2" => {
                bash_cmd("xrandr --output DP-0 --mode 800x600 --pos 5120x840");
            }
            "0" => {
                break;
            }
            _ => {
                println!("\nInvalid option\n");
            }
        }
    }
}
fn launch_monkeytype() {
    bash_cmd("nohup xdg-open https://www.monkeytype.com &");
    bash_cmd("clear"); //Gets rid of some stdout messages that hold up main() from re-launching
}

//Main application and the order that the functions are called
fn main() {
        menu();
}