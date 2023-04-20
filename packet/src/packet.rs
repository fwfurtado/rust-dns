use super::{
    record::{AdditionalRecord, Answer, Authority},
    header::Header, question::Question,
};

#[derive(Debug)]
pub struct Packet {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Answer>,
    authorities: Vec<Authority>,
    additional_records: Vec<AdditionalRecord>,
}

impl Packet {
    pub fn empty() -> Self {
        Self {
            header: Header::empty(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additional_records: Vec::new(),
        }
    }

    pub fn new(
        header: Header,
    ) -> Self {
        Self {
            header,
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additional_records: Vec::new(),
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
        self.header.increment_question_count();
    }

    pub fn add_answer(&mut self, answer: Answer) {
        self.answers.push(answer);
        self.header.increment_answer_count();
    }

    pub fn add_authority(&mut self, authority: Authority) {
        self.authorities.push(authority);
        self.header.increment_authority_count();
    }

    pub fn add_additional_record(&mut self, resource: AdditionalRecord) {
        self.additional_records.push(resource);
        self.header.increment_additional_record_count();
    }
}


#[cfg(test)]
mod test {
    use std::net::Ipv4Addr;

    use fake::{faker::{internet::raw::{DomainSuffix, IPv4}, boolean::en::Boolean}, locales::EN, Fake};

    use crate::{QueryType, QueryClass, opcode::Opcode, ResultCode};

    use super::*;

    #[test]
    fn test_packet() {

        let domain: String = DomainSuffix(EN).fake();
        let ip: &Ipv4Addr = &IPv4(EN).fake();
        let rcode = &ResultCode::from((0..23).fake::<u8>());
        let query_class = &QueryClass::from((1..4).fake::<u16>());
        let random_ttl = (0..100).fake::<u32>();

        let id = (0..100).fake::<u16>();
        let opcode = Opcode::from((0..15).fake::<u8>());
        let recursion_desired = Boolean(5).fake::<bool>();
        let truncated = Boolean(5).fake::<bool>();
        let authority_answer = Boolean(5).fake::<bool>();
        let query_response = Boolean(5).fake::<bool>();
        let checking_disable = Boolean(5).fake::<bool>();
        let authentic_data = Boolean(5).fake::<bool>();
        let z = Boolean(5).fake::<bool>();
        let recursion_available = Boolean(5).fake::<bool>();

        let query_count = (0..10).fake::<usize>();
        let answer_count = (0..10).fake::<usize>();
        let ns_count = (0..10).fake::<usize>();
        let ar_count = (0..10).fake::<usize>();


        let header = Header::new(
            id,
            recursion_desired,
            truncated,
            authority_answer,
            opcode,
            query_response,
            *rcode,
            checking_disable,
            authentic_data,
            z,
            recursion_available
        );
        let mut packet = Packet::new(header);

        for _ in 0..query_count {
            packet.add_question(Question::new(domain.clone(), QueryType::A, *query_class));
        }


        for _ in 0..answer_count {
            packet.add_answer(Answer::A {
                domain: domain.clone(),
                address: *ip,
                ttl: random_ttl,
            });
        }


        for _ in 0..ns_count {
            packet.add_authority(Authority::A {
                domain: domain.clone(),
                address: *ip,
                ttl: random_ttl,
            });
        }

        for _ in 0..ar_count {
            packet.add_additional_record(AdditionalRecord::A {
                domain: domain.clone(),
                address: *ip,
                ttl: random_ttl,
            });
        }


        assert_eq!(packet.header.id(), id);
        assert_eq!(packet.header.recursion_desired(), recursion_desired);
        assert_eq!(packet.header.truncated(), truncated);
        assert_eq!(packet.header.authority_answer(), authority_answer);
        assert_eq!(packet.header.opcode(), opcode);
        assert_eq!(packet.header.query_response(), query_response);
        assert_eq!(packet.header.rcode(), *rcode);
        assert_eq!(packet.header.checking_disable(), checking_disable);
        assert_eq!(packet.header.authentic_data(), authentic_data);
        assert_eq!(packet.header.z(), z);
        assert_eq!(packet.header.recursion_available(), recursion_available);

        assert_eq!(packet.questions.len(), packet.header.question_count() as usize);
        assert_eq!(packet.answers.len(), packet.header.answer_count() as usize);
        assert_eq!(packet.authorities.len(), packet.header.name_server_count() as usize);
        assert_eq!(packet.additional_records.len(), packet.header.additional_records_count() as usize);

        for question in packet.questions {
            assert_eq!(question.name, domain.clone());
            assert_eq!(question.query_class, *query_class);
            assert_eq!(question.query_type, QueryType::A);
        }

        for answer in packet.answers {
            if let Answer::A { domain, address, ttl } = answer {
                assert_eq!(domain, domain.clone());
                assert_eq!(address, *ip);
                assert_eq!(ttl, random_ttl);
            } else {
                panic!("Answer is not A");
            }
        }

        for authority in packet.authorities {
            if let Authority::A { domain, address, ttl } = authority {
                assert_eq!(domain, domain.clone());
                assert_eq!(address, *ip);
                assert_eq!(ttl, random_ttl);
            } else {
                panic!("Authority is not A");
            }
        }

        for additional_record in packet.additional_records {
            if let AdditionalRecord::A { domain, address, ttl } = additional_record {
                assert_eq!(domain, domain.clone());
                assert_eq!(address, *ip);
                assert_eq!(ttl, random_ttl);
            } else {
                panic!("AdditionalRecord is not A");
            }
        }

    }
}