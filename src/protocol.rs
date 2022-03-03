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
    response: String,
    ok: bool,
    error: StatusError,
    secondary: SecondaryStatus
}

impl StatusReport {
    pub fn new() -> StatusReport {
        StatusReport {
            response: String::new(),
            ok: true,
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
        StatusReport { response, ..self }
    }

    pub fn build_response(&self) -> String {
        if self.ok {
            String::new() + "00" + &self.response
        } else {
            (self.error.to_code() + self.secondary.to_code()).to_string() + &self.response
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