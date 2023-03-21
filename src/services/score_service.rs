pub struct ScoreService;

impl ScoreService {
    pub fn handle_message(sutom_msg: &String) -> Result<String, String> {
        let chaines = Self::get_score_from(sutom_msg);

        println!("chaine  : [{:?}]", chaines);

        Ok(sutom_msg.clone())
    }

    fn get_score_from(sutom_message: &String) -> Option<String> {
        sutom_message
            .replace("\n", " ")
            .split(" ")
            .filter(|chaine| chaine.contains("/") && chaine.len() == 3)
            .map(|chaine| chaine.to_string())
            .collect::<Vec<String>>()
            .get(0)
            .map(|e| e.clone())
    }
}