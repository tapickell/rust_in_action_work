#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
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

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug, Copy, Clone)]
enum StatusMessage {
    Ok,
}

fn check_status(_sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };
    let base = GroundStation {};
    let sats = fetch_sat_ids();
    for id in sats {
        let sat = base.connect(id);
        let msg = Message {
            to: id,
            content: String::from("Hey there CubeSat, this is GroundStation"),
        };

        base.send(&mut mail, msg);
        println!("{}, status: {:?}", id, check_status(sat));
        println!("{}, pre: {:?}", id, sat);
        let msg = sat.recv(&mut mail);
        println!("{}, msg.content: {:?}", id, msg.as_ref().unwrap().content);
        println!("{}, msg: {:?}", id, &msg);
        println!("{}, post: {:?}", id, sat);
    }
}
