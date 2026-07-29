use std::{collections::BTreeMap, fmt::Write as _};

use super::{
    AuthorityObservation, AuthorityObservationId, DomainError, ErrorCode, RevisionRef,
    SynchronizationState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceKind {
    Markdown,
    Json,
    GithubIssue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceIdentity {
    provider: String,
    resource_kind: String,
    opaque_id: String,
    navigation: Option<String>,
}

impl SourceIdentity {
    pub fn new(
        provider: impl Into<String>,
        resource_kind: impl Into<String>,
        opaque_id: impl Into<String>,
        navigation: Option<String>,
    ) -> Result<Self, DomainError> {
        let provider = provider.into();
        let resource_kind = resource_kind.into();
        let opaque_id = opaque_id.into();
        if provider.trim().is_empty()
            || resource_kind.trim().is_empty()
            || opaque_id.trim().is_empty()
        {
            return Err(DomainError::new(
                ErrorCode::InvalidObservation,
                "source.identity",
                "source identity requires provider, resource kind, and opaque identifier",
                false,
                "supply a complete external source identity",
            ));
        }
        Ok(Self {
            provider,
            resource_kind,
            opaque_id,
            navigation,
        })
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn resource_kind(&self) -> &str {
        &self.resource_kind
    }

    pub fn opaque_id(&self) -> &str {
        &self.opaque_id
    }

    pub fn navigation(&self) -> Option<&str> {
        self.navigation.as_deref()
    }

    fn authority_source(&self) -> String {
        format!(
            "{}:{}:{}",
            self.provider, self.resource_kind, self.opaque_id
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkFacts {
    title: String,
    goal: String,
    owner: String,
    work_class: String,
    lifecycle_stage: String,
    accepted_base: String,
    scope: Vec<String>,
    non_goals: Vec<String>,
    validation: Vec<String>,
    stop_conditions: Vec<String>,
    exit_gate: String,
    next_transition: String,
    supplemental: BTreeMap<String, Vec<String>>,
}

impl WorkFacts {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: impl Into<String>,
        goal: impl Into<String>,
        owner: impl Into<String>,
        work_class: impl Into<String>,
        lifecycle_stage: impl Into<String>,
        accepted_base: impl Into<String>,
        scope: Vec<String>,
        non_goals: Vec<String>,
        validation: Vec<String>,
        stop_conditions: Vec<String>,
        exit_gate: impl Into<String>,
        next_transition: impl Into<String>,
        supplemental: BTreeMap<String, Vec<String>>,
    ) -> Result<Self, DomainError> {
        let value = Self {
            title: title.into(),
            goal: goal.into(),
            owner: owner.into(),
            work_class: work_class.into(),
            lifecycle_stage: lifecycle_stage.into(),
            accepted_base: accepted_base.into(),
            scope,
            non_goals,
            validation,
            stop_conditions,
            exit_gate: exit_gate.into(),
            next_transition: next_transition.into(),
            supplemental,
        };
        if value.title.trim().is_empty()
            || value.goal.trim().is_empty()
            || value.owner.trim().is_empty()
            || value.work_class.trim().is_empty()
            || value.lifecycle_stage.trim().is_empty()
            || value.accepted_base.trim().is_empty()
            || value.scope.is_empty()
            || value.non_goals.is_empty()
            || value.validation.is_empty()
            || value.stop_conditions.is_empty()
            || value.exit_gate.trim().is_empty()
            || value.next_transition.trim().is_empty()
        {
            return Err(DomainError::new(
                ErrorCode::InvalidSource,
                "work.normalize",
                "work source is missing one or more material contract values",
                false,
                "add the missing accepted work-contract values to the authoritative source",
            ));
        }
        Ok(value)
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn goal(&self) -> &str {
        &self.goal
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn work_class(&self) -> &str {
        &self.work_class
    }

    pub fn lifecycle_stage(&self) -> &str {
        &self.lifecycle_stage
    }

    pub fn accepted_base(&self) -> &str {
        &self.accepted_base
    }

    pub fn scope(&self) -> &[String] {
        &self.scope
    }

    pub fn non_goals(&self) -> &[String] {
        &self.non_goals
    }

    pub fn validation(&self) -> &[String] {
        &self.validation
    }

    pub fn stop_conditions(&self) -> &[String] {
        &self.stop_conditions
    }

    pub fn exit_gate(&self) -> &str {
        &self.exit_gate
    }

    pub fn next_transition(&self) -> &str {
        &self.next_transition
    }

    pub fn supplemental(&self) -> &BTreeMap<String, Vec<String>> {
        &self.supplemental
    }

    fn authority_facts(&self) -> String {
        let mut output = String::new();
        append_fact(&mut output, "title", &self.title);
        append_fact(&mut output, "goal", &self.goal);
        append_fact(&mut output, "owner", &self.owner);
        append_fact(&mut output, "work_class", &self.work_class);
        append_fact(&mut output, "lifecycle_stage", &self.lifecycle_stage);
        append_fact(&mut output, "accepted_base", &self.accepted_base);
        append_values(&mut output, "scope", &self.scope);
        append_values(&mut output, "non_goals", &self.non_goals);
        append_values(&mut output, "validation", &self.validation);
        append_values(&mut output, "stop_conditions", &self.stop_conditions);
        append_fact(&mut output, "exit_gate", &self.exit_gate);
        append_fact(&mut output, "next_transition", &self.next_transition);
        for (section, values) in &self.supplemental {
            for value in values {
                append_fact(&mut output, &format!("supplemental.{section}"), value);
            }
        }
        output
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkSourceObservation {
    authority: AuthorityObservation,
    identity: SourceIdentity,
    kind: SourceKind,
    observed_at_unix_seconds: u64,
    source_mutable: bool,
    facts: WorkFacts,
    limitations: Vec<String>,
    raw_size: usize,
}

impl WorkSourceObservation {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        identity: SourceIdentity,
        kind: SourceKind,
        digest: impl Into<String>,
        observed_at_unix_seconds: u64,
        source_mutable: bool,
        facts: WorkFacts,
        limitations: Vec<String>,
        raw_size: usize,
    ) -> Result<Self, DomainError> {
        let digest = digest.into();
        if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(DomainError::new(
                ErrorCode::InvalidObservation,
                "source.observe",
                "source observation requires a SHA-256 digest",
                false,
                "calculate the digest over the exact bounded source bytes",
            ));
        }
        if raw_size == 0 {
            return Err(DomainError::new(
                ErrorCode::InvalidObservation,
                "source.observe",
                "source observation requires a non-empty exact payload",
                false,
                "retain the bounded authoritative source bytes",
            ));
        }
        let revision = RevisionRef {
            kind: "sha256".into(),
            value: digest,
            immutable: true,
        };
        let authority = AuthorityObservation::new(
            AuthorityObservationId::new(),
            identity.authority_source(),
            revision,
            facts.authority_facts(),
        )?;
        Ok(Self {
            authority,
            identity,
            kind,
            observed_at_unix_seconds,
            source_mutable,
            facts,
            limitations,
            raw_size,
        })
    }

    pub fn authority(&self) -> &AuthorityObservation {
        &self.authority
    }

    pub fn revision(&self) -> &RevisionRef {
        self.authority.revision()
    }

    pub fn identity(&self) -> &SourceIdentity {
        &self.identity
    }

    pub fn kind(&self) -> SourceKind {
        self.kind
    }

    pub fn digest(&self) -> &str {
        &self.authority.revision().value
    }

    pub fn observed_at_unix_seconds(&self) -> u64 {
        self.observed_at_unix_seconds
    }

    pub fn is_mutable(&self) -> bool {
        self.source_mutable
    }

    pub fn facts(&self) -> &WorkFacts {
        &self.facts
    }

    pub fn limitations(&self) -> &[String] {
        &self.limitations
    }

    pub fn raw_size(&self) -> usize {
        self.raw_size
    }

    pub fn synchronization_against(&self, prior: Option<&RevisionRef>) -> SynchronizationState {
        match prior {
            None => SynchronizationState::Current,
            Some(prior) if prior == self.revision() => SynchronizationState::Current,
            Some(prior) if prior.kind == self.revision().kind => SynchronizationState::Stale,
            Some(_) => SynchronizationState::Conflict,
        }
    }
}

fn append_values(output: &mut String, name: &str, values: &[String]) {
    for value in values {
        append_fact(output, name, value);
    }
}

fn append_fact(output: &mut String, name: &str, value: &str) {
    let _ = writeln!(output, "{name}:{}:{value}", value.len());
}
