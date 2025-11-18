// -*- coding: utf-8 -*-
//! Integration tests for SerenQA Framework

use level5_ai_scientist::serendipity_trace::*;
use level5_ai_scientist::AgentEvent::*;
use level5_ai_scientist::alignment::*;
use level5_ai_scientist::fold_multilingual_memory::*;
use level5_ai_scientist::ContributorStats::*;
use level5_ai_scientist::Journavx_Discovery::*;

#[test]
fn test_complete_serendipity_workflow() {
    // Create trace
    let mut trace = SerendipityTrace::new("researcher1", "backend", "TestDiscovery");
    
    // Log events
    trace.log_event(
        SerendipityStage::Exploration,
        SerendipityAgent::Explorer,
        "Search",
        "Found pattern",
        "en",
        0.7,
        0.85,
    );
    
    trace.log_event(
        SerendipityStage::UnexpectedConnection,
        SerendipityAgent::PatternRecognizer,
        "Analyze",
        "Unexpected link",
        "id",
        0.92,
        0.88,
    );
    
    // Verify trace
    assert_eq!(trace.events.len(), 2);
    assert_eq!(trace.transitions.len(), 1);
    assert_eq!(trace.languages.len(), 2);
    
    // Compute provenance
    let hash = trace.compute_provenance_hash();
    assert_eq!(hash.len(), 64);
    
    // Fold memory
    let folded = trace.fold_memory();
    assert!(folded.compression_ratio > 0.0);
    assert_eq!(folded.total_events, 2);
}

#[test]
fn test_language_aware_events() {
    let mut event = LanguageAwareAgentEvent::new(
        "Translator",
        "Hello",
        "Halo",
        "en",
        0.9,
    );
    
    event.add_secondary_language("id");
    event.set_alignment_score(0.88);
    event.set_translation_quality(0.90);
    
    assert!(event.is_multilingual());
    assert_eq!(event.all_languages().len(), 2);
    assert!(event.alignment_score.is_some());
    assert!(event.language_quality_score() > 0.0);
}

#[test]
fn test_multilingual_alignment() {
    let mut aligner = MultilingualAligner::new();
    
    let result = aligner.align(
        "Hello world",
        "Halo dunia",
        "en",
        "id",
    );
    
    assert!(result.overall_score > 0.0 && result.overall_score <= 1.0);
    assert!(result.semantic_score > 0.0);
    assert!(result.structural_score > 0.0);
    assert!(result.cultural_score > 0.0);
    
    // Check history
    let avg = aligner.get_average_alignment("en", "id");
    assert!(avg.is_some());
}

#[test]
fn test_multilingual_memory_folding() {
    let mut folder = MultilingualMemoryFolder::new();
    
    let event1 = LanguageAwareAgentEvent::new("Explorer", "input1", "output1", "en", 0.9);
    let mut event2 = LanguageAwareAgentEvent::new("Translator", "input2", "output2", "id", 0.85);
    event2.add_secondary_language("en");
    event2.set_alignment_score(0.88);
    
    let events = vec![event1, event2];
    let fold = folder.fold_memory("trace1", &events);
    
    assert_eq!(fold.total_events, 2);
    assert!(fold.compression_ratio > 0.0);
    assert!(fold.overall_alignment > 0.0);
    assert!(!fold.cross_language_patterns.is_empty());
}

#[test]
fn test_contributor_statistics() {
    let mut stats = LanguageAwareContributorStats::new("researcher1");
    
    stats.add_trace(
        20,
        0.85,
        0.9,
        vec!["en".to_string(), "id".to_string()],
        0.88,
        0.90,
    );
    
    stats.add_discovery("TestDiscovery");
    stats.add_expertise_domain("Quantum Computing");
    
    assert_eq!(stats.total_traces, 1);
    assert_eq!(stats.multilingual_traces, 1);
    assert_eq!(stats.languages_used.len(), 2);
    assert_eq!(stats.discoveries.len(), 1);
    assert!(stats.overall_score() > 0.0);
}

#[test]
fn test_language_aware_leaderboard() {
    let mut leaderboard = LanguageAwareLeaderboard::new();
    
    let mut stats1 = LanguageAwareContributorStats::new("researcher1");
    stats1.add_trace(20, 0.85, 0.9, vec!["en".to_string(), "id".to_string()], 0.88, 0.9);
    stats1.add_discovery("Discovery1");
    
    let mut stats2 = LanguageAwareContributorStats::new("researcher2");
    stats2.add_trace(15, 0.75, 0.8, vec!["en".to_string()], 0.85, 0.82);
    
    leaderboard.add_contributor(stats1);
    leaderboard.add_contributor(stats2);
    
    let top = leaderboard.get_top_n(2, LanguageAwareRankingCriteria::Overall);
    assert_eq!(top.len(), 2);
    
    // First should have higher score (multilingual + discovery)
    assert!(top[0].overall_score() > top[1].overall_score());
}

