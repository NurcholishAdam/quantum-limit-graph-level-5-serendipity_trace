// -*- coding: utf-8 -*-
//! Multilingual Memory Folding Extension
//! 
//! Extends MetaAgent memory folding with language awareness,
//! cross-language pattern detection, and multilingual insight extraction.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::AgentEvent::LanguageAwareAgentEvent;
use crate::alignment::{MultilingualAligner, AlignmentResult};

/// Multilingual memory fold with language-aware compression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualMemoryFold {
    /// Original trace ID
    pub trace_id: String,
    /// Total events before folding
    pub total_events: usize,
    /// Key insights extracted
    pub key_insights: Vec<String>,
    /// Language distribution
    pub language_distribution: HashMap<String, usize>,
    /// Cross-language patterns detected
    pub cross_language_patterns: Vec<CrossLanguagePattern>,
    /// Translation quality summary
    pub translation_summary: TranslationSummary,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Overall alignment score
    pub overall_alignment: f64,
}

/// Cross-language pattern detected in the trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossLanguagePattern {
    /// Pattern type
    pub pattern_type: String,
    /// Languages involved
    pub languages: Vec<String>,
    /// Pattern description
    pub description: String,
    /// Confidence in pattern
    pub confidence: f64,
}

/// Translation quality summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationSummary {
    /// Total translations performed
    pub total_translations: usize,
    /// Average translation quality
    pub average_quality: f64,
    /// Language pairs translated
    pub language_pairs: Vec<String>,
    /// Problematic translations (quality < 0.7)
    pub problematic_translations: usize,
}

/// Multilingual memory folder
pub struct MultilingualMemoryFolder {
    aligner: MultilingualAligner,
}

impl MultilingualMemoryFolder {
    /// Create a new multilingual memory folder
    pub fn new() -> Self {
        Self {
            aligner: MultilingualAligner::new(),
        }
    }

    /// Fold multilingual memory trace
    pub fn fold_memory(
        &mut self,
        trace_id: &str,
        events: &[LanguageAwareAgentEvent],
    ) -> MultilingualMemoryFold {
        let total_events = events.len();
        
        // Extract key insights (high-confidence, multilingual events)
        let key_insights = self.extract_key_insights(events);
        
        // Compute language distribution
        let language_distribution = self.compute_language_distribution(events);
        
        // Detect cross-language patterns
        let cross_language_patterns = self.detect_cross_language_patterns(events);
        
        // Compute translation summary
        let translation_summary = self.compute_translation_summary(events);
        
        // Calculate compression ratio
        let compression_ratio = (key_insights.len() as f64) / (total_events as f64);
        
        // Calculate overall alignment
        let overall_alignment = self.calculate_overall_alignment(events);
        
        MultilingualMemoryFold {
            trace_id: trace_id.to_string(),
            total_events,
            key_insights,
            language_distribution,
            cross_language_patterns,
            translation_summary,
            compression_ratio,
            overall_alignment,
        }
    }

    /// Extract key insights from events
    fn extract_key_insights(&self, events: &[LanguageAwareAgentEvent]) -> Vec<String> {
        events
            .iter()
            .filter(|e| e.confidence > 0.8 || e.is_multilingual())
            .map(|e| {
                if e.is_multilingual() {
                    format!(
                        "[Multilingual {}] {}: {} -> {}",
                        e.all_languages().join("+"),
                        e.agent_type,
                        e.input.chars().take(50).collect::<String>(),
                        e.output.chars().take(50).collect::<String>()
                    )
                } else {
                    format!(
                        "[{}] {}: {}",
                        e.primary_language,
                        e.agent_type,
                        e.output.chars().take(50).collect::<String>()
                    )
                }
            })
            .collect()
    }

    /// Compute language distribution
    fn compute_language_distribution(
        &self,
        events: &[LanguageAwareAgentEvent],
    ) -> HashMap<String, usize> {
        let mut distribution = HashMap::new();
        
        for event in events {
            for lang in event.all_languages() {
                *distribution.entry(lang).or_insert(0) += 1;
            }
        }
        
        distribution
    }

