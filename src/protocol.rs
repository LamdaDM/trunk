fn write_code(code: i32) -> String {
    if code == 0 {
        "00".to_string()
    } else {
    code.to_string() 
}
}

fn write_fullcode(code: i32, response: &str) -> String {
    write_code(code) + response
}

pub struct StatusReportFactory;

impl StatusReportFactory {
    pub fn new() -> StatusReportFactory {
        StatusReportFactory
    }

    pub fn create_blank(&self) -> StatusReport {
        StatusReport::new()
    }
}

pub struct StatusReport {
    response: Option<String>,
    error: StatusError,
    secondary: SecondaryStatus
}

impl StatusReport {
    pub fn new() -> StatusReport {
        StatusReport {
            response: None,
            error: StatusError::None,
            secondary: SecondaryStatus::None
        }
    }

    pub fn set_secondary(self, secondary: SecondaryStatus) -> StatusReport {
        StatusReport { secondary, ..self }
    }

    pub fn set_error(self, error: StatusError) -> StatusReport {
        StatusReport { error, ..self }
    }

    pub fn set_response(self, response: String) -> StatusReport {
        StatusReport { response: Some(response), ..self }
    }

    pub fn build_response(&self) -> String {
        if let Some(res) = &self.response {
            write_fullcode(0, res)
        } else {
            write_code(self.error.to_code() + self.secondary.to_code())
        }
    }
}

pub enum StatusError {
    Client,
    Server,
    None
}

impl StatusError {
    pub fn to_code(&self) -> i32 {
        match self {
            StatusError::Client => 10,
            StatusError::Server => 20,
            StatusError::None => 30,
        }
    }
}

pub enum SecondaryStatus {
    Service,
    Dependency,
    None
}

impl SecondaryStatus {
    pub fn to_code(&self) -> i32 {
        match self {
            SecondaryStatus::Service => 1,
            SecondaryStatus::Dependency => 2,
            SecondaryStatus::None => 0,
        }
    }
}

pub trait StatusSource {
    fn report(&self, origin: StatusReport) -> StatusReport;
}