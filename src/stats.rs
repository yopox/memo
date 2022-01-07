use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, PartialEq, EnumIter)]
pub enum Carac {
    Force,
    Dexterite,
    Endurance,
    Intelligence,
    Charisme,
}

impl Carac {
    pub fn get_abbr(&self) -> &str {
        return match self {
            Carac::Force => "for",
            Carac::Dexterite => "dex",
            Carac::Endurance => "end",
            Carac::Intelligence => "int",
            Carac::Charisme => "cha"
        }
    }
    
    pub fn from_abbr(abbr: &str) -> Option<Carac> {
        for carac in Carac::iter() {
            if carac.get_abbr() == abbr { return Some(carac); }
        }
        return None;
    }
}

impl ToString for Carac {
    fn to_string(&self) -> String {
        match self {
            Carac::Force => format!("[{}] Force", Carac::Force.get_abbr()),
            Carac::Dexterite => format!("[{}] Dextérité", Carac::Dexterite.get_abbr()),
            Carac::Endurance => format!("[{}] Endurance", Carac::Endurance.get_abbr()),
            Carac::Intelligence => format!("[{}] Intelligence", Carac::Intelligence.get_abbr()),
            Carac::Charisme => format!("[{}] Chance", Carac::Charisme.get_abbr()),
        }
    }
}

#[derive(Debug, PartialEq, EnumIter)]
pub enum Competence {
    Artisanat,
    CombatDistance,
    CombatRapproche,
    ConnaissanceNature,
    ConnaissanceSecret,
    CourirSauter,
    Discretion,
    Droit,
    Esquiver,
    Intimider,
    LireEcrire,
    MentirConvaincre,
    Perception,
    Piloter,
    Psychologie,
    Reflexes,
    SerrurePieges,
    Soigner,
    Survie,
    Voler,
}

impl Competence {
    pub fn associated_carac(&self) -> (Carac, Carac) {
        return match self {
            Competence::Artisanat => (Carac::Dexterite, Carac::Intelligence),
            Competence::CombatDistance => (Carac::Dexterite, Carac::Intelligence),
            Competence::CombatRapproche => (Carac::Force, Carac::Dexterite),
            Competence::ConnaissanceNature => (Carac::Dexterite, Carac::Intelligence),
            Competence::ConnaissanceSecret => (Carac::Intelligence, Carac::Charisme),
            Competence::CourirSauter => (Carac::Dexterite, Carac::Endurance),
            Competence::Discretion => (Carac::Dexterite, Carac::Charisme),
            Competence::Droit => (Carac::Intelligence, Carac::Charisme),
            Competence::Esquiver => (Carac::Dexterite, Carac::Intelligence),
            Competence::Intimider => (Carac::Force, Carac::Charisme),
            Competence::LireEcrire => (Carac::Intelligence, Carac::Charisme),
            Competence::MentirConvaincre => (Carac::Intelligence, Carac::Charisme),
            Competence::Perception => (Carac::Intelligence, Carac::Charisme),
            Competence::Piloter => (Carac::Dexterite, Carac::Endurance),
            Competence::Psychologie => (Carac::Endurance, Carac::Intelligence),
            Competence::Reflexes => (Carac::Dexterite, Carac::Intelligence),
            Competence::SerrurePieges => (Carac::Dexterite, Carac::Endurance),
            Competence::Soigner => (Carac::Intelligence, Carac::Charisme),
            Competence::Survie => (Carac::Endurance, Carac::Intelligence),
            Competence::Voler => (Carac::Dexterite, Carac::Intelligence),
        };
    }

    pub fn get_abbr(&self) -> &str {
        return match self {
            Competence::Artisanat => "ar",
            Competence::CombatDistance => "codi",
            Competence::CombatRapproche => "cora",
            Competence::ConnaissanceNature => "cona",
            Competence::ConnaissanceSecret => "cose",
            Competence::CourirSauter => "cosa",
            Competence::Discretion => "di",
            Competence::Droit => "dr",
            Competence::Esquiver => "es",
            Competence::Intimider => "in",
            Competence::LireEcrire => "liec",
            Competence::MentirConvaincre => "meco",
            Competence::Perception => "pe",
            Competence::Piloter => "pi",
            Competence::Psychologie => "ps",
            Competence::Reflexes => "re",
            Competence::SerrurePieges => "sepi",
            Competence::Soigner => "so",
            Competence::Survie => "su",
            Competence::Voler => "vo",
        }
    }

    pub fn from_abbr(abbr: &str) -> Option<Competence> {
        for competence in Competence::iter() {
            if competence.get_abbr() == abbr { return Some(competence); }
        }
        return None;
    }
}

impl ToString for Competence {
    fn to_string(&self) -> String {
        return match self {
            Competence::Artisanat => format!("[{}] Artisanat", Competence::Artisanat.get_abbr()),
            Competence::CombatDistance => format!("[{}] Combat à distance", Competence::CombatDistance.get_abbr()),
            Competence::ConnaissanceNature => format!("[{}] Connaissance de la nature", Competence::ConnaissanceNature.get_abbr()),
            Competence::CombatRapproche => format!("[{}] Combat rapproché", Competence::CombatRapproche.get_abbr()),
            Competence::CourirSauter => format!("[{}] Courir Sauter", Competence::CourirSauter.get_abbr()),
            Competence::ConnaissanceSecret => format!("[{}] Connaissance des secrets", Competence::ConnaissanceSecret.get_abbr()),
            Competence::Discretion => format!("[{}] Discrétion", Competence::Discretion.get_abbr()),
            Competence::Droit => format!("[{}] Droit", Competence::Droit.get_abbr()),
            Competence::Esquiver => format!("[{}] Esquiver", Competence::Esquiver.get_abbr()),
            Competence::Intimider => format!("[{}] Intimider", Competence::Intimider.get_abbr()),
            Competence::LireEcrire => format!("[{}] Lire Écrire", Competence::LireEcrire.get_abbr()),
            Competence::MentirConvaincre => format!("[{}] Mentir Convaincre", Competence::MentirConvaincre.get_abbr()),
            Competence::Perception => format!("[{}] Perception", Competence::Perception.get_abbr()),
            Competence::Piloter =>  format!("[{}] Piloter", Competence::Piloter.get_abbr()),
            Competence::Psychologie => format!("[{}] Psychologie", Competence::Psychologie.get_abbr()),
            Competence::Reflexes => format!("[{}] Réflexes", Competence::Reflexes.get_abbr()),
            Competence::SerrurePieges => format!("[{}] Serrures et Pièges", Competence::SerrurePieges.get_abbr()),
            Competence::Soigner => format!("[{}] Soigner", Competence::Soigner.get_abbr()),
            Competence::Survie => format!("[{}] Survie", Competence::Survie.get_abbr()),
            Competence::Voler => format!("[{}] Voler", Competence::Voler.get_abbr()),
        };
    }
}

#[test]
fn test_carac() {
    assert_eq!("for", Carac::Force.get_abbr());
    assert_eq!(Carac::Intelligence, Carac::from_abbr("int").unwrap());
}

#[test]
fn test_comp() {
    assert_eq!("cona", Competence::ConnaissanceNature.get_abbr());
    assert_eq!(Competence::MentirConvaincre, Competence::from_abbr("meco").unwrap());
}
