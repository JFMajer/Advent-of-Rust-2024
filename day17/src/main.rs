// Ensure all relevant items are marked with `pub` keyword

const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

// Your Solution here ...
pub trait Anonymize {
    fn anonymize_email(&self) -> String;
}

impl Anonymize for String {
    fn anonymize_email(&self) -> String {
        let mut to_return = String::new();

        // Basic email validation
        if let Some((local, domain)) = self.split_once('@') {
            if domain.contains('.') && !local.is_empty() && !domain.ends_with('.') {
                // Valid email
                for (i, _) in local.chars().enumerate() {
                    to_return.push(CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()]);
                }
                to_return.push('@');
                to_return.push_str(domain);
                return to_return;
            }
        }

        // Invalid email - replace all characters with emojis
        for (i, _) in self.chars().enumerate() {
            to_return.push(CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()]);
        }

        to_return
    }
}

pub fn main() {
    let emails = vec![
        "rudolph.therapysessions@northpole.com".to_string(),
        "elfhr.complaint@northpole.urgent".to_string(),
        "santas.rage.management@christmaschaos.noel".to_string(),
        "overtimepay.never@elfexploitation.workshop".to_string(),
        "mrs.claus.divorce.lawyer@northpole.legal".to_string(),
        "reindeer.workers.comp@antler.insurance".to_string(),
        "naughty.list.revenge@santasecrets.com".to_string(),
        "workshop.ptsd.support@elves.anonymous".to_string(),
        "performance.anxiety@santa.breakdown".to_string(),
        "existential.crisis@northpole.void".to_string(),
    ];

    for email in emails {
        let anonymized_email = email.anonymize_email(); // This is the API that Santa wants!
        println!("Original: {} -> Anonymized: {}", email, anonymized_email);
    }
}