    /// Detect cross-language patterns
    fn detect_cross_language_patterns(
        &self,
        events: &[LanguageAwareAgentEvent],
    ) -> Vec<CrossLanguagePattern> {
        let mut patterns = Vec::new();
        
        // Pattern 1: Language switching
        for window in events.windows(2) {
            if window[0].primary_language != window[1].primary_language {
                patterns.push(CrossLanguagePattern {
                    pattern_type: "LanguageSwitch".to_string(),
                    languages: vec![
                        window[0].primary_language.clone(),
                        window[1].primary_language.clone(),
                    ],
                    description: format!(
                        "Switch from {} to {}",
                        window[0].primary_language,
                        window[1].primary_language
                    ),
                    confidence: (window[0].confidence + window[1].confidence) / 2.0,
                });
            }
        }
        
        // Pattern 2: Multilingual reasoning
        let multilingual_events: Vec<_> = events.iter().filter(|e| e.is_multilingual()).collect();
        if multilingual_events.len() > 2 {
            let languages: Vec<String> = multilingual_events
                .iter()
                .flat_map(|e| e.all_languages())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            
            patterns.push(CrossLanguagePattern {
                pattern_type: "MultilingualReasoning".to_string(),
                languages,
                description: format!(
                    "{} multilingual reasoning steps detected",
                    multilingual_events.len()
                ),
                confidence: multilingual_events.iter().map(|e| e.confidence).sum::<f64>()
                    / multilingual_events.len() as f64,
            });
        }
        
        patterns
    }

    /// Compute translation summary
    fn compute_translation_summary(
        &mut self,
        events: &[LanguageAwareAgentEvent],
    ) -> TranslationSummary {
        let mut total_translations = 0;
        let mut quality_sum = 0.0;
        let mut language_pairs = Vec::new();
        let mut problematic_translations = 0;
        
        for window in events.windows(2) {
            if window[0].primary_language != window[1].primary_language {
                total_translations += 1;
                
                // Compute alignment
                let alignment = self.aligner.align(
                    &window[0].output,
                    &window[1].input,
                    &window[0].primary_language,
                    &window[1].primary_language,
                );
                
                quality_sum += alignment.overall_score;
                
                let pair = format!("{}-{}", window[0].primary_language, window[1].primary_language);
                if !language_pairs.contains(&pair) {
                    language_pairs.push(pair);
                }
                
                if alignment.overall_score < 0.7 {
                    problematic_translations += 1;
                }
            }
        }
        
        let average_quality = if total_translations > 0 {
            quality_sum / total_translations as f64
        } else {
            1.0
        };
        
        TranslationSummary {
            total_translations,
            average_quality,
            language_pairs,
            problematic_translations,
        }
    }

    /// Calculate overall alignment score
    fn calculate_overall_alignment(&self, events: &[LanguageAwareAgentEvent]) -> f64 {
        let alignment_scores: Vec<f64> = events
            .iter()
            .filter_map(|e| e.alignment_score)
            .collect();
        
        if alignment_scores.is_empty() {
            1.0
        } else {
            alignment_scores.iter().sum::<f64>() / alignment_scores.len() as f64
        }
    }
}

impl Default for MultilingualMemoryFolder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multilingual_memory_folding() {
        let mut folder = MultilingualMemoryFolder::new();
        
        let event1 = LanguageAwareAgentEvent::new("Explorer", "input1", "output1", "en", 0.9);
        let mut event2 = LanguageAwareAgentEvent::new("Translator", "input2", "output2", "id", 0.85);
        event2.add_secondary_language("en");
        
        let events = vec![event1, event2];
        let fold = folder.fold_memory("trace1", &events);
        
        assert_eq!(fold.total_events, 2);
        assert!(fold.compression_ratio > 0.0);
    }

    #[test]
    fn test_language_distribution() {
        let folder = MultilingualMemoryFolder::new();
        
        let event1 = LanguageAwareAgentEvent::new("Explorer", "input1", "output1", "en", 0.9);
        let mut event2 = LanguageAwareAgentEvent::new("Translator", "input2", "output2", "id", 0.85);
        event2.add_secondary_language("en");
        
        let events = vec![event1, event2];
        let dist = folder.compute_language_distribution(&events);
        
        assert!(dist.contains_key("en"));
        assert!(dist.contains_key("id"));
    }

    #[test]
    fn test_cross_language_patterns() {
        let folder = MultilingualMemoryFolder::new();
        
        let event1 = LanguageAwareAgentEvent::new("Explorer", "input1", "output1", "en", 0.9);
        let event2 = LanguageAwareAgentEvent::new("Translator", "input2", "output2", "id", 0.85);
        
        let events = vec![event1, event2];
        let patterns = folder.detect_cross_language_patterns(&events);
        
        assert!(!patterns.is_empty());
    }
}
