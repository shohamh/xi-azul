use xrl;

#[derive(Debug)]
pub enum UIMessage {
    Update(xrl::Update),
}