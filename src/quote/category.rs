use serde::Deserialize;

#[derive(Clone, Copy, Deserialize)]
pub enum Category {
    Wisdom,
    Philosophy,
    Life,
    Truth,
    Inspirational,
    Relationships,
    Love,
    Faith,
    Humor,
    Success,
    Courage,
    Happiness,
    Art,
    Writing,
    Fear,
    Nature,
    Time,
    Freedom,
    Death,
    Leadership,
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Category::Wisdom => "wisdom".to_string(),
            Category::Philosophy => "philosophy".to_string(),
            Category::Life => "life".to_string(),
            Category::Truth => "truth".to_string(),
            Category::Inspirational => "inspirational".to_string(),
            Category::Relationships => "relationships".to_string(),
            Category::Love => "love".to_string(),
            Category::Faith => "faith".to_string(),
            Category::Humor => "humor".to_string(),
            Category::Success => "success".to_string(),
            Category::Courage => "courage".to_string(),
            Category::Art => "art".to_string(),
            Category::Writing => "writing".to_string(),
            Category::Fear => "fear".to_string(),
            Category::Happiness => "happiness".to_string(),
            Category::Leadership => "leadership".to_string(),
            Category::Nature => "nature".to_string(),
            Category::Time => "time".to_string(),
            Category::Freedom => "freedom".to_string(),
            Category::Death => "death".to_string(),
        }
    }
}

impl std::str::FromStr for Category {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = match s {
            "wisdom" => Category::Wisdom,
            "philosophy" => Category::Philosophy,
            "life" => Category::Life,
            "truth" => Category::Truth,
            "inspirational" => Category::Inspirational,
            "relationships" => Category::Relationships,
            "love" => Category::Love,
            "faith" => Category::Faith,
            "humor" => Category::Humor,
            "success" => Category::Success,
            "courage" => Category::Courage,
            "art" => Category::Art,
            "writing" => Category::Writing,
            "fear" => Category::Fear,
            "happiness" => Category::Happiness,
            "leadership" => Category::Leadership,
            "nature" => Category::Nature,
            "time" => Category::Time,
            "freedom" => Category::Freedom,
            "death" => Category::Death,
            _ => return Err("Unknown category!".to_string())
        };

        Ok(c)
    }
}
