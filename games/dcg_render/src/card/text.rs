use crate::error::ParseError;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum TextEscapeType {
    Action,
    Arcana,
    FreeAction,
    Health,
    Intellect,
    NewLine,
    Sanity,
    Speed,
    Strength,
    TriggeredAction,
    Willpower,
}

impl FromStr for TextEscapeType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Action" => Ok(TextEscapeType::Action),
            "Arcana" => Ok(TextEscapeType::Arcana),
            "Health" => Ok(TextEscapeType::Health),
            "FreeAction" => Ok(TextEscapeType::FreeAction),
            "Intellect" => Ok(TextEscapeType::Intellect),
            "NewLine" => Ok(TextEscapeType::NewLine),
            "Sanity" => Ok(TextEscapeType::Sanity),
            "Speed" => Ok(TextEscapeType::Speed),
            "Strength" => Ok(TextEscapeType::Strength),
            "TriggeredAction" => Ok(TextEscapeType::TriggeredAction),
            "Willpower" => Ok(TextEscapeType::Willpower),
            _ => Err(ParseError::new(format!(
                "Could not parse TextEscapeType from: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CardTextComponent {
    Text(String),
    Icon(TextEscapeType),
}

pub struct CardText {
    pub contents: String,
}

impl CardText {
    pub fn new(contents: &str) -> Self {
        let contents = contents.to_string();
        CardText { contents }
    }

    pub fn get_components(&self) -> Result<Vec<CardTextComponent>, ParseError> {
        self.validate_braces()?;

        let mut components = Vec::new();
        let mut iterator = self.contents.split("{");

        // Skip empty first split
        // (e.g. splitting "{keyword}" gives ["", "keyword}"]
        if let Some(text) = iterator.next() {
            if !text.is_empty() {
                components.push(CardTextComponent::Text(text.to_string()));
            }
        }

        for first_split in iterator {
            let mut second_split = first_split.split("}");
            if let Some(keyword) = second_split.next() {
                components.push(CardTextComponent::Icon(TextEscapeType::from_str(&keyword)?));
            } else {
                return Err(ParseError::new(format!(
                    "Could not parse keyword from: {}",
                    first_split
                )));
            }

            for remaining_text in second_split {
                if !remaining_text.is_empty() {
                    components.push(CardTextComponent::Text(remaining_text.to_string()));
                }
            }
        }

        Ok(components)
    }

    fn validate_braces(&self) -> Result<(), ParseError> {
        let mut on_open_brace = false;

        for c in self.contents.chars() {
            match c {
                '{' => {
                    if on_open_brace {
                        return Err(ParseError::new(format!(
                            "Extra opening brace(s) in CardText: {}",
                            self.contents
                        )));
                    } else {
                        on_open_brace = true;
                    }
                }
                '}' => {
                    if on_open_brace {
                        on_open_brace = false;
                    } else {
                        return Err(ParseError::new(format!(
                            "Extra closing brace(s) in CardText: {}",
                            self.contents
                        )));
                    }
                }
                _ => continue,
            }
        }

        if on_open_brace {
            Err(ParseError::new(format!(
                "Extra opening brace(s) in CardText: {}",
                self.contents
            )))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_text_icon_type_succeeds_from_valid_strings() {
        assert_eq!(
            TextEscapeType::from_str(&"Action"),
            Ok(TextEscapeType::Action)
        );
        assert_eq!(
            TextEscapeType::from_str(&"Arcana"),
            Ok(TextEscapeType::Arcana)
        );
        assert_eq!(
            TextEscapeType::from_str(&"FreeAction"),
            Ok(TextEscapeType::FreeAction)
        );
        assert_eq!(
            TextEscapeType::from_str(&"Health"),
            Ok(TextEscapeType::Health)
        );
        assert_eq!(
            TextEscapeType::from_str(&"Intellect"),
            Ok(TextEscapeType::Intellect)
        );
        assert_eq!(
            TextEscapeType::from_str(&"NewLine"),
            Ok(TextEscapeType::NewLine)
        );
        assert_eq!(
            TextEscapeType::from_str(&"Sanity"),
            Ok(TextEscapeType::Sanity)
        );
        assert_eq!(
            TextEscapeType::from_str(&"Speed"),
            Ok(TextEscapeType::Speed)
        );
        assert_eq!(
            TextEscapeType::from_str(&"Strength"),
            Ok(TextEscapeType::Strength)
        );
        assert_eq!(
            TextEscapeType::from_str(&"TriggeredAction"),
            Ok(TextEscapeType::TriggeredAction)
        );
        assert_eq!(
            TextEscapeType::from_str(&"Willpower"),
            Ok(TextEscapeType::Willpower)
        );
    }

    #[test]
    fn card_text_icon_type_fails_from_invalid_strings() {
        assert_eq!(
            TextEscapeType::from_str(&"Invalid"),
            Err(ParseError::new(
                "Could not parse TextEscapeType from: Invalid".to_string()
            )),
        );
    }

    #[test]
    fn text_is_valid_if_all_braces_closed_single() {
        let card_text = CardText::new("{closed_on_both_sides}");
        assert_eq!(card_text.validate_braces(), Ok(()));
    }

    #[test]
    fn text_is_valid_if_all_braces_closed_multiple() {
        let card_text = CardText::new("{1} {2} {3}");
        assert_eq!(card_text.validate_braces(), Ok(()));
    }

    #[test]
    fn text_is_invalid_if_extra_closing_brace() {
        let raw_text = "extra_closing_brace}";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.validate_braces(),
            Err(ParseError::new(format!(
                "Extra closing brace(s) in CardText: {}",
                &raw_text
            ))),
        );
    }

    #[test]
    fn text_is_invalid_if_multiple_repeated_opening_braces() {
        let raw_text = "{{multiple_repeated}}";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.validate_braces(),
            Err(ParseError::new(format!(
                "Extra opening brace(s) in CardText: {}",
                &raw_text
            ))),
        );
    }

    #[test]
    fn text_is_invalid_if_multiple_repeated_closing_braces() {
        let raw_text = "multiple_repeated}}";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.validate_braces(),
            Err(ParseError::new(format!(
                "Extra closing brace(s) in CardText: {}",
                &raw_text
            ))),
        );
    }

    #[test]
    fn text_is_invalid_if_too_many_closing_braces() {
        let raw_text = "{too_many_closing_braces}}";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.validate_braces(),
            Err(ParseError::new(format!(
                "Extra closing brace(s) in CardText: {}",
                &raw_text
            ))),
        );
    }

    #[test]
    fn text_is_invalid_if_too_many_opening_braces() {
        let raw_text = "{too_many_opening_braces}{";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.validate_braces(),
            Err(ParseError::new(format!(
                "Extra opening brace(s) in CardText: {}",
                &raw_text
            ))),
        );
    }

    #[test]
    fn text_does_not_validate_on_parentheses_or_brackets() {
        let raw_text = "(parentheses_and_brackets]";
        let card_text = CardText::new(&raw_text);
        assert_eq!(card_text.validate_braces(), Ok(()));
    }

    #[test]
    fn get_components_for_card_text_icon_keywords() {
        let raw_text = "{Arcana}{Speed}{Strength}{Willpower}";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.get_components(),
            Ok(vec![
                CardTextComponent::Icon(TextEscapeType::Arcana),
                CardTextComponent::Icon(TextEscapeType::Speed),
                CardTextComponent::Icon(TextEscapeType::Strength),
                CardTextComponent::Icon(TextEscapeType::Willpower),
            ]),
        );
    }

    #[test]
    fn get_components_for_card_text_icon_keywords_with_normal_text_at_beginning() {
        let raw_text = "Test (3) {Willpower}";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.get_components(),
            Ok(vec![
                CardTextComponent::Text("Test (3) ".to_string()),
                CardTextComponent::Icon(TextEscapeType::Willpower),
            ]),
        );
    }

    #[test]
    fn get_components_for_card_text_icon_keywords_with_normal_text_at_end() {
        let raw_text = "{Willpower} increases by 1";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.get_components(),
            Ok(vec![
                CardTextComponent::Icon(TextEscapeType::Willpower),
                CardTextComponent::Text(" increases by 1".to_string()),
            ]),
        );
    }

    #[test]
    fn get_components_for_card_text_icon_keywords_with_normal_text_both_sides() {
        let raw_text = "You have +1 {Willpower}. {FreeAction} Take a Move action.";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.get_components(),
            Ok(vec![
                CardTextComponent::Text("You have +1 ".to_string()),
                CardTextComponent::Icon(TextEscapeType::Willpower),
                CardTextComponent::Text(". ".to_string()),
                CardTextComponent::Icon(TextEscapeType::FreeAction),
                CardTextComponent::Text(" Take a Move action.".to_string()),
            ]),
        );
    }

    #[test]
    fn get_components_also_validates_braces() {
        let raw_text = "{{Action}";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.get_components(),
            Err(ParseError::new(format!(
                "Extra opening brace(s) in CardText: {}",
                &raw_text
            ))),
        );
    }

    #[test]
    fn get_components_is_invalid_for_non_keywords() {
        let raw_text = "{NotAKeyword}";
        let card_text = CardText::new(&raw_text);
        assert_eq!(
            card_text.get_components(),
            Err(ParseError::new(
                "Could not parse TextEscapeType from: NotAKeyword".to_string()
            )),
        );
    }
}
