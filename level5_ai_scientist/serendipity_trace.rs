// -*- coding: utf-8 -*-
//! Serendipity Trace Module for SerenQA Framework Integration
//! 
//! This module logs each agent transition in the serendipity discovery process,
//! computes provenance hash for reproducibility, folds memory trace for leaderboard
//! integration, and prepares the trace for benchmarking and contributor crediting.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Serendipity discovery stage in the research process
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SerendipityStage {
    /// Initial exploration phase
    Exploration,
    /// Unexpected connection discovered
    UnexpectedConnection,
    /// Hypothesis formation from serendipitous finding
    HypothesisFormation,
    /// Validation of serendipitous discovery
    Validation,
    /// Integration into existing knowledge
    Integration,
    /// Publication/sharing of discovery
    Publication,
}

/// Agent type involved in serendipity discovery
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SerendipityAgent {
    /// Explores diverse information sources
    Explorer,
    /// Identifies unexpected patterns
    PatternRecognizer,
    /// Forms hypotheses from discoveries
    HypothesisGenerator,
    /// Validates serendipitous findings
    Validator,
    /// Synthesizes discoveries into knowledge
    Synthesizer,
    /// Translates across languages
    Translator,
    /// Meta-level orchestration
    MetaOrchestrator,
}

/// Serendipity event capturing a discovery moment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerendipityEvent {
    /// Unique event identifier
    pub event_id: String,
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    /// Discovery stage
    pub stage: SerendipityStage,
    /// Agent involved
    pub agent: SerendipityAgent,
    /// Input context
    pub input: String,
    /// Output/discovery
    pub output: String,
    /// Language of interaction
    pub language: String,
    /// Serendipity score (0.0-1.0, how unexpected)
    pub serendipity_score: f64,
    /// Confidence in the discovery
    pub confidence: f64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Transition between serendipity events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerendipityTransition {
    /// Source event ID
    pub from_event: String,
    /// Target event ID
    pub to_event: String,
    /// Source agent
    pub from_agent: SerendipityAgent,
    /// Target agent
    pub to_agent: SerendipityAgent,
    /// Transition score (quality of connection)
    pub transition_score: f64,
    /// Reason for transition
    pub reason: String,
    /// Language shift (if any)
    pub language_shift: Option<(String, String)>,
}

/// Complete serendipity trace for a discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerendipityTrace {
    /// Unique trace identifier
    pub trace_id: String,
    /// Contributor who made the discovery
    pub contributor_id: String,
    /// Backend/system used
    pub backend: String,
    /// Discovery name (e.g., "Journavx")
    pub discovery_name: String,
    /// All events in the trace
    pub events: Vec<SerendipityEvent>,
    /// All transitions between events
    pub transitions: Vec<SerendipityTransition>,
    /// Languages involved
    pub languages: Vec<String>,
    /// Overall serendipity score
    pub overall_serendipity: f64,
    /// Timestamp of trace creation
    pub created_at: DateTime<Utc>,
}

impl SerendipityTrace {
    /// Create a new serendipity trace
    pub fn new(
        contributor_id: &str,
        backend: &str,
        discovery_name: &str,
    ) -> Self {
        Self {
            trace_id: format!("seren_{}_{}", contributor_id, Utc::now().timestamp()),
            contributor_id: contributor_id.to_string(),
            backend: backend.to_string(),
            discovery_name: discovery_name.to_string(),
            events: Vec::new(),
            transitions: Vec::new(),
            languages: Vec::new(),
            overall_serendipity: 0.0,
            created_at: Utc::now(),
        }
    }

    /// Log a serendipity event
    pub fn log_event(
        &mut self,
        stage: SerendipityStage,
        agent: SerendipityAgent,
        input: &str,
        output: &str,
        language: &str,
        serendipity_score: f64,
        confidence: f64,
    ) {
        let event_id = format!("event_{}_{}", self.events.len(), Utc::now().timestamp_millis());
        
        // Track language if new
        if !self.languages.contains(&language.to_string()) {
            self.languages.push(language.to_string());
        }

        // Detect transition from previous event
        if let Some(prev_event) = self.events.last() {
            let language_shift = if prev_event.language != language {
                Some((prev_event.language.clone(), language.to_string()))
            } else {
                None
            };

            let transition = SerendipityTransition {
                from_event: prev_event.event_id.clone(),
                to_event: event_id.clone(),
                from_agent: prev_event.agent.clone(),
                to_agent: agent.clone(),
                transition_score: (prev_event.confidence + confidence) / 2.0,
                reason: format!("{:?} -> {:?}", prev_event.stage, stage),
                language_shift,
            };
            self.transitions.push(transition);
        }

        let event = SerendipityEvent {
            event_id: event_id.clone(),
            timestamp: Utc::now(),
            stage,
            agent,
            input: input.to_string(),
            output: output.to_string(),
            language: language.to_string(),
            serendipity_score,
            confidence,
            metadata: HashMap::new(),
        };

        self.events.push(event);
        self.update_overall_serendipity();
    }

    /// Update overall serendipity score
    fn update_overall_serendipity(&mut self) {
        if self.events.is_empty() {
            self.overall_serendipity = 0.0;
            return;
        }

        let sum: f64 = self.events.iter().map(|e| e.serendipity_score).sum();
        self.overall_serendipity = sum / self.events.len() as f64;
    }

