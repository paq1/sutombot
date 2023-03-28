use serenity::async_trait;
use crate::core::entities::party::Party;

#[async_trait]
pub trait PartyService {

    fn handle_message(&self, sutom_msg: &String) -> Result<Party, String> {
        let taille_mot_opt = Self::get_taille_du_mot_from(sutom_msg);
        let nombre_essai_total_opt = Self::get_nombre_essaies_total(sutom_msg);
        let nombre_essai_opt = Self::get_nombre_essaies(sutom_msg);

        match (taille_mot_opt, nombre_essai_total_opt, nombre_essai_opt) {
            (Some(taille), Some(total), Some(current)) => Ok(
                Party::new(taille as u32, total as u32, current as u32)
            ),
            _ => Err("une erreur est survenue".into())
        }
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

    fn get_taille_du_mot_from(sutom_message: &String) -> Option<usize> {
        sutom_message
            .split("\n\n")
            .map(|e| {
                e.to_string()
            })
            .collect::<Vec<String>>()
            .get(1)
            .map(|chaine| {
                chaine
                    .split("\n")
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .get(0)
                    .map(|premiere_ligne| {
                        (premiere_ligne.len() / 4) as usize
                    })
            })
            .flatten()
    }

    fn info_essaie(sutom_message: &String, index: usize) -> Option<usize> {
        Self::get_score_from(sutom_message)
            .map(|score| {
                score
                    .split("/")
                    .map(|value_str| value_str.parse::<usize>().unwrap_or(0usize))
                    .collect::<Vec<usize>>()
                    .get(index)
                    .map(|taille| taille.clone())
            })
            .flatten()
    }

    fn get_nombre_essaies_total(sutom_message: &String) -> Option<usize> {
        Self::info_essaie(sutom_message, 1)
    }

    fn get_nombre_essaies(sutom_message: &String) -> Option<usize> {
        Self::info_essaie(sutom_message, 0)
    }
}
