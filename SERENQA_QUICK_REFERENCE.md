# SerenQA Framework - Quick Reference

## Installation

```bash
cd quantum_integration/quantum-limit-graph-v2.4.0/rust/level5_ai_scientist
cargo build --release
```

## Quick Start

### 1. Create Serendipity Trace

```rust
use level5_ai_scientist::serendipity_trace::*;

let mut trace = SerendipityTrace::new("researcher_id", "backend", "Discovery");

trace.log_event(
    SerendipityStage::UnexpectedConnection,
    SerendipityAgent::PatternRecognizer,
    "input",
    "output",
    "en",
    0.92, // serendipity score
    0.85, // confidence
);
```

### 2. Language-Aware Events

```rust
use level5_ai_scientist::AgentEvent::*;

let event = LanguageAwareEventBuilder::new("Translator", "Hello", "Halo", "en")
    .confidence(0.9)
    .add_language("id")
    .alignment_score(0.88)
    .translation_quality(0.90)
    .build();
```

### 3. Multilingual Alignment

```rust
use level5_ai_scientist::alignment::MultilingualAligner;

let mut aligner = MultilingualAligner::new();
let result = aligner.align("Hello world", "Halo dunia", "en", "id");

println!("Alignment: {:.3}", result.overall_score);
```

### 4. Memory Folding

```rust
use level5_ai_scientist::fold_multilingual_memory::MultilingualMemoryFolder;

let mut folder = MultilingualMemoryFolder::new();
let fold = folder.fold_memory("trace_id", &events);

println!("Compression: {:.1}%", fold.compression_ratio * 100.0);
```

### 5. Contributor Stats

```rust
use level5_ai_scientist::ContributorStats::*;

let mut stats = LanguageAwareContributorStats::new("researcher");
stats.add_trace(20, 0.85, 0.9, vec!["en".to_string(), "id".to_string()], 0.88, 0.90);
stats.add_discovery("Journavx");

println!("Score: {:.3}", stats.overall_score());
```

### 6. Leaderboard

```rust
use level5_ai_scientist::ContributorStats::*;

let mut leaderboard = LanguageAwareLeaderboard::new();
leaderboard.add_contributor(stats);
leaderboard.display(LanguageAwareRankingCriteria::Overall);
```

## Run Journavx Demo

```bash
cargo run --example serenqa_journavx_demo
```

## Run Tests

```bash
cargo test test_serenqa_integration -- --nocapture
```

## Key Enums

### SerendipityStage
- `Exploration`
- `UnexpectedConnection`
- `HypothesisFormation`
- `Validation`
- `Integration`
- `Publication`

### SerendipityAgent
- `Explorer`
- `PatternRecognizer`
- `HypothesisGenerator`
- `Validator`
- `Synthesizer`
- `Translator`
- `MetaOrchestrator`

### LanguageAwareRankingCriteria
- `Overall`
- `Serendipity`
- `CrossLanguageExpertise`
- `Discoveries`
- `TranslationQuality`
- `LanguageDiversity`

## Scoring Guide

### Serendipity Score
- 0.0-0.6: Expected research
- 0.6-0.8: Interesting finding
- 0.8-0.9: Serendipitous discovery
- 0.9-1.0: Breakthrough innovation

### Alignment Score
- 0.0-0.5: Poor alignment
- 0.5-0.7: Acceptable
- 0.7-0.9: Good
- 0.9-1.0: Excellent

### Cross-Language Expertise
- 0.0-0.3: Monolingual
- 0.3-0.6: Bilingual
- 0.6-0.8: Multilingual
- 0.8-1.0: Polyglot expert

## Common Patterns

### Pattern 1: Bilingual Discovery

