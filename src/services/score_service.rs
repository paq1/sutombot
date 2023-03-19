pub struct ScoreService;
impl ScoreService {
    pub fn handle_message(sutom_msg: &String) -> Result<String, String> {
        Ok(sutom_msg.clone())
    }
}