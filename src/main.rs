#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

impl MailBox {
    fn post(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

impl CubeSat {
    fn new(id: u64) -> Self {
        Self {
            id,
            mailbox: MailBox { messages: vec![] },
        }
    }

    fn receive(&mut self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(self)
    }
}

struct GroundStation {}

impl GroundStation {
    fn send(&self, mailbox: &mut MailBox, message: Message) {
        mailbox.post(message);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();
    let mut mail = MailBox { messages: vec![] };

    for sat_id in sat_ids {
        base.connect(sat_id);
        let message = Message {
            to: sat_id,
            content: String::from("hello"),
        };

        base.send(&mut mail, message);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = sat.receive(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}
