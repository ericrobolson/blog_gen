enum FileNameCheck {
    CheckingYear,
    CheckingMonth,
    CheckingDay,
    CheckingTitle,
    CheckingExtension,
}

pub fn validate(file_name: &String) -> Result<(), String> {
    let mut state = FileNameCheck::CheckingYear;
    let mut state_idx = 0;

    let mut is_malformed = false;
    let mut error_reason = String::default();

    for c in file_name.chars() {
        match state {
            FileNameCheck::CheckingYear => {
                if c.is_numeric() {
                    state_idx += 1;
                } else if c == '.' && state_idx == 4 {
                    // Swap to next state
                    state_idx = 0;
                    state = FileNameCheck::CheckingMonth;
                } else {
                    is_malformed = true;
                    error_reason = "Invalid year".into();
                    break;
                }
            }
            FileNameCheck::CheckingMonth => {
                if c.is_numeric() {
                    state_idx += 1;
                } else if c == '.' && state_idx == 2 {
                    // Swap to next state
                    state_idx = 0;
                    state = FileNameCheck::CheckingDay;
                } else {
                    is_malformed = true;
                    error_reason = "Invalid month".into();
                    break;
                }
            }
            FileNameCheck::CheckingDay => {
                if c.is_numeric() {
                    state_idx += 1;
                } else if c == '_' && state_idx == 2 {
                    // Swap to next state
                    state_idx = 0;
                    state = FileNameCheck::CheckingTitle;
                } else {
                    is_malformed = true;
                    error_reason = "Invalid day".into();
                    break;
                }
            }
            FileNameCheck::CheckingTitle => {
                if c.is_alphanumeric() {
                    state_idx += 1;
                } else if c == '.' {
                    // Swap to next state
                    state_idx = 0;
                    state = FileNameCheck::CheckingExtension;
                } else {
                    is_malformed = true;
                    error_reason = "Invalid title".into();

                    break;
                }
            }
            FileNameCheck::CheckingExtension => {
                if c == 'm' && state_idx == 0 {
                    state_idx += 1;
                } else if c == 'd' && state_idx == 1 {
                    state_idx += 1;
                } else {
                    is_malformed = true;
                    error_reason = "Invalid extension".into();
                    break;
                }
            }
        }
    }

    if is_malformed {
        Err(format!(
            "{}. Expected: YYYY.MM.DD_AlphaNumericTitle.md",
            error_reason
        ))
    } else {
        Ok(())
    }
}
