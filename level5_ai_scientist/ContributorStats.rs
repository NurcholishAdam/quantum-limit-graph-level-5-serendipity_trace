// -*- coding: utf-8 -*-
//! Language-Aware Contributor Statistics and Leaderboard
//! 
//! Extends contributor ranking with multilingual metrics,
//! cross-language expertise tracking, and language-aware scoring.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Language-aware contributor statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageAwareContributorStats {
    /// Contributor ID
    pub contributor_id: String,
    
    /// Total traces submitted
    pub total_traces: usize,
    
    /// Average trace depth
    pub avg_trace_depth: f64,
    
    /// Average uniqueness score
    pub avg_uniqueness: f64,
    
    /// Average serendipity score
    pub avg_serendipity: f64,
    
    /// Languages used
    pub languages_used: Vec<String>,
    
    /// Language proficiency scores
    pub language_proficiency: HashMap<String, f64>,
    
    /// Cross-language expertise (ability to work across languages)
    pub cross_language_expertise: f64,
    
    /// Multilingual trace count
    pub multilingual_traces: usize,
    
    /// Average alignment score
    pub avg_alignment_score: f64,
    
    /// Translation quality average
    pub avg_translation_quality: f64,
    
    /// Discoveries made
    pub discoveries: Vec<String>,
    
    /// Expertise domains
    pub expertise_domains: Vec<String>,
}

impl LanguageAwareContributorStats {
    /// Create new contributor stats
    pub fn new(contributor_id: &str) -> Self {
        Self {
            contributor_id: contributor_id.to_string(),
            total_traces: 0,
            avg_trace_depth: 0.0,
            avg_uniqueness: 0.0,
            avg_serendipity: 0.0,
            languages_used: Vec::new(),
            language_proficiency: HashMap::new(),
            cross_language_expertise: 0.0,
            multilingual_traces: 0,
            avg_alignment_score: 0.0,
            avg_translation_quality: 0.0,
            discoveries: Vec::new(),
            expertise_domains: Vec::new(),
        }
    }

    /// Add a trace to statistics
    pub fn add_trace(
        &mut self,
        depth: usize,
        uniqueness: f64,
        serendipity: f64,
        languages: Vec<String>,
        alignment_score: f64,
        translation_quality: f64,
    ) {
        // Update basic stats
        self.total_traces += 1;
        self.avg_trace_depth = (self.avg_trace_depth * (self.total_traces - 1) as f64 + depth as f64)
            / self.total_traces as f64;
        self.avg_uniqueness = (self.avg_uniqueness * (self.total_traces - 1) as f64 + uniqueness)
            / self.total_traces as f64;
        self.avg_serendipity = (self.avg_serendipity * (self.total_traces - 1) as f64 + serendipity)
            / self.total_traces as f64;
        
        // Update language stats
        if languages.len() > 1 {
            self.multilingual_traces += 1;
        }
        
        for lang in &languages {
            if !self.languages_used.contains(lang) {
                self.languages_used.push(lang.clone());
            }
            
            // Update language proficiency
            let current_prof = self.language_proficiency.get(lang).unwrap_or(&0.0);
            let new_prof = (current_prof + uniqueness) / 2.0;
            self.language_proficiency.insert(lang.clone(), new_prof);
        }
        
        // Update cross-language expertise
        self.cross_language_expertise = (self.languages_used.len() as f64).min(10.0) / 10.0
            * (self.multilingual_traces as f64 / self.total_traces as f64);
        
        // Update alignment and translation quality
        self.avg_alignment_score = (self.avg_alignment_score * (self.total_traces - 1) as f64 + alignment_score)
            / self.total_traces as f64;
        self.avg_translation_quality = (self.avg_translation_quality * (self.total_traces - 1) as f64 + translation_quality)
            / self.total_traces as f64;
    }

    /// Add a discovery
    pub fn add_discovery(&mut self, discovery_name: &str) {
        if !self.discoveries.contains(&discovery_name.to_string()) {
            self.discoveries.push(discovery_name.to_string());
        }
    }

    /// Add expertise domain
    pub fn add_expertise_domain(&mut self, domain: &str) {
        if !self.expertise_domains.contains(&domain.to_string()) {
            self.expertise_domains.push(domain.to_string());
        }
    }

    /// Calculate overall score
    pub fn overall_score(&self) -> f64 {
        let depth_score = (self.avg_trace_depth / 50.0).min(1.0);
        let uniqueness_score = self.avg_uniqueness;
        let serendipity_score = self.avg_serendipity;
        let language_score = self.cross_language_expertise;
        let quality_score = (self.avg_alignment_score + self.avg_translation_quality) / 2.0;
        let discovery_score = (self.discoveries.len() as f64 / 10.0).min(1.0);
        
        // Weighted combination
        0.20 * depth_score +
        0.25 * uniqueness_score +
        0.20 * serendipity_score +
        0.15 * language_score +
        0.10 * quality_score +
        0.10 * discovery_score
    }
}

