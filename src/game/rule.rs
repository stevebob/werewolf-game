use ecs::entity::EntityTable;
use ecs::update::UpdateSummary;
use ecs::update_monad::Action;

pub enum RuleResult {
    After(Vec<Action>),
    Instead(Vec<Action>),
}

pub fn pass() -> RuleResult { RuleResult::After(vec![]) }
pub fn fail() -> RuleResult { RuleResult::Instead(vec![]) }

pub trait Rule {
    fn check(&self,
             summary: &UpdateSummary,
             before: &EntityTable,
             after: &EntityTable)
        -> RuleResult;
}

impl<F: Fn(&UpdateSummary, &EntityTable, &EntityTable) -> RuleResult> Rule for F {
    fn check(&self,
             summary: &UpdateSummary,
             before: &EntityTable,
             after: &EntityTable)
        -> RuleResult
    {
        self(summary, before, after)
    }
}
