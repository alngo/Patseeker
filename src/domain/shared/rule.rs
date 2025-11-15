#[cfg(test)]
use mockall::automock;

use super::error::DomainError;

#[allow(dead_code)]
pub trait Check {
    fn check_rule(rule: impl Rule) -> Result<(), DomainError> {
        if !rule.is_valid() {
            return Err(DomainError {
                message: rule.message(),
            });
        }
        Ok(())
    }
}

#[cfg_attr(test, automock)]
pub trait Rule {
    fn is_valid(&self) -> bool;
    fn message(&self) -> String;
}

#[cfg(test)]
mod domain_check_tests {
    use crate::domain::shared::rule::MockRule;

    use super::*;

    struct ConcreteEntity;

    impl Check for ConcreteEntity {}

    #[test]
    fn test_check_rule_is_valid() {
        let mut rule = MockRule::new();
        rule.expect_is_valid().times(1).returning(|| true);
        let result = ConcreteEntity::check_rule(rule);
        assert!(result.is_ok())
    }

    #[test]
    fn test_check_rule_is_invalid() {
        let mut rule = MockRule::new();
        rule.expect_is_valid().times(1).returning(|| false);
        rule.expect_message()
            .times(1)
            .returning(|| "An error message".to_string());
        let result = ConcreteEntity::check_rule(rule);
        assert!(result.is_err())
    }
}
