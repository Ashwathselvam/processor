use std::fmt;

#[derive(Debug)]
pub struct FunctionResponseError {
    pub function: String,
    pub code: u32,
    pub message: String,
}

impl FunctionResponseError {
    pub fn new(function: String, code: u32, message: String) -> Self {
        FunctionResponseError { function, code, message }
    }
}

pub struct WorkflowResponseError {
    pub workflow_id: String,
    pub version: u16,
    pub code: u16,
    pub desciption: String,
}

impl WorkflowResponseError {
    pub fn new(workflow_id: String, version: u16, code: u16, desciption: String) -> Self {
        WorkflowResponseError { workflow_id, version, code, desciption }
    }
}

impl fmt::Display for WorkflowResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Workflow error - ID: {}, Version: {}, Code: {}, Message: {}",
            self.workflow_id, self.version, self.code, self.desciption
        )
    }
}