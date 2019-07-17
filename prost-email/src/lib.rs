extern crate rand;

pub mod email {
    include!(concat!(env!("OUT_DIR"), "/email.rs"));
}

pub use email::Email;

use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand_core::RngCore;

extern crate lipsum;
use lipsum::{MarkovChain, LOREM_IPSUM};

pub fn lipsum_with_rng<'a, R: RngCore>(rng: R) -> MarkovChain<'a, R> {
    let mut chain = MarkovChain::new_with_rng(rng);
    chain.learn(LOREM_IPSUM);
    chain
}

impl Distribution<Email> for Standard {
    fn sample<R: RngCore + ?Sized>(&self, rng: &mut R) -> Email {
        let mut email = Email::default();

        let mut participants = ["alpha@gmail.com", "bravo@gmail.com", "charlie@gmail.com"];
        participants.shuffle(rng);
        email.sender = String::from(participants[0]);
        email.recipients_to = participants[1..]
            .into_iter()
            .map(|s| String::from(*s))
            .collect();

        let mut lipsum = lipsum_with_rng(rng);
        let words: Vec<_> = lipsum.iter().take(7).collect();
        email.subject = words.join(" ");

        email
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::distributions::{Distribution, Standard};
    use rand_core::SeedableRng;
    use rand_xorshift::XorShiftRng;

    #[test]
    fn test_email_distribution() {
        let mut rng = XorShiftRng::seed_from_u64(0);
        let actual_email: Email = Standard.sample_iter(&mut rng).next().unwrap();
        let expected_email = Email {
            sender: "bravo@gmail.com".to_owned(),
            subject: "minim veniam, quis nostrud exercitation ullamco laboris".to_owned(),
            recipients_to: vec!["alpha@gmail.com".to_owned(), "charlie@gmail.com".to_owned()],
        };
        assert_eq!(actual_email, expected_email);
    }
}