/// Language-aware ranking criteria
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LanguageAwareRankingCriteria {
    /// Overall combined score
    Overall,
    /// Serendipity score
    Serendipity,
    /// Cross-language expertise
    CrossLanguageExpertise,
    /// Number of discoveries
    Discoveries,
    /// Translation quality
    TranslationQuality,
    /// Language diversity
    LanguageDiversity,
}

/// Language-aware leaderboard
#[derive(Debug, Clone)]
pub struct LanguageAwareLeaderboard {
    contributors: HashMap<String, LanguageAwareContributorStats>,
}

impl LanguageAwareLeaderboard {
    /// Create a new leaderboard
    pub fn new() -> Self {
        Self {
            contributors: HashMap::new(),
        }
    }

    /// Add or update contributor
    pub fn add_contributor(&mut self, stats: LanguageAwareContributorStats) {
        self.contributors.insert(stats.contributor_id.clone(), stats);
    }

    /// Get top N contributors by criteria
    pub fn get_top_n(
        &self,
        n: usize,
        criteria: LanguageAwareRankingCriteria,
    ) -> Vec<LanguageAwareContributorStats> {
        let mut contributors: Vec<_> = self.contributors.values().cloned().collect();
        
        contributors.sort_by(|a, b| {
            let score_a = self.get_score(a, criteria);
            let score_b = self.get_score(b, criteria);
            score_b.partial_cmp(&score_a).unwrap()
        });
        
        contributors.into_iter().take(n).collect()
    }

    /// Get score based on criteria
    fn get_score(&self, stats: &LanguageAwareContributorStats, criteria: LanguageAwareRankingCriteria) -> f64 {
        match criteria {
            LanguageAwareRankingCriteria::Overall => stats.overall_score(),
            LanguageAwareRankingCriteria::Serendipity => stats.avg_serendipity,
            LanguageAwareRankingCriteria::CrossLanguageExpertise => stats.cross_language_expertise,
            LanguageAwareRankingCriteria::Discoveries => stats.discoveries.len() as f64,
            LanguageAwareRankingCriteria::TranslationQuality => stats.avg_translation_quality,
            LanguageAwareRankingCriteria::LanguageDiversity => stats.languages_used.len() as f64,
        }
    }

    /// Display leaderboard
    pub fn display(&self, criteria: LanguageAwareRankingCriteria) {
        let top_contributors = self.get_top_n(10, criteria);
        
        println!("\n╔════════════════════════════════════════════════════════════════╗");
        println!("║     Language-Aware Serendipity Discovery Leaderboard          ║");
        println!("║     Ranking by: {:?}                                    ║", criteria);
        println!("╚════════════════════════════════════════════════════════════════╝\n");
        
        for (i, stats) in top_contributors.iter().enumerate() {
            let medal = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };
            
            println!("{} #{} {}", medal, i + 1, stats.contributor_id);
            println!("   Score: {:.3} | Traces: {} | Languages: {}",
                self.get_score(stats, criteria),
                stats.total_traces,
                stats.languages_used.join(", "));
            println!("   Serendipity: {:.3} | Cross-Lang: {:.3} | Discoveries: {}",
                stats.avg_serendipity,
                stats.cross_language_expertise,
                stats.discoveries.len());
            println!();
        }
    }
}

impl Default for LanguageAwareLeaderboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contributor_stats() {
        let mut stats = LanguageAwareContributorStats::new("researcher1");
        stats.add_trace(10, 0.8, 0.85, vec!["en".to_string(), "id".to_string()], 0.9, 0.88);
        
        assert_eq!(stats.total_traces, 1);
        assert_eq!(stats.multilingual_traces, 1);
        assert_eq!(stats.languages_used.len(), 2);
    }

    #[test]
    fn test_overall_score() {
        let mut stats = LanguageAwareContributorStats::new("researcher1");
        stats.add_trace(20, 0.85, 0.9, vec!["en".to_string(), "id".to_string()], 0.88, 0.9);
        stats.add_discovery("Journavx");
        
        let score = stats.overall_score();
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_leaderboard() {
        let mut leaderboard = LanguageAwareLeaderboard::new();
        
        let mut stats1 = LanguageAwareContributorStats::new("researcher1");
        stats1.add_trace(20, 0.85, 0.9, vec!["en".to_string(), "id".to_string()], 0.88, 0.9);
        
        let mut stats2 = LanguageAwareContributorStats::new("researcher2");
        stats2.add_trace(15, 0.75, 0.8, vec!["en".to_string()], 0.85, 0.82);
        
        leaderboard.add_contributor(stats1);
        leaderboard.add_contributor(stats2);
        
        let top = leaderboard.get_top_n(2, LanguageAwareRankingCriteria::Overall);
        assert_eq!(top.len(), 2);
    }
}