#[test]
fn test_journavx_discovery_simulation() {
    let trace = simulate_journavx_discovery();
    
    assert_eq!(trace.discovery_name, "Journavx");
    assert!(trace.events.len() >= 9);
    assert!(trace.languages.contains(&"en".to_string()));
    assert!(trace.languages.contains(&"id".to_string()));
    assert!(trace.overall_serendipity > 0.8);
    
    let hash = trace.compute_provenance_hash();
    assert_eq!(hash.len(), 64);
    
    let folded = trace.fold_memory();
    assert!(folded.compression_ratio > 0.0);
}

#[test]
fn test_cross_language_patterns() {
    let mut folder = MultilingualMemoryFolder::new();
    
    let event1 = LanguageAwareAgentEvent::new("Explorer", "input1", "output1", "en", 0.9);
    let event2 = LanguageAwareAgentEvent::new("Translator", "input2", "output2", "id", 0.85);
    let mut event3 = LanguageAwareAgentEvent::new("Synthesizer", "input3", "output3", "en", 0.88);
    event3.add_secondary_language("id");
    
    let events = vec![event1, event2, event3];
    let fold = folder.fold_memory("trace1", &events);
    
    assert!(!fold.cross_language_patterns.is_empty());
    
    // Should detect language switch pattern
    let has_switch = fold.cross_language_patterns.iter()
        .any(|p| p.pattern_type == "LanguageSwitch");
    assert!(has_switch);
}

#[test]
fn test_translation_quality_tracking() {
    let mut folder = MultilingualMemoryFolder::new();
    
    let event1 = LanguageAwareAgentEvent::new("Explorer", "Hello", "output1", "en", 0.9);
    let event2 = LanguageAwareAgentEvent::new("Translator", "Halo", "output2", "id", 0.85);
    
    let events = vec![event1, event2];
    let fold = folder.fold_memory("trace1", &events);
    
    assert!(fold.translation_summary.total_translations > 0);
    assert!(fold.translation_summary.average_quality > 0.0);
    assert!(!fold.translation_summary.language_pairs.is_empty());
}

#[test]
fn test_event_builder() {
    let event = LanguageAwareEventBuilder::new("Translator", "Hello", "Halo", "en")
        .confidence(0.9)
        .add_language("id")
        .alignment_score(0.88)
        .translation_quality(0.90)
        .semantic_similarity(0.85)
        .cultural_context(0.87)
        .build();
    
    assert_eq!(event.confidence, 0.9);
    assert!(event.is_multilingual());
    assert_eq!(event.alignment_score, Some(0.88));
    assert_eq!(event.translation_quality, Some(0.90));
    assert_eq!(event.semantic_similarity, Some(0.85));
    assert_eq!(event.cultural_context_score, Some(0.87));
}

#[test]
fn test_language_metadata() {
    let mut metadata = LanguageMetadata::new("id", "Halo dunia", "Latin", "Austronesian");
    metadata.set_formality("informal");
    metadata.add_domain_term("greeting");
    metadata.add_domain_term("world");
    
    assert_eq!(metadata.language_code, "id");
    assert_eq!(metadata.script, "Latin");
    assert_eq!(metadata.language_family, "Austronesian");
    assert_eq!(metadata.formality, "informal");
    assert_eq!(metadata.domain_terms.len(), 2);
}

#[test]
fn test_serendipity_stages() {
    let mut trace = SerendipityTrace::new("researcher", "backend", "Discovery");
    
    // Test all stages
    let stages = vec![
        SerendipityStage::Exploration,
        SerendipityStage::UnexpectedConnection,
        SerendipityStage::HypothesisFormation,
        SerendipityStage::Validation,
        SerendipityStage::Integration,
        SerendipityStage::Publication,
    ];
    
    for (i, stage) in stages.iter().enumerate() {
        trace.log_event(
            stage.clone(),
            SerendipityAgent::Explorer,
            &format!("input{}", i),
            &format!("output{}", i),
            "en",
            0.8,
            0.85,
        );
    }
    
    assert_eq!(trace.events.len(), 6);
    assert_eq!(trace.transitions.len(), 5);
}

#[test]
fn test_ranking_criteria() {
    let mut stats = LanguageAwareContributorStats::new("researcher");
    stats.add_trace(25, 0.9, 0.95, vec!["en".to_string(), "id".to_string(), "es".to_string()], 0.92, 0.93);
    stats.add_discovery("Discovery1");
    stats.add_discovery("Discovery2");
    
    let mut leaderboard = LanguageAwareLeaderboard::new();
    leaderboard.add_contributor(stats);
    
    // Test all ranking criteria
    let criteria = vec![
        LanguageAwareRankingCriteria::Overall,
        LanguageAwareRankingCriteria::Serendipity,
        LanguageAwareRankingCriteria::CrossLanguageExpertise,
        LanguageAwareRankingCriteria::Discoveries,
        LanguageAwareRankingCriteria::TranslationQuality,
        LanguageAwareRankingCriteria::LanguageDiversity,
    ];
    
    for criterion in criteria {
        let top = leaderboard.get_top_n(1, criterion);
        assert_eq!(top.len(), 1);
    }
}