    /// Compute provenance hash for reproducibility
    pub fn compute_provenance_hash(&self) -> String {
        let mut hasher = Sha256::new();
        
        // Hash trace metadata
        hasher.update(self.trace_id.as_bytes());
        hasher.update(self.contributor_id.as_bytes());
        hasher.update(self.backend.as_bytes());
        hasher.update(self.discovery_name.as_bytes());
        
        // Hash all events
        for event in &self.events {
            hasher.update(event.event_id.as_bytes());
            hasher.update(event.input.as_bytes());
            hasher.update(event.output.as_bytes());
            hasher.update(event.language.as_bytes());
            hasher.update(format!("{}", event.serendipity_score).as_bytes());
        }
        
        // Hash all transitions
        for transition in &self.transitions {
            hasher.update(transition.from_event.as_bytes());
            hasher.update(transition.to_event.as_bytes());
            hasher.update(format!("{}", transition.transition_score).as_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }

    /// Fold memory trace for leaderboard integration
    pub fn fold_memory(&self) -> FoldedSerendipityTrace {
        let key_discoveries: Vec<String> = self.events
            .iter()
            .filter(|e| e.serendipity_score > 0.7)
            .map(|e| format!("{:?}: {}", e.stage, e.output))
            .collect();

        let language_transitions: Vec<String> = self.transitions
            .iter()
            .filter_map(|t| {
                t.language_shift.as_ref().map(|(from, to)| {
                    format!("{} -> {}", from, to)
                })
            })
            .collect();

        let compression_ratio = if self.events.is_empty() {
            0.0
        } else {
            (key_discoveries.len() as f64) / (self.events.len() as f64)
        };

        FoldedSerendipityTrace {
            trace_id: self.trace_id.clone(),
            discovery_name: self.discovery_name.clone(),
            total_events: self.events.len(),
            key_discoveries,
            language_transitions,
            overall_serendipity: self.overall_serendipity,
            compression_ratio,
            languages: self.languages.clone(),
        }
    }

    /// Get trace depth (number of events)
    pub fn depth(&self) -> usize {
        self.events.len()
    }

    /// Get uniqueness score based on diversity
    pub fn uniqueness_score(&self) -> f64 {
        let agent_diversity = self.agent_diversity();
        let language_diversity = self.language_diversity();
        let stage_diversity = self.stage_diversity();
        
        // Weighted combination
        0.4 * agent_diversity + 0.3 * language_diversity + 0.3 * stage_diversity
    }

    /// Calculate agent diversity
    fn agent_diversity(&self) -> f64 {
        let unique_agents: std::collections::HashSet<_> = 
            self.events.iter().map(|e| format!("{:?}", e.agent)).collect();
        (unique_agents.len() as f64) / 7.0 // 7 possible agent types
    }

    /// Calculate language diversity
    fn language_diversity(&self) -> f64 {
        (self.languages.len() as f64).min(5.0) / 5.0 // Cap at 5 languages
    }

    /// Calculate stage diversity
    fn stage_diversity(&self) -> f64 {
        let unique_stages: std::collections::HashSet<_> = 
            self.events.iter().map(|e| format!("{:?}", e.stage)).collect();
        (unique_stages.len() as f64) / 6.0 // 6 possible stages
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Folded/compressed serendipity trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoldedSerendipityTrace {
    pub trace_id: String,
    pub discovery_name: String,
    pub total_events: usize,
    pub key_discoveries: Vec<String>,
    pub language_transitions: Vec<String>,
    pub overall_serendipity: f64,
    pub compression_ratio: f64,
    pub languages: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serendipity_trace_creation() {
        let trace = SerendipityTrace::new("researcher1", "quantum_backend", "Journavx");
        assert_eq!(trace.contributor_id, "researcher1");
        assert_eq!(trace.discovery_name, "Journavx");
        assert_eq!(trace.events.len(), 0);
    }

    #[test]
    fn test_log_event() {
        let mut trace = SerendipityTrace::new("researcher1", "backend", "Discovery");
        trace.log_event(
            SerendipityStage::Exploration,
            SerendipityAgent::Explorer,
            "Search for patterns",
            "Found unexpected connection",
            "en",
            0.85,
            0.9,
        );
        assert_eq!(trace.events.len(), 1);
        assert_eq!(trace.languages.len(), 1);
    }

    #[test]
    fn test_provenance_hash() {
        let mut trace = SerendipityTrace::new("researcher1", "backend", "Discovery");
        trace.log_event(
            SerendipityStage::Exploration,
            SerendipityAgent::Explorer,
            "input",
            "output",
            "en",
            0.8,
            0.9,
        );
        let hash = trace.compute_provenance_hash();
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
    }

    #[test]
    fn test_memory_folding() {
        let mut trace = SerendipityTrace::new("researcher1", "backend", "Discovery");
        trace.log_event(
            SerendipityStage::Exploration,
            SerendipityAgent::Explorer,
            "input1",
            "output1",
            "en",
            0.9,
            0.85,
        );
        trace.log_event(
            SerendipityStage::UnexpectedConnection,
            SerendipityAgent::PatternRecognizer,
            "input2",
            "output2",
            "id",
            0.95,
            0.9,
        );
        
        let folded = trace.fold_memory();
        assert_eq!(folded.total_events, 2);
        assert!(folded.compression_ratio > 0.0);
    }

    #[test]
    fn test_uniqueness_score() {
        let mut trace = SerendipityTrace::new("researcher1", "backend", "Discovery");
        trace.log_event(
            SerendipityStage::Exploration,
            SerendipityAgent::Explorer,
            "input",
            "output",
            "en",
            0.8,
            0.9,
        );
        let score = trace.uniqueness_score();
        assert!(score >= 0.0 && score <= 1.0);
    }
}
