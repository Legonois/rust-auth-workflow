pub fn help_menu() -> String {

    let help_menu = String::from(
        "Usage: 
        ./main.exe <search> <filename>
        
        Search for a pattern in a file and display the lines that contain it.
        "
    );

    help_menu
}

pub fn version_menu() -> String {

    let version_menu = String::from(
        "Version: 0.1.0"
    );

    version_menu
}

pub fn about_menu() -> String {

    let about_menu = String::from(
        "About: 
        This is a simple program that hashes and salts passwords.
        "
    );

    about_menu
}
