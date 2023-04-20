use super::{result_code::ResultCode, opcode::Opcode};



#[derive(Debug)]
pub struct Header {
    id: u16,

    recursion_desired: bool,
    truncated: bool,
    authority_answer: bool,
    opcode: Opcode,
    query_response: bool,

    rcode: ResultCode,
    checking_disable: bool,
    authentic_data: bool,
    z: bool,
    recursion_available: bool,

    question_count: u16,
    answer_count: u16,
    name_server_count: u16,
    additional_records_count: u16,
}


impl Header {
    pub fn empty() -> Self {
        Self {
            id: 0,
            recursion_desired: false,
            truncated: false,
            authority_answer: false,
            opcode: Opcode::Query,
            query_response: false,
            rcode: ResultCode::NoError,
            checking_disable: false,
            authentic_data: false,
            z: false,
            recursion_available: false,
            question_count: 0,
            answer_count: 0,
            name_server_count: 0,
            additional_records_count: 0,
        }
    }

    pub fn new(
        id: u16,
        recursion_desired: bool,
        truncated: bool,
        authority_answer: bool,
        opcode: Opcode,
        query_response: bool,
        rcode: ResultCode,
        checking_disable: bool,
        authentic_data: bool,
        z: bool,
        recursion_available: bool,
    ) -> Self {
        Self {
            id,
            recursion_desired,
            truncated,
            authority_answer,
            opcode,
            query_response,
            rcode,
            checking_disable,
            authentic_data,
            z,
            recursion_available,
            question_count: 0,
            answer_count: 0,
            name_server_count: 0,
            additional_records_count: 0,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn recursion_desired(&self) -> bool {
        self.recursion_desired
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn authority_answer(&self) -> bool {
        self.authority_answer
    }

    pub fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn query_response(&self) -> bool {
        self.query_response
    }

    pub fn rcode(&self) -> ResultCode {
        self.rcode
    }

    pub fn checking_disable(&self) -> bool {
        self.checking_disable
    }

    pub fn authentic_data(&self) -> bool {
        self.authentic_data
    }

    pub fn z(&self) -> bool {
        self.z
    }

    pub fn recursion_available(&self) -> bool {
        self.recursion_available
    }

    pub fn question_count(&self) -> u16 {
        self.question_count
    }

    pub fn answer_count(&self) -> u16 {
        self.answer_count
    }

    pub fn name_server_count(&self) -> u16 {
        self.name_server_count
    }

    pub fn additional_records_count(&self) -> u16 {
        self.additional_records_count
    }

    pub fn increment_question_count(&mut self) {
        self.question_count += 1;
    }

    pub fn increment_answer_count(&mut self)  {
        self.answer_count += 1;
    }

    pub fn increment_authority_count(&mut self)  {
        self.name_server_count += 1;
    }

    pub fn increment_additional_record_count(&mut self)  {
        self.additional_records_count += 1;
    }

}