```rust
let mut trace = SerendipityTrace::new("researcher", "backend", "Discovery");

// English exploration
trace.log_event(SerendipityStage::Exploration, SerendipityAgent::Explorer,
    "Research topic", "Found pattern", "en", 0.7, 0.85);

// Indonesian unexpected connection
trace.log_event(SerendipityStage::UnexpectedConnection, SerendipityAgent::PatternRecognizer,
    "Analisis pola", "Koneksi tak terduga", "id", 0.92, 0.88);

// Back to English for synthesis
trace.log_event(SerendipityStage::HypothesisFormation, SerendipityAgent::HypothesisGenerator,
    "Synthesize", "Novel hypothesis", "en", 0.95, 0.90);
```

### Pattern 2: Complete Analysis

```rust
// 1. Create trace
let trace = simulate_journavx_discovery();

// 2. Compute provenance
let hash = trace.compute_provenance_hash();

// 3. Fold memory
let folded = trace.fold_memory();

// 4. Update contributor stats
let mut stats = LanguageAwareContributorStats::new(&trace.contributor_id);
stats.add_trace(
    trace.depth(),
    trace.uniqueness_score(),
    trace.overall_serendipity,
    trace.languages.clone(),
    0.88,
    0.90,
);

// 5. Add to leaderboard
let mut leaderboard = LanguageAwareLeaderboard::new();
leaderboard.add_contributor(stats);
leaderboard.display(LanguageAwareRankingCriteria::Overall);
```

### Pattern 3: Event Builder

```rust
let event = LanguageAwareEventBuilder::new("Translator", "input", "output", "en")
    .confidence(0.9)
    .add_language("id")
    .alignment_score(0.88)
    .translation_quality(0.90)
    .semantic_similarity(0.85)
    .cultural_context(0.87)
    .build();
```

## API Reference

### SerendipityTrace
- `new(contributor_id, backend, discovery_name)` - Create trace
- `log_event(stage, agent, input, output, language, serendipity, confidence)` - Log event
- `compute_provenance_hash()` - Get SHA-256 hash
- `fold_memory()` - Compress trace
- `depth()` - Get event count
- `uniqueness_score()` - Get uniqueness

### LanguageAwareAgentEvent
- `new(agent_type, input, output, language, confidence)` - Create event
- `add_secondary_language(language)` - Add language
- `set_alignment_score(score)` - Set alignment
- `set_translation_quality(quality)` - Set quality
- `is_multilingual()` - Check if multilingual
- `language_quality_score()` - Get quality score

### MultilingualAligner
- `new()` - Create aligner
- `align(source, target, source_lang, target_lang)` - Compute alignment
- `get_average_alignment(source_lang, target_lang)` - Get average
- `get_statistics()` - Get stats

### MultilingualMemoryFolder
- `new()` - Create folder
- `fold_memory(trace_id, events)` - Fold memory

### LanguageAwareContributorStats
- `new(contributor_id)` - Create stats
- `add_trace(depth, uniqueness, serendipity, languages, alignment, translation)` - Add trace
- `add_discovery(name)` - Add discovery
- `add_expertise_domain(domain)` - Add domain
- `overall_score()` - Get score

### LanguageAwareLeaderboard
- `new()` - Create leaderboard
- `add_contributor(stats)` - Add contributor
- `get_top_n(n, criteria)` - Get top N
- `display(criteria)` - Display leaderboard

## Troubleshooting

### Issue: Events not tracked
**Solution**: Ensure `log_event()` is called for each step

### Issue: Low alignment scores
**Solution**: Check language pair compatibility, consider using embeddings

### Issue: Memory folding produces low compression
**Solution**: Increase serendipity threshold for key insights

### Issue: Leaderboard ranking unexpected
**Solution**: Verify ranking criteria, check contributor stats

## Documentation

- **Full Guide**: `SERENQA_INTEGRATION_GUIDE.md`
- **Delivery Summary**: `SERENQA_FRAMEWORK_DELIVERY.md`
- **Level 5 Docs**: `LEVEL_5_COMPLETE.md`

## Support

- GitHub Issues
- Documentation files
- Test examples

---

**Version**: 2.4.0  
**Last Updated**: 2025-11-18
